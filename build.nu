use std log

# Fetch a URL with automatic retries on failure.
def http-get-with-retry [ # nu-lint-ignore: missing_output_type
    url: string
    --max-retries (-r): int = 3  # number of attempts before giving up
    --timeout (-t): duration = 10sec  # per-attempt timeout
] {
    mut last_err = null
    for attempt in 1..$max_retries {
        let err = try {
            return (http get --max-time $timeout $url)
            null
        } catch {|e| $e }

        $last_err = $err
        if $attempt < $max_retries {
            sleep (($attempt * 200) * 1ms)
        }
    }
    error make {
        msg: $"Failed to fetch ($url) after ($max_retries) attempts"
        help: ($last_err | get msg? | default "unknown error")
    }
}

let features = [
    default
    lite
    all-decoders
]

# Return the Rust target triple for the current platform.
# Errors on unsupported platforms (caller should use try/catch).
def detect_target []: nothing -> string {
    let os = $nu.os-info.name
    let arch = $nu.os-info.arch

    match [$os $arch] {
        ["linux", "x86_64"] => "x86_64-unknown-linux-gnu",
        ["linux", "aarch64"] => "aarch64-unknown-linux-gnu",
        ["macos", "x86_64"] => "x86_64-apple-darwin",
        ["macos", "aarch64"] => "aarch64-apple-darwin",
        ["windows", "x86_64"] => "x86_64-pc-windows-msvc",
        _ => { error make {msg: $"unsupported platform: ($os)/($arch)"} }
    }
}

# Download a release archive, verify its checksum (or GPG signature), extract
# it, and copy the named binary into <install-root>/bin/.  Returns true on
# success, false on any recoverable failure.
def download_and_install [
    url: string       # URL of the release archive
    name: string      # binary name to locate inside the extracted archive
    --filename (-f): path     # archive filename (derived from URL basename if omitted)
    --install-root (-i): path # destination root; defaults to $env.NUPM_HOME/plugins
    --checksum-url (-c): string        # URL of a .sha256 file to verify against
    --expected-checksum (-e): string   # inline expected hash (sha256 or md5)
    --signature-path (-s): path        # path to a detached GPG signature
]: nothing -> bool {
    let filename = if ($filename == null) { $url | path basename } else { $filename }
    let install_root = if ($install_root == null) { $env.NUPM_HOME | path join "plugins" } else { $install_root }

    log info $"downloading prebuilt binary..."
    let tmp_dir = (mktemp --directory)
    let archive_path = ($tmp_dir | path join $filename)

    try {
        http-get-with-retry $url --timeout 30sec | save $archive_path
    } catch {
        log warning "failed to download artifact"
        return false
    }

    let gpg_available = (which gpg | is-not-empty)

    if ($signature_path != null) and $gpg_available {
        # Use | complete so we get the exit code without pipefail aborting the script.
        let result = (gpg --verify $signature_path $archive_path | complete)
        if $result.exit_code != 0 {
            log warning "gpg signature verification failed"
            try { rm --recursive --force $tmp_dir }
            return false
        }
    } else {
        let expected = if ($expected_checksum != null) {
            $expected_checksum
        } else if ($checksum_url != null) {
            try { http-get-with-retry $checksum_url | str trim } catch {
                log warning $"failed to fetch checksum from ($checksum_url)"
                try { rm --recursive --force $tmp_dir }
                return false
            }
        } else {
            null
        }

        if ($expected != null) {
            let is_md5 = ($expected | str length) == 32
            let actual = try {
                if $is_md5 {
                    open $archive_path | hash md5
                } else {
                    open $archive_path | hash sha256
                }
            } catch {
                log warning "failed to read archive for checksum"
                try { rm --recursive --force $tmp_dir }
                return false
            }

            # sha256sum files are "<hash>  <filename>" — extract just the hash
            let actual_hash = ($actual | str trim)
            let expected_hash = ($expected | str trim | split words | first)

            if $actual_hash != $expected_hash {
                log warning $"checksum mismatch: expected ($expected_hash), got ($actual_hash)"
                try { rm --recursive --force $tmp_dir }
                return false
            }
        }
    }

    log info extracting...
    let extract_ok: bool = try {
        if ($filename | str ends-with .zip) {
            if ($nu.os-info.name == windows) {
                (powershell -c $"Expand-Archive -Path '($archive_path)' -DestinationPath '($tmp_dir)' -Force" | complete).exit_code == 0
            } else {
                (unzip -o $archive_path -d $tmp_dir | complete).exit_code == 0
            }
        } else if ($filename | str ends-with .tar.xz) {
            # Use tar's built-in xz support (--xz / -J) to avoid losing the
            # intermediate exit code that the old `^xz | ^tar` pipeline silently swallowed.
            (tar --extract --xz --file $archive_path --directory $tmp_dir | complete).exit_code == 0
        } else {
            (tar --extract --file $archive_path --directory $tmp_dir | complete).exit_code == 0
        }
    } catch {
        false
    }

    if not $extract_ok {
        log warning "failed to extract artifact"
        try { rm --recursive --force $tmp_dir }
        return false
    }

    let bin_name = if $nu.os-info.name == windows { $"($name).exe" } else { $name }
    let found = (glob ($tmp_dir | path join "**" $bin_name))

    if ($found | is-empty) {
        log warning $"binary ($bin_name) not found in archive"
        try { rm --recursive --force $tmp_dir }
        return false
    }

    let extracted_bin = ($found | first)
    let bin_dir = ($install_root | path join bin)
    try { mkdir $bin_dir }
    let dest_path = ($bin_dir | path join $bin_name)

    try {
        cp --force $extracted_bin $dest_path
    } catch {
        log warning $"failed to copy binary to ($dest_path)"
        try { rm --recursive --force $tmp_dir }
        return false
    }
    try { rm --recursive --force $tmp_dir }

    log info $"installed prebuilt binary to ($dest_path)"
    true
}

# HEAD-check a URL then delegate to download_and_install.
# Returns false immediately if the release asset does not exist.
def check_and_download_prebuilt [
    url: string
    name: string
    --filename (-f): path
    --install-root (-i): path
    --checksum-url (-c): string
]: nothing -> bool {
    try {
        http head $url

        # Conditionally pass optional flags for all combinations
        if ($filename != null) and ($install_root != null) and ($checksum_url != null) {
            download_and_install $url $name --filename $filename --install-root $install_root --checksum-url $checksum_url
        } else if ($filename != null) and ($install_root != null) {
            download_and_install $url $name --filename $filename --install-root $install_root
        } else if ($filename != null) and ($checksum_url != null) {
            download_and_install $url $name --filename $filename --checksum-url $checksum_url
        } else if ($install_root != null) and ($checksum_url != null) {
            download_and_install $url $name --install-root $install_root --checksum-url $checksum_url
        } else if ($filename != null) {
            download_and_install $url $name --filename $filename
        } else if ($install_root != null) {
            download_and_install $url $name --install-root $install_root
        } else if ($checksum_url != null) {
            download_and_install $url $name --checksum-url $checksum_url
        } else {
            download_and_install $url $name
        }
    } catch {
        log warning "prebuilt binary not found on GitHub releases, falling back to source build"
        false
    }
}

# Try to install a prebuilt release binary for the current platform.
# Returns true when a binary was successfully installed, false otherwise.
def install_prebuilt [
    name: string
    version: string
    --install-root (-i): path  # defaults to $env.NUPM_HOME/plugins
]: nothing -> bool {
    let install_root = if ($install_root == null) { $env.NUPM_HOME | path join plugins } else { $install_root }

    # detect_target errors on unknown platforms; treat that as "no prebuilt available".
    let target = try { detect_target } catch {
        log warning "prebuilt binary not available for this platform, falling back to source build"
        return false
    }

    let ext = if $nu.os-info.name == windows { "zip" } else { "tar.gz" }
    let filename = $"($name)-v($version)-($target).($ext)"
    let url = $"https://github.com/SuaveIV/($name)/releases/download/v($version)/($filename)"
    let checksum_url = $"($url).sha256"

    log info $"checking for prebuilt binary at ($url)"

    if (check_and_download_prebuilt $url $name --filename $filename --install-root $install_root --checksum-url $checksum_url) {
        return true
    }

    # Try .tar.xz as a fallback if .tar.gz was not found.
    if $ext == tar.gz {
        let filename_xz = $"($name)-v($version)-($target).tar.xz"
        let url_xz = $"https://github.com/SuaveIV/($name)/releases/download/v($version)/($filename_xz)"
        let checksum_url_xz = $"($url_xz).sha256"
        log info $"checking for prebuilt binary at ($url_xz)"

        if (check_and_download_prebuilt $url_xz $name --filename $filename_xz --install-root $install_root --checksum-url $checksum_url_xz) {
            return true
        }
    }

    false
}

def main [package_file: path = nupm.nuon]: nothing -> nothing {
    let repo_root = try {
        ls --full-paths $package_file | first | get name | path dirname
    } catch {
        error make {msg: $"Cannot open package file: ($package_file)"}
    }
    let install_root = $env.NUPM_HOME | path join plugins

    let cargo = try {
        open ($repo_root | path join Cargo.toml)
    } catch {
        error make {msg: "Cannot open Cargo.toml"}
    }
    let name = $cargo.package.name
    let version = $cargo.package.version

    if (install_prebuilt $name $version --install-root $install_root) {
        let ext: string = if ($nu.os-info.name == windows) { ".exe" } else { "" }
        plugin add $"($install_root | path join bin $name)($ext)"
        log info "do not forget to restart Nushell for the plugin to be fully available!"
        return
    }

    let selected_features = $features | input list --multi "select cargo features"
    let feature_list = ($selected_features | str join ,)
    log info $"building plugin using: (ansi blue)cargo install --path ($repo_root) --root ($install_root) --features=($feature_list)(ansi reset)"
    cargo install --path $repo_root --root $install_root $"--features=($feature_list)"
    let ext: string = if ($nu.os-info.name == windows) { ".exe" } else { "" }
    plugin add $"($install_root | path join bin $name)($ext)"
    log info "do not forget to restart Nushell for the plugin to be fully available!"
}

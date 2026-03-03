use std log
let features = [
    flac
    minimp3
    symphonia-aac
    symphonia-flac
    symphonia-isomp4
    symphonia-mp3
    symphonia-vorbis
    symphonia-wav
    vorbis
    wav
]

def detect_target [] {
    let os = $nu.os-info.name
    let arch = $nu.os-info.arch

    match [$os, $arch] {
        ["linux", "x86_64"] => "x86_64-unknown-linux-gnu",
        ["linux", "aarch64"] => "aarch64-unknown-linux-gnu",
        ["macos", "x86_64"] => "x86_64-apple-darwin",
        ["macos", "aarch64"] => "aarch64-apple-darwin",
        ["windows", "x86_64"] => "x86_64-pc-windows-msvc",
        _ => null
    }
}

def download_and_install [url: string, filename: string, install_root: path, name: string] {
    log info $"downloading prebuilt binary..."
    let tmp_dir = (mktemp -d)
    let archive_path = $tmp_dir | path join $filename

    try {
        http get $url | save $archive_path
    } catch {
        log warning "failed to download artifact"
        return false
    }

    log info "extracting..."
    try {
        if ($filename | str ends-with ".zip") {
            try {
                ^tar -xf $archive_path -C $tmp_dir
            } catch {
                if ($nu.os-info.name == "windows") {
                    ^powershell -c $"Expand-Archive -Path '($archive_path)' -DestinationPath '($tmp_dir)' -Force"
                } else {
                    ^unzip -o $archive_path -d $tmp_dir
                }
            }
        } else {
            ^tar -xf $archive_path -C $tmp_dir
        }
    } catch {
        log warning "failed to extract artifact"
        return false
    }

    let bin_name = if $nu.os-info.name == "windows" { $"($name).exe" } else { $name }
    let found = (glob ($tmp_dir | path join "**" $bin_name))

    if ($found | is-empty) {
        log warning $"binary ($bin_name) not found in archive"
        return false
    }

    let extracted_bin = ($found | first)
    let bin_dir = $install_root | path join "bin"
    mkdir $bin_dir
    let dest_path = $bin_dir | path join $bin_name

    cp -f $extracted_bin $dest_path
    rm -rf $tmp_dir

    log info $"installed prebuilt binary to ($dest_path)"
    return true
}

def install_prebuilt [
    repo_root: path,
    install_root: path,
    name: string,
    version: string
] {
    let target = detect_target
    if $target == null {
        log warning "prebuilt binary not available for this platform, falling back to source build"
        return false
    }

    let ext = if $nu.os-info.name == "windows" { "zip" } else { "tar.gz" }
    let filename = $"($name)-v($version)-($target).($ext)"
    let url = $"https://github.com/SuaveIV/($name)/releases/download/v($version)/($filename)"

    log info $"checking for prebuilt binary at ($url)"

    try {
        http head $url
    } catch {
        if $ext == "tar.gz" {
             let ext_xz = "tar.xz"
             let filename_xz = $"($name)-v($version)-($target).($ext_xz)"
             let url_xz = $"https://github.com/SuaveIV/($name)/releases/download/v($version)/($filename_xz)"
             log info $"checking for prebuilt binary at ($url_xz)"
             try {
                 http head $url_xz
                 return (download_and_install $url_xz $filename_xz $install_root $name)
             } catch {
                 log warning "prebuilt binary not found on GitHub releases, falling back to source build"
                 return false
             }
        }
        log warning "prebuilt binary not found on GitHub releases, falling back to source build"
        return false
    }

    return (download_and_install $url $filename $install_root $name)
}

def main [package_file: path = nupm.nuon] {
    let repo_root = (ls -f $package_file | first | get name | path dirname)
    let install_root = $env.NUPM_HOME | path join "plugins"

    let cargo = open ($repo_root | path join "Cargo.toml")
    let name = $cargo.package.name
    let version = $cargo.package.version

    if (install_prebuilt $repo_root $install_root $name $version) {
        let ext: string = if ($nu.os-info.name == 'windows') { '.exe' } else { '' }
        plugin add $"($install_root | path join "bin" $name)($ext)"
        log info "do not forget to restart Nushell for the plugin to be fully available!"
        return
    }

    let features = $features | input list --multi "select cargo features"

    let cmd = $"cargo install --path '($repo_root)' --root '($install_root)' --features=($features | str join ",")"
    log info $"building plugin using: (ansi blue)($cmd)(ansi reset)"
    nu -c $cmd
    let ext: string = if ($nu.os-info.name == 'windows') { '.exe' } else { '' }
    plugin add $"($install_root | path join "bin" $name)($ext)"
    log info "do not forget to restart Nushell for the plugin to be fully available!"
}

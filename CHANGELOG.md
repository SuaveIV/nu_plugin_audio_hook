# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.2.3 (2026-03-08)

### Chore

 - <csr-id-d448783bd8166c11c09218bf188239aa05cfd0b4/> pin dist action SHAs in dist-workspace.toml
 - <csr-id-87324c1ca5523c3caae9ec143111c631a8cf25b4/> reduce release wait timeout from 30m to 15m
   Increase poll interval from 5s to 10s and reduce iterations from
   360 to 90, halving unnecessary API calls while still providing
   sufficient time for the main dist release to complete.
 - <csr-id-2076bf2c8486ee0512ccdf52aab5511d20444ce5/> pin action SHAs and enable artifact attestations
   Pin all GitHub Actions in release.yml and release-arm64.yml to
   specific commit SHAs to prevent supply-chain attacks via tag mutation.
   Enable github-attestations in dist-workspace.toml to cryptographically
   bind release artifacts to their workflow run.
 - <csr-id-ccd7677f11e2c3e3b748b7e50eb8c98c6ea775fb/> update actions/checkout and action-gh-release versions in workflows
 - <csr-id-1999f45c9b840472674f6032b79fef0fd11c9ad6/> bump dependencies to 0.2.2 and update contributors

### Documentation

 - <csr-id-35391808753eadabcf468e7a94ae3686d1c780b8/> update installation instructions and add target details for prebuilt binaries

### New Features

 - <csr-id-c3680a994cb8948cfd7bbd28b1c8c667338e8922/> add tag format validation and improve checksum command syntax
 - <csr-id-df0b440ab98843c02b2dc236a8fa33f9cb08d9da/> add checksum verification for downloaded binaries
 - <csr-id-639724bd386a7fe20c92774f0944b76f3cba451f/> add checksum generation and include in release upload

### Bug Fixes

 - <csr-id-25785de4250463f915a80d14c67bf61b72deb4d2/> clear frozen header line when terminal narrows below compact tier
 - <csr-id-a94d52afe780b5788bb976498f53b4b06f95ef01/> track header-line reservation separately from first_render
 - <csr-id-9479ea0e9c291c17d279ad53e3d3a58760bd49a2/> continue check even if fmt or clippy fails
 - <csr-id-69e331e56e017276e15b602e7be67b24e858c208/> replace dev recipes with platform-specific install-nx/win variants
 - <csr-id-af43fefe237f5d21f2345b2cad2113e6cedc6677/> guarantee bar_width + vol_bar_width never exceeds available columns
 - <csr-id-02086f255b5afd6df785faf8c46a1772b53e5af1/> wrap plugin add calls with nu -c using portable double quotes
 - <csr-id-fc4cc50ab683b682990ef7208cd955464053e8a1/> wrap plugin add calls with nu -c
 - <csr-id-2f382785c8218bacb0dcbafefb9fe35aab174d8b/> simplify progress rendering logic in wait_with_progress function
 - <csr-id-0969f0af5e2b01c1ad7f3ca312156c9b1f16c1e7/> remove dead assignment, fix bar overflow, and deduplicate marquee gap
   - Remove the dead `first_render = false` after the final render_progress
     call in wait_with_progress — the function returns immediately afterward
     so the assignment never had any effect
   - Fix bar_width + vol_bar_width potentially exceeding available columns
     when the .max(10) clamp pushed bar_width back above the safe limit on
     narrow terminals; constrained is now computed first and .max(10) is
     not applied when available space is too small to honour it
   - Extract the marquee gap string into a MARQUEE_GAP constant so the
     scrolling math in wait_with_progress and the renderer in render_progress
     are guaranteed to use the same value; a mismatch between the two would
     have caused visible scroll drift
 - <csr-id-d17767d9d9f6b1f485ad59fe2edfa6857f460010/> degrade gracefully on narrow terminals and scroll long headers
   - Replace the hard bail-out width guard with four layout tiers (full,
     compact, minimal, bare) so the progress display renders something
     useful at any terminal width rather than silently dropping frames
   - Add marquee scrolling for artist/title headers that are too wide to
     fit on one line; scroll_offset advances only on successful draws and
     freezes when the header fits statically
   - Fix first_render tracking so MoveUp(1) is only emitted after a frame
     was actually drawn, preventing header duplication and output smear on
     narrow terminals
   - Use unicode-width column counting in the marquee slicer instead of
     char counts to handle multibyte and CJK characters correctly
   - Extract WIDTH_FULL, WIDTH_COMPACT, WIDTH_MINIMAL, WIDTH_BARE constants
     to replace the former MIN_RENDER_WIDTH magic number
   - Change render_progress return type from () to bool to communicate
     whether a frame was actually drawn back to the caller
   - Fix dead initial assignments for bar_width and vol_bar_width by
     restructuring the size() block as an expression with an explicit
     fallback, eliminating the unused_assignments warnings
 - <csr-id-ba64249713b6857e7b6ffbb48098cde007d6ede5/> scope permissions to job level and attest before upload
 - <csr-id-ecdc036370dddd81f5d1c4288d3fafc6e6f48958/> add attestations permissions and implement artifact attestation in release workflow
   Fixes SuaveIV/nu_plugin_audio#13
 - <csr-id-24b9e4a2346576ced214ef6d02aa23e719b41a03/> improve regex for updating dependency version format in Cargo.toml
 - <csr-id-8d929dabd270b348149ecae7d99d7ff9a4b9edb2/> update dependency version extraction logic and adjust commit message in PR creation
 - <csr-id-eef7f3f84ac88c53b37db56753a2d76c07f4f15f/> checkout requested tag on dispatch and handle dev-dependency table form
 - <csr-id-e52cc30ec7e468e0532eea5c452f5fbb3bbfc7aa/> fix nushell script bugs and upgrade to 0.111.0
   - Replace `str matches` with `=~` operator (command does not exist)
   - Replace string `+` concatenation with `$"..."` interpolation
   - Fix `| let name: type` pipeline annotation (unsupported form)
   - Fix `.dev-dependencies?` dot access ambiguity with `get "dev-dependencies"?`
   - Replace `$it` in `where` with explicit closure for 0.111 compatibility
   - Prefix external `cargo` calls with `^` to prevent shadowing
   - Open Cargo.toml once with `--raw` and derive parsed form via `from toml`
   - Escape `$dep` before regex interpolation to handle metacharacters
   - Fix `{escaped_dep}` and `{sanitized_plugin_version}` bare braces to `($var)`
   - Wrap `let nu_deps` pipeline in outer parens for correct parse
   - Simplify `validate-version-format` to single `=~` check
   - Remove unreachable `null` after `return` in `http-get-with-retry`

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge pull request #16 from SuaveIV/issue-10-player-bar-fix ([`002ddef`](https://github.com/SuaveIV/nu_plugin_audio/commit/002ddeff64866f58243d62904623937d490a7839))
    - Clear frozen header line when terminal narrows below compact tier ([`25785de`](https://github.com/SuaveIV/nu_plugin_audio/commit/25785de4250463f915a80d14c67bf61b72deb4d2))
    - Track header-line reservation separately from first_render ([`a94d52a`](https://github.com/SuaveIV/nu_plugin_audio/commit/a94d52afe780b5788bb976498f53b4b06f95ef01))
    - Continue check even if fmt or clippy fails ([`9479ea0`](https://github.com/SuaveIV/nu_plugin_audio/commit/9479ea0e9c291c17d279ad53e3d3a58760bd49a2))
    - Replace dev recipes with platform-specific install-nx/win variants ([`69e331e`](https://github.com/SuaveIV/nu_plugin_audio/commit/69e331e56e017276e15b602e7be67b24e858c208))
    - Guarantee bar_width + vol_bar_width never exceeds available columns ([`af43fef`](https://github.com/SuaveIV/nu_plugin_audio/commit/af43fefe237f5d21f2345b2cad2113e6cedc6677))
    - Wrap plugin add calls with nu -c using portable double quotes ([`02086f2`](https://github.com/SuaveIV/nu_plugin_audio/commit/02086f255b5afd6df785faf8c46a1772b53e5af1))
    - Wrap plugin add calls with nu -c ([`fc4cc50`](https://github.com/SuaveIV/nu_plugin_audio/commit/fc4cc50ab683b682990ef7208cd955464053e8a1))
    - Simplify progress rendering logic in wait_with_progress function ([`2f38278`](https://github.com/SuaveIV/nu_plugin_audio/commit/2f382785c8218bacb0dcbafefb9fe35aab174d8b))
    - Remove dead assignment, fix bar overflow, and deduplicate marquee gap ([`0969f0a`](https://github.com/SuaveIV/nu_plugin_audio/commit/0969f0af5e2b01c1ad7f3ca312156c9b1f16c1e7))
    - Degrade gracefully on narrow terminals and scroll long headers ([`d17767d`](https://github.com/SuaveIV/nu_plugin_audio/commit/d17767d9d9f6b1f485ad59fe2edfa6857f460010))
    - Merge pull request #15 from SuaveIV/issue-13-arm64-attest ([`92ce614`](https://github.com/SuaveIV/nu_plugin_audio/commit/92ce614b45ff45456ada533e83975b245a732f15))
    - Scope permissions to job level and attest before upload ([`ba64249`](https://github.com/SuaveIV/nu_plugin_audio/commit/ba64249713b6857e7b6ffbb48098cde007d6ede5))
    - Add attestations permissions and implement artifact attestation in release workflow ([`ecdc036`](https://github.com/SuaveIV/nu_plugin_audio/commit/ecdc036370dddd81f5d1c4288d3fafc6e6f48958))
    - Merge pull request #12 from SuaveIV/security_upgrrade_1 ([`ef955eb`](https://github.com/SuaveIV/nu_plugin_audio/commit/ef955ebd8fc5d87dcfc4adf80dd6bd4b38971b09))
    - Improve regex for updating dependency version format in Cargo.toml ([`24b9e4a`](https://github.com/SuaveIV/nu_plugin_audio/commit/24b9e4a2346576ced214ef6d02aa23e719b41a03))
    - Update dependency version extraction logic and adjust commit message in PR creation ([`8d929da`](https://github.com/SuaveIV/nu_plugin_audio/commit/8d929dabd270b348149ecae7d99d7ff9a4b9edb2))
    - Checkout requested tag on dispatch and handle dev-dependency table form ([`eef7f3f`](https://github.com/SuaveIV/nu_plugin_audio/commit/eef7f3f84ac88c53b37db56753a2d76c07f4f15f))
    - Pin dist action SHAs in dist-workspace.toml ([`d448783`](https://github.com/SuaveIV/nu_plugin_audio/commit/d448783bd8166c11c09218bf188239aa05cfd0b4))
    - Reduce release wait timeout from 30m to 15m ([`87324c1`](https://github.com/SuaveIV/nu_plugin_audio/commit/87324c1ca5523c3caae9ec143111c631a8cf25b4))
    - Pin action SHAs and enable artifact attestations ([`2076bf2`](https://github.com/SuaveIV/nu_plugin_audio/commit/2076bf2c8486ee0512ccdf52aab5511d20444ce5))
    - Fix nushell script bugs and upgrade to 0.111.0 ([`e52cc30`](https://github.com/SuaveIV/nu_plugin_audio/commit/e52cc30ec7e468e0532eea5c452f5fbb3bbfc7aa))
    - Add tag format validation and improve checksum command syntax ([`c3680a9`](https://github.com/SuaveIV/nu_plugin_audio/commit/c3680a994cb8948cfd7bbd28b1c8c667338e8922))
    - Update actions/checkout and action-gh-release versions in workflows ([`ccd7677`](https://github.com/SuaveIV/nu_plugin_audio/commit/ccd7677f11e2c3e3b748b7e50eb8c98c6ea775fb))
    - Merge pull request #11 from SuaveIV/chore/update-dependencies ([`67d2255`](https://github.com/SuaveIV/nu_plugin_audio/commit/67d2255cc674339dd5caef7b34a278e9b6700485))
    - Bump dependencies to 0.2.2 and update contributors ([`1999f45`](https://github.com/SuaveIV/nu_plugin_audio/commit/1999f45c9b840472674f6032b79fef0fd11c9ad6))
    - Update installation instructions and add target details for prebuilt binaries ([`3539180`](https://github.com/SuaveIV/nu_plugin_audio/commit/35391808753eadabcf468e7a94ae3686d1c780b8))
    - Add checksum verification for downloaded binaries ([`df0b440`](https://github.com/SuaveIV/nu_plugin_audio/commit/df0b440ab98843c02b2dc236a8fa33f9cb08d9da))
    - Add checksum generation and include in release upload ([`639724b`](https://github.com/SuaveIV/nu_plugin_audio/commit/639724bd386a7fe20c92774f0944b76f3cba451f))
</details>

## v0.2.2 (2026-03-05)

<csr-id-3809e33e90edbc4ec545e883f7cb7ee86cef16e9/>

### Chore

 - <csr-id-3809e33e90edbc4ec545e883f7cb7ee86cef16e9/> update copyright years in LICENSE file

### New Features

 - <csr-id-37c0f45e56c7671fffc094ddbf1bc599c381ea8a/> add Justfile for common tasks and contributor guidance

### Bug Fixes

 - <csr-id-15632ccaad3ccf6dfd0562596c2006c0a2bf2ef6/> suppress header render when terminal too narrow to fit on one line
   MoveUp(1) assumes the header occupies exactly one physical row. When the
   terminal is narrower than the header string, the header wraps and the
   cursor drifts up by one row short each cycle, leaving ghost copies behind
   on every frame.
   
   Extend the MIN_RENDER_WIDTH bail-out to also return early when a header
   is present and the terminal width is less than the header display width.
 - <csr-id-e487a117c0e3b2755c975113bf835cf638a8bc5d/> cast sample to f64 before multiply to support 64bit feature
 - <csr-id-f4850e2cb8c511137df1ca4cdb652b3156469443/> ensure credentials are not persisted during checkout in dependency update workflow
 - <csr-id-1c0ce8935d3f2a83a612e375e6bdcea09d2384e8/> enable ARM64 build job in GitHub Actions workflow
 - <csr-id-17810c54a98aa05c37cc3d9283cdd0c66f23a513/> improve wait message and check for published release

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Improve wait message and check for published release ([`17810c5`](https://github.com/SuaveIV/nu_plugin_audio/commit/17810c54a98aa05c37cc3d9283cdd0c66f23a513))
    - Release nu_plugin_audio v0.2.2 ([`438b17a`](https://github.com/SuaveIV/nu_plugin_audio/commit/438b17a0834562c2432b2e2106c4b34d9da4fac8))
    - Suppress header render when terminal too narrow to fit on one line ([`15632cc`](https://github.com/SuaveIV/nu_plugin_audio/commit/15632ccaad3ccf6dfd0562596c2006c0a2bf2ef6))
    - Cast sample to f64 before multiply to support 64bit feature ([`e487a11`](https://github.com/SuaveIV/nu_plugin_audio/commit/e487a117c0e3b2755c975113bf835cf638a8bc5d))
    - Ensure credentials are not persisted during checkout in dependency update workflow ([`f4850e2`](https://github.com/SuaveIV/nu_plugin_audio/commit/f4850e2cb8c511137df1ca4cdb652b3156469443))
    - Enable ARM64 build job in GitHub Actions workflow ([`1c0ce89`](https://github.com/SuaveIV/nu_plugin_audio/commit/1c0ce8935d3f2a83a612e375e6bdcea09d2384e8))
    - Update copyright years in LICENSE file ([`3809e33`](https://github.com/SuaveIV/nu_plugin_audio/commit/3809e33e90edbc4ec545e883f7cb7ee86cef16e9))
    - Add Justfile for common tasks and contributor guidance ([`37c0f45`](https://github.com/SuaveIV/nu_plugin_audio/commit/37c0f45e56c7671fffc094ddbf1bc599c381ea8a))
</details>

## v0.2.1 (2026-03-04)

<csr-id-a2eda3f6b11eaefc0235efc777f8531b34f334c0/>
<csr-id-8f3c1ec53d218acada8a05945d199bab30793c0c/>

### Documentation

 - <csr-id-ccdb7d2d0e87bfe10420b5e3fff68f52d378fdc9/> add CONTRIBUTING.md to guide new contributors

### New Features

 - <csr-id-1b6fcda80fa803fbe4d9263400c71337e6ebfcc7/> bump version to 0.2.1
 - <csr-id-a1e095631655da6915593f1bc064432cc0d01d34/> update README and metadata for version 0.2.0; improve code formatting and error handling
 - <csr-id-31dc072bafe4c903487d5b2f48525910c73e2ddf/> restore pre-build steps for aarch64 target in Cross.toml
 - <csr-id-15a1da7df713bf2f8267a7517f18840f9b74834d/> add pre-build steps for aarch64 target in Cross.tonl

### Refactor

 - <csr-id-a2eda3f6b11eaefc0235efc777f8531b34f334c0/> simplify return statements and improve code readability in SoundBeepCmd and generate_wav functions

### Bug Fixes

 - <csr-id-c1628912cd9fee2c43bd546b67afd7b6447a1dcd/> disable build-arm64 job in release workflow
 - <csr-id-54f309d802c84c6f28ff4874a36305ddd64b2091/> update cross installation command to remove version specification
 - <csr-id-8d300a0ebcb3a5b6db189a749cbe3048dbd6106c/> update release workflow to use input tag for packaging and uploading

### Chore

 - <csr-id-8f3c1ec53d218acada8a05945d199bab30793c0c/> remove GitHub Actions workflow for tagging on merge

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Remove GitHub Actions workflow for tagging on merge ([`8f3c1ec`](https://github.com/SuaveIV/nu_plugin_audio/commit/8f3c1ec53d218acada8a05945d199bab30793c0c))
    - Release nu_plugin_audio v0.2.1 ([`3b1d2ba`](https://github.com/SuaveIV/nu_plugin_audio/commit/3b1d2bad6a7f5d69853b5b916c5bcba779d8c67c))
    - Bump version to 0.2.1 ([`1b6fcda`](https://github.com/SuaveIV/nu_plugin_audio/commit/1b6fcda80fa803fbe4d9263400c71337e6ebfcc7))
    - Simplify return statements and improve code readability in SoundBeepCmd and generate_wav functions ([`a2eda3f`](https://github.com/SuaveIV/nu_plugin_audio/commit/a2eda3f6b11eaefc0235efc777f8531b34f334c0))
    - Add CONTRIBUTING.md to guide new contributors ([`ccdb7d2`](https://github.com/SuaveIV/nu_plugin_audio/commit/ccdb7d2d0e87bfe10420b5e3fff68f52d378fdc9))
    - Update README and metadata for version 0.2.0; improve code formatting and error handling ([`a1e0956`](https://github.com/SuaveIV/nu_plugin_audio/commit/a1e095631655da6915593f1bc064432cc0d01d34))
    - Release nu_plugin_audio v0.2.0 ([`50e1c47`](https://github.com/SuaveIV/nu_plugin_audio/commit/50e1c47930835cd9b5c9ec7464e5cf299f1a0794))
    - Restore pre-build steps for aarch64 target in Cross.toml ([`31dc072`](https://github.com/SuaveIV/nu_plugin_audio/commit/31dc072bafe4c903487d5b2f48525910c73e2ddf))
    - Disable build-arm64 job in release workflow ([`c162891`](https://github.com/SuaveIV/nu_plugin_audio/commit/c1628912cd9fee2c43bd546b67afd7b6447a1dcd))
    - Add pre-build steps for aarch64 target in Cross.tonl ([`15a1da7`](https://github.com/SuaveIV/nu_plugin_audio/commit/15a1da7df713bf2f8267a7517f18840f9b74834d))
    - Update cross installation command to remove version specification ([`54f309d`](https://github.com/SuaveIV/nu_plugin_audio/commit/54f309d802c84c6f28ff4874a36305ddd64b2091))
    - Update release workflow to use input tag for packaging and uploading ([`8d300a0`](https://github.com/SuaveIV/nu_plugin_audio/commit/8d300a0ebcb3a5b6db189a749cbe3048dbd6106c))
</details>

## v0.2.0 (2026-03-03)

### New Features

<csr-id-31dc072bafe4c903487d5b2f48525910c73e2ddf/>
<csr-id-15a1da7df713bf2f8267a7517f18840f9b74834d/>

 - <csr-id-46cfd591cc1bb53ecf602fc36a8c02695cfdc714/> enhance audio playback support and improve WAV generation
 - <csr-id-c79d8d369c286a89a3f52177c622f43e1cbcd91e/> update dependencies and enhance audio handling
   - Updated `rodio` dependency version from 0.21.1 to 0.22.1 in Cargo.toml.

### Bug Fixes

 - <csr-id-82d461e5d751b16791c2536d6ffb382dbbcbf872/> adjust release workflow to handle pull requests and improve tag handling
 - <csr-id-c1628912cd9fee2c43bd546b67afd7b6447a1dcd/> disable build-arm64 job in release workflow
 - <csr-id-54f309d802c84c6f28ff4874a36305ddd64b2091/> update cross installation command to remove version specification
 - <csr-id-8d300a0ebcb3a5b6db189a749cbe3048dbd6106c/> update release workflow to use input tag for packaging and uploading

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Adjust release workflow to handle pull requests and improve tag handling ([`82d461e`](https://github.com/SuaveIV/nu_plugin_audio/commit/82d461e5d751b16791c2536d6ffb382dbbcbf872))
    - Merge pull request #7 from SuaveIV/rodio_update ([`51f8cd5`](https://github.com/SuaveIV/nu_plugin_audio/commit/51f8cd5d64e91785f8a10f18e019aa4062fca586))
    - Enhance audio playback support and improve WAV generation ([`46cfd59`](https://github.com/SuaveIV/nu_plugin_audio/commit/46cfd591cc1bb53ecf602fc36a8c02695cfdc714))
    - Update dependencies and enhance audio handling ([`c79d8d3`](https://github.com/SuaveIV/nu_plugin_audio/commit/c79d8d369c286a89a3f52177c622f43e1cbcd91e))
</details>

## v0.1.0 (2026-03-03)

<csr-id-43528fc4d388c45aebd99bf178f7251085f1dabe/>

### Chore

 - <csr-id-43528fc4d388c45aebd99bf178f7251085f1dabe/> update version to 0.1.0 across all relevant files

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge pull request #6 from SuaveIV/version_decouple ([`22b2edf`](https://github.com/SuaveIV/nu_plugin_audio/commit/22b2edff167b9b35123e43b10158f2f7057f25e8))
    - Update version to 0.1.0 across all relevant files ([`43528fc`](https://github.com/SuaveIV/nu_plugin_audio/commit/43528fc4d388c45aebd99bf178f7251085f1dabe))
</details>

## v0.111.0-test.1 (2026-03-03)

<csr-id-7f93b35850a9d3ff5a6863e07fba39a1a5513a1d/>
<csr-id-c1c286ac414b18310e699856e77f9b021989c6bb/>
<csr-id-8301c35d75314be1c3159c436c3c285a81cad1d4/>
<csr-id-4ef01757bb9ede3c70429653c86c76513c894508/>

### Documentation

 - <csr-id-ad7d2ca9314bc7675f18df2cd75846e0f40410f7/> update installation instructions
   Rewrites the Installation section in `README.md` to prioritize `nupm` (now using prebuilts) and add instructions for shell installers, `cargo binstall`, and manual downloads. Moves manual compilation to a fallback section.

### New Features

 - <csr-id-f40c6355d6b96ff01b7eaff18e1e60cbd391d6a0/> enhance download_and_install function with checksum verification and GPG signature support
 - <csr-id-e6228d9d0f13ffbe502160c937670d62a084431c/> add allow-dirty configuration for CI
 - <csr-id-f219aee3af53d3953b0d5bb10f785172857c7e54/> update workflows to use actions/checkout@v6 and actions/download-artifact@v8; add tag-on-merge workflow
 - <csr-id-c2ca3dea85ecabc97d991b42e421bca0ac82edec/> add ARM64 release workflow for cross-compilation and packaging
 - <csr-id-d802b04fa7ea336b75ad3395c1728600b56bba93/> support prebuilt binary installation
   Updates `build.nu` to detect the current platform and attempt to download a prebuilt binary from GitHub Releases. It falls back to compiling from source if the binary is unavailable or the download fails. This significantly speeds up installation for users on supported platforms.

### Bug Fixes

 - <csr-id-19ea06eafa3ac3551d23f4a2b53f38c1c767c80d/> downgrade actions versions for compatibility and remove allow-dirty configuration
 - <csr-id-ebfbfddff78e7aa8daa790550a8ae311ebed57b5/> rename plugin references from nu_plugin_audio_hook to nu_plugin_audio
 - <csr-id-ad5d2a0378e47fd3ff47493aff44533816d71fef/> update README to reflect project renaming and improve clarity

### Other

 - <csr-id-7f93b35850a9d3ff5a6863e07fba39a1a5513a1d/> add installation hint for plugin usage
 - <csr-id-c1c286ac414b18310e699856e77f9b021989c6bb/> delegate release creation to cargo-dist
   Removes the explicit "Create GitHub Release" step from the dependency update workflow. Instead, it now pushes a tag which triggers the `cargo-dist` release workflow. This prevents race conditions where two workflows might attempt to create the same release.
 - <csr-id-8301c35d75314be1c3159c436c3c285a81cad1d4/> add arm64 linux cross-compilation job
   Adds a manual `build-arm64` job to the release workflow. `cargo-dist` does not currently support cross-compiling with system dependencies (ALSA) easily via `cargo-zigbuild` without a custom sysroot. This job uses `cross` to build for `aarch64-unknown-linux-gnu` and uploads the artifact to the release created by `cargo-dist`.
 - <csr-id-4ef01757bb9ede3c70429653c86c76513c894508/> add linux system dependencies
   Adds `libasound2-dev` and `pkg-config` to the `[dist.dependencies.apt]` section in `dist-workspace.toml`. This ensures that the generated GitHub Actions workflow installs the necessary ALSA development headers before building on Linux runners.

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge pull request #5 from SuaveIV/binary_dist ([`1ce8350`](https://github.com/SuaveIV/nu_plugin_audio/commit/1ce8350d6293c6b90707692d66774425ea640f36))
    - Enhance download_and_install function with checksum verification and GPG signature support ([`f40c635`](https://github.com/SuaveIV/nu_plugin_audio/commit/f40c6355d6b96ff01b7eaff18e1e60cbd391d6a0))
    - Downgrade actions versions for compatibility and remove allow-dirty configuration ([`19ea06e`](https://github.com/SuaveIV/nu_plugin_audio/commit/19ea06eafa3ac3551d23f4a2b53f38c1c767c80d))
    - Add allow-dirty configuration for CI ([`e6228d9`](https://github.com/SuaveIV/nu_plugin_audio/commit/e6228d9d0f13ffbe502160c937670d62a084431c))
    - Update workflows to use actions/checkout@v6 and actions/download-artifact@v8; add tag-on-merge workflow ([`f219aee`](https://github.com/SuaveIV/nu_plugin_audio/commit/f219aee3af53d3953b0d5bb10f785172857c7e54))
    - Feat(ci): enhance ARM64 release workflow with retry logic and version locking for dependencies fix: update artifact upload actions to latest versions docs: improve installation instructions for clarity and platform specifics refactor: implement retry mechanism for HTTP requests in build script ([`c0932d6`](https://github.com/SuaveIV/nu_plugin_audio/commit/c0932d6e91722cec95bc58c7dd5effaeaec78a0a))
    - Add ARM64 release workflow for cross-compilation and packaging ([`c2ca3de`](https://github.com/SuaveIV/nu_plugin_audio/commit/c2ca3dea85ecabc97d991b42e421bca0ac82edec))
    - Add installation hint for plugin usage ([`7f93b35`](https://github.com/SuaveIV/nu_plugin_audio/commit/7f93b35850a9d3ff5a6863e07fba39a1a5513a1d))
    - Update installation instructions ([`ad7d2ca`](https://github.com/SuaveIV/nu_plugin_audio/commit/ad7d2ca9314bc7675f18df2cd75846e0f40410f7))
    - Support prebuilt binary installation ([`d802b04`](https://github.com/SuaveIV/nu_plugin_audio/commit/d802b04fa7ea336b75ad3395c1728600b56bba93))
    - Delegate release creation to cargo-dist ([`c1c286a`](https://github.com/SuaveIV/nu_plugin_audio/commit/c1c286ac414b18310e699856e77f9b021989c6bb))
    - Add arm64 linux cross-compilation job ([`8301c35`](https://github.com/SuaveIV/nu_plugin_audio/commit/8301c35d75314be1c3159c436c3c285a81cad1d4))
    - Add linux system dependencies ([`4ef0175`](https://github.com/SuaveIV/nu_plugin_audio/commit/4ef01757bb9ede3c70429653c86c76513c894508))
    - Rename plugin references from nu_plugin_audio_hook to nu_plugin_audio ([`ebfbfdd`](https://github.com/SuaveIV/nu_plugin_audio/commit/ebfbfddff78e7aa8daa790550a8ae311ebed57b5))
    - Update README.md ([`08801a7`](https://github.com/SuaveIV/nu_plugin_audio/commit/08801a74152b97bd51e14bdf5650ea7fef3efc66))
    - Update README to reflect project renaming and improve clarity ([`ad5d2a0`](https://github.com/SuaveIV/nu_plugin_audio/commit/ad5d2a0378e47fd3ff47493aff44533816d71fef))
</details>

## v0.111.0 (2026-03-03)

<csr-id-37d73dff2fccb240b449a0cb8735e9cc84058dc7/>
<csr-id-20822a1287b050b8c093daa38a355a03fb1639b1/>
<csr-id-18bb887257f46def5650105ac9b05d6e5287dfe4/>
<csr-id-96e495a96b6a7caa7698545d081addc0887db9a5/>
<csr-id-23da40b935119528871e06950ed6e0ea264c6e73/>
<csr-id-bac83c5ef4ee78eb96cfa3c82abe694cac927c77/>
<csr-id-4539614f120b7b5bdec4330706f4f048a223ede6/>
<csr-id-0f09882df564029b14cdd0893214988c7de2b64d/>
<csr-id-79412a36e5f154ade35215826dda808c69d2a7fa/>
<csr-id-4ff643243ff994adaec17710648f9528759f9030/>
<csr-id-4909d55a260d298f75f9115240d6463e3adba238/>
<csr-id-18582856c3b49d6c4cbe553959b57768366749f7/>
<csr-id-d6295e248a1eef439e08d3adeef144e0f20bc480/>
<csr-id-b618985a0ac521caa566cbeca7fa03d3e9bb1a13/>

### Chore

 - <csr-id-37d73dff2fccb240b449a0cb8735e9cc84058dc7/> bump dependencies and update contributors
 - <csr-id-20822a1287b050b8c093daa38a355a03fb1639b1/> bump dependencies and update contributors
 - <csr-id-18bb887257f46def5650105ac9b05d6e5287dfe4/> bump dependencies and update contributors
 - <csr-id-96e495a96b6a7caa7698545d081addc0887db9a5/> modernize dependency-update workflow for Nushell 0.110.0
   - Fixed regex patterns to prevent Nushell interpolation conflicts with capture groups.
   - Refactored scripts to use idiomatic `| let` bindings and `reduce` operations.
   - Expanded dependency discovery to include `dev-dependencies`.
   - Updated Nushell environment to version 0.110.0.
 - <csr-id-23da40b935119528871e06950ed6e0ea264c6e73/> add AUTHORS file and .mailmap for contributor identity merging
 - <csr-id-bac83c5ef4ee78eb96cfa3c82abe694cac927c77/> clean up Cargo.toml — remove lazy_static, bump id3, add crossterm, document features
 - <csr-id-4539614f120b7b5bdec4330706f4f048a223ede6/> migrate constants.rs from lazy_static to std::sync::LazyLock

### Documentation

 - <csr-id-3d0403fcaf52609aca77395543482cf6cace7a41/> enhance documentation for audio metadata commands and constants
 - <csr-id-b45e901d21f3bf7b4ce3507cf6d173398aa7c189/> update README with playback display, controls, and format support table
 - <csr-id-a99cff6faa92a00a8dc1162ca75d57a1128c794f/> update README with new features and fixes
   - Add `sound make --data` example for saving generated tones to WAV files

### New Features

<csr-id-e84d51fc0df6fbb3a5c2e5b792e000315ce8b29c/>
<csr-id-bd0168c63ca7daa4dd5b236eaafe235d79f10a20/>

 - <csr-id-9e71bea71ac2fb76fa3a569b0dd292ed13a80aea/> expand TAG_MAP, expose FileProperties, artwork, and fix duration
   - TAG_MAP: HashMap → BTreeMap; add 22 new keys (comment, lyrics, label,
   producer, remixer, replaygain_*, compilation, barcode, script, etc.)
- Space to toggle play/pause
- Left/Right (or h/l) to seek
- Up/Down (or k/j) to adjust volume
- 'm' to toggle mute
- 'q' or Esc to quit

### Bug Fixes

<csr-id-2df0c9230724b0545096bfeb742f0ab68ff7170a/>
<csr-id-ff24e05e3dd03944d5580e4ffbb7e0a58a38d264/>
<csr-id-b24841936746732faf879c5769d841158a3c924c/>
<csr-id-ed5f34422c415a3e63d3a1cd054baa382062910b/>
<csr-id-f30e460c481882501cd64177a800843f4cbd846b/>
<csr-id-d72f14dd22def9f359c23635b2aea3187f8e3939/>
<csr-id-b964eb009777bea33a7e5167356cae66f3e61c71/>
<csr-id-2d73711822d1c8ec1c777c6ea527a77bd6d863bf/>
<csr-id-e085674ee0c3a6c0dbe89f42fca14001fd7000b0/>
<csr-id-624c183913cfe3b8c7bb3c05df3444140bcff7bf/>
<csr-id-e3d43c8d570c06fc753bb8694bbbf7130f344a8c/>
<csr-id-6f3f4c0d6336fc925bf2a8c4aec7955b1e01bf44/>
<csr-id-ab0613dc35334f009e4b257291b5fcaec498d9d7/>
<csr-id-a63143ae4de82100a4584ad42bbd0f2cc09cc315/>
<csr-id-9a064c2d9b7d056c1ac8581214775f8edc5b7501/>
<csr-id-da1fff2ca4e2fde047be672b6efe9438d9afb3bd/>
<csr-id-741a149f8126fe8a64438a8977bc977d7a1072a1/>
<csr-id-fd81afde55c82f5904372ddfa902a585092b0d6d/>
<csr-id-07f095a291d49678aa45a493eeb6cc5ebe0faa5b/>
<csr-id-43a3e1a92791712f725f32f25e182c9a77d5c4e7/>
<csr-id-e3a9c860bcfb1116a310ef6bef275157d4da91cb/>
<csr-id-375bc65f9d47978161a0f978216a0247817a3e39/>
<csr-id-8bde4e2eda666731aad62d58229fb087afce5ddd/>
<csr-id-df6d0b22d81511f38eb098350ce62ae072f5b988/>
<csr-id-71d0a4c79f65594f8d52a2187d92b49e039e5f7d/>
<csr-id-c8f00e68cc05a38542eac351c7f2a4223a57720b/>
<csr-id-3aee04039914ea0dad5a8d9feffead93d1b4fff4/>
<csr-id-416655971a94f73b189b3a683303c602d41d3e14/>

 - <csr-id-5fdd4d3ac0348b285cd723b7c5a1b65dd181c6ee/> update dependencies to latest versions and clean up Cargo.lock
 - <csr-id-274bfc72ca8597f3c80f6ffebf265da9d69d2a5a/> unblock dependency updates and secure workflow
   - Remove explicit interprocess=2.2.1 pin from workflow and Cargo.toml

### Other

 - <csr-id-0f09882df564029b14cdd0893214988c7de2b64d/> upgrade checkout action, add fetch-depth, commit message, release guard, and CONTRIBUTORS step
 - <csr-id-79412a36e5f154ade35215826dda808c69d2a7fa/> optimize file loading and improve WAV header generation

### Refactor

 - <csr-id-4ff643243ff994adaec17710648f9528759f9030/> replace ID3 frame strings with generic tag keys and split parse_meta
   - Update `src/constants.rs`: Rename `ID3_HASHMAP` to `TAG_MAP` and map human-readable keys (e.g., "artist") to `lofty::ItemKey` variants, replacing raw ID3 frame strings.
   - Update `src/audio_meta.rs`:
     - Split `parse_meta` into `parse_tags` (handles file-based tags via Lofty) and `parse_stream_meta` (handles stream properties via Rodio) to facilitate future streaming support.
     - Update `SoundMetaSetCmd` to use the new `TAG_MAP`, allowing format-agnostic key usage (e.g., `-k artist` instead of `-k TPE1`).
   - Update `README.md`: Reflect the change to human-readable metadata keys in usage examples and documentation.
 - <csr-id-4909d55a260d298f75f9115240d6463e3adba238/> improve position tracking and icon consistency
   - Use icons.music() for header prefix to match render_progress behavior
     across all icon sets (NerdFont, Unicode, ASCII)
   
   - Replace manual wall-clock position tracking with sink.get_pos() to
     eliminate drift; remove last_tick variable and manual accumulation
     logic, simplify seek handlers to rely on authoritative sink position
   
   - Enhance Windows Unicode detection to support VS Code integrated
     terminal (TERM_PROGRAM=vscode) and ANSICON in addition to existing
     WT_SESSION and ConEmuPID checks
 - <csr-id-18582856c3b49d6c4cbe553959b57768366749f7/> update example command to use --raw flag for saving output
 - <csr-id-d6295e248a1eef439e08d3adeef144e0f20bc480/> remove unused `mp3-duration` dependency and improve command descriptions
 - <csr-id-b618985a0ac521caa566cbeca7fa03d3e9bb1a13/> file loading, add `sound make --data`, and enhance metadata
   - **Refactor**: Moved file loading and path resolution logic to a new `utils` module to eliminate code duplication between `audio_player` and `audio_meta`.
   - **Feature**: Added `--data` flag to `sound make`. This allows generating WAV binary data directly to stdout, enabling file saving via pipes (e.g., `sound make ... --data | save output.wav`).
   - **Enhancement**: Updated `sound meta` to provide additional technical metadata, including file size, file format extension, sample rate, and channel count.
   - **Cleanup**: Simplified error handling in `sound_make.rs` and removed unused imports across the project.

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump dependencies and update contributors ([`37d73df`](https://github.com/SuaveIV/nu_plugin_audio/commit/37d73dff2fccb240b449a0cb8735e9cc84058dc7))
    - Update dependencies to latest versions and clean up Cargo.lock ([`5fdd4d3`](https://github.com/SuaveIV/nu_plugin_audio/commit/5fdd4d3ac0348b285cd723b7c5a1b65dd181c6ee))
    - Unblock dependency updates and secure workflow ([`274bfc7`](https://github.com/SuaveIV/nu_plugin_audio/commit/274bfc72ca8597f3c80f6ffebf265da9d69d2a5a))
    - Bump dependencies and update contributors ([`20822a1`](https://github.com/SuaveIV/nu_plugin_audio/commit/20822a1287b050b8c093daa38a355a03fb1639b1))
    - Command string formatting in build.nu ([`2df0c92`](https://github.com/SuaveIV/nu_plugin_audio/commit/2df0c9230724b0545096bfeb742f0ab68ff7170a))
    - Bump dependencies and update contributors ([`18bb887`](https://github.com/SuaveIV/nu_plugin_audio/commit/18bb887257f46def5650105ac9b05d6e5287dfe4))
    - Merge pull request #27 from SuaveIV/meta_refactor ([`129ea34`](https://github.com/SuaveIV/nu_plugin_audio/commit/129ea348aa0b52976094c01429835188f9a74d8b))
    - Improve error handling in audio metadata parsing and update dependency version extraction ([`ff24e05`](https://github.com/SuaveIV/nu_plugin_audio/commit/ff24e05e3dd03944d5580e4ffbb7e0a58a38d264))
    - Modernize dependency-update workflow for Nushell 0.110.0 ([`96e495a`](https://github.com/SuaveIV/nu_plugin_audio/commit/96e495a96b6a7caa7698545d081addc0887db9a5))
    - Add publisher alias in README and improve error handling in audio metadata parsing ([`b248419`](https://github.com/SuaveIV/nu_plugin_audio/commit/b24841936746732faf879c5769d841158a3c924c))
    - Correct duration format in README and improve error handling in audio metadata parsing ([`ed5f344`](https://github.com/SuaveIV/nu_plugin_audio/commit/ed5f34422c415a3e63d3a1cd054baa382062910b))
    - Address review findings across meta, player, constants, and README ([`f30e460`](https://github.com/SuaveIV/nu_plugin_audio/commit/f30e460c481882501cd64177a800843f4cbd846b))
    - Enhance documentation for audio metadata commands and constants ([`3d0403f`](https://github.com/SuaveIV/nu_plugin_audio/commit/3d0403fcaf52609aca77395543482cf6cace7a41))
    - Expand TAG_MAP, expose FileProperties, artwork, and fix duration ([`9e71bea`](https://github.com/SuaveIV/nu_plugin_audio/commit/9e71bea71ac2fb76fa3a569b0dd292ed13a80aea))
    - Coerce vec to boxed slice for Type::Record in input_output_types ([`d72f14d`](https://github.com/SuaveIV/nu_plugin_audio/commit/d72f14dd22def9f359c23635b2aea3187f8e3939))
    - Replace ID3 frame strings with generic tag keys and split parse_meta ([`4ff6432`](https://github.com/SuaveIV/nu_plugin_audio/commit/4ff643243ff994adaec17710648f9528759f9030))
    - Migrate from `id3` to `lofty` for multi-format metadata support ([`635155a`](https://github.com/SuaveIV/nu_plugin_audio/commit/635155a19aae7593abad122457456196c99c61d1))
    - Merge pull request #25 from SuaveIV/dep-up-fix ([`8fc43db`](https://github.com/SuaveIV/nu_plugin_audio/commit/8fc43db0c6fcbb009e5ee30f132b3c498f61bb96))
    - Ci: refactor dependency update workflow` - Avoid round-tripping Cargo.toml to preserve comments and formatting - Update crates.io version extraction to use `default_version` ([`be9731b`](https://github.com/SuaveIV/nu_plugin_audio/commit/be9731b65e8e20401f7f2b03637cdd21af12cf96))
    - Change cargo build to cargo check in dependency update workflow ([`b964eb0`](https://github.com/SuaveIV/nu_plugin_audio/commit/b964eb009777bea33a7e5167356cae66f3e61c71))
    - Sanitize plugin version in dependency update workflow ([`2d73711`](https://github.com/SuaveIV/nu_plugin_audio/commit/2d73711822d1c8ec1c777c6ea527a77bd6d863bf))
    - Inline dep bump script and pin interprocess to =2.2.1 ([`e085674`](https://github.com/SuaveIV/nu_plugin_audio/commit/e085674ee0c3a6c0dbe89f42fca14001fd7000b0))
    - Downgrade interprocess to version 2.2.1 ([`624c183`](https://github.com/SuaveIV/nu_plugin_audio/commit/624c183913cfe3b8c7bb3c05df3444140bcff7bf))
    - Merge pull request #20 from SuaveIV/fix/progress-bar-flicker ([`dcf0bdc`](https://github.com/SuaveIV/nu_plugin_audio/commit/dcf0bdc67e77ae03a3d16c186c13dcf386ab0146))
    - Prevent flicker and make header responsive ([`e3d43c8`](https://github.com/SuaveIV/nu_plugin_audio/commit/e3d43c8d570c06fc753bb8694bbbf7130f344a8c))
    - Optimize header rendering and clear progress line to reduce flicker ([`6f3f4c0`](https://github.com/SuaveIV/nu_plugin_audio/commit/6f3f4c0d6336fc925bf2a8c4aec7955b1e01bf44))
    - Improve progress header display and handle truncation ([`ab0613d`](https://github.com/SuaveIV/nu_plugin_audio/commit/ab0613dc35334f009e4b257291b5fcaec498d9d7))
    - Stop progress bar from flickering ([`a63143a`](https://github.com/SuaveIV/nu_plugin_audio/commit/a63143ae4de82100a4584ad42bbd0f2cc09cc315))
    - Merge pull request #18 from SuaveIV/player_refactor ([`49e7002`](https://github.com/SuaveIV/nu_plugin_audio/commit/49e70027f292d19b736aed07fb55efae5bfc833b))
    - Harden CI workflow, fix bar overflow, update unicode-width ([`9a064c2`](https://github.com/SuaveIV/nu_plugin_audio/commit/9a064c2d9b7d056c1ac8581214775f8edc5b7501))
    - Remove dead code, guard codec overrun, skip narrow terminals ([`da1fff2`](https://github.com/SuaveIV/nu_plugin_audio/commit/da1fff2ca4e2fde047be672b6efe9438d9afb3bd))
    - Improve position tracking and icon consistency ([`4909d55`](https://github.com/SuaveIV/nu_plugin_audio/commit/4909d55a260d298f75f9115240d6463e3adba238))
    - Update playback command description to include 5s seeking functionality ([`741a149`](https://github.com/SuaveIV/nu_plugin_audio/commit/741a149f8126fe8a64438a8977bc977d7a1072a1))
    - Update Nushell setup action to version 3 ([`fd81afd`](https://github.com/SuaveIV/nu_plugin_audio/commit/fd81afde55c82f5904372ddfa902a585092b0d6d))
    - Enhance character width handling and improve progress display logic ([`07f095a`](https://github.com/SuaveIV/nu_plugin_audio/commit/07f095a291d49678aa45a493eeb6cc5ebe0faa5b))
    - Fix double keypress registration and optimize metadata rendering ([`ea942a3`](https://github.com/SuaveIV/nu_plugin_audio/commit/ea942a3579f13273ffedb614d4b118e33cd87630))
    - Improve playback logic, display rendering, and workflow stability ([`43a3e1a`](https://github.com/SuaveIV/nu_plugin_audio/commit/43a3e1a92791712f725f32f25e182c9a77d5c4e7))
    - Update README with playback display, controls, and format support table ([`b45e901`](https://github.com/SuaveIV/nu_plugin_audio/commit/b45e901d21f3bf7b4ce3507cf6d173398aa7c189))
    - Add interactive audio player with rich progress display ([`e84d51f`](https://github.com/SuaveIV/nu_plugin_audio/commit/e84d51fc0df6fbb3a5c2e5b792e000315ce8b29c))
    - Upgrade checkout action, add fetch-depth, commit message, release guard, and CONTRIBUTORS step ([`0f09882`](https://github.com/SuaveIV/nu_plugin_audio/commit/0f09882df564029b14cdd0893214988c7de2b64d))
    - Add AUTHORS file and .mailmap for contributor identity merging ([`23da40b`](https://github.com/SuaveIV/nu_plugin_audio/commit/23da40b935119528871e06950ed6e0ea264c6e73))
    - Clean up Cargo.toml — remove lazy_static, bump id3, add crossterm, document features ([`bac83c5`](https://github.com/SuaveIV/nu_plugin_audio/commit/bac83c5ef4ee78eb96cfa3c82abe694cac927c77))
    - Migrate constants.rs from lazy_static to std::sync::LazyLock ([`4539614`](https://github.com/SuaveIV/nu_plugin_audio/commit/4539614f120b7b5bdec4330706f4f048a223ede6))
    - Merge pull request #15 from SuaveIV/util_refactor ([`a583c7d`](https://github.com/SuaveIV/nu_plugin_audio/commit/a583c7d62df3e79244a616dde6a11683043bcc99))
    - Change env_logger initialization to try_init for safer logging setup ([`e3a9c86`](https://github.com/SuaveIV/nu_plugin_audio/commit/e3a9c860bcfb1116a310ef6bef275157d4da91cb))
    - Improve error handling for negative and invalid duration values in load_values function ([`375bc65`](https://github.com/SuaveIV/nu_plugin_audio/commit/375bc65f9d47978161a0f978216a0247817a3e39))
    - Correct key name for recording year in ID3_HASHMAP ([`8bde4e2`](https://github.com/SuaveIV/nu_plugin_audio/commit/8bde4e2eda666731aad62d58229fb087afce5ddd))
    - Fix metadata writing, overflow safety, and logging init ([`c2040cf`](https://github.com/SuaveIV/nu_plugin_audio/commit/c2040cf00b257d97d761e63217f5d78e3d82c544))
    - Update audio file references in metadata commands and examples ([`df6d0b2`](https://github.com/SuaveIV/nu_plugin_audio/commit/df6d0b22d81511f38eb098350ce62ae072f5b988))
    - Update README with new features and fixes ([`a99cff6`](https://github.com/SuaveIV/nu_plugin_audio/commit/a99cff6faa92a00a8dc1162ca75d57a1128c794f))
    - Windows file locking, improve WAV safety, and enhance error handling ([`71d0a4c`](https://github.com/SuaveIV/nu_plugin_audio/commit/71d0a4c79f65594f8d52a2187d92b49e039e5f7d))
    - Optimize file loading and improve WAV header generation ([`79412a3`](https://github.com/SuaveIV/nu_plugin_audio/commit/79412a36e5f154ade35215826dda808c69d2a7fa))
    - Update example command to use --raw flag for saving output ([`1858285`](https://github.com/SuaveIV/nu_plugin_audio/commit/18582856c3b49d6c4cbe553959b57768366749f7))
    - Remove unused `mp3-duration` dependency and improve command descriptions ([`d6295e2`](https://github.com/SuaveIV/nu_plugin_audio/commit/d6295e248a1eef439e08d3adeef144e0f20bc480))
    - Merge branch 'fmotalleb:main' into util_refactor ([`689efbc`](https://github.com/SuaveIV/nu_plugin_audio/commit/689efbc98d196e8484f0c0de1c346fe5891937d4))
    - File loading, add `sound make --data`, and enhance metadata ([`b618985`](https://github.com/SuaveIV/nu_plugin_audio/commit/b618985a0ac521caa566cbeca7fa03d3e9bb1a13))
    - Merge pull request #13 from SuaveIV/sound_play_amp ([`1f3a688`](https://github.com/SuaveIV/nu_plugin_audio/commit/1f3a6888cf663c8397e954d485714c41545e2d71))
    - Add volume control to sound play and make ([`bd0168c`](https://github.com/SuaveIV/nu_plugin_audio/commit/bd0168c63ca7daa4dd5b236eaafe235d79f10a20))
    - Commit from GitHub Actions (Update dependencies) ([`9c5fb7b`](https://github.com/SuaveIV/nu_plugin_audio/commit/9c5fb7b375b675300825f3ca2c1ec6608227982e))
    - Commit from GitHub Actions (Update dependencies) ([`41456d0`](https://github.com/SuaveIV/nu_plugin_audio/commit/41456d06a543f7e1e7a15df1d6add2801c083124))
    - Commit from GitHub Actions (Update dependencies) ([`21d6072`](https://github.com/SuaveIV/nu_plugin_audio/commit/21d6072486e16957dff405fb161e5d33eff1198a))
    - Commit from GitHub Actions (Update dependencies) ([`86e661b`](https://github.com/SuaveIV/nu_plugin_audio/commit/86e661b58b2177e4419c142eb758dcea8bf03a4c))
    - Commit from GitHub Actions (Update dependencies) ([`815bddd`](https://github.com/SuaveIV/nu_plugin_audio/commit/815bddd152eda8b2a5528e4d149ff3096472e53f))
    - Commit from GitHub Actions (Update dependencies) ([`ef56d5c`](https://github.com/SuaveIV/nu_plugin_audio/commit/ef56d5c169d0304fb66b3ddacf67e019001d5f80))
    - Commit from GitHub Actions (Update dependencies) ([`c2ae1a4`](https://github.com/SuaveIV/nu_plugin_audio/commit/c2ae1a4d1d1ec1ae4859e08af259857da93b4d87))
    - Commit from GitHub Actions (Update dependencies) ([`4927faa`](https://github.com/SuaveIV/nu_plugin_audio/commit/4927faad3b23028553ce1727cb4367c3a4328311))
    - Commit from GitHub Actions (Update dependencies) ([`ad035c2`](https://github.com/SuaveIV/nu_plugin_audio/commit/ad035c207a861a7a0dfe00e0fa1d9754be96b5f7))
    - Commit from GitHub Actions (Update dependencies) ([`6c530ff`](https://github.com/SuaveIV/nu_plugin_audio/commit/6c530ffd49d6bffb8bfd0f6377c00201a75d1239))
    - Commit from GitHub Actions (Update dependencies) ([`b59697b`](https://github.com/SuaveIV/nu_plugin_audio/commit/b59697bda90377c730705cfbba7a00b6aed725d1))
    - Commit from GitHub Actions (Update dependencies) ([`c72cbe7`](https://github.com/SuaveIV/nu_plugin_audio/commit/c72cbe703563057ed1195b89dd1f8fcec5ba943d))
    - Commit from GitHub Actions (Update dependencies) ([`8abada5`](https://github.com/SuaveIV/nu_plugin_audio/commit/8abada5ddc0652c607789cc2196b2660d095c71c))
    - Commit from GitHub Actions (Update dependencies) ([`051938e`](https://github.com/SuaveIV/nu_plugin_audio/commit/051938e44cc2ea232ab2d7bf1ca656a100df2642))
    - Commit from GitHub Actions (Update dependencies) ([`bcb49c6`](https://github.com/SuaveIV/nu_plugin_audio/commit/bcb49c660d817eb858b1fb0aa4caa5309f95611e))
    - Commit from GitHub Actions (Update dependencies) ([`ceacdbd`](https://github.com/SuaveIV/nu_plugin_audio/commit/ceacdbd9a4d599b2cf53b30e93f6e75032a560a2))
    - Commit from GitHub Actions (Update dependencies) ([`8f3c533`](https://github.com/SuaveIV/nu_plugin_audio/commit/8f3c533011fe85df67bd72b366935e5004bdad8a))
    - Commit from GitHub Actions (Update dependencies) ([`31edc10`](https://github.com/SuaveIV/nu_plugin_audio/commit/31edc10b0f773a362f009369a15b0ceb91b496e2))
    - Commit from GitHub Actions (Update dependencies) ([`a50ff29`](https://github.com/SuaveIV/nu_plugin_audio/commit/a50ff29368f5af00d85dc4a29bc1372965d31f91))
    - Commit from GitHub Actions (Update dependencies) ([`9070fd4`](https://github.com/SuaveIV/nu_plugin_audio/commit/9070fd4c5d966db79bf829c6b7a839bd6b2719e1))
    - Commit from GitHub Actions (Update dependencies) ([`f9a64ee`](https://github.com/SuaveIV/nu_plugin_audio/commit/f9a64ee06c1a92864b7cae5bf18263f11f4cabad))
    - Commit from GitHub Actions (Update dependencies) ([`7261ba5`](https://github.com/SuaveIV/nu_plugin_audio/commit/7261ba5ffd3ac64acba86a6f4f5bffaff1e888e5))
    - Commit from GitHub Actions (Update dependencies) ([`f5fd36a`](https://github.com/SuaveIV/nu_plugin_audio/commit/f5fd36abb600e2789bffe0cafe99d5c0d77ff622))
    - Commit from GitHub Actions (Update dependencies) ([`0b699ec`](https://github.com/SuaveIV/nu_plugin_audio/commit/0b699ecc16052b880ff57ecdc89c9b088ed0563a))
    - Commit from GitHub Actions (Update dependencies) ([`66f4edd`](https://github.com/SuaveIV/nu_plugin_audio/commit/66f4edd686053105a64b081df7288de339b6a3fd))
    - Merge pull request #12 from tguichaoua/fix_sound_play ([`a6aa10d`](https://github.com/SuaveIV/nu_plugin_audio/commit/a6aa10db7581aa9f9f3cd16c00efb047e3519e6e))
    - Commit from GitHub Actions (Update dependencies) ([`3192647`](https://github.com/SuaveIV/nu_plugin_audio/commit/3192647dc559a35a5ee295edb7f8495a1c1fea74))
    - Remove print in stderr for `sound play` ([`c8f00e6`](https://github.com/SuaveIV/nu_plugin_audio/commit/c8f00e68cc05a38542eac351c7f2a4223a57720b))
    - Update README.md ([`49ff6bd`](https://github.com/SuaveIV/nu_plugin_audio/commit/49ff6bd8ec82fff37d448a883133421fe6d8d55e))
    - Merge pull request #11 from tguichaoua/output_stream_disable_log_on_drop ([`ac02a7e`](https://github.com/SuaveIV/nu_plugin_audio/commit/ac02a7ea9e587d3dad5ed9ab38990da7de9fe8fe))
    - Disable OutputStream log on drop ([`3aee040`](https://github.com/SuaveIV/nu_plugin_audio/commit/3aee04039914ea0dad5a8d9feffead93d1b4fff4))
    - Commit from GitHub Actions (Update dependencies) ([`cc7f642`](https://github.com/SuaveIV/nu_plugin_audio/commit/cc7f64237291f9abc45fb6a3fe44d6679193a4a7))
    - Commit from GitHub Actions (Update dependencies) ([`d469deb`](https://github.com/SuaveIV/nu_plugin_audio/commit/d469deb49b31ae0eb77ca05d887b3744f589ccdd))
    - Merge pull request #10 from v1olen/fix/quote-paths ([`75b1792`](https://github.com/SuaveIV/nu_plugin_audio/commit/75b17923f85dc9b4ef020e7b365c173837fc06b1))
    - Quote paths so they can contain spaces ([`4166559`](https://github.com/SuaveIV/nu_plugin_audio/commit/416655971a94f73b189b3a683303c602d41d3e14))
    - Commit from GitHub Actions (Update dependencies) ([`3460019`](https://github.com/SuaveIV/nu_plugin_audio/commit/3460019b15d32acd84c2c3efb9a71d6463fb5dfd))
</details>

## v0.109.1 (2025-12-03)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Commit from GitHub Actions (Update dependencies) ([`b7ae7ed`](https://github.com/SuaveIV/nu_plugin_audio/commit/b7ae7ed39afdadc13b9df0bac00759c9474e7b7d))
</details>

## v0.109.0 (2025-11-30)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Commit from GitHub Actions (Update dependencies) ([`0d5baa5`](https://github.com/SuaveIV/nu_plugin_audio/commit/0d5baa50a929eaaef991e97f384befad278d66a1))
    - Set Nushell version to 0.108.0 ([`1eaab07`](https://github.com/SuaveIV/nu_plugin_audio/commit/1eaab07d82ca349e4f236d35a818184a7b06648f))
    - Commit from GitHub Actions (Update dependencies) ([`2bd6c37`](https://github.com/SuaveIV/nu_plugin_audio/commit/2bd6c372638f159b0f42dd12290fbbb2f667b9df))
    - Commit from GitHub Actions (Update dependencies) ([`7fbed15`](https://github.com/SuaveIV/nu_plugin_audio/commit/7fbed15a0678b989490100a9fb0470cb408e0f5a))
    - Commit from GitHub Actions (Update dependencies) ([`198dfd9`](https://github.com/SuaveIV/nu_plugin_audio/commit/198dfd92fa2b6ed8312ab64f467adbf503cc34d8))
    - Commit from GitHub Actions (Update dependencies) ([`357694c`](https://github.com/SuaveIV/nu_plugin_audio/commit/357694c6b0d4303d3c217fddeb6638111b7f38f1))
    - Commit from GitHub Actions (Update dependencies) ([`cba4a59`](https://github.com/SuaveIV/nu_plugin_audio/commit/cba4a591128d1870a6cc2e7cd73afe48faa54e8f))
    - Commit from GitHub Actions (Update dependencies) ([`4f3ffaa`](https://github.com/SuaveIV/nu_plugin_audio/commit/4f3ffaaaa0ba5efafcb1a519a3d56877b5fda14c))
    - Commit from GitHub Actions (Update dependencies) ([`dff1f61`](https://github.com/SuaveIV/nu_plugin_audio/commit/dff1f6115e3d609c6a91c25cb197cd800ef7c82e))
    - Commit from GitHub Actions (Update dependencies) ([`a652118`](https://github.com/SuaveIV/nu_plugin_audio/commit/a6521185f78692e8ed05f4c2d66e216e1a8e9d6c))
    - Commit from GitHub Actions (Update dependencies) ([`f0432c4`](https://github.com/SuaveIV/nu_plugin_audio/commit/f0432c40ded7a1951440e2f2ff6e4a12e70cf2da))
    - Commit from GitHub Actions (Update dependencies) ([`23055ff`](https://github.com/SuaveIV/nu_plugin_audio/commit/23055ff7303905ba36a36cead7d83c12521c4ef4))
    - Commit from GitHub Actions (Update dependencies) ([`2db8e23`](https://github.com/SuaveIV/nu_plugin_audio/commit/2db8e23f1bad6232ca1c80d5e8a3fe5665ef6428))
    - Commit from GitHub Actions (Update dependencies) ([`26e65ed`](https://github.com/SuaveIV/nu_plugin_audio/commit/26e65ed0690a9ea31091d2492c61ca0c1e26c177))
    - Commit from GitHub Actions (Update dependencies) ([`76195fa`](https://github.com/SuaveIV/nu_plugin_audio/commit/76195fae9392f4dbbef902d6c01eb8b9b4086784))
    - Commit from GitHub Actions (Update dependencies) ([`e0ac80c`](https://github.com/SuaveIV/nu_plugin_audio/commit/e0ac80c422e46d7b12380ed834c3009fdd87e16a))
    - Commit from GitHub Actions (Update dependencies) ([`6795cec`](https://github.com/SuaveIV/nu_plugin_audio/commit/6795cec582ead0cd82245e0db25e540466d86909))
    - Commit from GitHub Actions (Update dependencies) ([`8922473`](https://github.com/SuaveIV/nu_plugin_audio/commit/89224734d676963ce978f46dcf03b4b10962bb8e))
    - Commit from GitHub Actions (Update dependencies) ([`4b89335`](https://github.com/SuaveIV/nu_plugin_audio/commit/4b89335e06e8d69b8f1a3b034297a57b27bc7e8a))
</details>

## v0.108.0 (2025-10-17)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Commit from GitHub Actions (Update dependencies) ([`db29987`](https://github.com/SuaveIV/nu_plugin_audio/commit/db29987f07d6916b7588f31236197f8602304371))
    - Commit from GitHub Actions (Update dependencies) ([`416270e`](https://github.com/SuaveIV/nu_plugin_audio/commit/416270e9d2972ea1a0d34f382f4aa43572b47c54))
    - Commit from GitHub Actions (Update dependencies) ([`5151cb8`](https://github.com/SuaveIV/nu_plugin_audio/commit/5151cb8de248e7e40d86323afd1cc8fcabebf8f5))
    - Commit from GitHub Actions (Update dependencies) ([`6101841`](https://github.com/SuaveIV/nu_plugin_audio/commit/6101841b8c89f39ab35963f7aa656328486501cf))
    - Commit from GitHub Actions (Update dependencies) ([`e1c8521`](https://github.com/SuaveIV/nu_plugin_audio/commit/e1c852112525451b31ad508bc1343bd7b3b118df))
    - Commit from GitHub Actions (Update dependencies) ([`362d8c2`](https://github.com/SuaveIV/nu_plugin_audio/commit/362d8c289d10d913b9cff8e09990e71acc0621fe))
    - Commit from GitHub Actions (Update dependencies) ([`5d8867f`](https://github.com/SuaveIV/nu_plugin_audio/commit/5d8867fca805acda8938b0fb07a33f70c76e6c0d))
    - Commit from GitHub Actions (Update dependencies) ([`424393c`](https://github.com/SuaveIV/nu_plugin_audio/commit/424393c64e2d0e3e485794f16ed5bf7374c98e73))
    - Commit from GitHub Actions (Update dependencies) ([`de074c3`](https://github.com/SuaveIV/nu_plugin_audio/commit/de074c34e2155d52af3ac407ff7b50984fb328a2))
    - Commit from GitHub Actions (Update dependencies) ([`fd6e21b`](https://github.com/SuaveIV/nu_plugin_audio/commit/fd6e21b3029ce53d7ba7b2c5e359674931e1dd81))
    - Commit from GitHub Actions (Update dependencies) ([`08ddf5a`](https://github.com/SuaveIV/nu_plugin_audio/commit/08ddf5ad08c4f190c2600cc1de937a4799d3de87))
    - Commit from GitHub Actions (Update dependencies) ([`68f3ab9`](https://github.com/SuaveIV/nu_plugin_audio/commit/68f3ab9d3f92c85215c6ed04163571f69c01eda9))
    - Commit from GitHub Actions (Update dependencies) ([`fcf29da`](https://github.com/SuaveIV/nu_plugin_audio/commit/fcf29daac8257511ab554201bfddf22a6172d904))
    - Commit from GitHub Actions (Update dependencies) ([`6bd5829`](https://github.com/SuaveIV/nu_plugin_audio/commit/6bd58295641e40b3a28c015a08aff538e1a6d9a5))
    - Commit from GitHub Actions (Update dependencies) ([`8978767`](https://github.com/SuaveIV/nu_plugin_audio/commit/8978767c4fd8a80acdaa7429ddfdbf2509e87b04))
    - Commit from GitHub Actions (Update dependencies) ([`c7e1572`](https://github.com/SuaveIV/nu_plugin_audio/commit/c7e15724d62d962cfeaf8db60e807cc529e2cd31))
    - Commit from GitHub Actions (Update dependencies) ([`7bfce31`](https://github.com/SuaveIV/nu_plugin_audio/commit/7bfce317fd4a5a16b15656177e4c1478a7a70521))
    - Commit from GitHub Actions (Update dependencies) ([`95586e5`](https://github.com/SuaveIV/nu_plugin_audio/commit/95586e518b50f872927d977045a91ac5cf397f04))
    - Commit from GitHub Actions (Update dependencies) ([`62f630f`](https://github.com/SuaveIV/nu_plugin_audio/commit/62f630feaefb7fc373c356518e1851ed36f86967))
    - Commit from GitHub Actions (Update dependencies) ([`96cc855`](https://github.com/SuaveIV/nu_plugin_audio/commit/96cc85584338220f39299bd637cc33bbfe2f214d))
    - Commit from GitHub Actions (Update dependencies) ([`b544045`](https://github.com/SuaveIV/nu_plugin_audio/commit/b5440450eb093ea976729c52cdaff817589ba616))
</details>

## v0.107.0 (2025-09-03)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Commit from GitHub Actions (Update dependencies) ([`900fe42`](https://github.com/SuaveIV/nu_plugin_audio/commit/900fe42fb520826098d68514bf006bb1bf8bafd4))
    - Commit from GitHub Actions (Update dependencies) ([`00a671a`](https://github.com/SuaveIV/nu_plugin_audio/commit/00a671a0beb255e2f65b138917e9e65c3ad2e52e))
    - Commit from GitHub Actions (Update dependencies) ([`34c8d58`](https://github.com/SuaveIV/nu_plugin_audio/commit/34c8d58bd709e3d45a03a5e9deed6d8dbba7f0bf))
    - Commit from GitHub Actions (Update dependencies) ([`24bcb02`](https://github.com/SuaveIV/nu_plugin_audio/commit/24bcb023ba9d5ee6ec0194dd55e7dbe385f0f945))
    - Commit from GitHub Actions (Update dependencies) ([`3eb2d56`](https://github.com/SuaveIV/nu_plugin_audio/commit/3eb2d56551d9b748f41651091c9b37804caef232))
    - Commit from GitHub Actions (Update dependencies) ([`ade5deb`](https://github.com/SuaveIV/nu_plugin_audio/commit/ade5debe3862f74015883150d6094b185a60ef36))
    - Commit from GitHub Actions (Update dependencies) ([`78b191f`](https://github.com/SuaveIV/nu_plugin_audio/commit/78b191f65352915bc37ae6ab133e1ffb1876ad51))
    - Commit from GitHub Actions (Update dependencies) ([`578c014`](https://github.com/SuaveIV/nu_plugin_audio/commit/578c0140ee229796650a66bb391ef25fe24c2145))
    - Commit from GitHub Actions (Update dependencies) ([`1c7106a`](https://github.com/SuaveIV/nu_plugin_audio/commit/1c7106a6bddf99123ed3867d13aef4e72ba266da))
    - Commit from GitHub Actions (Update dependencies) ([`fd9ef08`](https://github.com/SuaveIV/nu_plugin_audio/commit/fd9ef08febc9db273a165a7ad6a1d646ed8d3de6))
    - Commit from GitHub Actions (Update dependencies) ([`66b6e16`](https://github.com/SuaveIV/nu_plugin_audio/commit/66b6e163ab15ce0238df307ba080e95ff66a0d8b))
    - Commit from GitHub Actions (Update dependencies) ([`003a91a`](https://github.com/SuaveIV/nu_plugin_audio/commit/003a91a1ea84ed4cfa0d6877ffadd6da8981be91))
    - Commit from GitHub Actions (Update dependencies) ([`9994af2`](https://github.com/SuaveIV/nu_plugin_audio/commit/9994af283126160c5d051c2743c8f092b6d56bb9))
</details>

## v0.106.1 (2025-08-03)

<csr-id-98ab6aaf1a776a189e318950b9486a7689907155/>
<csr-id-45bd16201b4d9918fe86fa820ea07026f94caab9/>
<csr-id-9119a1d3f8c6c64a96aea770b20ed0013a8dafbc/>
<csr-id-a67c093edcf0e9005f134e2d821a44ff8420f092/>
<csr-id-4a0ef45f94b01cc069220e039778be31ff1d0cc8/>
<csr-id-e2061a932043792ca517478a367f6eb991d56c05/>
<csr-id-7500966cb46bdc9736f40e881f41fe1b7fc0d74e/>
<csr-id-e3bad554084913238986cd3621eaeef10ce493ea/>
<csr-id-1c3a1b798dc0875af9dded383aa143e3566652b2/>
<csr-id-9a7d7d23d0aeffa11a154887541dcde17344d763/>
<csr-id-40518de058b294cbb23348d2a09253c340d8716a/>
<csr-id-6fde8d3a232bff33b8985ccfdf5018834f58d7ac/>
<csr-id-2f778866fa580367000b7125d66cef4940e4f931/>
<csr-id-4d02c133629c429b164f97bf846b9f5f12ef8a50/>
<csr-id-f71f448a430baf778d7a848bf1c1d232490933ee/>
<csr-id-6a640e8ff5a6fd833937af9628916d487a138062/>
<csr-id-fc21199bd842e3f74c57568879adc91630162156/>
<csr-id-332e1232c820905a460f1d8e120bd87988779b09/>
<csr-id-d73259f2473e79653f11890091a7af5b789a1230/>
<csr-id-5aa55fb893fd0e952158cf8b269063c393a27701/>

### Chore

 - <csr-id-98ab6aaf1a776a189e318950b9486a7689907155/> update nu version
 - <csr-id-45bd16201b4d9918fe86fa820ea07026f94caab9/> add rust-toolchain configuration file
 - <csr-id-9119a1d3f8c6c64a96aea770b20ed0013a8dafbc/> update nu-plugin and dependencies to version 0.105.1
 - <csr-id-a67c093edcf0e9005f134e2d821a44ff8420f092/> update dependencies and version to 0.104.0
 - <csr-id-4a0ef45f94b01cc069220e039778be31ff1d0cc8/> bump nu-protocol to v0.101.0
 - <csr-id-e2061a932043792ca517478a367f6eb991d56c05/> update packages

### New Features

 - <csr-id-4e21a525c531eff89474b8c56d85ca03d126595e/> player now accept relative paths
 - <csr-id-21adaa164ace69d9cb1f579476e189566d53795d/> add GitHub Actions workflow for automatic dependency updates
 - <csr-id-6831b19bf4636434573f44ebcbe90e2c4ad241b8/> auto publish script

### Bug Fixes

 - <csr-id-fc224800bf1869dcc4d1817038db5acd858f1a18/> swapped `unwrap` with error mapping
 - <csr-id-feb493dd2dd15f124930f1cf83c6dc4cad565709/> impossible to interrupt player task
 - <csr-id-009341711ec629392875f9c8a0cbbb91edb237a9/> update examples method signature to include lifetime parameter
 - <csr-id-09a548fca38fd7a8ea3fb90502df3f4d051e60b4/> remove unnecessary echo statement from build.nu
 - <csr-id-8c9290301e672bddef77f6c92c5929144c2f7c5e/> nupm package version
 - <csr-id-c88864f5f37cc3c999e0e88a1c60c8131fbfcbb3/> permission issue on apt
 - <csr-id-22828558d23d47e39777760a42b5d5cd64f84fb6/> changed `register` to `plugin add` in README
 - <csr-id-0069342830baefafbeba42e04eaae55857c5b6fd/> added alsa sys package to workflow
 - <csr-id-ffc95ad22865e1326a927eec085de5d0f2c94a17/> workflow trigger
 - <csr-id-f7e4f203e16a9106856499c05b524404a2997df9/> nupm build script feature list
 - <csr-id-dd8f83a3f3a2c56fcd8106a8a9705270deccd462/> bump nu-protocol to v0.94
 - <csr-id-1e7f55b1e8c395d4179a4de3afb0e89e7b05b766/> `sound meta -a` error
 - <csr-id-31cf298aaa67c0a0141b063bc1e88dfd0a8a25e7/> bump to nu-protocol v0.93

### Other

 - <csr-id-7500966cb46bdc9736f40e881f41fe1b7fc0d74e/> rodio to v0.21.1
 - <csr-id-e3bad554084913238986cd3621eaeef10ce493ea/> replaced `unwrap` with error mapping
 - <csr-id-1c3a1b798dc0875af9dded383aa143e3566652b2/> update nu_plugin_audio_hook to version 0.103.0 and bump dependencies
 - <csr-id-9a7d7d23d0aeffa11a154887541dcde17344d763/> update nu_plugin_audio_hook to version 0.102.0 and bump dependencies
 - <csr-id-40518de058b294cbb23348d2a09253c340d8716a/> upgraded to nu-protocol v0.100.0
 - <csr-id-6fde8d3a232bff33b8985ccfdf5018834f58d7ac/> bump nu-protocol to 0.99
 - <csr-id-2f778866fa580367000b7125d66cef4940e4f931/> Bump nu-protocol to v0.98.0
 - <csr-id-4d02c133629c429b164f97bf846b9f5f12ef8a50/> bump rodio version to 0.19.0
 - <csr-id-f71f448a430baf778d7a848bf1c1d232490933ee/> updated packages
 - <csr-id-6a640e8ff5a6fd833937af9628916d487a138062/> lockfile dependencies
 - <csr-id-fc21199bd842e3f74c57568879adc91630162156/> updated nu-protocol to 0.96.0
 - <csr-id-332e1232c820905a460f1d8e120bd87988779b09/> bump nu deps to v0.95
 - <csr-id-d73259f2473e79653f11890091a7af5b789a1230/> bump id3 version
 - <csr-id-5aa55fb893fd0e952158cf8b269063c393a27701/> Bump nu-protocol version to 0.93

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#6](https://github.com/SuaveIV/nu_plugin_audio/issues/6)**
    - Player now accept relative paths ([`4e21a52`](https://github.com/SuaveIV/nu_plugin_audio/commit/4e21a525c531eff89474b8c56d85ca03d126595e))
 * **[#7](https://github.com/SuaveIV/nu_plugin_audio/issues/7)**
    - Impossible to interrupt player task ([`feb493d`](https://github.com/SuaveIV/nu_plugin_audio/commit/feb493dd2dd15f124930f1cf83c6dc4cad565709))
 * **Uncategorized**
    - Swapped `unwrap` with error mapping ([`fc22480`](https://github.com/SuaveIV/nu_plugin_audio/commit/fc224800bf1869dcc4d1817038db5acd858f1a18))
    - Merge branch 'main' of https://github.com/FMotalleb/nu_plugin_audio_hook ([`8218bb0`](https://github.com/SuaveIV/nu_plugin_audio/commit/8218bb031bb29e7dbea78b3ae5182182ec3a2552))
    - Rodio to v0.21.1 ([`7500966`](https://github.com/SuaveIV/nu_plugin_audio/commit/7500966cb46bdc9736f40e881f41fe1b7fc0d74e))
    - Merge pull request #9 from Larandar/feat/relative-path ([`dcf368e`](https://github.com/SuaveIV/nu_plugin_audio/commit/dcf368e06c12d417ed3adeb5a8d7a637773b69f4))
    - Replaced `unwrap` with error mapping ([`e3bad55`](https://github.com/SuaveIV/nu_plugin_audio/commit/e3bad554084913238986cd3621eaeef10ce493ea))
    - Merge pull request #8 from Larandar/fix/interrupts ([`2d8f223`](https://github.com/SuaveIV/nu_plugin_audio/commit/2d8f223b1d758ebcf4f30c9566b4c141abb02928))
    - Commit from GitHub Actions (Update dependencies) ([`894587f`](https://github.com/SuaveIV/nu_plugin_audio/commit/894587f992b5f82459b52bcbe09333580103e6ef))
    - Commit from GitHub Actions (Update dependencies) ([`9d5d45e`](https://github.com/SuaveIV/nu_plugin_audio/commit/9d5d45e01ca36ebd79c95017b2676e9b2cf3f359))
    - Update dependency-update.yaml ([`a79a461`](https://github.com/SuaveIV/nu_plugin_audio/commit/a79a461a88faf06ac07a578862a1dee4a591c659))
    - Merge pull request #4 from Larandar/main ([`c201841`](https://github.com/SuaveIV/nu_plugin_audio/commit/c201841b9191c8684440b1fbcfaa62d072dcb8b2))
    - Update nu version ([`98ab6aa`](https://github.com/SuaveIV/nu_plugin_audio/commit/98ab6aaf1a776a189e318950b9486a7689907155))
    - Added version field for nu-plugin ([`c3083fd`](https://github.com/SuaveIV/nu_plugin_audio/commit/c3083fd2798630d3036038ae78cef21ee27148aa))
    - Add GitHub Actions workflow for automatic dependency updates ([`21adaa1`](https://github.com/SuaveIV/nu_plugin_audio/commit/21adaa164ace69d9cb1f579476e189566d53795d))
    - Update examples method signature to include lifetime parameter ([`0093417`](https://github.com/SuaveIV/nu_plugin_audio/commit/009341711ec629392875f9c8a0cbbb91edb237a9))
    - Add rust-toolchain configuration file ([`45bd162`](https://github.com/SuaveIV/nu_plugin_audio/commit/45bd16201b4d9918fe86fa820ea07026f94caab9))
    - Update nu-plugin and dependencies to version 0.105.1 ([`9119a1d`](https://github.com/SuaveIV/nu_plugin_audio/commit/9119a1d3f8c6c64a96aea770b20ed0013a8dafbc))
    - Update dependencies and version to 0.104.0 ([`a67c093`](https://github.com/SuaveIV/nu_plugin_audio/commit/a67c093edcf0e9005f134e2d821a44ff8420f092))
    - Remove unnecessary echo statement from build.nu ([`09a548f`](https://github.com/SuaveIV/nu_plugin_audio/commit/09a548fca38fd7a8ea3fb90502df3f4d051e60b4))
    - Update nu_plugin_audio_hook to version 0.103.0 and bump dependencies ([`1c3a1b7`](https://github.com/SuaveIV/nu_plugin_audio/commit/1c3a1b798dc0875af9dded383aa143e3566652b2))
    - Update nu_plugin_audio_hook to version 0.102.0 and bump dependencies ([`9a7d7d2`](https://github.com/SuaveIV/nu_plugin_audio/commit/9a7d7d23d0aeffa11a154887541dcde17344d763))
    - Nupm package version ([`8c92903`](https://github.com/SuaveIV/nu_plugin_audio/commit/8c9290301e672bddef77f6c92c5929144c2f7c5e))
    - Bump nu-protocol to v0.101.0 ([`4a0ef45`](https://github.com/SuaveIV/nu_plugin_audio/commit/4a0ef45f94b01cc069220e039778be31ff1d0cc8))
    - Update packages ([`e2061a9`](https://github.com/SuaveIV/nu_plugin_audio/commit/e2061a932043792ca517478a367f6eb991d56c05))
    - Upgraded to nu-protocol v0.100.0 ([`40518de`](https://github.com/SuaveIV/nu_plugin_audio/commit/40518de058b294cbb23348d2a09253c340d8716a))
    - Bump nu-protocol to 0.99 ([`6fde8d3`](https://github.com/SuaveIV/nu_plugin_audio/commit/6fde8d3a232bff33b8985ccfdf5018834f58d7ac))
    - Bump nu-protocol to v0.98.0 ([`2f77886`](https://github.com/SuaveIV/nu_plugin_audio/commit/2f778866fa580367000b7125d66cef4940e4f931))
    - Merge branch 'main' of https://github.com/FMotalleb/nu_plugin_audio_hook ([`9aeb785`](https://github.com/SuaveIV/nu_plugin_audio/commit/9aeb785fae5ed5736e7abe8c04031d2605a9a348))
    - Permission issue on apt ([`c88864f`](https://github.com/SuaveIV/nu_plugin_audio/commit/c88864f5f37cc3c999e0e88a1c60c8131fbfcbb3))
    - Changed `register` to `plugin add` in README ([`2282855`](https://github.com/SuaveIV/nu_plugin_audio/commit/22828558d23d47e39777760a42b5d5cd64f84fb6))
    - Added alsa sys package to workflow ([`0069342`](https://github.com/SuaveIV/nu_plugin_audio/commit/0069342830baefafbeba42e04eaae55857c5b6fd))
    - Workflow trigger ([`ffc95ad`](https://github.com/SuaveIV/nu_plugin_audio/commit/ffc95ad22865e1326a927eec085de5d0f2c94a17))
    - Bump rodio version to 0.19.0 ([`4d02c13`](https://github.com/SuaveIV/nu_plugin_audio/commit/4d02c133629c429b164f97bf846b9f5f12ef8a50))
    - Updated packages ([`f71f448`](https://github.com/SuaveIV/nu_plugin_audio/commit/f71f448a430baf778d7a848bf1c1d232490933ee))
    - Auto publish script ([`6831b19`](https://github.com/SuaveIV/nu_plugin_audio/commit/6831b19bf4636434573f44ebcbe90e2c4ad241b8))
    - Lockfile dependencies ([`6a640e8`](https://github.com/SuaveIV/nu_plugin_audio/commit/6a640e8ff5a6fd833937af9628916d487a138062))
    - Nupm build script feature list ([`f7e4f20`](https://github.com/SuaveIV/nu_plugin_audio/commit/f7e4f203e16a9106856499c05b524404a2997df9))
    - Bump nu-protocol to version 0.97.1 ([`d3924c7`](https://github.com/SuaveIV/nu_plugin_audio/commit/d3924c7e4a422558430b32031e527dd6d0aad684))
    - Updated nu-protocol to 0.96.0 ([`fc21199`](https://github.com/SuaveIV/nu_plugin_audio/commit/fc21199bd842e3f74c57568879adc91630162156))
    - Bump nu deps to v0.95 ([`332e123`](https://github.com/SuaveIV/nu_plugin_audio/commit/332e1232c820905a460f1d8e120bd87988779b09))
    - Bump nu-protocol to v0.94 ([`dd8f83a`](https://github.com/SuaveIV/nu_plugin_audio/commit/dd8f83a3f3a2c56fcd8106a8a9705270deccd462))
    - `sound meta -a` error ([`1e7f55b`](https://github.com/SuaveIV/nu_plugin_audio/commit/1e7f55b1e8c395d4179a4de3afb0e89e7b05b766))
    - Merge branch 'main' of https://github.com/FMotalleb/nu_plugin_audio_hook ([`10adbf2`](https://github.com/SuaveIV/nu_plugin_audio/commit/10adbf2cee1d38c6be9921194fec032e620f1543))
    - Versioning ([`7bfa0f3`](https://github.com/SuaveIV/nu_plugin_audio/commit/7bfa0f33313b866b34c190794f781f53780650a8))
    - Merge pull request #2 from FMotalleb:nu-93 ([`461b53f`](https://github.com/SuaveIV/nu_plugin_audio/commit/461b53f6e7dc9b3769224f3bcae84c86bf833c09))
    - Bump id3 version ([`d73259f`](https://github.com/SuaveIV/nu_plugin_audio/commit/d73259f2473e79653f11890091a7af5b789a1230))
    - Bump to nu-protocol v0.93 ([`31cf298`](https://github.com/SuaveIV/nu_plugin_audio/commit/31cf298aaa67c0a0141b063bc1e88dfd0a8a25e7))
    - Bump nu-protocol version to 0.93 ([`5aa55fb`](https://github.com/SuaveIV/nu_plugin_audio/commit/5aa55fb893fd0e952158cf8b269063c393a27701))
    - Bump nu packages to 0.90.1 ([`c981f95`](https://github.com/SuaveIV/nu_plugin_audio/commit/c981f95ed95127d6c295d3e7481d20f420fce889))
    - Bump versions ([`fb4b87e`](https://github.com/SuaveIV/nu_plugin_audio/commit/fb4b87ed7717ac0604461079df2316dd57602d32))
    - Bump nu-plugin and protocol version ([`4d9af48`](https://github.com/SuaveIV/nu_plugin_audio/commit/4d9af48f0b8918c380233305e8b1373f202c03b7))
    - Bump nu-plugin and protocol versions to 0.88.1 ([`98e81f5`](https://github.com/SuaveIV/nu_plugin_audio/commit/98e81f58dbb90a34379e7e22a7ea747cb0e947bc))
    - Rename package.nuon to nupm.nuon ([`d2eb046`](https://github.com/SuaveIV/nu_plugin_audio/commit/d2eb046650bf151ad2efaf8e012742f12821d98c))
    - Commit from GitHub Actions (Update dependencies) ([`51274c2`](https://github.com/SuaveIV/nu_plugin_audio/commit/51274c293c4564c4ed445696f4346b8ff34f7683))
    - Update dependency-update.yaml ([`680bde0`](https://github.com/SuaveIV/nu_plugin_audio/commit/680bde009f563f851cea2b4e6dc7bff82aec33ea))
    - Commit from GitHub Actions (Update dependencies) ([`8eebaec`](https://github.com/SuaveIV/nu_plugin_audio/commit/8eebaecea1cdcdf057740da0749392d4ac3be5c5))
    - Commit from GitHub Actions (Update dependencies) ([`2512b32`](https://github.com/SuaveIV/nu_plugin_audio/commit/2512b32fe27be94aa87a52fb0ef9e951223fa06d))
    - Commit from GitHub Actions (Update dependencies) ([`832268e`](https://github.com/SuaveIV/nu_plugin_audio/commit/832268ef5874e4efed9789529af89a627c6528c5))
    - Commit from GitHub Actions (Update dependencies) ([`cd08aa7`](https://github.com/SuaveIV/nu_plugin_audio/commit/cd08aa7f2a6608cd0df91f99c9d827622ca3519e))
    - Auto update dependencies ([`23d2556`](https://github.com/SuaveIV/nu_plugin_audio/commit/23d2556ffea229859da64c5c556f300ff46cb626))
    - Minor ([`66aec72`](https://github.com/SuaveIV/nu_plugin_audio/commit/66aec722be2b7aad7fc2d79814c947e8d485f554))
    - Merge branch 'main' of github.com:FMotalleb/nu_plugin_audio_hook ([`226df96`](https://github.com/SuaveIV/nu_plugin_audio/commit/226df96e7b0e67f101130cee42940559056d419f))
    - Bump nushell integration version ([`4fd1d4a`](https://github.com/SuaveIV/nu_plugin_audio/commit/4fd1d4a7e0fd522b0e42599f88ab82f9160beb11))
    - Update build.nu ([`2bed317`](https://github.com/SuaveIV/nu_plugin_audio/commit/2bed317874ee94e5de158bae8aeb206799070c98))
    - Update README.md ([`5110a3c`](https://github.com/SuaveIV/nu_plugin_audio/commit/5110a3c960fbea7c56809431d3043d5c06a6ee74))
    - Added nupm section ([`4a8b4d4`](https://github.com/SuaveIV/nu_plugin_audio/commit/4a8b4d4bad7f221992c88ebdab9437a63e7770dc))
    - [Ench] nupm support ([`9b38d44`](https://github.com/SuaveIV/nu_plugin_audio/commit/9b38d44d7ce1d47eb2f7a10e7530e15e9113fa7d))
    - [Minor-Fix] tags ([`69236f4`](https://github.com/SuaveIV/nu_plugin_audio/commit/69236f4680bcb207ffb6cee7abf4645422842cc7))
    - [Minor] sound meta list ([`99964d5`](https://github.com/SuaveIV/nu_plugin_audio/commit/99964d520559a1f22bdae3e15615230e2f61bae6))
</details>

## v0.1.2 (2023-11-07)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [Docs] ([`4c8a8ed`](https://github.com/SuaveIV/nu_plugin_audio/commit/4c8a8edea3ddf371ff92494cb5a758916bebaf4a))
    - [Fix-typo] ([`e769911`](https://github.com/SuaveIV/nu_plugin_audio/commit/e769911e7c8d7801799a6156c2bee9091d330f98))
    - [Versioning] ([`5adca67`](https://github.com/SuaveIV/nu_plugin_audio/commit/5adca673a792fd3a28136173a8cd4819fa626b24))
    - [Ench] ability to set id3 frames ([`bc32d84`](https://github.com/SuaveIV/nu_plugin_audio/commit/bc32d8487184ecabf4a6b8a76978e6cbf857c300))
</details>

## v0.1.1 (2023-11-07)

<csr-id-0e2b1039d49a088f0f9de18585019ff0b642d313/>

### Other

 - <csr-id-0e2b1039d49a088f0f9de18585019ff0b642d313/> sound make

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [Docs] ([`54c26cc`](https://github.com/SuaveIV/nu_plugin_audio/commit/54c26cc9f25eab027465ca37b7db477601558dce))
    - [Minor] added help message ([`ec115b7`](https://github.com/SuaveIV/nu_plugin_audio/commit/ec115b7a7a1c3fea80ffb86540f5941176f4f094))
    - [Minor] ([`6dbaef6`](https://github.com/SuaveIV/nu_plugin_audio/commit/6dbaef618b4a46b18bac2998a7fbedc678e32356))
    - [Minor] ([`c7a648a`](https://github.com/SuaveIV/nu_plugin_audio/commit/c7a648a33f3f53062246f32296f3a88688675b39))
    - [Minor] ([`a2e3024`](https://github.com/SuaveIV/nu_plugin_audio/commit/a2e3024f59566cbae10f2d2950721dd24388b09a))
    - [Docs] ([`8e1b815`](https://github.com/SuaveIV/nu_plugin_audio/commit/8e1b815e4b4ae267abca9758e7b8ca35a3b3834e))
    - [Feat] sound meta ([`b6529d0`](https://github.com/SuaveIV/nu_plugin_audio/commit/b6529d0a9a4079a5c5f6f0a394d8c13eba459484))
    - [Docs+Features] ([`ebd2afa`](https://github.com/SuaveIV/nu_plugin_audio/commit/ebd2afa3110d4bf53d96b869f5f4d45a0d554bda))
    - [Minor] minor ([`81574ed`](https://github.com/SuaveIV/nu_plugin_audio/commit/81574ed4998b00fb504c5b6427b6a50331ac6b49))
    - [Feat] Audio player ([`b267915`](https://github.com/SuaveIV/nu_plugin_audio/commit/b267915f01a80375f6cfccb3ac2f7cee817d5524))
    - [Feat] sound beep ([`cc8fde6`](https://github.com/SuaveIV/nu_plugin_audio/commit/cc8fde6be06a2382344fe936297b121e5991dbfe))
    - [Minor] ([`2076b8f`](https://github.com/SuaveIV/nu_plugin_audio/commit/2076b8f49d2a163c7311cceccf5fc95f83535900))
    - [Docs] ([`618c07d`](https://github.com/SuaveIV/nu_plugin_audio/commit/618c07d904a7156a99d2538e02cd21a8ad818787))
    - [Minor] reducing binary size in release ([`0789fb5`](https://github.com/SuaveIV/nu_plugin_audio/commit/0789fb5122954ae465605c5a46cd6ead7ff742e6))
    - [Ref] ([`c7b4b91`](https://github.com/SuaveIV/nu_plugin_audio/commit/c7b4b91f0435eecd8d65ab1da2ff168db1112c18))
    - Sound make ([`0e2b103`](https://github.com/SuaveIV/nu_plugin_audio/commit/0e2b1039d49a088f0f9de18585019ff0b642d313))
    - [Init] ([`26d21f1`](https://github.com/SuaveIV/nu_plugin_audio/commit/26d21f1dd6ba01a83bcd5208a6197846daa40576))
</details>


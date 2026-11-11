# VST3, AU, AAX and LV2

The native formats wrap the same engine, editor page and state record as the
CLAP plug-in. `crates/dw303-native` exposes the engine through a small C ABI
(`include/dw303.h`); `native/` holds one adapter per format, modelled on the
Shine Reverb RAVET port. All C++ stays in those adapters.

| Platform | Formats | Panel editor |
|---|---|---|
| macOS 11+ (Apple Silicon, Intel) | VST3, AU, AAX, LV2, CLAP, standalone | Cocoa/WebKit |
| Windows 10/11 x64 | VST3, CLAP, standalone | Win32/WebView2 (needs the Edge WebView2 Runtime) |
| Linux (x86_64, arm64) | VST3, LV2, CLAP, standalone | GTK 3/WebKitGTK 4.1 over X11 or XWayland |

Every editor is RAVET's web view (`ravet-webview`) showing `editor/index.html`.

| Format | Adapter | Identity |
|---|---|---|
| VST3 | `native/vst3.cpp`: single component (processor and controller), event input bus | class `6EBA43A8-0B10434D-BB6916BD-D029E63A`, `Instrument\|Synth` |
| AUv2 | `native/au.mm`: `MusicDeviceBase`, Cocoa view factory | `aumu` `D303` `Djwh` |
| AAX | `native/aax.cpp`: Native, MIDI input and transport nodes | manufacturer `Djwh`, product `D303`, native `D3n2` |
| LV2 | `native/lv2.cpp`: atom MIDI/time input, Cocoa (macOS) or X11 (Linux) UI via instance-access | `https://djwaudio.co/plugins/dw-303` |

Every format has stereo output, MIDI note input, the twelve parameters with
the CLAP parameter IDs, host transport for SYNC = Host, the full panel editor
and the CLAP state record, so a session's pattern memory moves between
formats. AU, VST3 and AAX report panel and sequencer changes (for example
RUN/STOP from the panel) back to the host. AU also accepts MIDI clock. LV2
hosts own their control port values, so engine-originated changes are not
reported there.

Windows and X11 editors use physical pixels: VST3 `IPlugViewContentScaleSupport`
and CLAP `set_scale` widen the permitted panel size. Linux editors run GTK from
the host's UI loop (VST3 `Linux::IRunLoop` timer, CLAP `timer-support`, LV2
`ui:idleInterface`); a host that offers none of these gets generic controls, as
does a host that has already loaded another GTK (GTK 2/4, Ardour's YTK).

## Build

`scripts/bundle-plugins.py` builds every format for the platform it runs on
and stages it under `target/plugins`; it installs nothing.

### macOS

Requires the RAVET checkout beside this repository, Rust with both macOS
targets, CMake 3.25+, Ninja, Xcode command-line tools and external SDKs in one
directory (default `target/sdks`, git-ignored):

```text
target/sdks/vst3          Steinberg VST3 SDK 3.8 (MIT)
target/sdks/AudioUnitSDK  Apple AudioUnitSDK (Apache-2.0)
target/sdks/AAX           Avid AAX SDK (GPLv3 option); optional
target/sdks/lv2           LV2 headers (ISC); optional
```

```sh
rustup target add aarch64-apple-darwin x86_64-apple-darwin
python3 scripts/bundle-plugins.py --universal --check
```

Bundles are staged, ad hoc signed, under `target/plugins/{VST3,AU,AAX,LV2,CLAP,Standalone}`
with `LICENSE.txt` and `THIRD-PARTY.txt`. Nothing is installed. `--check` runs:

- Steinberg's VST3 validator (47 tests).
- `dw303-au-smoke`: registers the component in-process, renders a MIDI note,
  drives a knob and RUN/STOP through the editor page, plays the factory
  pattern and round-trips state.
- `dw303-lv2-smoke`: a minimal LV2 host that renders a note and round-trips state.
- AAX SDK entry points. Pro Tools loads only PACE-signed AAX bundles, so AAX
  loading is verified with a release build.

`auval -v aumu D303 Djwh` also passes when the component is installed.

### Windows

From a Developer PowerShell for VS 2022 (x64), with Rust (MSVC toolchain),
CMake 3.25+, Ninja and Python 3.11+. SDKs in `target/sdks`:

```text
target/sdks/vst3          Steinberg VST3 SDK 3.8 (MIT)
target/sdks/WebView2      Microsoft.Web.WebView2 NuGet package, extracted (1.0.3650.58)
```

The `.nupkg` is a zip: download it from
<https://www.nuget.org/packages/Microsoft.Web.WebView2> and extract it there
(or point `RAVET_WEBVIEW2_SDK` at another copy). Then:

```powershell
py -3 scripts\bundle-plugins.py --check
```

This stages `VST3\DW-303.vst3`, `CLAP\DW-303.clap` and `Standalone\DW-303.exe`,
each with the SDK's `WebView2Loader.dll` beside the binary that loads it, and
`--check` runs the VST3 validator. Its licence is added to `THIRD-PARTY.txt`.

#### Cross-building Windows on Linux

The released Windows installer is built on Linux with MinGW-w64, inside
`installer/windows/cross.Dockerfile` (MinGW, Rust, Wine and Inno Setup):

```sh
docker build -t dw303-cross:win -f installer/windows/cross.Dockerfile installer/windows
# from the directory that holds both ravet/ and dw-303/:
docker run --rm -v "$PWD:/w" -e CARGO_TARGET_DIR=/w/dw-303/target -w /w/dw-303 dw303-cross:win \
  bash -c 'python3 scripts/bundle-plugins.py --cross-windows --check && installer/build-windows-installer.sh --skip-build'
```

`--cross-windows` stages into `target/plugins-windows` with self-contained
binaries (static libstdc++, libgcc and winpthread; only Windows system DLLs are
imported), assembles the VST3 bundle itself (the SDK only does so on a Windows
host) and, with `--check`, runs the VST3 validator under Wine.
`installer/windows/mingw-w64.cmake` is the CMake toolchain; it shims the
SDK's unused `std::aligned_alloc` call, which MinGW lacks.
`installer/build-windows-installer.sh` compiles `installer/dw-303.iss` with
Inno Setup under Wine.

### Linux

Debian/Ubuntu packages: `build-essential cmake ninja-build pkg-config
libgtk-3-dev libwebkit2gtk-4.1-dev libjson-glib-dev libasound2-dev`, plus Rust,
Python 3.11+ and `vst3`/`lv2` in `target/sdks` as above. Then:

```sh
python3 scripts/bundle-plugins.py --check
```

This stages `VST3/DW-303.vst3`, `LV2/dw-303.lv2`, `CLAP/DW-303.clap` and
`Standalone/DW-303`; `--check` runs the VST3 validator and `dw303-lv2-smoke`.
RAVET and Rust symbols are kept private to each Linux binary, so the CLAP,
VST3 and LV2 copies can load in one host process. Linux builds paint the panel
with WebKit's software path (`RAVET_WEB_LINUX_SOFTWARE_RENDERING=1`, set by the
script): WebKitGTK's DMA-BUF/GBM path painted nothing with NVIDIA's driver under
XWayland. The setting applies to the editor's own views only.

## Release installer

```sh
installer/build-mac-installer.sh
```

Builds the universal bundles, signs them with `scripts/sign-macos.sh` (PACE
`wraptool` for AAX), and produces `target/installer/DW-303-<version>.pkg` with
one choice per format, inside `DW-303 <version>.dmg`. Signing, notarization
and PACE settings come only from the environment variables listed at the top
of the script, or from `~/.config/djwaudio/dw303-signing.env` if that file
exists (it is sourced first and stays outside the repository). `SKIP_SIGN=1` builds an unsigned local test installer;
`SKIP_AAX=1` leaves AAX out.

The website serves the DMG as `downloads/dw-303-mac-latest.dmg`.

## Windows installer

Prerequisites (x64): Visual Studio 2022 Build Tools (Desktop development with
C++), Rust with the `x86_64-pc-windows-msvc` toolchain, CMake 3.25+, Ninja,
Python 3.11+ (with the `py` launcher), [Inno Setup 6.3+](https://jrsoftware.org/isinfo.php)
and the Microsoft WebView2 SDK. Run from a Developer PowerShell for VS 2022:

```powershell
powershell -ExecutionPolicy Bypass -File installer\build-windows-installer.ps1
```

The script reads the version from `Cargo.toml`, stages the bundles with
`py -3 scripts\bundle-plugins.py` (skip with `-SkipBuild`), checks every staged
file, finds `ISCC.exe` (PATH, then `Program Files (x86)\Inno Setup 6`) and
compiles `installer/dw-303.iss` into
`target/installer/DW-303-<version>-windows-x64.exe`. Windows 10+, x64 only,
per machine, with one component per format:

| Component | Installed to |
|---|---|
| VST3 | `C:\Program Files\Common Files\VST3\DW-303.vst3` |
| CLAP | `C:\Program Files\Common Files\CLAP\DJW Audio\` (`DW-303.clap`, `WebView2Loader.dll`) |
| Standalone | `C:\Program Files\DJW Audio\DW-303\` with a Start menu shortcut (desktop icon optional) |

`LICENSE.txt`, `THIRD-PARTY.txt` and the uninstaller go to the app folder.
The panel editor needs the Microsoft Edge WebView2 Runtime; Setup does not
bundle it, but warns when it is missing and offers to open the download page.
The installer is unsigned for now; `installer/dw-303.iss` has a commented
`SignTool` hook. The website serves it as `downloads/dw-303-win-latest.exe`
(the script writes that copy too).

## Linux tarball

Prerequisites (x86_64, Debian/Ubuntu names): `build-essential cmake ninja-build
pkg-config libgtk-3-dev libwebkit2gtk-4.1-dev libjson-glib-dev libasound2-dev`,
Rust and Python 3.11+.

```sh
installer/build-linux-tarball.sh            # --skip-build to reuse target/plugins
```

Release tarballs are built on an Ubuntu 22.04 baseline so the binaries need
only glibc 2.35 (Ubuntu 22.04+, Debian 12, Mint 21+, current Fedora and Arch).
Building on a rolling distribution raises that requirement. With Docker:

```sh
docker build -t dw303-release:22.04 -f installer/linux/release.Dockerfile installer/linux
# from the directory that holds both ravet/ and dw-303/:
docker run --rm -v "$PWD:/w" -e CARGO_TARGET_DIR=/w/dw-303/target -w /w/dw-303 dw303-release:22.04 \
  bash -c 'python3 scripts/bundle-plugins.py --check && installer/build-linux-tarball.sh --skip-build'
```

Stages the bundles with `python3 scripts/bundle-plugins.py`, checks them and
writes `target/installer/DW-303-<version>-linux-x86_64.tar.gz` (sorted entries,
root ownership, mtimes from `SOURCE_DATE_EPOCH` or the last commit; GNU tar
needed for a byte-reproducible archive). The tarball holds `VST3/`, `CLAP/`,
`LV2/`, `Standalone/`, `install.sh`, `uninstall.sh`, `README.txt` and the
licence files. `./install.sh` installs per user (`~/.vst3`, `~/.clap`, `~/.lv2`,
app in `~/.local/lib/dw-303`, `~/.local/bin/dw-303`, a desktop entry);
`sudo ./install.sh --system` installs under `/usr/local`; `--prefix DIR` and
`--dry-run` are also accepted. It warns, with the package-manager command,
when WebKitGTK 4.1, GTK 3, json-glib or ALSA is missing. Every installed path
is recorded in `<lib dir>/install-manifest.txt`, and
`<lib dir>/uninstall.sh` removes exactly those. The panel needs GTK 3,
WebKitGTK 4.1 (2.40+) and X11 or XWayland. The plug-ins do not link those
libraries (RAVET loads them when the panel opens), so without them DW-303
still loads and plays, with the host's generic controls instead of the panel.
That includes Flatpak hosts, whose runtimes do not see system libraries and
lack WebKitGTK (Bitwig Studio's Flatpak), and hosts that load a different GTK
(for example Ardour). The website
serves the tarball as `downloads/dw-303-linux-latest.tar.gz`.

## Limits

- macOS Intel slices are built but have not been run on Intel hardware.
- Windows: the released build is MinGW-w64. Under Wine: VST3 validator 47/47,
  the CLAP plays a note, and the installer installs, detects a missing WebView2
  Runtime and uninstalls cleanly. On a Windows 11 24H2 VM (WebView2 156): the
  installer upgrades in place, the standalone app shows its panel and starts
  WASAPI output, and REAPER 7.82 loads the VST3 and CLAP, shows both panels,
  survives closing and reopening them, and renders the same audio as Linux. The
  MSVC build path is documented but unexercised. The installer is unsigned, so
  SmartScreen warns; AAX for Windows needs PACE signing there.
- Linux was exercised on x86_64 (Ubuntu 22.04 release container) and arm64
  (Ubuntu 24.04) under Xvfb: the VST3 validator, the LV2 smoke host, the
  standalone UI test, and the CLAP and VST3 editors embedded in a bare X11 host
  window; the tarball installs and uninstalls on Arch. REAPER 7.82 on Arch
  (Hyprland, XWayland, NVIDIA) loads the VST3, CLAP and LV2, shows all three
  panels, survives closing and reopening them, and renders identical audio.
  Bitwig and Ardour (generic controls expected) are unchecked. Native Wayland parents are not
  supported; XWayland is.
- 32-bit float processing.
- Hosts tested so far: the offline checks above. Loading in Logic, Live,
  Cubase, Pro Tools, REAPER and Ardour still needs checking by hand.

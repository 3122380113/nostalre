# nostalRE

A native Renewal client for desktop and Android, built in Rust with wgpu.
Android app builds pair the client with local servers for offline play.

![Rust](https://img.shields.io/badge/Rust-1.97.1-DEA584?style=flat&logo=rust&logoColor=white)
![wgpu](https://img.shields.io/badge/Rendering-wgpu_28-5E4AE3?style=flat)
![winit](https://img.shields.io/badge/Windowing-winit_0.30-455A64?style=flat)
![Android](https://img.shields.io/badge/Android-11%2B-3DDC84?style=flat&logo=android&logoColor=white)
![Protocol](https://img.shields.io/badge/Protocol-20250716-1976D2?style=flat)
[![Based on nostalro-client](https://img.shields.io/badge/Based_on-nostalro--client-607D8B?style=flat)](https://github.com/nmeylan/nostalro-client)
[![AI-assisted development](https://img.shields.io/badge/Development-AI--assisted-8E44AD?style=flat)](#ai-use-disclosure)

[Download APK](#download-the-apk) · [Contribute](#for-contributors) · [Attribution](#attribution) · [AI use](#ai-use-disclosure)

## Download the APK

**[Open GitHub Releases](https://github.com/si-klyde/nostalre/releases)** and choose
an attached `.apk` under **Assets**. If a release has no APK attached, no
installable download is available from that release.

You need:

| Requirement | Details |
| --- | --- |
| Android | Android 11 or newer (API 30+) |
| Architecture | 64-bit ARM (`arm64-v8a`) |
| Game resources | Your own compatible installation, including `DATA.INI` and its referenced GRF files |

### Install and play

1. Download the APK to your device and open it. Allow installation from the app you used to download it if Android asks.
2. Place your compatible resource folder on the device. Keep `DATA.INI` and the archives it references together.
3. Open the app, grant the requested file access, and select that folder.
4. Choose your local username, create or select a character, and play.

The Android app runs its servers locally. You do not need to build the project
or run a separate server on your computer to use the APK.

Resource folders can include optional `BGM/` music and `data/` overrides. The
repository and APK do not supply original game archives, client executables,
music, or screenshots; bring your own compatible resources.

For problems, [open an issue](https://github.com/si-klyde/nostalre/issues) with
the APK version, device model, Android version, and steps to reproduce.

## For contributors

This repository contains the native Rust client and its resource tools.
The Android launcher, local servers, and APK packaging are maintained in the
separate integration project. The Android build instructions below produce the
client library; APK packaging also requires that integration project.

Development is ongoing. See [Features.md](Features.md) and the
[protocol coverage report](docs/packet-status-20250716.md) for current implementation details.

### Set up a desktop checkout

Install Rust through rustup; [rust-toolchain.toml](rust-toolchain.toml) selects
Rust `1.97.1`. Desktop play also requires a compatible rAthena Renewal server
built with `PACKETVER=20250716` and your own resource files.

On Debian/Ubuntu, install the native build dependencies:

```sh
sudo apt-get install pkg-config libasound2-dev libx11-dev libxkbcommon-dev libwayland-dev
```

```sh
git clone https://github.com/si-klyde/nostalre.git
cd nostalre
cp config.sample.json config.json
```

Edit `config.json` for your server and resource installation:

```json
{
  "login_servers": [
    { "name": "Local", "host": "127.0.0.1", "port": 6900 }
  ],
  "grf_paths": ["/path/to/install/data.grf"],
  "install_dir": "/path/to/install",
  "bgm_path": "/path/to/install/BGM"
}
```

`install_dir` identifies the installation folder containing `System` and
`SystemEN` tables. Archives in `grf_paths` are checked in priority order;
the first wins. An optional `data_dir` provides loose overrides with paths
relative to the archive's `data/` folder.

Build and run from the repository root:

```sh
cargo build --release --bin nostalre-client
cargo run --release --bin nostalre-client
```

The client reads and saves `config.json` in the working directory. Local config,
`data/`, and build output are ignored by Git. See
[configuration](docs/configuration.md) for display, audio, input, and other settings.

### Build the Android native library

Install the Android NDK and set `ANDROID_NDK_HOME` to its directory, then:

```sh
rustup target add aarch64-linux-android
cargo install cargo-ndk
cargo ndk -t arm64-v8a -P 26 build --release -p nostalre-client --lib
```

Output: `target/aarch64-linux-android/release/libnostalre_client.so`.
`CARGO_TARGET_DIR` can select a different build directory. The native library's
build API level is separate from the packaged app's Android 11 minimum.

### Test and submit changes

```sh
cargo build --workspace --all-targets
cargo test --workspace
```

Some integration tests require locally supplied resources and return early when
those files are unavailable.

Keep contributions focused, use short commit messages, and describe the change
and how it was tested. For rendering or input issues, include reproduction steps
and relevant screenshots or recordings. Never commit game archives, extracted
game assets, credentials, or personal configuration.

### Tools and documentation

```sh
cargo run --release --bin sprite-viewer -- --grf data/data.grf
cargo run --release --bin nostalre-grf-editor
cargo run --release --bin grf-verify -- data/data.grf
```

Additional viewers and archive tools live in `tools/`. Hot-reload development
scripts require `cargo-watch`.

- [Architecture](docs/internal/architecture.md)
- [Rendering](docs/internal/rendering.md)
- [Configuration](docs/configuration.md)
- [Profiling](docs/profiling.md)

## Attribution

nostalRE is developed independently from **nostalro-client**, created by
**Nicolas Meylan (nmeylan)**. This project retains that upstream work and
extends its Renewal protocol support, rendering, interface, input, and platform
integration. Fresh repository history does not replace upstream attribution.

| Project | Contribution |
| --- | --- |
| [nostalro-client](https://github.com/nmeylan/nostalro-client) — Nicolas Meylan | Upstream native client; Apache-2.0 |
| [rust-ro](https://github.com/nmeylan/rust-ro) — Nicolas Meylan | Models, packets, movement, macros, and packet generator; MIT |
| [Korangar](https://github.com/vE5li/korangar) — Lucas Pütz | Referenced RSM model/animation implementations; MIT |
| [winit](https://github.com/rust-windowing/winit) | Windowing and input, including the local Android modification; Apache-2.0, with embedded W3C notices |
| [Lua](https://www.lua.org/) — Lua.org / PUC-Rio | Lua 5.1 runtime; MIT |
| Noto Project Authors / Adobe | Bundled Noto fonts; SIL Open Font License |
| [rAthena](https://github.com/rathena/rathena) — rAthena Development Team | Protocol and generated server datasets; source GPL notices retained |
| [AzzyAI](https://github.com/SpenceKonde/AzzyAI) — Dr. Azzy / Spence Konde | AI design credit and tactic defaults inherited through nostalro-client |

See [LICENSE.md](LICENSE.md), [NOTICE](NOTICE), and [LICENSES](LICENSES) for the
license texts and component details. The [bundled-content review](docs/third-party-content.md)
records remaining permission/license-scope questions for inherited tactic
defaults and generated server datasets.

The repository includes original charm artwork, OFL fonts, authored parser
fixtures, and vendored examples/test data. Game-specific compatibility tables
and resource names remain in the source.

## AI use disclosure

AI tools are used in this project's development, including implementation,
debugging, protocol and rendering work, tests, refactoring, and documentation.
This README was also written with AI assistance.

Validation includes builds, automated tests, and manual play checks. AI output
can contain mistakes; contributors should verify and understand generated
changes before submitting them. AI assistance does not replace the project's
attribution and licensing requirements.

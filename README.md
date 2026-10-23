# nostalRE

An independent native Rust client targeting rAthena Renewal with packet version
`20250716`. Derived from [nostalro-client](https://github.com/nmeylan/nostalro-client)
by Nicolas Meylan.

The client includes map, sprite, model, and effect rendering; a scalable interface;
keyboard, mouse, touch, and gamepad input; and tools for inspecting resource files.
Development is ongoing. See [Features.md](Features.md) and the
[protocol coverage report](docs/packet-status-20250716.md) for implementation details.

## Requirements

- Rust through rustup. [rust-toolchain.toml](rust-toolchain.toml) selects the pinned toolchain.
- A compatible Renewal server built with `PACKETVER=20250716`.
- Your own compatible game installation and resource files.

For Linux, install the native build dependencies. On Debian/Ubuntu:

```sh
sudo apt-get install pkg-config libasound2-dev libx11-dev libxkbcommon-dev libwayland-dev
```

## Build and run

```sh
git clone https://github.com/si-klyde/nostalre.git
cd nostalre
cp config.sample.json config.json
cargo build --release --bin nostalre-client
cargo run --release --bin nostalre-client
```

Before running, edit `config.json` to point to your server and resource files.
For example:

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
`SystemEN` tables. `grf_paths` lists archives in priority order: the first wins.
An optional `data_dir` supplies loose overrides, with paths relative to the
archive's `data/` folder.

Run from the repository root; the client reads and saves `config.json` there.
The local config, `data/`, and build output are ignored by Git. See
[configuration](docs/configuration.md) for display, audio, input, and other settings.

## Resource files

Supply your own game resources. No original game archives, client executables,
music, or screenshots are included in this repository.

The repository does include original charm artwork, OFL-licensed fonts,
authored parser test fixtures, and examples/test data within vendored dependencies.
Game-specific compatibility tables and resource names remain in the source.
Content origins and notices are documented in [NOTICE](NOTICE) and the
[bundled-content review](docs/third-party-content.md).

## Android native library

Android app/server packaging is maintained in a separate integration repository.
This repository builds the native client library.

With the Android NDK installed, set `ANDROID_NDK_HOME` to its directory, then:

```sh
rustup target add aarch64-linux-android
cargo install cargo-ndk
cargo ndk -t arm64-v8a -P 26 build --release -p nostalre-client --lib
```

Output: `target/aarch64-linux-android/release/libnostalre_client.so`.
`CARGO_TARGET_DIR` can select a different build directory.

## Development and tools

Build all workspace targets and run tests:

```sh
cargo build --workspace --all-targets
cargo test --workspace
```

Some integration tests require locally supplied resources and return early when
those files are unavailable.

Resource tools:

```sh
cargo run --release --bin sprite-viewer -- --grf data/data.grf
cargo run --release --bin nostalre-grf-editor
cargo run --release --bin grf-verify -- data/data.grf
```

Additional viewers and archive tools live in `tools/`. Hot-reload development
scripts require `cargo-watch`.

Technical documentation: [architecture](docs/internal/architecture.md),
[rendering](docs/internal/rendering.md), and [profiling](docs/profiling.md).

## Origin and licenses

nostalRE is developed independently from its upstream project, nostalro-client.
Changes include Renewal protocol support, rendering, interface, input, and
platform integration. The repository starts with fresh history and retains
upstream attribution.

The upstream client uses [Apache-2.0](LICENSE.md). Incorporated components retain
their applicable notices in [NOTICE](NOTICE), [LICENSES](LICENSES), and their
source directories. The [bundled-content review](docs/third-party-content.md)
records the remaining permission/license-scope questions for inherited tactic
defaults and generated server datasets.

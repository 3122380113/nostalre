# nostalRE

Native Rust client for Renewal servers using packet version `20250716`.

Derived from [nostalro-client](https://github.com/nmeylan/nostalro-client) by
Nicolas Meylan. nostalRE is developed independently; upstream attribution and
third-party notices are preserved in [NOTICE](NOTICE) and [LICENSES](LICENSES).

This checkout is a release preparation workspace. Implementations are retained;
the remaining bundled-content/license questions are documented in
[docs/third-party-content.md](docs/third-party-content.md).

## Build and test

Install Rust through rustup; `rust-toolchain.toml` pins the toolchain.
Linux builds need `pkg-config`, `libasound2-dev`, `libx11-dev`,
`libxkbcommon-dev`, and `libwayland-dev`.

```sh
cargo build --workspace --all-targets
cargo test --workspace
```

## Run

```sh
cp config.sample.json config.json
cargo run --release --bin nostalre-client
```

Provide a compatible server and your own resource files. This export contains
no game archives, client executables, or music. Configure `grf_paths`,
`data_dir`, and `login_servers` in your local `config.json`. The sample connects
to `127.0.0.1:6900` and reads `data/data.grf`.

Configuration details: [docs/configuration.md](docs/configuration.md).
Architecture: [docs/internal/architecture.md](docs/internal/architecture.md).
Rendering: [docs/internal/rendering.md](docs/internal/rendering.md).

## Tools

```sh
cargo run --release --bin sprite-viewer
cargo run --release --bin nostalre-grf-editor
cargo run --release --bin grf-verify -- data/data.grf
```

Development scripts in `tools/` support hot reload with `cargo-watch`.

## Android

With the Android NDK, the `aarch64-linux-android` Rust target, and `cargo-ndk` installed:

```sh
cargo ndk -t arm64-v8a -P 26 build --release -p nostalre-client --lib
```

The output is `target/aarch64-linux-android/release/libnostalre_client.so` unless
`CARGO_TARGET_DIR` selects another build directory. Android app and server
packaging remain in the separate integration repository; this checkout builds
the native client library.

## License and origin

The client is derived from Apache-2.0-licensed nostalro-client; see
[LICENSE.md](LICENSE.md). Incorporated components retain their own licenses;
see [NOTICE](NOTICE). Changes include Renewal protocol support, rendering,
interface, input, and platform integration.

This repository starts with fresh history. Earlier integration history and
private executable research are excluded; attribution remains preserved.

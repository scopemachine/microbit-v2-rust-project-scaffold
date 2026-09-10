# BBC micro:bit v2 Project Scaffold

Extracted and adapted from the [Rust Discovery Book](https://github.com/rust-embedded/discovery) (its micro:bit chapters) and the [nrf-rs/microbit](https://github.com/nrf-rs/microbit) board support crate.

A ready-to-clone starting point for `no_std` Rust firmware on the **BBC micro:bit v2** (nRF52833), pinned to a specific Rust toolchain and wired up so `cargo run` builds, flashes, and streams logs with no extra flags.

> **Board:** micro:bit **v2** only (nRF52833, Cortex-M4F, target `thumbv7em-none-eabihf`). Not compatible with the v1 board (nRF51822, Cortex-M0) — that's a different target, HAL crate, and linker memory map. Discovery and the `nrf-rs/microbit` repo both support v1 via a `microbit`/`v1` feature; this scaffold drops that split to stay a plain, single-target starter.

## How this scaffold works

Unlike the ESP-IDF side of this project, there's no environment to activate — `rustup` reads [rust-toolchain.toml](rust-toolchain.toml) and transparently fetches the pinned toolchain (and the `thumbv7em-none-eabihf` target) the first time you run `cargo` in this directory.

Flashing and running is handled by [`probe-rs`](https://probe.rs/) acting as the Cargo runner: [.cargo/config.toml](.cargo/config.toml) sets `target.thumbv7em-none-eabihf.runner` to `probe-rs run`, so `cargo run --release` builds the ELF, flashes it over the board's built-in CMSIS-DAP debug probe, resets it, and streams [`defmt`](https://defmt.ferrous-systems.com/) log output back over RTT — no separate flash tool, OpenOCD config, or serial terminal needed.

## 1. Install prerequisites

- **Rust toolchain**: install `rustup` from [rustup.rs](https://rustup.rs) if you don't have it. The pinned toolchain and target install automatically on first use in this directory.
- **[`probe-rs`](https://probe.rs/docs/getting-started/installation/)** — flashes and debugs the board, and its `cargo run` integration replaces `openocd`/`gdb` for everyday use:

  ```sh
  cargo install --locked probe-rs-tools
  ```

- **[`flip-link`](https://github.com/knurling-rs/flip-link)** — a linker wrapper that turns stack overflows into a clean hard fault instead of silent RAM corruption. Referenced from `rustflags` in `.cargo/config.toml`:

  ```sh
  cargo install --locked flip-link
  ```

- **Linux only — udev rule** so `probe-rs` can access the board's USB debug interface without `sudo`:

  ```sh
  sudo cp udev/69-microbit.rules /etc/udev/rules.d/
  sudo udevadm control --reload
  sudo udevadm trigger   # or just unplug/replug the board
  ```

## 2. Build, flash, run

Plug in the micro:bit v2 over USB, then:

```sh
cargo run --release
```

This builds for `thumbv7em-none-eabihf` (set as the default target in `.cargo/config.toml`), flashes it via `probe-rs`, and prints `defmt` log lines to your terminal as the firmware runs. `Ctrl+C` stops watching logs (the firmware keeps running on the board).

`cargo build --release` alone compiles without touching the hardware — useful with no board attached, e.g. in an editor or CI. `probe-rs list` shows whether a debug probe is currently detected.

The starter firmware in [src/main.rs](src/main.rs) logs `"Hello, micro:bit v2!"` once, then blinks the top-left LED of the 5x5 display matrix, logging `"blink"` on every cycle.

## Editor support (rust-analyzer)

Because the default Cargo target here is the embedded `thumbv7em-none-eabihf` triple rather than your host, rust-analyzer needs to be told the same thing so it type-checks against the right standard library. [.vscode/settings.json](.vscode/settings.json) sets `rust-analyzer.cargo.target` to match; other editors need the equivalent rust-analyzer config (or read `.cargo/config.toml`'s `[build] target` automatically, depending on version).

[.vscode/extensions.json](.vscode/extensions.json) recommends `rust-lang.rust-analyzer` and `probe-rs.probe-rs-debugger` (VS Code debug adapter for breakpoint/step debugging over the same probe, as an alternative to log-and-rerun).

## Folder contents

```txt
├── rust-toolchain.toml   pins the Rust channel + thumbv7em-none-eabihf target (auto-installed by rustup)
├── .cargo/
│   └── config.toml       default build target, probe-rs runner, linker flags
├── Cargo.toml
├── build.rs              copies memory.x onto the linker search path
├── memory.x              nRF52833 FLASH/RAM layout
├── src/
│   └── main.rs
├── udev/
│   └── 69-microbit.rules install into /etc/udev/rules.d for probe access without sudo (Linux)
├── .vscode/
│   ├── extensions.json
│   └── settings.json
└── README.md             this file
```

Not checked in (generated locally, see `.gitignore`): `target/`.

`Cargo.lock` **is** checked in — this crate produces a firmware binary, not a library, so its dependency versions should be reproducible across machines (Cargo's own recommendation for binary crates).

## Using this as a template

- Rename the package/binary in [Cargo.toml](Cargo.toml) (`[package] name` and `[[bin]] name`) and replace [src/main.rs](src/main.rs) with your app.
- Bump the pinned toolchain by updating `channel` in `rust-toolchain.toml`.
- Add peripherals as needed — the `microbit-v2` crate's [`Board`](https://docs.rs/microbit-v2) struct exposes the display matrix, buttons, speaker, edge connector GPIO, and I2C/UART pins already wired to the right nRF52833 peripherals; see the [nrf-rs/microbit examples](https://github.com/nrf-rs/microbit/tree/main/examples) and the [Discovery Book](https://docs.rust-embedded.org/discovery/microbit/) for patterns (serial, I2C to the onboard accelerometer/magnetometer, RTIC-based scheduling, etc.).
- `defmt`'s log level is controlled by the `DEFMT_LOG` env var, defaulted to `debug` in `.cargo/config.toml`'s `[env]` table — override per-run with `DEFMT_LOG=trace cargo run --release`.

## Troubleshooting

- **`probe-rs run` fails to find a probe / "No probes found"**
  - Check the USB cable is a data cable (not charge-only) and the board shows a steady (not blinking) yellow LED near the USB port once connected.
  - Run `probe-rs list` to confirm the OS sees the probe at all. On Linux, this is almost always the udev rule (see above) — replug the board after installing it.

- **Flash succeeds but no `defmt` output appears**
  - `defmt-rtt` (imported in `main.rs`) needs the flashing tool to keep the RTT channel open — `probe-rs run` does this automatically; a bare `probe-rs download` does not.
  - Confirm `DEFMT_LOG` isn't set to something that filters out your log level (e.g. `off`).

- **rust-analyzer shows errors on `core`/`std` items or can't find the target**
  - Make sure `rust-analyzer.cargo.target` in `.vscode/settings.json` matches `thumbv7em-none-eabihf`, and that `rustup target list --installed` includes it (it should install automatically from `rust-toolchain.toml` on first `cargo check`/`build`).

- **Linker errors mentioning `memory.x` or `link.x`**
  - `link.x` comes from `cortex-m-rt` and is pulled in automatically; `memory.x` is this crate's own file (copied into the link search path by `build.rs`). If you moved `main.rs` into a workspace, make sure `memory.x`/`build.rs` live in the same crate that builds the final binary — `cortex-m-rt` needs them there, not in a library crate.

- **Linker error: `undefined symbol: _critical_section_1_0_acquire` / `_critical_section_1_0_release`**
  - `defmt` (and anything else pulling in the `critical-section` crate) needs some crate in the dependency graph to actually implement interrupt-disabling critical sections for the target. `cortex-m`'s `critical-section-single-core` feature provides that; this scaffold's `Cargo.toml` already enables it on the direct `cortex-m` dependency (Cargo feature unification then applies it project-wide, even though nothing calls `cortex-m` directly). If you remove that dependency or its feature, this error comes back — put it back rather than adding a separate `critical-section` impl crate.

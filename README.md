# Daemon

Daemon is Shuyra Vanllia's resident supervision and recovery server.

## Layout and policy

- `server/`: `daemon-server` library and `daemon` executable.
- `unit.toml`: resident policy, readiness signal and supervision group.
- `declaration/`: retained Rust `UNIT` and `BUILD` compatibility declarations.
- `tests/runtime/`: independent Rust runtime-model tests.
- `tests/unit-document/`: independent TOML, record and native conversion tests.
- `tests/recovery/`: Python recovery-evidence parser regression tests.

Daemon requires RTC and waits for it. It supervises the `supervised` group
without joining that group. System completes Operator bootstrap before Unit
creation. Daemon adds no namespace, identity, supply, construction or retirement
rights.

## Build and test

Install Rust through rustup. `rust-toolchain.toml` selects `nightly-2026-09-04`;
`.cargo/config.toml` defaults to `riscv64gc-unknown-none-elf`. From the root:

```sh
cargo build -p daemon-server --bin daemon --release --locked
cargo test --manifest-path tests/runtime/Cargo.toml --target x86_64-unknown-linux-gnu --locked
cargo test --manifest-path tests/unit-document/Cargo.toml --target x86_64-unknown-linux-gnu --locked
python3 -m unittest discover -s tests/recovery -p 'test_*.py' -v
```

The first Rust run needs network access. Python tests use synthetic legacy logs
and validate parser behavior only. Actual restart, readiness and reclamation
require a run with the selected Vanllia deployment.

## Attribution

Implementation and original tests derive from
[anreonyr/sqware](https://github.com/anreonyr/sqware/tree/94e114614bec53424ac55e640fc80cf19a3e7107).
The filtered Git history retains original authors and relevant changes.

Follow the [Shuyra maintenance standard](https://github.com/shuyra-arche/vanllia/blob/main/CONTRIBUTING.md).

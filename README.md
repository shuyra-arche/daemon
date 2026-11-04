# Daemon

This workspace contains the resident daemon runtime and its data-only declaration:

- `server/` — package `daemon-server`, library `daemon_server`, binary `daemon`
- `declaration/` — package `daemon-declaration`, library `daemon_declaration`
- `tests/runtime/` — an independent host test workspace

`tests/runtime/` is intentionally excluded from the root workspace. It is a separate host
workspace so the daemon runtime tests can be compiled without building the `no_std` server
dependencies for the host.

## Commands

From the candidate root:

```sh
cargo build -p daemon-server --bin daemon --release --locked
```

The runtime test workspace remains independent:

```sh
cargo test --manifest-path tests/runtime/Cargo.toml --target x86_64-unknown-linux-gnu --locked
python3 -m unittest discover -s tests/recovery -p 'test_*.py' -v
```

The root `.cargo/config.toml` defaults to `riscv64gc-unknown-none-elf`; pass the host
target explicitly when testing the independent host workspace.

## Provenance

The implementation and test cases are imported from
`anreonyr/sqware@94e114614bec53424ac55e640fc80cf19a3e7107`. The daemon runtime keeps the
existing observation, readiness, supervision, and recovery behavior. `system-api` and
`system-client` use the distinct revisions from that source because they are intentionally
separate revisions.

The path-filtered import preserves 14 relevant commits before the standalone adaptation.
The server depends only on fixed public provider revisions; it does not link `programs`,
Source composition, or System's server implementation. Declaration depends only on `unit`
and exports `UNIT` plus the explicit `BUILD` coordinates. The existing core, retry budgets,
startup denial window, face host/mark checks, ready signal and recovery policy are retained.

## Validation

Validated with the declared `nightly-2026-09-04` on 2026-11-04:

- The five original Rust runtime cases and 80 Python recovery-parser cases pass.
  All case files and assertions are retained; the Rust harness changes only source paths.
- Locked offline host/RISC-V dependency graphs each contain 25 packages, with no
  Source dependency. Workspace RISC-V check and actual debug/release ELF builds pass.
- Both ELFs are static RISC-V64 ET_EXEC images with valid entry/load segments, one
  `_start`, one `clean_ret`, one panic handler and no undefined symbols.
- Existing `vanllia-build` at `17fd7ddbd18b3aef3a8d348cd32d8642e81b3c7f` builds the
  explicit target through `BUILD`. Its artifact matches the direct release build:
  259,168 bytes, SHA-256 `f2f023f59eb689a288a9bb60b5ea8c0333b09f8ed9b0f17069093f0de8a482a3`.

The Python cases validate recovery-evidence parsers with synthetic controls. They do not
establish a new machine-level restart result. Initial compile failures and corrections
are retained in the local evidence rather than counted as successful checks.

## Scope

This candidate does not include product wiring, QEMU execution, or machine boot validation.
It has not been published. Vanllia still needs a fixed provider revision, the imported
declaration in the existing registration slot, and replacement of only its legacy Daemon
image target. Registry/deployment order and the separate RTC API migration remain with
the product integration.

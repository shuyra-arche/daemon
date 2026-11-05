# Daemon

This workspace contains the resident daemon runtime and its declarations:

- `server/` — package `daemon-server`, library `daemon_server`, binary `daemon`
- `unit.toml` — schema-1 runtime policy, id/image `daemon`
- `declaration/` — legacy `daemon_declaration::{UNIT, BUILD}` compatibility contract
- `tests/runtime/` — an independent host test workspace
- `tests/unit-document/` — independent TOML/record/native contract tests

Both test workspaces are excluded from the root workspace so host tests can run
without building the `no_std` server dependencies for the host.

The runtime document preserves the resident ending, the server's
`unit::READY_MARK` signal and supervision of `supervised`. System completes
Operator bootstrap before Unit creation. The old PRODUCT target had only RTC,
so the document requires and waits for `rtc`; RTC's own document declares
`groups = ["supervised"]`. Daemon adds no namespace, identity, supply,
construction or retirement rights, and does not join the supervised group.

## Commands

From the candidate root:

```sh
cargo build -p daemon-server --bin daemon --release --locked
```

The runtime test workspace remains independent:

```sh
cargo test --manifest-path tests/runtime/Cargo.toml --target x86_64-unknown-linux-gnu --locked
cargo test --manifest-path tests/unit-document/Cargo.toml --target x86_64-unknown-linux-gnu --locked
python3 -m unittest discover -s tests/recovery -p 'test_*.py' -v
```

The root `.cargo/config.toml` defaults to `riscv64gc-unknown-none-elf`; pass the host
target explicitly when testing the independent host workspace.

The document suite pins the public Unit document, record and native conversion
APIs at `ad50b803c3bd5cd2ef90010d80badf23551e88d5` and includes its own formal
`Cargo.lock`. A test-only `legacy-unit` alias keeps the old declaration's Unit
revision for comparisons. Production pins are unchanged. The suite runs
directly from `tests/unit-document` without a local Unit checkout or Cargo
patch. See [VALIDATION.md](VALIDATION.md) for the locked commands and evidence.

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

## Current validation

On 2026-11-05, the seven document tests passed against the public Unit revision
with the committed test lock, `--locked --offline` and `-Dwarnings`. They compare
every legacy field, roundtrip the complete TOML and binary record, verify borrowed strings, call the
actual native runtime conversion, check full-width ready marks and validate the
existing Cargo bin/BUILD contract. The five unchanged Rust runtime tests and
80 historical Python parser tests passed without a Unit patch. The actual
release RISC-V `daemon` binary also builds with the existing public pins and
lock. Commands, ELF details and review results are in [VALIDATION.md](VALIDATION.md).

The old Python cases use synthetic legacy log evidence. They establish parser
regression coverage only; they are not evidence for the new native exit/reclaim
receipt protocol or a current supervision run. Production Rust and old scenarios
are unchanged. This declaration migration has no QEMU or machine boot validation.

## Earlier validation

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

This migration changes the declaration and its validation. Product intake still
needs provider revisions, artifact packaging and runtime acceptance. It does
not migrate old product scenes. Publication is handled separately; QEMU
execution and machine boot validation are outside this change.

# Daemon runtime declaration validation

Validated on 2026-11-05 from local branch `codex/unit-toml-runtime`, based on
`4e4ae06`. Commands use `/workspace/.vanllia-tools/env.sh`,
`nightly-2026-10-09`, rustc `1.101.0-nightly (a30aa9064 2026-10-08)`, host
`x86_64-unknown-linux-gnu` and target `riscv64gc-unknown-none-elf`.

## Policy mapping

`unit.toml` is the schema-1 runtime declaration. The original Rust `UNIT`
initializer and `BUILD` coordinates are preserved as the compatibility contract;
only that module's documentation changes.

| Legacy field or composition fact | Document and native contract |
| --- | --- |
| `UNIT.name`, loader image | `id = image = "daemon"` |
| User space, empty args, stack 0 | `space = "user"`, `args = []`, `stack = 0` |
| Empty Operator namespace | `operator.namespace = []` |
| Empty identity aliases and grants | Both arrays remain empty |
| Listed `control.after = Some(...)` | `creation = "startup"`; native `listed()` remains true |
| `Bootstrap::Operator` dependency | System completes Operator bootstrap before creating Units; no Operator Unit is invented |
| `Target("supervised")`, whose only PRODUCT member was RTC | `requires = ["rtc"]`, `after = ["rtc"]`; RTC owns `groups = ["supervised"]` |
| `Ready::Signal(unit::READY_MARK)` | Quoted decimal signal `9887014109901452468` |
| `Some(Ending::Resident)` | `ending = "resident"` |
| `supervision = Some("supervised")` | `supervision = "supervised"` |
| Empty construction and retirement policies | Both arrays remain empty |
| Daemon is the supervisor, not a supervised member | `groups = []`, `guarded = false` |
| Empty supply bindings | `supply.bindings = []` |
| Existing `BUILD` | Image/bin `daemon`, manifest `server/Cargo.toml`, package `daemon-server`, RISC-V target, release, no features, default features enabled |

A real Rust probe compiled against the unchanged public Unit pin and the old
Daemon declaration confirmed the ready mark's decimal value and hexadecimal
form `0x8935baf305f374b4`. The server still sends this same `unit::READY_MARK`.
The document grants no new publication, identity, resource, construction or
retirement authority.

## Results

| Check | Result |
| --- | --- |
| Every legacy field versus TOML and decoded record | Passed |
| Complete document/record/TOML roundtrip and borrowed strings | Passed |
| Actual `runtime::UnitFile::try_from` conversion versus legacy native fields | Passed |
| Decimal/hex ready mark, numeric/quoted `u64`, full-width values and invalid range rejection | Passed |
| Real Cargo package/bin/source and all `BUILD` fields | Passed |
| New document suite, public Unit revision, formal test lock, `--locked --offline -Dwarnings` | 7 passed |
| Independent reviewer rerun against public pins and formal lock, separate output directory | 7 passed |
| Locked metadata: both Unit revisions resolved from public Git, no local Unit path | Passed; 35 packages |
| Existing Rust runtime suite, original lock/pins, `-Dwarnings` | 5 passed |
| Existing synthetic legacy recovery-parser suite, unchanged | 80 passed |
| Actual release RISC-V `daemon` executable, original lock/pins, `-Dwarnings` | Passed |
| Original production dependency pins and two preexisting locks | Unchanged; formal document-test lock added |
| New Rust formatting and whitespace checks | Passed |

The new suite now pins the real schema, encoder/decoder and runtime conversion
at public Unit revision `ad50b803c3bd5cd2ef90010d80badf23551e88d5`, available in
[Unit Draft PR #1](https://github.com/shuyra-arche/unit/pull/1). The formal
`tests/unit-document/Cargo.lock` records that exact Git source. The test-only
`legacy-unit` alias and `daemon-declaration` retain the existing
`90396f0fda376e0693fb1b9553aa21864d241057` revision, allowing the tests to compare
the actual old declaration against the new API. Cross-revision ready and ending
enums are compared explicitly by variant and value; both ready constants must
match. No production pin changes.

The final suite runs directly from its committed manifest with
`--locked --offline` and `RUSTFLAGS=-Dwarnings`, using
`/tmp/daemon-unit-public-target`. Locked metadata
resolves both Unit packages from public Git checkouts and contains no local
Unit path override. The earlier local-patch development runs are superseded
by this public-pin validation; no patch or staged manifest is needed.

The actual release binary uses the existing public pins without a Unit patch.
ELF inspection confirms little-endian RISC-V ELF64 `EXEC`, executable entry
`0x12526`, first load address `0x10000`, page-aligned load segments and no
interpreter segment. Its SHA-256 is
`52f198bbb946d3bc9baefa090f8712b72cc2bea48a2b7ee0bd19d6572caeb78e`.

## Reproduce

The formal document-test lock is committed at
`tests/unit-document/Cargo.lock`. Build output and the separate `flock` lock
are under `/tmp`. After fetching the pinned public dependencies, run from the
repository root:

```sh
source /workspace/.vanllia-tools/env.sh
RUSTFLAGS='-Dwarnings' flock /tmp/daemon-unit-public-target.lock cargo test --locked --offline \
  --manifest-path tests/unit-document/Cargo.toml \
  --target-dir /tmp/daemon-unit-public-target --target x86_64-unknown-linux-gnu
cargo metadata --locked --offline --manifest-path tests/unit-document/Cargo.toml --format-version 1
RUSTFLAGS='-Dwarnings' flock /tmp/daemon-legacy-runtime-target.lock cargo test --locked --offline \
  --manifest-path tests/runtime/Cargo.toml \
  --target-dir /tmp/daemon-legacy-runtime-target --target x86_64-unknown-linux-gnu
python3 -m unittest discover -s tests/recovery -p 'test_*.py' -v
RUSTFLAGS='-Dwarnings -Crelocation-model=static -Cforce-frame-pointers=yes -Ccode-model=medium' \
  flock /tmp/daemon-image-runtime-target.lock cargo build --locked --offline \
  -p daemon-server --bin daemon --release --target riscv64gc-unknown-none-elf \
  --target-dir /tmp/daemon-image-runtime-target
readelf -h /tmp/daemon-image-runtime-target/riscv64gc-unknown-none-elf/release/daemon
readelf -l /tmp/daemon-image-runtime-target/riscv64gc-unknown-none-elf/release/daemon
sha256sum /tmp/daemon-image-runtime-target/riscv64gc-unknown-none-elf/release/daemon
rustfmt --edition 2024 --check tests/unit-document/lib.rs
git diff --check
git diff --exit-code -- Cargo.lock tests/runtime/Cargo.lock
```

## Review and limits

Namer confirmed the stable id/image, actual Cargo coordinates, startup creation,
RTC dependency mapping and unchanged empty permission fields. The earlier
independent review checked the legacy Rust declaration against the new document
and native conversion, including a complete 69-byte record roundtrip, the
ready mark, empty authority arrays and RTC dependency mapping. It also reran
all seven then-local document tests and the five unchanged runtime tests with
no blocking findings. The same reviewer independently reran all seven tests
against the public pins and formal lock with `--locked --offline -Dwarnings`,
no Cargo patch and the separate `/tmp/daemon-unit-reviewer-public-target`.
Locked metadata confirmed 35 packages, both exact public Git Unit revisions,
and no local Unit override. Review confirmed the cross-revision comparisons
retain the old native contract and the production pins, preexisting locks and
`unit.toml` are unchanged. Final code, lock and documentation review passed
with no blocking findings.

The Python suite checks synthetic historical log parsers only. Kernel event
logging is being removed separately; those parsers are not evidence for the new
native exit/reclaim receipt protocol. This PR does not change the old parsers,
product scenarios or production Rust. No new native receipt integration,
QEMU, boot, supervision recovery or machine acceptance run was performed.
This worker creates reviewed local commits; the coordinating agent handles
publication separately.

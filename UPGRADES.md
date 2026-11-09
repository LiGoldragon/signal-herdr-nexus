# Upgrades

## 0.4.0 — signal 11.0.0

Only the signal pin moves: signal 11.0.0 (6d4eb74), which carries no transport feature and no tokio; the datom feature keeps signal/datom. Datom-codec 09a0526 and protos c93b598 stay. No change to the authored ethos, the generated Rust, the archive or the Datom text.

## 0.3.2 — the datom feature carries signal/datom

The `datom` feature now also turns on signal's `datom` feature, the edge the build base's feature set asks of a repository that builds signal with it. The pins stay: signal 10.4.1 (d77169d), datom-codec 09a0526, protos c93b598. No change to the authored ethos, the generated Rust, the archive or the Datom text.

## 0.3.1 — signal 10.4.1, datom-codec 09a0526, protos c93b598, ethos-zero 1976e45

Only the pins move: signal 10.4.1 (d77169d), datom-codec 0.33.1 (09a0526) and protos 0.33.0 (c93b598), with the build-dependency ethos-zero 22.0.0 (1976e45) that signal brings. No change to the authored ethos, the generated Rust, the archive or the Datom text. A consumer repins the three together.

## 0.3.0 — signal 10.4.0, datom-codec 0bbeb11, protos 4024614

Only the pins move: signal 10.4.0 (7e6dfe5), datom-codec 0.33.1 (0bbeb11) and protos
0.33.0 (4024614). No change to the authored ethos, the generated Rust, the archive or the
Datom text. A consumer repins the three together.

## 0.2.0 — signal 10.3.0, datom-codec 0.33.0, protos 0.33.0

Repins signal 10.3.0 (7afb), datom-codec 0.33.0 (eabe) and protos 0.33.0 (3c7a), each spelled as signal spells it so the lock holds one source of each. signal 10.3.0 changes its exchange ledger, not `Contracted`, `Signalizable` or `Restorable`, which this contract uses unchanged.

- The generated contract is regenerated with ethos-zero 19.2.0 (5a5d). The authored `signal.ethos` is unchanged; the generated types add `PartialOrd` and `Ord` (and `Copy` where the scalar is `Copy`). No type, variant or field changes, and the rkyv layout is unchanged.
- protos 0.33.0 prints a balanced guillemet pair inside a string without an escape; a consumer holding datom text with an unbalanced guillemet re-reads it.
- The repository gains a Nix check route: `flake.nix` with build, test, test-datom, doc, fmt and clippy checks, and `rust-toolchain.toml` selecting Rust 1.99.1 by version with that manifest's hash. `tests/datom_contract.rs` reads and prints datom text for the query and response shapes and refuses wrong shapes.

To deploy: repin signal-herdr-nexus 0.2.0 in the consumer and enable `datom` only where text is spoken.

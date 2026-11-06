# Upgrades

## 0.2.0 — signal 10.3.0, datom-codec 0.33.0, protos 0.33.0

Repins signal 10.3.0 (7afb), datom-codec 0.33.0 (eabe) and protos 0.33.0 (3c7a), each spelled as signal spells it so the lock holds one source of each. signal 10.3.0 changes its exchange ledger, not `Contracted`, `Signalizable` or `Restorable`, which this contract uses unchanged.

- The generated contract is regenerated with ethos-zero 19.2.0 (5a5d). The authored `signal.ethos` is unchanged; the generated types add `PartialOrd` and `Ord` (and `Copy` where the scalar is `Copy`). No type, variant or field changes, and the rkyv layout is unchanged.
- protos 0.33.0 prints a balanced guillemet pair inside a string without an escape; a consumer holding datom text with an unbalanced guillemet re-reads it.
- The repository gains a Nix check route: `flake.nix` with build, test, test-datom, doc, fmt and clippy checks, and `rust-toolchain.toml` selecting Rust 1.99.1 by version with that manifest's hash. `tests/datom_contract.rs` reads and prints datom text for the query and response shapes and refuses wrong shapes.

To deploy: repin signal-herdr-nexus 0.2.0 in the consumer and enable `datom` only where text is spoken.

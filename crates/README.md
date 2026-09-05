# DePIN-Mesh Crates Workspace

This directory contains the core Rust implementation crates for the Physical Evidence Consensus (PEC) protocol.

## Workspace Crates

* `pec-core`: Foundational domain types, Physical Evidence Hypergraph (PEH) data structures, and the `PEC-MATH-01` Q64.64 deterministic fixed-point math engine.
* `pec-vm`: Deterministic execution engine, invariant conservation solvers ($I_0 - I_6$), dependency tensor computation ($N_{eff}$), and conflict resolution routines.

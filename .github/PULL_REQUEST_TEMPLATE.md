## Summary of Changes

Provide a concise, technical description of the modifications, architectural rationale, and targeted subsystems.

## Associated Issue or RFC

Fixes #
Related RFC: `spec/RFC-xxxx`

## Engineering Checklist

- [ ] Adheres strictly to the [Conventional Commits v1.0.0](docs/CONTRIBUTING.md#2-commit-message-conventions) specification.
- [ ] No IEEE 754 floating-point operations introduced in consensus-critical code paths.
- [ ] Any mathematical operations comply with the `PEC-MATH-01` Q64.64 deterministic standard.
- [ ] All new Protobuf schema changes retain backward compatibility and compile cleanly.
- [ ] Deterministic unit test vectors or invariant fuzzing tests have been added.
- [ ] Documentation updated to reflect changes without marketing hype or em-dashes.

## Subsystems Touched

- [ ] `spec/` (Protocol RFCs / Schemas)
- [ ] `crates/pec-core` (Data Structures & Fixed-Point Math)
- [ ] `crates/pec-vm` (Verification Engine)
- [ ] `firmware/` (Embedded Hardware & RoT)
- [ ] `relayer/` (P2P Transport)
- [ ] `docs/` (Architecture & Guides)

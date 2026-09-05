# Community Backlog: Good First Issues

```text
Document: COMMUNITY_BACKLOG.md
Status: Active Maintainer Backlog
Labels: good first issue, help wanted
Repository: https://github.com/Chamath-Adithya/depin-mesh-protocol
```

This backlog contains actionable, highly scoped tasks designed for new contributors to the DePIN-Mesh Physical Evidence Consensus (PEC) repository. Each entry corresponds to an active, trackable issue on GitHub.

## Active Issues Tracker

| Issue | Title | Subsystem Area | Complexity | Status |
| :--- | :--- | :--- | :--- | :--- |
| [#1](https://github.com/Chamath-Adithya/depin-mesh-protocol/issues/1) | Implement Q64.64 Saturating Addition and Subtraction | `area/math` | Beginner | Open |
| [#2](https://github.com/Chamath-Adithya/depin-mesh-protocol/issues/2) | Setup Protobuf Code Generation via buf and prost | `area/proto` | Intermediate | Open |
| [#3](https://github.com/Chamath-Adithya/depin-mesh-protocol/issues/3) | Enrich CORDIC Transcendental Formulations in RFC-0001 | `area/docs` | Beginner | Open |

---

## Issue 1: Implement Q64.64 Saturating Addition and Subtraction

* **GitHub Issue**: [#1](https://github.com/Chamath-Adithya/depin-mesh-protocol/issues/1) (Status: Open on GitHub)
* **Title**: `[Good First Issue]: Implement Q64.64 Saturating Addition and Subtraction`
* **Labels**: `good first issue`, `help wanted`, `area/math`, `crates/pec-core`
* **Mentor**: `@depinmesh-core`

### Background and Context
The PEC protocol mandates deterministic fixed-point mathematics (`PEC-MATH-01`) to prevent floating-point consensus forks across heterogeneous nodes (x86_64, ARM Cortex-M, and RISC-V), as established in `docs/adr/0001-deterministic-q64-fixed-point-math.md`. 

The foundational numerical primitive is the signed Q64.64 fixed-point integer, where values are represented as a signed 128-bit integer with 64 fractional bits:

$$X = x \cdot 2^{-64}, \quad x \in [-2^{127}, 2^{127}-1]$$

Consensus safety requires that addition and subtraction do not wrap modulo $2^{128}$; instead, they must saturate at numeric boundaries.

### Targeted File
* `crates/pec-core/src/math/q64.rs`

### Detailed Implementation Steps

1. Define the core `Q64` tuple struct wrapping `i128`:
   ```rust
   #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
   pub struct Q64(pub i128);
   ```

2. Implement numeric boundary constants on `Q64`:
   * `Q64::MAX = Q64(i128::MAX)` (representing approximately $9.223372036854775807 \times 10^{18}$)
   * `Q64::MIN = Q64(i128::MIN)` (representing approximately $-9.223372036854775808 \times 10^{18}$)
   * `Q64::ZERO = Q64(0)`
   * `Q64::ONE = Q64(1i128 << 64)`

3. Implement saturating arithmetic methods:
   * `pub const fn saturating_add(self, rhs: Self) -> Self`: Uses `self.0.saturating_add(rhs.0)`.
   * `pub const fn saturating_sub(self, rhs: Self) -> Self`: Uses `self.0.saturating_sub(rhs.0)`.

4. Implement `core::ops::Add` and `core::ops::Sub` operator overloads forwarding to the saturating implementations.

### Test Vectors and Acceptance Criteria

Add unit tests in `crates/pec-core/src/math/q64.rs` verifying the following exact vectors:

```rust
#[test]
fn test_saturating_add_identity_and_basic() {
    let one = Q64::ONE;
    let two = one.saturating_add(one);
    assert_eq!(two.0, 2i128 << 64);
    assert_eq!(two.saturating_sub(one), one);
}

#[test]
fn test_saturating_add_positive_overflow() {
    let max = Q64::MAX;
    let one = Q64::ONE;
    assert_eq!(max.saturating_add(one), Q64::MAX);
    assert_eq!(max.saturating_add(Q64(1)), Q64::MAX);
}

#[test]
fn test_saturating_sub_negative_underflow() {
    let min = Q64::MIN;
    let one = Q64::ONE;
    assert_eq!(min.saturating_sub(one), Q64::MIN);
    assert_eq!(min.saturating_sub(Q64(1)), Q64::MIN);
}
```

* **Verification**: `cargo test -p pec-core` and `cargo clippy -- -D warnings` must pass with zero diagnostics.

---

## Issue 2: Setup Protobuf Generation Script via `buf`

* **GitHub Issue**: [#2](https://github.com/Chamath-Adithya/depin-mesh-protocol/issues/2) (Status: Open on GitHub)
* **Title**: `[Good First Issue]: Setup Protobuf Code Generation via buf and prost`
* **Labels**: `good first issue`, `help wanted`, `area/proto`, `build-system`
* **Mentor**: `@depinmesh-core`

### Background and Context
Canonical protocol schemas are defined in Protocol Buffers v3 under `spec/proto/depinmesh/pec/v1/`:
* `evidence.proto`: Defines `PhysicalEvidenceObject`, `EvidenceType`, and `ObservationAuthorityEnvelope`.
* `claim.proto`: Defines `PhysicalClaim`, `DecisionState`, and `AssuranceCoordinate`.

We require a deterministic code generation pipeline using `buf` to generate Rust data structures into `crates/pec-core/src/proto/generated/`.

### Targeted Files
* `spec/proto/buf.gen.yaml` (new file)
* `crates/pec-core/build.rs` or `scripts/generate_protos.sh` (new script)

### Detailed Implementation Steps

1. Create `spec/proto/buf.gen.yaml` configured for Rust code generation using the official `community-rust` plugin or a dedicated build script:
   ```yaml
   version: v1
   plugins:
     - plugin: buf.build/prost/plugins/prost:v0.4.0
       out: ../../crates/pec-core/src/proto/generated
       opt:
         - compile_well_known_types
         - extern_path=.google.protobuf=::prost_types
   ```

2. Add a helper script `scripts/generate_protos.sh`:
   ```bash
   #!/usr/bin/env bash
   set -euo pipefail

   SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
   REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

   echo "Generating Rust Protobuf bindings..."
   mkdir -p "${REPO_ROOT}/crates/pec-core/src/proto/generated"
   cd "${REPO_ROOT}/spec/proto"
   buf generate

   echo "Formatting generated code..."
   cargo fmt -p pec-core
   echo "Protobuf generation completed successfully."
   ```

3. Ensure generated structs derive `serde::Serialize`, `serde::Deserialize`, `Clone`, and `PartialEq`.

### Acceptance Criteria
* Running `scripts/generate_protos.sh` regenerates `crates/pec-core/src/proto/generated/depinmesh.pec.v1.rs` cleanly from any modified `.proto` file.
* `buf lint spec/proto` runs with zero warnings.
* All generated Rust code compiles cleanly under `cargo check -p pec-core`.

---

## Issue 3: Add CORDIC Sine/Cosine LaTeX Formulations to Protocol Specification

* **GitHub Issue**: [#3](https://github.com/Chamath-Adithya/depin-mesh-protocol/issues/3) (Status: Open on GitHub)
* **Title**: `[Good First Issue]: Enrich CORDIC Transcendental Formulations in RFC-0001`
* **Labels**: `good first issue`, `help wanted`, `area/docs`, `spec`
* **Mentor**: `@depinmesh-core`

### Background and Context
Section 9.3 of `spec/RFC-0001-PHYSICAL-EVIDENCE-CONSENSUS.md` specifies the `PEC-MATH-01` CORDIC (Coordinate Rotation Digital Computer) algorithm for computing trigonometric functions without floating-point instructions. To eliminate ambiguity for independent implementation teams building verification nodes, the formal specification needs expanded mathematical derivations of the scaling factor $K_n$, domain reduction, and the pseudo-rotation angle accumulator.

### Targeted File
* `spec/RFC-0001-PHYSICAL-EVIDENCE-CONSENSUS.md` (Section 9.3: Transcendental Functions via CORDIC)

### Detailed Implementation Steps

1. Enrich Section 9.3 by adding the formal mathematical definition of the cumulative CORDIC gain $K_n$:
   $$K_n = \prod_{i=0}^{n-1} \frac{1}{\sqrt{1 + 2^{-2i}}}$$
   For $n = 64$ iterations, document the exact limiting constant in decimal and hexadecimal Q64.64:
   $$K_{\infty} \approx 0.6072529350088812561694$$
   $$\text{Hex Q64.64}: \texttt{0x9B74EDA8435E52D1}$$

2. Add the formal domain reduction algorithm mapping any input angle $\theta \in [-\infty, +\infty]$ into the convergence region $[-\frac{\pi}{2}, \frac{\pi}{2}]$ using exact fixed-point integer modulo arithmetic over $2\pi$.

3. Provide a clear ASCII algorithm trace diagram depicting rotation mode versus vectoring mode.

### Acceptance Criteria
* Formulations use standard LaTeX math blocks compatible with GitHub Markdown rendering.
* Documentation strictly avoids em-dashes and adheres to professional technical American English.
* The expanded section passes documentation linting via `.github/workflows/lint-docs.yml`.

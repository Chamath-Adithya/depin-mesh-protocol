# Contributing to DePIN-Mesh Protocol

```text
Document: CONTRIBUTING.md
Status: Living Standard
Target: Contributors, Researchers, Core Maintainers
Version: 1.1.0
```

We welcome research and engineering contributions to the DePIN-Mesh Physical Evidence Consensus (PEC) protocol. To maintain research-grade institutional quality, all contributors must adhere to the standards outlined in this document.

---

## 1. Request for Comments (RFC) Process

All architectural, cryptographic, or physical specification changes must begin with a formal Request for Comments (RFC) document placed in `spec/`.

### 1.1 RFC Lifecycle

The RFC process governs protocol evolution through three formal stages:

1. **Draft**:
   - Author opens an issue using `.github/ISSUE_TEMPLATE/02_rfc_proposal.yml` or submits a PR with a candidate `spec/RFC-xxxx-<TITLE>.md`.
   - The document specifies problem taxonomy, mathematical equations, IETF RATS alignment, and physical attack vectors.
2. **Review**:
   - The proposal undergoes open technical review by the Protocol Working Group.
   - Formal mathematical reviews analyze invariant bounds, fixed-point rounding stability, and dependency tensor properties.
   - Working group maintainers solicit peer reviews from domain experts in sensor metrology and distributed consensus.
3. **Final**:
   - The working group confirms technical consensus.
   - An official sequential number is assigned (for example, `RFC-0002-ACOUSTIC-DOPPLER.md`).
   - The status is updated to `Accepted` and marked as canonical for downstream implementation in `crates/` and `firmware/`.

### 1.2 RFC Quality Standards

* **Mathematical Precision**: Formulations must define coordinate spaces, invariant bounds, and uncertainty propagation rules.
* **Determinism First**: Floating-point operations are prohibited in consensus-critical logic. Specifications must specify Q64.64 fixed-point operations complying with `PEC-MATH-01`.
* **Threat Modeling**: Proposals must analyze Byzantine behaviors, sensor spoofing economics, and hardware attack surfaces.

---

## 2. Commit Message Conventions

This repository strictly enforces [Conventional Commits v1.0.0](https://www.conventionalcommits.org/).

Each commit message must follow this structure:

```text
<type>(<scope>): <short summary>

[optional body]

[optional footer(s)]
```

### 2.1 Types
* `spec`: Protocol specifications, RFC updates, or Protobuf schema modifications.
* `feat`: New software or firmware features.
* `fix`: Bug fixes in code or errors in mathematical documentation.
* `math`: Implementations or optimizations of deterministic fixed-point algorithms.
* `refactor`: Code restructuring without functional behavior changes.
* `test`: Adding or refining unit tests, invariant fuzzing, or test vectors.
* `ci`: Continuous integration workflow modifications.
* `docs`: Documentation clarifications outside the formal `spec/` tree.

### 2.2 Scope Guidelines
Approved scopes include: `core`, `vm`, `proto`, `firmware`, `relayer`, `deps`, `cordic`, `peh`, `invariants`.

### 2.3 Style Rules
* Write summaries in the imperative mood (for example, "add Q64.64 square root test vectors" instead of "added" or "adds").
* Do not end the summary line with a period.
* Keep the summary line under 72 characters.

---

## 3. Local Build and Test Workflow

We recommend using the provided Devcontainer (`.devcontainer/`) for zero-friction setup. If building directly on your host machine, follow these deterministic verification steps:

### 3.1 Prerequisites
Ensure the following tools are present in your environment:
* Rust `stable` (>= 1.80) with `rustfmt`, `clippy`, and `wasm32-unknown-unknown` target.
* Protocol Buffers compiler (`protoc` >= 3.21) and `buf` CLI (>= 1.30).
* Python 3.10+ (for documentation and script checks).

### 3.2 Step-by-Step Verification Commands

1. **Verify Code Formatting**:
   ```bash
   cargo fmt --all -- --check
   ```

2. **Execute Static Analysis & Lints**:
   ```bash
   cargo clippy --workspace --all-targets -- -D warnings
   ```

3. **Validate Protobuf Schemas**:
   ```bash
   buf lint spec/proto
   ```

4. **Run Unit and Invariant Tests**:
   ```bash
   # Native test suite execution
   cargo test --workspace

   # Deterministic cross-architecture compilation check
   cargo check --target wasm32-unknown-unknown
   ```

5. **Verify Documentation & Prohibited Dash Scan**:
   ```bash
   python3 -c '
   import glob, re, sys
   bad = [ch for f in glob.glob("**/*.md", recursive=True) for ch in ["\u2014", "\u2013"] if ch in open(f, errors="ignore").read()]
   if bad: sys.exit("Found prohibited dashes in markdown files.")
   print("Docs check passed.")
   '
   ```

---

## 4. Code Ownership and Review SLA

### 4.1 Review SLA Commitment
The core maintainer team adheres to a strict review service level agreement:
* **Initial Response**: Within 48 business hours of PR submission.
* **Technical Feedback**: Clear, actionable guidance referencing specific lines and mathematical invariants.
* **Merge Criteria**: Requires at least two approving reviews from subsystem codeowners and a clean CI run.

### 4.2 Code Ownership Boundaries

| Directory / Subsystem | Primary Focus | Required Domain Review |
| :--- | :--- | :--- |
| `spec/` | Protocol RFCs and Protobuf schemas | Protocol Working Group, Cryptographic Engineers |
| `crates/pec-core/` | Math routines (`PEC-MATH-01`), PEH data structures | Distributed Systems Engineers, Numerical Analysts |
| `crates/pec-vm/` | Execution engine, invariant solvers, graph solver | Consensus Architects, Virtual Machine Engineers |
| `firmware/` | Embedded drivers, silicon RoT integration | Embedded Systems Engineers, Hardware Security Specialists |
| `relayer/` | P2P gossip, networking, streaming transport | Networking Engineers, Systems Programmers |
| `docs/` | Architecture records (ADRs), guides | Documentation Team, Lead Maintainers |

---

## 5. Code of Conduct

All contributors are expected to uphold the standards established in our [Code of Conduct](../CODE_OF_CONDUCT.md) (Contributor Covenant v2.1). Violations may be reported to `security@depinmesh.org`.

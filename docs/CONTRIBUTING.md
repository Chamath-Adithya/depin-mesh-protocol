# Contributing to DePIN-Mesh Protocol

```text
Document: CONTRIBUTING.md
Status: Living Standard
Target: Contributors, Researchers, Core Maintainers
```

We welcome research and engineering contributions to the DePIN-Mesh Physical Evidence Consensus (PEC) protocol. To maintain research-grade institutional quality, all contributors must adhere to the standards outlined in this document.

---

## 1. Request for Comments (RFC) Process

All architectural, cryptographic, or physical specification changes must begin with a formal Request for Comments (RFC) document placed in `spec/`.

### 1.1 RFC Lifecycle

1. **Draft**: Initial proposal submitted via pull request using `.github/ISSUE_TEMPLATE/rfc_proposal.yml`.
2. **Review**: Open technical review period where working group maintainers evaluate mathematical soundness, security bounds, and IETF RATS alignment.
3. **Accepted**: Consensus reached; formal RFC number assigned (for example, `RFC-0002-NAME.md`).
4. **Implemented**: Reference implementations merged into `crates/`, `firmware/`, or `relayer/`.
5. **Obsoleted**: Superseded by a subsequent RFC document.

### 1.2 RFC Quality Standards

* **Mathematical Precision**: Formulations must define coordinate spaces, invariant bounds, and uncertainty propagation rules.
* **Determinism First**: Floating-point operations are prohibited in consensus-critical logic. Specifications must specify Q64.64 fixed-point operations complying with PEC-MATH-01.
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
Examples of approved scopes: `core`, `vm`, `proto`, `firmware`, `relayer`, `deps`, `cordic`.

### 2.3 Style Rules
* Write summaries in the imperative mood (for example, "add Q64.64 square root test vectors" instead of "added" or "adds").
* Do not end the summary line with a period.
* Keep the summary line under 72 characters.

---

## 3. Pull Request Guidelines

1. Open an issue or refer to an approved RFC before submitting substantial code changes.
2. Complete the checklist in `.github/PULL_REQUEST_TEMPLATE.md`.
3. Ensure CI workflows (`lint-docs.yml`, `spec-validation.yml`) pass without warnings.
4. Maintain deterministic test coverage for any mathematical routines.

---

## 4. Code of Conduct

### 4.1 Our Standard
We are committed to providing an open, professional, and respectful environment. Contributors must treat all participants with dignity and professional courtesy regardless of background, identity, or level of experience.

### 4.2 Unacceptable Behavior
Harassment, personal attacks, trolling, derogatory language, or bad-faith interactions are unacceptable and will result in permanent exclusion from project participation.

### 4.3 Reporting
Violations can be reported directly to the core maintainers via the security contact listed in repository administration.

# Security Policy: DePIN-Mesh Protocol

```text
Document: SECURITY.md
Status: Official Policy
Version: 1.0.0
Security Contact: security@depinmesh.org
```

## 1. Reporting a Vulnerability

The DePIN-Mesh Protocol Working Group takes the security of physical evidence consensus systems seriously. If you discover a vulnerability affecting the protocol specification, reference virtual machine (`crates/pec-vm`), core data structures (`crates/pec-core`), or firmware roots of trust (`firmware/`), please report it responsibly.

### 1.1 Reporting Channels
* **Email**: Send encrypted disclosures to `security@depinmesh.org`.
* **PGP Key Fingerprint**: `9D2E 4B8A 7C1F 3E50 8A62  D4B9 1F7C 2E3D 5B6A 7C8E` (available on standard keyservers).
* Please include:
  * Affected subsystem (for example, `crates/pec-vm/src/invariants/thermodynamics.rs` or `spec/RFC-0001`).
  * Detailed description of the vulnerability and attack vector.
  * Deterministic reproduction script, failing test case, or proof-of-concept evidence vector.
  * Evaluation of physical vs cryptographic exploitability.

### 1.2 Coordinated Disclosure Timeline
We operate under a coordinated 90-day vulnerability disclosure timeline:
* **Initial Acknowledgment**: Within 24 hours of receipt.
* **Triage and Impact Assessment**: Within 72 hours of acknowledgment.
* **Mitigation / Patch Development**: Within 30 days for critical vulnerabilities.
* **Public Disclosure**: Coordinated at 90 days post-report, or sooner if a verified fix is deployed across the network.

---

## 2. Security Boundaries and Scope

Understanding what constitutes an in-scope protocol vulnerability versus an expected physical limitation is essential.

### 2.1 In-Scope Security Vulnerabilities
* **Consensus Divergence**: Floating-point drift or non-deterministic behavior in `PEC-MATH-01` allowing nodes on different architectures (x86_64 vs aarch64 vs wasm32) to compute differing state transitions from identical evidence.
* **Hypergraph Evasion**: Flaws in the conflict resolution algorithm permitting contradictory evidence sets ($CS$) to settle as `VALID`.
* **Cryptographic Bypasses**: Forgery or replay vulnerabilities in `PhysicalEvidenceObject` attestation envelopes bypassing monotonic silicon counters.
* **Denial of Service in PEC-VM**: Unbounded algorithmic complexity or memory exhaustion in graph traversal and eigenvalue calculations ($N_{eff}$).

### 2.2 Out-of-Scope (Expected Protocol Boundaries)
* **Isolated Physical Transducer Compromise**: An adversary with physical access who mechanically forces a single transducer to register anomalous values while remaining within $A_{obs}$ dynamic range bounds. This is mitigated by cross-sensor dependency matrices ($D$) and spatial correlation invariants ($I_4$), not cryptographic proofs.
* **Firmware Extraction via Destructive Decapping**: Focused Ion Beam (FIB) attacks on silicon dies extracting private keys from secure elements. Protection is bounded by physical hardware certification levels (Common Criteria EAL6+).
* **Social Engineering and Staking Delegation**: Staking key theft or validator operator credential compromise unrelated to protocol cryptographic primitives.

---

## 3. Responsible Disclosure Bounty

Security researchers who discover novel vulnerabilities in accordance with this policy may be eligible for recognition in release advisories and compensation through the DePIN-Mesh Vulnerability Reward Program.

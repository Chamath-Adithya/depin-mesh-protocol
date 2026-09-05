# DePIN-Mesh Physical Evidence Consensus: System Architecture

```text
Document: ARCHITECTURE.md
Status: Architectural Baseline
Audience: Systems Architects, Core Developers, Cryptographic Engineers
Version: 0.1.0-alpha
```

## 1. System Overview

The DePIN-Mesh Physical Evidence Consensus (PEC) protocol provides deterministic verification of physical phenomena reported by distributed embedded devices. Rather than relying on simple cryptographic signature verification, PEC implements an empirical evaluation pipeline based on remote attestation standards (IETF RATS RFC 9334), graph-based conflict resolution, and deterministic fixed-point physics models.

```text
+-------------------------------------------------------------------------------+
|                             Subsystem Overview                                |
+-------------------------------------------------------------------------------+
  [Edge Firmware / Hardware RoT]
                │
                │  Protobuf Streaming over TLS / Noise Protocol
                ▼
  [P2P Relayer Network & Hypergraph Gossip]
                │
                │  Topological Graph Insertion
                ▼
  [Physical Evidence Hypergraph (PEH)]
                │
                │  Batch Extraction
                ▼
  [PEC Virtual Machine (PEC-VM)]
   ├── PEC-MATH-01 (Q64.64 Deterministic Math)
   ├── Dependency Tensor & Independence Solver (N_eff)
   ├── Physics Invariant Conservation Engine (I_0 - I_6)
   └── Conflict Resolution & Contradiction Pruning
                │
                │  4D Assurance Metric Coordinate
                ▼
  [Settlement Bridges & Consensus State Machines]
```

---

## 2. Subsystem Boundaries

### 2.1 Hardware Root of Trust and Firmware Layer (`firmware/`)

The edge boundary consists of physical sensors paired directly with secure silicon:
* **Microcontroller Core**: Typically Espressif ESP32-S3 or ARM Cortex-M33 running embedded Rust.
* **Silicon Root of Trust (RoT)**: Dedicated secure elements (for example, Microchip ATECC608B or Infineon OPTIGA Trust M) or silicon Physical Unclonable Functions (PUF).
* **Cryptographic Attestation**: The RoT maintains an asymmetric private key that never leaves the hardware boundary. Firmware builds generate measured boot measurements (DICE/TCG profile) committed into hardware registers.
* **Transducer Authority**: Telemetry is tagged with factory calibration polynomials and strict dynamic range bounds ($A_{obs}$). Readings outside certified operating temperatures or slew rates trigger fault flags at the hardware interface.

### 2.2 Relayer Transport and Aggregation Layer (`relayer/`)

The relayer topology forms an unpermissioned peer-to-peer transport network:
* **Evidence Ingestion**: Accepts signed `PhysicalEvidenceObject` payloads via QUIC or gRPC.
* **Deduplication and Caching**: Computes canonical `evidence_id = SHA-256(canonical_protobuf_bytes)` and deduplicates across gossiped routes.
* **Batching and Epoch Windows**: Groups evidence into temporal discretization epochs matching physical relaxation time constants (for example, 10-second consensus epochs).

### 2.3 Physical Evidence Hypergraph (`crates/pec-core`)

The Physical Evidence Hypergraph (PEH) is a typed directed hypergraph:
* **Vertices**: Typed entities representing Observations, Hardware Attestations, Invariant Models, Spatiotemporal Contexts, and Verifier Assertions.
* **Hyperedges**: N-ary directed relations representing Support, Contradiction, Conditioning, Derivation, and Dependency.
* **Contradiction Detection**: Identifies conflicting assertions (such as multiple sensors asserting contradictory temperatures at proximate coordinates exceeding diffusion tolerances).

### 2.4 PEC Virtual Machine (`crates/pec-vm`)

The deterministic evaluation engine executes identically across all verification nodes:
* **PEC-MATH-01 Execution**: Strictly relies on Q64.64 fixed-point arithmetic, banker's convergent rounding, and fixed 64-step CORDIC transcendentals to eliminate IEEE 754 floating-point platform discrepancies.
* **Dependency Matrix Computation**: Calculates the multi-factor cross-correlation matrix $D$ spanning physical distance, device architecture, operating keys, and economic incentives. Computes the effective evidence count $N_{eff} = M^2 / \sum D_{ij}^2$.
* **Physics Invariant Engine**: Evaluates conservation laws $I_0$ through $I_6$ against dynamic tolerance thresholds $\epsilon_E$.
* **Assurance Metric Emission**: Outputs the formal 4D Assurance Metric coordinate tuple:
  $$\langle \text{DecisionState}, \text{PhysicalAssurance}, \text{Freshness}, \text{SettlementFinality} \rangle$$

---

## 3. Threat Model and Verification Boundaries

| Threat Vector | Naive DePIN Vulnerability | PEC Protocol Mitigation |
| :--- | :--- | :--- |
| **Private Key Extraction** | Extracted key can sign arbitrary fabricated telemetry indefinitely. | Silicon RoT with monotonic counters; physics invariant cross-checks detect fabricated signals. |
| **Sybil Replication** | Single operator creates thousands of virtual nodes with simulated data. | Dependency matrix collapses $N_{eff} \to 1.0$ due to identical hardware fingerprints and zero spatial divergence. |
| **Environmental Spoofing** | Adversary alters local environment (such as heating a temperature probe with a lighter). | Spatial correlation envelopes ($I_4$) and active challenge-response protocols expose boundary discontinuities. |
| **Replay Attacks** | Capturing valid past telemetry and replaying it during peak reward windows. | Freshness verification ($F_0 - F_3$) bound to protocol challenge nonces and monotonic hardware counters. |
| **Transducer Drift** | Aging sensors reporting degraded telemetry. | Dynamic tolerance threshold $\epsilon_E$ incorporates time-since-calibration drift penalties $\kappa_{drift}(t - t_0)$. |

---

## 4. Deterministic State Transition

Every state transition in PEC is pure and side-effect free:

$$\sigma_{t+1} = \text{PEC-VM}(\sigma_t, \mathcal{E}_{epoch}, \mathcal{R}_{reference})$$

Where:
* $\sigma_t$: Pre-state snapshot of verified claims and assurance coordinates.
* $\mathcal{E}_{epoch}$: Batch of newly validated physical evidence hypergraph components.
* $\mathcal{R}_{reference}$: Canonical physics models, calibration registries, and geographic reference databases.

Any two independent nodes evaluating identical input streams will arrive at bit-for-bit identical evaluation reports and assurance coordinates.

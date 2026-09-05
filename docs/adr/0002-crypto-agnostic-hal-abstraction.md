# ADR 0002: Cryptographically Agnostic Hardware Abstraction Layer

```text
Status: Accepted
Date: September 2026
Deciders: Protocol Working Group, Hardware Architects
Consulted: Cryptographic Engineers, Embedded Developers
Informed: Core Contributors
```

## Context

Decentralized physical devices interface with heterogeneous cryptographic environments across edge silicon and settlement state machines:

1. **Secure Silicon Reality**: Commercial embedded secure elements (such as Microchip ATECC608B, NXP SE050, and STMicroelectronics STSAFE) are pre-provisioned in hardware factories to perform operations over the NIST P-256 (secp256r1) elliptic curve. Few cost-effective secure elements natively support Ed25519 or secp256k1 at commodity price points.
2. **Blockchain Settlement Constraints**: Primary smart contract settlement layers have differing native cryptographic precompiles. EVM networks favor secp256k1, whereas Solana and Polkadot prioritize Ed25519, and enterprise environments often interface via TPM 2.0 or HSM infrastructure.
3. **Curve Monoculture Risk**: Tying the protocol strictly to a single elliptic curve would either exclude commodity secure elements or penalize downstream settlement networks with prohibitively expensive signature verification overhead.

## Decision

We establish an abstract Hardware Abstraction Layer (`pec-hal`) that decouples protocol evidence structures from specific cryptographic curves.

Specifically:
1. **Abstract Root of Trust (RoT) Interface**: Define a generic Rust trait in `crates/pec-core/src/hal/` specifying hardware capabilities:
   * Device identity generation and key agreement.
   * Monotonic counter querying.
   * Measured boot attestation digest generation.
   * Nonce signing.
2. **Multi-Curve Signature Envelope**: Evidence and attestation envelopes identify the signature scheme via an explicit type tag:
   * `SIGNATURE_SCHEME_ED25519`
   * `SIGNATURE_SCHEME_ECDSA_SECP256K1`
   * `SIGNATURE_SCHEME_ECDSA_NIST_P256`
   * `SIGNATURE_SCHEME_TPM2_RSA_PSS`
3. **Hardware Driver Modularity**: Embedded firmware builds (`firmware/`) compile the specific driver corresponding to the target PCB configuration via cargo feature flags (for example, `features = ["rot-atecc608"]` or `features = ["rot-optiga"]`).

## Consequences

### Positive
* **Silicon Flexibility**: Hardware vendors can manufacture PEC-compatible nodes using any certified secure element or microcontroller PUF without redesigning the protocol.
* **Multi-Chain Interoperability**: Downstream relayers can convert or bridge attestations to native smart contract precompiles across EVM, Solana, and Cosmos networks without breaking end-to-end provenance.
* **Future Proofing**: Simplifies eventual post-quantum signature integration (such as ML-DSA / Dilithium) when hardware support matures.

### Negative
* **Trait Abstraction Complexity**: Requires additional trait indirection in the codebase compared to hardcoding a single cryptographic library like `ed25519-dalek`.
* **Verifier Overhead**: Verification nodes must bundle cryptographic implementations for multiple curves to validate arbitrary evidence payloads.

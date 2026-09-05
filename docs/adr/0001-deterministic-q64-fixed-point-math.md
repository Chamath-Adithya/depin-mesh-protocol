# ADR 0001: Deterministic Q64.64 Fixed-Point Mathematics Profile

```text
Status: Accepted
Date: September 2026
Deciders: Protocol Working Group, Lead Maintainers
Consulted: Systems Architects, Numerical Analysts
Informed: Core Contributors
```

## Context

Decentralized physical infrastructure networks require cross-node verification of continuous physical quantities, such as thermodynamic temperature dissipation, fluid flow velocity, and RF signal propagation. 

In standard software environments, continuous mathematics is implemented using IEEE 754 floating-point arithmetic (`f32` and `f64`). However, IEEE 754 floating-point operations are fundamentally non-deterministic across heterogeneous computing architectures:

1. **Transcendental Non-Determinism**: Trigonometric, logarithmic, and exponential functions (`sin`, `cos`, `exp`, `sqrt`) are not bit-exact across standard libraries (such as glibc vs musl vs compiler intrinsics).
2. **Fused Multiply-Add (FMA) Variance**: Architectures supporting FMA (such as modern x86_64 with AVX2 or ARM Cortex-M7) execute multiplication and addition with a single rounding step, whereas architectures without FMA execute two distinct rounding steps.
3. **Compiler Optimization Reordering**: Associative rearrangements by optimizing compilers under varying optimization levels (`-O2` vs `-O3`) alter low-order bits.

If verification nodes run on heterogeneous infrastructure (such as edge Raspberry Pi nodes, cloud x86 servers, and browser WASM clients), floating-point divergence will lead to irreconcilable consensus splits over identical physical evidence.

## Decision

We mandate the **`PEC-MATH-01`** profile for all consensus-critical computations within the DePIN-Mesh protocol.

Specifically:
1. **Representation**: All real numbers must be represented as two's-complement signed 128-bit integers with a fixed binary scale factor of $2^{-64}$ (Q64.64 format): 64 bits for the signed integer component and 64 bits for the fractional component.
2. **Saturation Arithmetic**: Operations that exceed the representation boundaries $[-2^{63}, 2^{63} - 2^{-64}]$ must saturate at `MAX` or `MIN` values rather than overflowing or wrapping.
3. **Convergent Rounding**: Halfway fractional cases round to the nearest even integer (banker's rounding), preventing statistical bias in numeric integration.
4. **CORDIC Trigonometry**: All transcendental functions (sine, cosine, arctangent, vector rotation) must be computed using a 64-iteration CORDIC algorithm with hardcoded fixed-point lookup tables.

## Consequences

### Positive
* **Bit-Level Determinism**: Verification outcomes are identical across x86_64, aarch64, riscv32, xtensa, and wasm32 targets.
* **Consensus Safety**: Eliminates the possibility of state machine forks caused by compiler or CPU floating-point discrepancies.
* **Embedded Compatibility**: Operates directly on low-power microcontrollers lacking hardware floating-point units (FPUs).
* **Formal Verification**: Bounded precision ($2^{-64} \approx 5.421 \times 10^{-20}$) enables formal verification of error margins and dynamic tolerances ($\epsilon_E$).

### Negative
* **Computational Overhead**: Software-emulated 128-bit fixed-point arithmetic introduces execution latency compared to native FPU instructions.
* **Developer Ergonomics**: Engineers must work with explicit fixed-point wrappers rather than standard primitive floating-point types.

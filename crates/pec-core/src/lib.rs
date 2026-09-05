//! # `pec-core`
//!
//! Core primitives and deterministic mathematics for the Physical Evidence Consensus (PEC) protocol.
//!
//! This crate implements:
//! - Deterministic signed Q64.64 fixed-point arithmetic (`PEC-MATH-01`).
//! - Physical Evidence Hypergraph (PEH) data structures.
//! - Protocol Buffer conversions and validation utilities.

#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub mod math;

pub use math::q64::Q64;

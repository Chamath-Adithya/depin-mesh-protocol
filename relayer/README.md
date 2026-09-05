# DePIN-Mesh Relayer Service

This directory contains the peer-to-peer evidence transport and hypergraph synchronization daemon.

## Functional Scope

* **P2P Transport**: High-throughput QUIC and gRPC streaming listeners for edge device telemetry ingestion.
* **Hypergraph Gossip**: Decentralized gossip protocol disseminating `PhysicalEvidenceObject` payloads across verification nodes.
* **Epoch Batching**: Discretization of continuous physical telemetry into consensus epochs for PEC-VM batch evaluation.

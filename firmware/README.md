# DePIN-Mesh Firmware Workspace

This directory contains reference embedded firmware implementations for physical transducer nodes.

## Target Hardware Support

* **Microcontroller Targets**: Espressif ESP32-S3 (Xtensa dual-core) and ARM Cortex-M33 running bare-metal or Embassy-rs embedded Rust.
* **Silicon Roots of Trust**: Hardware secure elements including Microchip ATECC608B, Infineon OPTIGA Trust M, and silicon Physical Unclonable Functions (PUF).
* **Transducer Interfacing**: Precision ADC, I2C, and SPI sensor telemetry drivers with calibrated authority envelopes ($A_{obs}$).

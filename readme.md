# 🏎️ Asynchronous Bare-Metal Rust Architecture for EV Safety Systems (Front Control Node)

![Rust](https://img.shields.io/badge/Rust-Bare--Metal-orange?style=for-the-badge&logo=rust)
![ESP32-S3](https://img.shields.io/badge/Hardware-ESP32--S3-red?style=for-the-badge&logo=espressif)
![Status](https://img.shields.io/badge/Status-HIL_Validated-success?style=for-the-badge)
![FSAE](https://img.shields.io/badge/Standard-FSAE_EV.5.5_&_EV.5.6-blue?style=for-the-badge)

This repository contains the firmware and documentation for a highly reliable, memory-safe, and deterministic **Front Control Node (Vehicle Control Unit)** designed for Formula Student Electric Vehicles. The system abandons traditional C/C++ architectures in favor of an asynchronous, bare-metal `no_std` Rust ecosystem to achieve absolute memory safety and zero-cost abstraction.

## ✨ Key Features & Novelty

* **Asymmetric Dual-Core Task Isolation (Separation Kernel):** * **Core 0:** Strictly executes safety-critical loops (APPS acquisition, zero-offset calibration, and plausibility algorithms).
    * **Core 1:** Handles deterministic CAN communication and diagnostic telemetry without preempting Core 0.
* **Software Zero-Offset Calibration:** A mathematical linear interpolation filter implemented at the firmware level to neutralize hardware-induced ADC voltage bias offsets, eliminating the need for passive hardware filters.
* **FSAE Rule EV.5.5 Compliance (Sensor Redundancy):** Continuous mathematical deviation monitoring between APPS 1 and APPS 2 ($<10\%$ tolerance).
* **FSAE Rule EV.5.6 Compliance (Panic Braking):** Plausibility checks that instantaneously cut off motor torque ($0\%$) in under 50ms if hard braking ($>10\%$) and acceleration ($>25\%$) occur simultaneously.
* **Hardware-in-the-Loop (HIL) Validated:** Physical validation proven via Wokwi (SIL) and dynamic hardware stress testing using GNUPlot logging.

## FOR THE FULL DOCUMENTATION, YOU CAN ACCESS THIS LINK
https://drive.google.com/drive/folders/1nqE1XG8Y_QH-t3CGdDA5vijLVmUt-Rg6?usp=drive_link

## 🗂️ Repository Structure

```text
├── .cargo/                 # Target architecture and probe-rs runner configs
├── assets/                 # System block diagrams, flowcharts, and GNUPlot results
├── src/                    # Rust source code (main, adc, plausibility logic)
├── Cargo.toml              # Rust dependencies (esp-hal, embassy-rs)
├── wokwi.toml              # Wokwi simulation configuration
└── README.md



# Technical Report: ESP32-C3 Embedded Rust Implementations
**Prepared by:** Senior Embedded Systems Engineer  
**Project:** Blink Lab (std vs no_std)  
**Target Architecture:** RISC-V (RV32IMC)

---

## 1. Executive Summary
This project evaluates two distinct methodologies for developing firmware on the ESP32-C3 SoC using the Rust programming language. We implemented a "Button-Triggered LED Toggle" application using both the **ESP-IDF (Standard Library)** and **Bare-Metal (no_std)** approaches. This report details the architectural decisions, resource utilization, and operational trade-offs observed during development and simulation.

## 2. Hardware Specification: ESP32-C3
The ESP32-C3 is a single-core, 32-bit, RISC-V-based MCU. 
- **Core:** RISC-V RV32IMC at up to 160 MHz.
- **Memory:** 400 KB SRAM, 384 KB ROM.
- **Connectivity:** 2.4 GHz Wi-Fi and Bluetooth 5 (LE).
- **Peripherals utilized:** GPIO8 (Output/LED), GPIO9 (Input/Button).

## 3. Implementation Methodologies

### 3.1 Bare-Metal Approach (`no_std`)
The `no_std` implementation utilizes the `esp-hal` crate, targeting a zero-dependency environment suitable for mission-critical and resource-constrained applications.

- **Architectural Flow:**
    1. **HAL Initialization:** Hardware abstraction layer is initialized directly from peripherals.
    2. **Static Configuration:** GPIO8 is configured as a Push-Pull output; GPIO9 as an input with internal Pull-Up resistors.
    3. **The Super-Loop:** A polling loop monitors the button state. Upon a low logic level (button press), the LED state is toggled using a software delay.
- **Key Characteristics:**
    - **Binary Size:** Exceptionally small footprint (< 50KB).
    - **Control:** Precise control over the boot sequence and watchdog timers.
    - **Determinism:** No underlying OS scheduler ensures predictable execution timing.

### 3.2 Standard Library Approach (`std`)
The `std` implementation leverages `esp-idf-hal`, which provides a wrapper around the Espressif IoT Development Framework (ESP-IDF).

- **Architectural Flow:**
    1. **ESP-IDF Integration:** Bootstraps the FreeRTOS kernel and Espressif system services.
    2. **High-Level Abstraction:** Uses standard Rust `std::time` and `std::thread` for timing and concurrency.
    3. **Logic:** Similar super-loop pattern, but abstracted through the ESP-IDF hardware drivers.
- **Key Characteristics:**
    - **Ease of Development:** Access to the full Rust standard library (collections, networking, etc.).
    - **Underlying OS:** Runs on top of FreeRTOS, enabling complex multi-tasking.
    - **Binary Size:** Significantly larger due to the inclusion of the ESP-IDF stack.

## 4. Technical Comparison & Analysis

| Metric | `no_std` (Bare-Metal) | `std` (ESP-IDF) |
| :--- | :--- | :--- |
| **Runtime** | Bare-metal entry point | FreeRTOS Kernel |
| **Standard Library** | `core` + `alloc` (optional) | Full `std` |
| **Memory Footprint** | Minimal (SRAM optimized) | Moderate (OS overhead) |
| **Interrupt Latency** | Low / Deterministic | Variable (Scheduler dependent) |
| **Development Speed** | Moderate (HAL-specific) | High (Standard Rust) |
| **Binary Portability** | MCU specific | ESP-IDF specific |

## 5. Development Workflow & Tooling
The project utilizes the modern Rust embedded ecosystem:
- **Build System:** `Cargo` with `cargo-espflash` for compilation and deployment.
- **Linker:** `ldproxy` (required for `std` builds to link C-based ESP-IDF components).
- **Toolchain:** `nightly-riscv32imc-unknown-none-elf` for bare-metal targets.

## 6. Verification via Simulation
Functional verification was performed using **Wokwi**, a high-fidelity web-based simulator.
- **Peripheral Mapping:** Defined in `diagram.json`, mapping GPIO8 to a red LED and GPIO9 to a tactile switch.
- **Firmware Integration:** `wokwi.toml` specifies the ELF/BIN output paths for automated loading.

## 7. Engineering Recommendations
- **Choose `no_std`** for low-power sensors, real-time control systems, and devices where flash/RAM budget is extremely tight.
- **Choose `std`** for IoT gateways, devices requiring complex networking (HTTPS, MQTT), and rapid prototyping where developer productivity outweighs binary size.

---
*End of Report*

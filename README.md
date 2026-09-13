Velocity ARMv8 Memory Bus & Cache Saturation Evaluation Harness
A minimal, zero-dependency userspace measurement harness written in Rust to quantify contiguous memory bus throughput, cache-line invalidation recovery, and execution variance under adversarial bus saturation on ARMv8-A (AArch64) silicon.
1. Scope, Deliverables & Intellectual Property Boundary
What This Repository Contains
￼ Open Source Measurement Code (Apache 2.0): The software in this repository (⁠src/⁠, ⁠Cargo.toml⁠, and associated build configuration) is an open-source evaluation utility designed to profile memory bus throughput, cache-line invalidation recovery, and timer behavior.
￼ Diagnostic Tooling Only: This repository consists strictly of userspace evaluation code running against standard Linux APIs and architectural CPU timer registers.
What This Repository Does Not Contain
￼ No Hardware RTL or Gate Logic: This codebase does not contain Verilog/VHDL netlists, coprocessor RTL, or gate-level silicon logic.
￼ Patent Reservation: Underlying microarchitectural thermodynamic governance circuits, out-of-order execution gating registers, and coprocessor designs are proprietary intellectual property held under patent applications by Velocity Technical Infrastructure Statutory Trust. They are not disclosed, embodied, or licensed herein.
￼ Grant Boundary: The Apache License, Version 2.0 patent license grant (Section 3) applies exclusively to the measurement harness software committed in ⁠src/⁠ and does not extend to offline hardware patent claims.
2. Microarchitectural Methodology & Timing Limits
Timing Source & Resolution Limits
￼ Register Used: Hardware timer register ⁠CNTVCT_EL0⁠ read via inline assembly (⁠mrs x0, cntvct_el0⁠).
￼ Counter Base Frequency: On the Broadcom BCM2711, the ARM Generic Timer counter increments at a fixed nominal frequency of ￼ (￼ per tick).
￼ Quantization & Statistical Treatment:
￼ Because each timer tick represents ￼, sub-nanosecond jitter or fine-grained per-iteration ￼ latency distributions cannot be physically resolved by sampling this register per pass.
￼ The harness measures elapsed ticks across a continuous batch of ￼ iterations bracketed by start and end timestamps.
￼ Reported metrics reflect mean amortized latency per loop pass (￼), characterizing sustained memory bus throughput under saturation.
￼ Fine-grained cycle-by-cycle tail distribution analysis (￼, ￼) requires non-virtualized access to the hardware Performance Monitor Unit cycle counter (⁠PMCCNTR_EL0⁠ at ￼ resolution), which requires an EL1 kernel driver module.
Memory & Contention Model
￼ Static Region: Allocates a contiguous ￼ static ⁠.bss⁠ ring buffer to ensure accesses exceed the ￼ shared L2 cache of the Cortex-A72.
￼ Runtime Allocation: Zero dynamic heap requests (⁠malloc⁠/⁠alloc⁠) occur during active measurement loops.
￼ Runtime Standard Library: The Rust standard library (⁠std⁠) is utilized solely for command-line parsing, initial environment setup, and timer fallback on non-ARM host architectures.
￼ Access Pattern: Strided read/write traversals across the ring buffer designed to enforce deterministic cache-line evictions and stress the memory bus.
3. Empirical Test Environment & Baseline Telemetry
Hardware & Operating System Configuration
Empirical Measurements (Broadcom BCM2711)
1. Amortized Saturation Batch (￼ Iterations)
￼ Total Batch Wall Time: ￼
￼ Hardware Timer Ticks Elapsed: ￼ (@ ￼)
￼ Mean Amortized Access Latency: ￼ (￼ per cache-line dispatch)
￼ Microarchitectural Interpretation: The observed ￼ mean latency reflects the physical LPDDR4 DRAM random access load-to-use floor following deterministic L1/L2 cache misses on the BCM2711 memory controller.
2. Thermal Envelope (Separate 60-Second Continuous Soak)
￼ Start Core Temperature: ￼
￼ End Core Temperature: ￼
￼ Thermal Delta: ￼ under active dual-fan cooling over a sustained 60-second saturation loop.
(Raw execution log archived in ⁠results/bcm2711_run.log⁠.)
4. Building & Execution
Prerequisites
Install the stable Rust toolchain for AArch64:
Build and Run
5. Continuous Integration
Automated compilation and format verification are maintained via GitHub Actions across standard AArch64 and x86_64 runners:
￼ Workflow configuration: ⁠.github/workflows/ci.yml⁠.
6. Research & Technical Correspondence
For inquiries regarding replication logs, cross-compilation on ARMv8 server platforms (such as AWS Graviton), or microarchitectural collaboration:
￼ Timothy Darcelien — ⁠timdarcelien@icloud.com⁠

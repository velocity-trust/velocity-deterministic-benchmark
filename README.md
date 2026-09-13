Velocity Memory Bus & Cache Saturation Evaluation Harness
A minimal userspace measurement harness written in Rust to quantify contiguous memory bus throughput, cache line invalidation recovery, and execution variance on ARMv8-A (AArch64) hardware.
1. Scope, Deliverables & Intellectual Property Boundary
What This Repository Contains
￼ Apache License, Version 2.0: The software in this repository (⁠src/⁠, ⁠Cargo.toml⁠, and associated build configuration) is an open-source evaluation and benchmarking utility designed to stress memory buses and measure cycle latency.
￼ Benchmarking Code Only: This repository consists solely of userspace diagnostic routines executing against standard Linux APIs and architectural timer registers.
What This Repository Does Not Contain
￼ No Coprocessor Gate Logic: This repository does not contain hardware RTL, Verilog/VHDL netlists, or gate-level microarchitectural specifications.
￼ Patent Reservation: The underlying hardware-enforced thermodynamic governance circuits, out-of-order execution gating registers, and physical coprocessor architectures held under patent applications by Velocity Technical Infrastructure Statutory Trust are proprietary trade secrets. They are not disclosed, licensed, embodied, or distributed in this codebase.
￼ Grant Boundary: The Apache 2.0 patent license grant (Section 3) applies exclusively to the software measurement harness code committed in ⁠src/⁠ and does not extend to offline hardware patent claims.
2. Microarchitectural Methodology & Timing Limits
Timing Source & Resolution Limits
￼ Register Used: Physical counter register ⁠CNTVCT_EL0⁠ read via inline assembly (⁠mrs x0, cntvct_el0⁠).
￼ Counter Base Frequency: The Broadcom BCM2711 ARM Generic Timer increments at a fixed nominal frequency of ￼ (￼ per tick).
￼ Quantization Floor: Because a single clock tick represents ￼, sub-nanosecond jitter or fine-grained per-iteration ￼ latency cannot be physically resolved using this register alone.
￼ Statistical Treatment:
￼ The harness records elapsed hardware ticks over continuous batches of ￼ iterations.
￼ Reported figures reflect mean amortized latency per loop dispatch (￼) to characterize average memory bus throughput under saturation.
￼ Fine-grained cycle-by-cycle tail variance (￼, ￼) requires non-virtualized access to the hardware Performance Monitor Unit (⁠PMCCNTR_EL0⁠ at ￼ resolution), which requires an EL1 kernel driver module.
Memory & Contention Model
￼ Static Region: Allocates a continuous ￼ ⁠.bss⁠ ring buffer. Zero dynamic heap allocations (⁠malloc⁠/⁠alloc⁠) occur during active measurement iterations.
￼ Standard Runtime: The Rust standard library (⁠std⁠) is utilized for command-line parsing, initial environment setup, and timer fallback on non-ARM host architectures.
￼ Access Pattern: Sequential and strided cache line read/write traversals designed to induce deterministic L1/L2 data cache evictions.
3. Empirical Test Environment & Baseline Measurements
Hardware & Operating System Baseline
Amortized Throughput Metrics (￼ Iteration Batch)
￼ Total Batch Wall Time: ￼
￼ Hardware Timer Ticks Elapsed: ￼ (￼)
￼ Mean Amortized Dispatch Latency: ￼ (￼ per pass)
￼ Heap Memory Allocated: ￼ during active loop
￼ Thermal Envelope: ￼ (￼ delta over 60 seconds)
(Raw execution log available in ⁠results/bcm2711_run.log⁠.)
4. Building & Execution
Prerequisites
Install the stable Rust toolchain for AArch64:
Build and Run
5. Continuous Integration
Automated compilation and format verification are maintained via GitHub Actions across standard AArch64 and x86_64 runners:
￼ See ⁠.github/workflows/ci.yml⁠.
6. Research Correspondence
For inquiries regarding replication data, cross-compilation configurations on ARMv8 server platforms, or academic discussion:
￼ Timothy Darcelien — ⁠timdarcelien@icloud.com⁠

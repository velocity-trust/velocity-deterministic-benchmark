Velocity Memory Bus & Cache Saturation Evaluation Harness
A minimal userspace evaluation harness written in Rust to quantify contiguous memory bus throughput, cache line invalidation recovery, and execution variance on ARMv8-A (AArch64) hardware.
1. Scope, Deliverables & Intellectual Property Boundary
What This Repository Contains
￼ Apache License, Version 2.0: The software in this repository (⁠src/⁠, ⁠Cargo.toml⁠, and associated build configuration) is an open-source evaluation and benchmarking utility designed to stress memory buses and measure cycle latency.
￼ Benchmarking Code Only: This repository consists solely of userspace diagnostic routines executing against standard Linux APIs and architectural timer registers.
What This Repository Does Not Contain
￼ No Coprocessor Gate Logic: This repository does not contain hardware RTL, Verilog/VHDL netlists, or gate-level microarchitectural specifications.
￼ Patent Reservation: Underlying hardware architectures and execution technologies are protected under pending patent applications held by Velocity Technical Infrastructure Statutory Trust. No patent rights are granted, implied, or waived beyond the software evaluation terms of the Apache 2.0 license.
2. Microarchitectural Methodology & Timing Limits
Timing Source & Resolution Limits
￼ Register Used: Physical counter register ⁠CNTVCT_EL0⁠ read via inline assembly (⁠mrs x0, cntvct_el0⁠).
￼ Counter Base Frequency: The Broadcom BCM2711 ARM Generic Timer increments at a fixed nominal frequency of 54 MHz (~18.518 ns per tick).
￼ Quantization Floor: Because a single clock tick represents ~18.5 ns, sub-nanosecond jitter or fine-grained per-iteration tail latency cannot be physically resolved using this register alone. Fine-grained cycle-by-cycle tail variance requires non-virtualized access to the hardware Performance Monitor Unit (⁠PMCCNTR_EL0⁠ at 1.8 GHz ~ 0.55 ns resolution) via an EL1 kernel driver module.
Memory & Contention Model
￼ Static Region: Allocates a continuous 16 MB ⁠.bss⁠ ring buffer. Zero dynamic heap allocations (⁠malloc⁠/⁠alloc⁠) occur during active measurement iterations.
￼ Standard Runtime: The Rust standard library (⁠std⁠) is utilized for command-line parsing, initial environment setup, and timer fallback on non-ARM host architectures.
3. Baseline Validation & Microarchitectural Envelope
To validate the harness against independent C implementations on identical hardware (Broadcom BCM2711, 4x Cortex-A72 @ 1.8 GHz, active cooling, pinned to core 3), a two-mode null test was executed across an out-of-cache 16 MB working set (1,000,000 iterations, 11 median runs):
Access Pattern
Description
Measured Median Latency
Linear Stride (Mode A)
Prefetcher-friendly streaming traversal (+64 B)
~16.05 ns / pass
Velocity Harness
Contiguous ring traversal batch
~55.20 ns / pass
Random Pointer Chase (Mode B)
Full 16 MB un-cached DRAM + TLB page walk
~161.13 ns / pass

￼ Analysis: Mode A demonstrates that naive streaming loads saturate the prefetcher at ~16 ns. Mode B establishes the true un-cached LPDDR4 DRAM and TLB page-walk load-to-use floor at ~161 ns. The harness's amortized ~55.2 ns metric reflects intermediate bus traversal characteristics under batch execution.
4. Empirical Test Environment
Hardware & Operating System Baseline
￼ SoC: Broadcom BCM2711 (ARM Cortex-A72, 4 cores).
￼ Clock Configuration: Base clock 1.5 GHz; overclocked to 1.8 GHz via ⁠arm_freq=1800⁠ in ⁠/boot/firmware/config.txt⁠.
￼ Frequency Pinning: CPU governor set to ⁠performance⁠ via sysfs.
￼ Kernel Configuration: Linux 6.6.20+rpt-rpi-v8 (Debian aarch64) under standard ⁠CONFIG_PREEMPT⁠ (soft real-time desktop preemption). Note: Not a hard real-time ⁠CONFIG_PREEMPT_RT⁠ patch; core isolation flags (⁠isolcpus⁠, ⁠nohz_full⁠, ⁠rcu_nocbs⁠) were not enabled, establishing an unisolated userspace baseline.
(Raw execution telemetry archived in ⁠results/bcm2711_run.log⁠.)
5. Building & Execution
Prerequisites
Install the stable Rust toolchain for AArch64:
Build and Run
6. Continuous Integration
Automated compilation and format verification are maintained via GitHub Actions across standard AArch64 and x86_64 runners:
￼ See ⁠.github/workflows/ci.yml⁠.
7. Research Correspondence
For inquiries regarding replication logs or cross-compilation configurations:
￼ Timothy Darcelien — ⁠timdarcelien@icloud.com⁠

Velocity ARMv8 Memory Bus & Cache Saturation Evaluation Harness
A minimal, zero-dependency userspace measurement harness written in Rust to evaluate contiguous memory bus throughput under continuous cache-line invalidation on ARMv8-A (AArch64) hardware.
1. Scope and Intellectual Property Boundary
What This Repository Contains
￼ Open Source Measurement Software (Apache 2.0): The code in this repository (⁠src/⁠, ⁠Cargo.toml⁠, and associated build configuration) is an open-source evaluation utility designed to profile memory bus throughput under continuous cache-line invalidation.
￼ Userspace Tooling Only: This repository consists strictly of userspace software running against standard Linux APIs and architectural CPU timer registers.
Hardware and Patent Notice
￼ Underlying proprietary hardware architectures and circuit designs are held by Velocity Technical Infrastructure Statutory Trust under pending patent applications.
￼ No hardware RTL, netlists, or gate-level silicon logic are disclosed or distributed in this repository.
￼ Formal licensing terms and definitive IP covenants are governed by separate legal instruments and counsel review.
2. Microarchitectural Methodology and Timing Limits
Timing Source and Resolution Limits
￼ Register Used: Hardware timer register ⁠CNTVCT_EL0⁠ read via inline assembly (⁠mrs x0, cntvct_el0⁠).
￼ Counter Base Frequency: On the Broadcom BCM2711, the ARM Generic Timer counter increments at a fixed nominal frequency of 54 MHz (~18.518 ns per tick).
￼ Quantization and Statistical Scope:
￼ Because each timer tick represents ~18.518 ns, fine-grained sub-nanosecond jitter or cycle-by-cycle tail latency distributions (P99, P99.9) cannot be physically resolved by sampling this register per pass.
￼ The harness measures elapsed ticks across a continuous batch of 1,000,000 iterations bracketed by start and end timestamps.
￼ Reported metrics reflect mean amortized latency per loop pass (total elapsed ticks / 1,000,000), characterizing sustained memory bus throughput under saturation.
￼ Fine-grained cycle-by-cycle tail distribution analysis requires non-virtualized access to the hardware Performance Monitor Unit cycle counter (⁠PMCCNTR_EL0⁠ at 1.8 GHz ~ 0.55 ns resolution), which requires an EL1 kernel driver module.
Memory and Contention Model
￼ Static Buffer: Allocates a contiguous 16 MB static ⁠.bss⁠ ring buffer to ensure accesses exceed the 1 MB shared L2 cache of the Cortex-A72.
￼ Runtime Allocation: Zero dynamic heap requests (⁠malloc⁠/⁠alloc⁠) occur during active measurement loops.
￼ Standard Library Runtime: The Rust standard library (⁠std⁠) is utilized solely for command-line parsing, initial environment setup, and timer fallback on non-ARM host architectures.
￼ Access Pattern: Strided traversals across the buffer designed to induce continuous L1/L2 cache evictions and stress the memory bus.
3. Empirical Test Environment and Baseline Telemetry
Hardware and Operating System Configuration
Empirical Measurements (Broadcom BCM2711)
1. Amortized Saturation Batch (1,000,000 Iteration Batch)
￼ Total Batch Wall Time: ~55.24 ms
￼ Hardware Timer Ticks Elapsed: ~2,983,000 ticks (@ 54 MHz)
￼ Mean Amortized Access Latency: ~55.2 ns (~2.98 timer ticks per cache-line dispatch)
￼ Microarchitectural Context: The observed ~55.2 ns mean access duration aligns with typical LPDDR4 DRAM random access latency following deterministic L1/L2 cache misses on the BCM2711 memory controller.
2. Thermal Profile (Separate 60-Second Continuous Soak)
￼ Start Core Temperature: 38.0 C
￼ End Core Temperature: 41.8 C
￼ Thermal Delta: +3.8 C over a sustained 60-second saturation loop under active dual-fan cooling.
(Raw execution log archived in ⁠results/bcm2711_run.log⁠.)
4. Building and Execution
Prerequisites
Install the stable Rust toolchain for AArch64:
Build and Run
5. Continuous Integration
Automated compilation and format verification are maintained via GitHub Actions:
￼ Workflow configuration: ⁠.github/workflows/ci.yml⁠.
6. Research and Technical Correspondence
For inquiries regarding replication logs, cross-compilation on ARMv8 server platforms, or microarchitectural collaboration:
￼ Timothy Darcelien — ⁠timdarcelien@icloud.com⁠

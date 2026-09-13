Velocity Deterministic Systems Benchmark Harness
A minimal, zero-allocation evaluation harness measuring cycle timing determinism and contiguous memory bus behavior on ARMv8/AArch64 silicon.
Microarchitectural Focus & Methodology
￼ Target Microarchitecture: Broadcom BCM2711 (ARM Cortex-A72 @ 1.5–1.8 GHz). Cross-compilation compatible with AArch64 cloud instances (AWS Graviton).
￼ Memory Model: Static 16 MB contiguous ⁠.bss⁠ ring buffer. Execution loops operate with zero dynamic heap allocations (⁠malloc⁠/⁠alloc⁠ bypassed during measurement iterations). The ⁠std⁠ runtime is utilized solely for environment initialization and timer fallback on non-ARM hosts.
￼ Timer Source & Resolution:
￼ Telemetry on ARMv8 is derived directly from the physical counter register ⁠CNTVCT_EL0⁠.
￼ Clock Base: The BCM2711 system counter clock operates at a nominal 54 MHz (~18.518 ns per tick).
￼ Amortization: Single-digit nanosecond metrics reflect mean amortized loop dispatch across 1,000,000 contiguous iterations to account for hardware tick quantization.
￼ Contention Profile: Evaluates stride determinism under deliberate cache-line invalidation and bus saturation.
Empirical Test Conditions & Results
Hardware Test Environment
￼ Platform: Raspberry Pi 4 Model B (4GB LPDDR4)
￼ Kernel: Linux 6.6.20+rpt-rpi-v8 (preemptive AArch64)
￼ CPU Governor: ⁠performance⁠ (pinned @ 1.8 GHz via ⁠scaling_governor⁠)
￼ Thermals & Cooling: Active 5V fan with aluminum heatsink enclosure at 21°C ambient. Continuous run duration: 60 seconds.
Telemetry Summary (1,000,000 Contiguous Iterations)
￼ Mean Amortized Latency: ~55.2 ns per pass (~3 timer ticks @ 54 MHz)
￼ Observed Loop Jitter: Bounded within hardware tick resolution (±1 tick / ~18.5 ns)
￼ Dynamic Heap Allocations: 0.00 bytes during benchmark execution
￼ Thermal Range: 38.0°C to 41.8°C under active heatsink cooling (+3.8°C delta)
(Raw terminal output and environment telemetry are archived in ⁠results/bcm2711_run.log⁠.)
Building & Reproducing
Prerequisites
Install the stable Rust toolchain:
Build and Run
Continuous Integration
Automated compilation and lint checks run via GitHub Actions on every pull request and push to ⁠main⁠ across AArch64 and x86_64 target profiles.
License & Intellectual Property
￼ The benchmark harness software is licensed under the Apache License, Version 2.0 (see ⁠LICENSE⁠).
￼ Underlying coprocessor specifications and hardware execution invariants are proprietary intellectual property held by Velocity Technical Infrastructure Statutory Trust.
Technical Inquiries & Contact
For technical inquiries, architectural replication questions, or research correspondence:
￼ Timothy Darcelien — ⁠timdarcelien@icloud.com⁠

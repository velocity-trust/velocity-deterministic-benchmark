Velocity Deterministic Systems Benchmark Harness
A minimal, zero-allocation evaluation harness measuring cycle timing determinism and contiguous memory bus behavior on ARMv8/AArch64 silicon.
Microarchitectural Focus & Methodology
￼ Target Architecture: ARMv8-A (Broadcom BCM2711, 4x Cortex-A72 @ 1.5–1.8 GHz) & AArch64 cloud platforms (AWS Graviton).
￼ Memory Model: Static 16 MB contiguous ⁠.bss⁠ ring buffer. Zero dynamic heap requests (⁠malloc⁠/⁠alloc⁠ unused during measurement iterations).
￼ Timer Source & Resolution:
￼ Telemetry is derived directly from the physical counter register ⁠CNTVCT_EL0⁠.
￼ Clock Base: The BCM2711 system counter clock operates at nominal 54 MHz (~18.518 ns per tick).
￼ Amortization: Single-digit nanosecond metrics reflect mean amortized loop dispatch over 1,000,000 batch iterations to account for hardware tick quantization. Non-ARM host environments fall back to ⁠std::time::Instant⁠.
￼ Contention Profile: Evaluates stride determinism under deliberate cache-line invalidation and bus saturation.
Empirical Test Conditions & Results
Hardware Environment
￼ Host Platform: Raspberry Pi 4 Model B (4GB LPDDR4)
￼ Kernel: Linux 6.6.20+rpt-rpi-v8 (preemptive AArch64)
￼ CPU Governor: ⁠performance⁠ (pinned @ 1.8 GHz via ⁠scaling_governor⁠)
￼ Thermals & Cooling: Active 5V fan with aluminum heatsink enclosure at 21°C ambient. Test duration: 60 seconds continuous iteration.
Telemetry Summary (1,000,000 Iterations)
￼ Mean Amortized Latency: ~55 ns per memory pass (~3 timer ticks @ 54 MHz)
￼ P99 Amortized Latency: ~56 ns
￼ Dynamic Heap Overhead: 0.00 bytes allocated at runtime
￼ Thermal Envelope: 38.0°C to 41.8°C under active heatsink cooling (+3.8°C delta)
(Raw output logs and environmental parameters are recorded in ⁠results/bcm2711_run.log⁠.)
Building & Reproducing
Prerequisites
Install the stable Rust toolchain for AArch64:
Build and Run
Continuous Integration
Automated compilation and lint checks run via GitHub Actions on every pull request and push to ⁠main⁠.
License & Intellectual Property
￼ The benchmark harness software is licensed under the Apache License, Version 2.0 (see ⁠LICENSE⁠).
￼ Underlying coprocessor architectures and execution invariants are proprietary intellectual property held by Velocity Technical Infrastructure Statutory Trust.
Inquiries & Verification
For technical inquiries, Graviton reproduction logs, or architecture pilot discussions:
￼ Timothy Darcelien — ⁠timdarcelien@icloud.com⁠

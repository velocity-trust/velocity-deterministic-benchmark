Velocity Deterministic Systems Benchmark Harness
A minimal, zero-allocation (⁠#![no_std]⁠) evaluation harness measuring cycle-level instruction timing and contiguous memory bus determinism on ARMv8/AArch64 silicon.
Architecture & Focus
￼ Target Microarchitecture: Broadcom BCM2711 (ARM Cortex-A72 @ 1.5–1.8 GHz) & AArch64 datacenter platforms (AWS Graviton).
￼ Memory Model: Contiguous 16MB ⁠.bss⁠ static ring buffer; 0.00 bytes heap allocation (⁠alloc⁠ disabled).
￼ Hardware Telemetry: Direct cycle-counter register reads via ⁠CNTVCT_EL0⁠ with fallback to ⁠std::time::Instant⁠ on non-ARM hosts.
￼ Deterministic Boundary: Measures loop dispatch latency under adversarial cache-saturation memory access.
Observed Edge Benchmarks (Raspberry Pi 4 / BCM2711)
The following metrics represent local runs over 1,000,000 iterations on standard Raspberry Pi OS:
￼ Latency: ￼ | ￼
￼ Measured Loop Jitter: ￼ across iteration cycles
￼ Operating Thermals: ￼ baseline operating envelope
￼ Memory Allocation: 0 bytes dynamic heap
(See ⁠results/bcm2711_run.log⁠ for raw terminal output.)
Building & Running
Ensure you have a modern Rust toolchain installed:
To run the full criterion benchmark harness:
License & IP Notice
￼ Harness code and evaluation scripts are licensed under the Apache License, Version 2.0 (see ⁠LICENSE⁠).
￼ Hardware architecture, coprocessor specifications, and associated statutory trust filings are referenced in ⁠NOTICE⁠.
Contact
For questions regarding multi-core determinism replication, Graviton test runs, or research inquiries:
￼ Contact: Timothy Darcelien — ⁠timdarcelien@icloud.com⁠

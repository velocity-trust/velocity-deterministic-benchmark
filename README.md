# Velocity ARMv8 Memory Bus Evaluation Harness

A minimal, zero-allocation userspace harness for measuring contiguous memory
bus throughput on ARMv8-A (AArch64) hardware, with cross-validation against an
independent C implementation.

Every figure below was measured on the hardware described in section 4 and is
reproducible with the commands in section 6.

---

## 1. Scope and Intellectual Property Boundary

### What this repository contains

- **Open source measurement software (Apache 2.0).** The code in `src/`,
  `benches/`, `null_test.c`, and the associated build configuration is an
  evaluation utility for profiling memory bus throughput under strided access.
- **Userspace tooling only.** Everything here runs against standard Linux APIs
  and architectural CPU timer registers.

### What this repository does not contain

- No hardware RTL, netlists, or gate-level silicon logic.
- Underlying proprietary hardware architectures and circuit designs are held by
  Velocity Technical Infrastructure Statutory Trust under pending patent
  applications.
- Formal licensing terms and definitive IP covenants are governed by separate
  legal instruments and counsel review.

---

## 2. Methodology and Timing Limits

### Timer source

- **Register:** `CNTVCT_EL0`, read via inline assembly (`mrs {}, cntvct_el0`),
  preceded by `isb` to prevent reordering against surrounding instructions.
- **Frequency:** read at runtime from `CNTFRQ_EL0`. On the BCM2711 this reports
  **54.000 MHz**, or **18.519 ns per tick**.

### Why measurements are batch-amortized

A single tick is 18.519 ns. A memory access on this platform costs 15 to 50 ns,
so per-iteration sampling of this register resolves an access to two or three
ticks — a quantization error of roughly 33 percent per sample, before any
consideration of what the barrier semantics do to the measured window.

The harness therefore brackets a continuous batch of 1,000,000 iterations with
two counter reads and divides. Reported figures are mean nanoseconds per pass,
taken as the min, median, and max across 11 independent batches.

Cycle-by-cycle tail distribution analysis (P99, P99.9) is **not** possible from
this register. It requires `PMCCNTR_EL0`, the PMU cycle counter, which needs an
EL1 kernel driver to enable from userspace. No percentile figures are reported
here because none can be honestly derived.

Section 5 demonstrates empirically why per-iteration timing on this counter
fails.

### Memory and contention model

- **Static buffer:** a contiguous 16 MB `.bss` ring buffer, chosen to exceed the
  1 MB shared L2 of the Cortex-A72 so accesses reach DRAM.
- **Access pattern:** 64-byte strided traversal, inducing continuous L1/L2 cache
  eviction.
- **Runtime allocation:** zero. This is measured by a counting global allocator
  wrapping `System`, not asserted. See section 4.
- **Standard library:** `std` is used for setup and printing only, and for the
  timer fallback path on non-ARM hosts.

---

## 3. Test Environment

### Hardware

- Raspberry Pi 4 Model B, 4 GB LPDDR4
- Broadcom BCM2711, 4x Cortex-A72
- Active dual-fan aluminium heatsink, 21.0 C ambient

### Kernel configuration

Linux `6.18.34+rpt-rpi-v8` (Debian aarch64), running under standard
`CONFIG_PREEMPT` — soft real-time desktop preemption.

**This is not a hard real-time `CONFIG_PREEMPT_RT` kernel.** Core isolation
flags (`isolcpus`, `nohz_full`, `rcu_nocbs`) were not enabled for this baseline.
Measurements reflect standard unisolated userspace scheduling, and the spread
between min and max in section 4 includes ordinary scheduler jitter.

The CPU governor was set to `performance` and the benchmark was pinned to core 3
with `taskset`.

---

## 4. Results

All figures in nanoseconds per pass, 1,000,000 iterations, 11 batches.

| Mode | min | median | max |
|---|---:|---:|---:|
| 1. Batch-bracketed, write + read | 47.84 | **50.74** | 55.75 |
| 2. Batch-bracketed, read only | 15.20 | **15.76** | 15.92 |
| 3. Timer overhead (2 counter reads) | 27.22 | **27.26** | 27.31 |
| 4. Per-iteration, `isb` only | 27.52 | 27.63 | 30.16 |
| 5. Per-iteration, `dsb sy` | 193.09 | 193.85 | 194.81 |

**Heap allocated during measurement: 0 bytes** (counted, not asserted).

### Read versus write cost

A write + read pass costs **34.98 ns more** than a read-only pass (50.74 vs
15.76). The difference is read-for-ownership on the store plus the eventual
dirty-line writeback — roughly 3.2x the cost of a load on this memory
controller.

### Cross-validation against independent C

`null_test.c` implements the same access patterns in C, timed with
`clock_gettime(CLOCK_MONOTONIC)` — a different language and a completely
independent clock source. Compiled with `gcc -O2`:

| Pattern | C (`CLOCK_MONOTONIC`) | Rust (`CNTVCT_EL0`) |
|---|---:|---:|
| Linear 64 B stride, read only | 16.05 | 15.76 |

Agreement within 2 percent across two independent measurement paths. The Rust
harness carries no measurable overhead relative to naive C.

### DRAM random access floor

`null_test.c` also runs a randomized pointer chase over the full 16 MB buffer
(Sattolo single-cycle permutation, one node per cache line), which defeats the
hardware prefetcher:

| Pattern | ns per access |
|---|---:|
| Linear stride (prefetched) | 16.05 |
| Random pointer chase | **161.13** |

The 161 ns figure includes page-table walk cost: 16 MB spans 4,096 4 KB pages
against a 1,024-entry L2 TLB, so most accesses miss the TLB as well as the
cache.

### Thermal profile

Measured in a separate 60-second continuous saturation soak, not during the
batches above:

- Start: 38.0 C
- End: 41.8 C
- Delta: +3.8 C under active dual-fan cooling

---

## 5. Why Per-Iteration Timing Fails on This Counter

Modes 4 and 5 exist to test whether the memory access can be timed per
iteration rather than per batch. It cannot, and the failure mode is instructive.

Subtracting the measured timer overhead (mode 3) from each instrumented mode
gives the per-pass cost each one implies:

| Timing approach | Implied cost | Error vs. batch (50.74) |
|---|---:|---|
| `isb` only | **0.37 ns** | 137x undercount |
| `dsb sy` | **166.59 ns** | 3.3x overcount |

Neither is close, and the error changes sign depending on the barrier.

**With `isb` alone**, the volatile store retires into the write buffer and the
closing counter read fires immediately. The memory work completes outside the
measured window. The result times an instruction issue, not a memory access.

**With `dsb sy`**, the barrier stalls the pipeline until every outstanding
access completes. That closes the leak but destroys the inter-iteration overlap
that real code depends on, so the result measures a fully serialized access plus
the barrier itself.

The batch-bracketed figure of 50.74 ns is correct precisely *because*
consecutive iterations overlap in the memory pipeline, which is how the hardware
actually behaves under load.

Note also that mode 5 has the tightest spread of any mode (193.09 to 194.81, about
1 percent) while mode 1 spans 47.84 to 55.75. The most apparently "deterministic"
configuration here is the least representative one — the barrier suppresses
variance by suppressing the overlap that causes it.

**Conclusion:** sub-100 ns memory operations cannot be correctly timed per
iteration using `CNTVCT_EL0`. Either amortize across a batch, or use
`PMCCNTR_EL0`, whose read cost is low enough not to dominate.

---

## 6. Building and Running

### Prerequisites

```
rustup target add aarch64-unknown-linux-gnu
```

### Rust harness

```
cargo build --release
echo performance | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor
sudo taskset -c 3 ./target/release/velocity-deterministic-benchmark
```

### C cross-validation

```
gcc -O2 -o null_test null_test.c
sudo taskset -c 3 ./null_test
```

### Verifying the environment

```
cat /sys/devices/system/cpu/cpu3/cpufreq/scaling_cur_freq
uname -a
vcgencmd measure_temp
```

---

## 7. Continuous Integration

Format, lint, and cross-compilation checks run via GitHub Actions on every push
and pull request to `main`, covering both the AArch64 `CNTVCT_EL0` path and the
x86_64 `std::time` fallback path.

Workflow: `.github/workflows/ci.yml`

---

## 8. License

The measurement software in this repository is licensed under the Apache
License, Version 2.0. See `LICENSE` and `NOTICE`.

---

## 9. Contact

Timothy Darcelien — timdarcelien@icloud.com

Inquiries regarding replication logs, cross-compilation on other ARMv8
platforms, or the methodology in section 5 are welcome.

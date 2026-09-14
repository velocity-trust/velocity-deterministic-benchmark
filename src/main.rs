//! Velocity ARMv8 memory bus evaluation harness.
//!
//! Five modes:
//!   1. batch write+read   - two timer reads total. Honest per-pass cost.
//!   2. batch read-only    - same, loads only. Comparable to the C null test.
//!   3. timer overhead     - two counter reads per iteration, no memory work.
//!   4. instrumented isb   - legacy design: isb + mrs around each pass.
//!   5. instrumented dsb   - same, but dsb sy before the closing read so
//!                           outstanding stores must complete inside the window.
//!
//! If (5) - (3) lands near (1) while (4) sits below (1), stores escaping the
//! measured window is the explanation for the legacy undercount.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

const BUFFER_SIZE: usize = 16 * 1024 * 1024; // 16 MB, power of two
const STRIDE: usize = 64; // cache-line stride
const ITERATIONS: u64 = 1_000_000;
const BATCHES: usize = 11; // odd, for a clean median

static mut RING: [u8; BUFFER_SIZE] = [0u8; BUFFER_SIZE];

// ---------------------------------------------------------------- allocator

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

struct CountingAllocator;

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATED.fetch_add(layout.size(), Ordering::Relaxed);
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static GLOBAL: CountingAllocator = CountingAllocator;

// -------------------------------------------------------------------- timer

/// `isb` prevents the counter read being reordered against surrounding
/// instructions. It does NOT wait for outstanding stores to complete.
#[inline(always)]
fn read_counter() -> u64 {
    #[cfg(target_arch = "aarch64")]
    {
        let val: u64;
        unsafe {
            core::arch::asm!(
                "isb",
                "mrs {out}, cntvct_el0",
                out = out(reg) val,
                options(nostack)
            );
        }
        val
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
    }
}

/// `dsb sy` waits for all outstanding memory accesses to complete before the
/// counter is read, so buffered stores cannot drain outside the window.
#[inline(always)]
fn read_counter_drained() -> u64 {
    #[cfg(target_arch = "aarch64")]
    {
        let val: u64;
        unsafe {
            core::arch::asm!(
                "dsb sy",
                "isb",
                "mrs {out}, cntvct_el0",
                out = out(reg) val,
                options(nostack)
            );
        }
        val
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        read_counter()
    }
}

/// Counter frequency in Hz, read from the hardware rather than hardcoded.
fn counter_freq() -> u64 {
    #[cfg(target_arch = "aarch64")]
    {
        let val: u64;
        unsafe {
            core::arch::asm!(
                "mrs {out}, cntfrq_el0",
                out = out(reg) val,
                options(nostack, nomem)
            );
        }
        val
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        1_000_000_000 // the fallback path counts nanoseconds
    }
}

// ------------------------------------------------------------------- passes

#[inline(always)]
fn offset_for(i: u64) -> usize {
    ((i as usize).wrapping_mul(STRIDE)) & (BUFFER_SIZE - 1)
}

#[inline(always)]
fn write_read_pass(base: *mut u8, i: u64) {
    unsafe {
        let p = base.add(offset_for(i));
        std::ptr::write_volatile(p, (i & 0xFF) as u8);
        let _ = std::ptr::read_volatile(p);
    }
}

#[inline(always)]
fn read_pass(base: *const u8, i: u64) -> u8 {
    unsafe { std::ptr::read_volatile(base.add(offset_for(i))) }
}

// -------------------------------------------------------------------- modes

/// (1) Two timer reads total, write+read per pass.
fn batch_write_read(iterations: u64) -> u64 {
    let base = std::ptr::addr_of_mut!(RING) as *mut u8;
    let start = read_counter_drained();
    for i in 0..iterations {
        write_read_pass(base, i);
    }
    let end = read_counter_drained();
    end.saturating_sub(start)
}

/// (2) Two timer reads total, loads only. Matches the C null test.
fn batch_read_only(iterations: u64) -> u64 {
    let base = std::ptr::addr_of!(RING) as *const u8;
    let mut acc = 0u64;
    let start = read_counter_drained();
    for i in 0..iterations {
        acc = acc.wrapping_add(read_pass(base, i) as u64);
    }
    let end = read_counter_drained();
    std::hint::black_box(acc);
    end.saturating_sub(start)
}

/// (3) Two counter reads per iteration, no memory work.
fn timer_overhead(iterations: u64) -> u64 {
    let mut total = 0u64;
    for _ in 0..iterations {
        let s = read_counter();
        let e = read_counter();
        total = total.wrapping_add(e.saturating_sub(s));
    }
    total
}

/// (4) Legacy: isb + mrs around each pass. Stores may drain after the read.
fn instrumented_isb(iterations: u64) -> u64 {
    let base = std::ptr::addr_of_mut!(RING) as *mut u8;
    let mut total = 0u64;
    for i in 0..iterations {
        let s = read_counter();
        write_read_pass(base, i);
        let e = read_counter();
        total = total.wrapping_add(e.saturating_sub(s));
    }
    total
}

/// (5) Same, but dsb sy forces stores to complete before the closing read.
fn instrumented_dsb(iterations: u64) -> u64 {
    let base = std::ptr::addr_of_mut!(RING) as *mut u8;
    let mut total = 0u64;
    for i in 0..iterations {
        let s = read_counter();
        write_read_pass(base, i);
        let e = read_counter_drained();
        total = total.wrapping_add(e.saturating_sub(s));
    }
    total
}

// ------------------------------------------------------------------ harness

struct Stats {
    min_ns: f64,
    median_ns: f64,
    max_ns: f64,
}

fn measure<F: Fn(u64) -> u64>(f: F, iterations: u64, freq: u64) -> Stats {
    let ns_per_tick = 1e9 / freq as f64;
    let mut per_pass = [0f64; BATCHES];

    for slot in per_pass.iter_mut() {
        let ticks = f(iterations);
        *slot = ticks as f64 * ns_per_tick / iterations as f64;
    }

    per_pass.sort_by(|a, b| a.partial_cmp(b).unwrap());
    Stats {
        min_ns: per_pass[0],
        median_ns: per_pass[BATCHES / 2],
        max_ns: per_pass[BATCHES - 1],
    }
}

fn row(label: &str, s: &Stats) {
    println!(
        "{:<32} min {:8.2}   median {:8.2}   max {:8.2}",
        label, s.min_ns, s.median_ns, s.max_ns
    );
}

fn main() {
    let freq = counter_freq();

    // Fault in every page before measuring.
    {
        let base = std::ptr::addr_of_mut!(RING) as *mut u8;
        for i in 0..BUFFER_SIZE {
            unsafe { std::ptr::write_volatile(base.add(i), (i & 0xFF) as u8) };
        }
    }

    println!("Velocity memory bus evaluation harness");
    println!(
        "buffer {} MB | stride {} B | {} iterations | {} batches",
        BUFFER_SIZE / (1024 * 1024),
        STRIDE,
        ITERATIONS,
        BATCHES
    );
    println!(
        "counter frequency {:.3} MHz ({:.3} ns per tick)\n",
        freq as f64 / 1e6,
        1e9 / freq as f64
    );

    let heap_before = ALLOCATED.load(Ordering::Relaxed);

    let batch_wr = measure(batch_write_read, ITERATIONS, freq);
    let batch_rd = measure(batch_read_only, ITERATIONS, freq);
    let overhead = measure(timer_overhead, ITERATIONS, freq);
    let instr_isb = measure(instrumented_isb, ITERATIONS, freq);
    let instr_dsb = measure(instrumented_dsb, ITERATIONS, freq);

    let heap_during = ALLOCATED.load(Ordering::Relaxed) - heap_before;

    println!("all figures are nanoseconds per pass\n");
    row("1. batch write+read", &batch_wr);
    row("2. batch read-only", &batch_rd);
    row("3. timer overhead", &overhead);
    row("4. instrumented (isb only)", &instr_isb);
    row("5. instrumented (dsb sy)", &instr_dsb);

    let implied_isb = instr_isb.median_ns - overhead.median_ns;
    let implied_dsb = instr_dsb.median_ns - overhead.median_ns;

    println!("\nper-pass cost implied by each instrumented mode:");
    println!(
        "  isb only : {:8.2} ns   (batch says {:.2})",
        implied_isb, batch_wr.median_ns
    );
    println!(
        "  dsb sy   : {:8.2} ns   (batch says {:.2})",
        implied_dsb, batch_wr.median_ns
    );
    println!(
        "\nwrite+read costs {:.2} ns more than read-only ({:.2} vs {:.2})",
        batch_wr.median_ns - batch_rd.median_ns,
        batch_wr.median_ns,
        batch_rd.median_ns
    );
    println!("heap allocated during measurement: {heap_during} bytes");
}

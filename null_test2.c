/* null_test2.c - corrected DRAM floor measurement
 *
 * Fix vs v1: the chase array was 1 MB, exactly the A72's L2 size, so
 * mode B measured L2-boundary latency rather than DRAM. Here each chase
 * node lives on its own 64 B cache line inside the full 16 MB buffer,
 * so the working set is 16x L2 and genuinely reaches DRAM.
 *
 * Build:  gcc -O2 -o null_test2 null_test2.c
 * Run:    sudo taskset -c 3 ./null_test2
 */

#define _POSIX_C_SOURCE 199309L
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <time.h>

#define BUFFER_SIZE (16u * 1024u * 1024u)
#define STRIDE      64u
#define NLINES      (BUFFER_SIZE / STRIDE)   /* 262144 nodes, 16 MB total */
#define ITERATIONS  1000000u
#define RUNS        11

static uint8_t  buffer[BUFFER_SIZE];
static uint32_t perm[NLINES];

static double now_ns(void)
{
    struct timespec t;
    clock_gettime(CLOCK_MONOTONIC, &t);
    return (double)t.tv_sec * 1e9 + (double)t.tv_nsec;
}

static int cmp_double(const void *a, const void *b)
{
    double x = *(const double *)a, y = *(const double *)b;
    return (x > y) - (x < y);
}

static void build_chase(void)
{
    for (uint32_t i = 0; i < NLINES; i++)
        perm[i] = i;

    srand(12345);
    for (uint32_t i = NLINES - 1; i > 0; i--) {
        uint32_t j = (uint32_t)(((uint64_t)rand() * i) / ((uint64_t)RAND_MAX + 1));
        uint32_t t = perm[i]; perm[i] = perm[j]; perm[j] = t;
    }

    for (uint32_t k = 0; k < NLINES; k++) {
        uint32_t here = perm[k];
        uint32_t next = perm[(k + 1) % NLINES];
        *(uint32_t *)&buffer[(size_t)here * STRIDE] = next;
    }
}

static double run_linear(void)
{
    uint64_t acc = 0;
    size_t   off = 0;

    double t0 = now_ns();
    for (size_t i = 0; i < ITERATIONS; i++) {
        acc += buffer[off];
        off = (off + STRIDE) & (BUFFER_SIZE - 1);
    }
    double t1 = now_ns();

    volatile uint64_t sink = acc; (void)sink;
    return (t1 - t0) / (double)ITERATIONS;
}

static double run_chase(void)
{
    uint32_t idx = perm[0];

    double t0 = now_ns();
    for (size_t i = 0; i < ITERATIONS; i++) {
        idx = *(uint32_t *)&buffer[(size_t)idx * STRIDE];
    }
    double t1 = now_ns();

    volatile uint32_t sink = idx; (void)sink;
    return (t1 - t0) / (double)ITERATIONS;
}

static void report(const char *label, double (*fn)(void))
{
    double r[RUNS];
    for (int i = 0; i < RUNS; i++)
        r[i] = fn();

    qsort(r, RUNS, sizeof(double), cmp_double);
    printf("%-22s min %7.2f ns   median %7.2f ns   max %7.2f ns\n",
           label, r[0], r[RUNS / 2], r[RUNS - 1]);
}

int main(void)
{
    for (size_t i = 0; i < BUFFER_SIZE; i++)
        buffer[i] = (uint8_t)(i & 0xFF);

    build_chase();

    printf("=== Null Test v2 (corrected working set) ===\n");
    printf("Buffer %u MB | stride %u B | %u nodes | %u iterations\n\n",
           BUFFER_SIZE / (1024u * 1024u), STRIDE, NLINES, ITERATIONS);

    report("A linear stride", run_linear);
    report("B pointer chase 16MB", run_chase);

    printf("\nHarness reference: 55.2 ns/pass\n");
    return 0;
}

#define _POSIX_C_SOURCE 199309L
#include <stdio.h>
#include <stdlib.h>
#include <stddef.h>
#include <stdint.h>
#include <time.h>
#define BUFFER_SIZE (16u * 1024u * 1024u)
#define STRIDE      64u
#define NLINES      (BUFFER_SIZE / STRIDE)
#define ITERATIONS  1000000u
#define RUNS        11

static uint8_t  buffer[BUFFER_SIZE];
static uint32_t chase[NLINES];

static double now_ns(void) {
    struct timespec t;
    clock_gettime(CLOCK_MONOTONIC, &t);
    return (double)t.tv_sec * 1e9 + (double)t.tv_nsec;
}

static int cmp_double(const void *a, const void *b) {
    double x = *(const double *)a, y = *(const double *)b;
    return (x > y) - (x < y);
}

static void build_chase(void) {
    for (uint32_t i = 0; i < NLINES; i++) chase[i] = i;
    srand(12345);
    for (uint32_t i = NLINES - 1; i > 0; i--) {
        uint32_t j = (uint32_t)(((uint64_t)rand() * i) / ((uint64_t)RAND_MAX + 1));
        uint32_t t = chase[i]; chase[i] = chase[j]; chase[j] = t;
    }
}

static double run_linear(void) {
    uint64_t acc = 0;
    size_t   off = 0;
    double t0 = now_ns();
    for (size_t i = 0; i < ITERATIONS; i++) {
        acc += buffer[off];
        off = (off + STRIDE) & (BUFFER_SIZE - 1);
    }
    double t1 = now_ns();
    volatile uint64_t sink = acc;
    (void)sink;
    return (t1 - t0) / (double)ITERATIONS;
}

static double run_chase(void) {
    uint32_t idx = 0;
    double t0 = now_ns();
    for (size_t i = 0; i < ITERATIONS; i++) {
        idx = chase[idx];
    }
    double t1 = now_ns();
    volatile uint32_t sink = idx;
    (void)sink;
    return (t1 - t0) / (double)ITERATIONS;
}

static void report(const char *label, double (*fn)(void)) {
    double r[RUNS];
    for (int i = 0; i < RUNS; i++) r[i] = fn();
    qsort(r, RUNS, sizeof(double), cmp_double);
    printf("%-22s min %7.2f ns   median %7.2f ns   max %7.2f ns\n", label, r[0], r[RUNS / 2], r[RUNS - 1]);
}

int main(void) {
    for (size_t i = 0; i < BUFFER_SIZE; i++) buffer[i] = (uint8_t)(i & 0xFF);
    build_chase();
    printf("=== Null Test Baseline ===\n");
    report("A linear stride", run_linear);
    report("B pointer chase", run_chase);
    return 0;
}

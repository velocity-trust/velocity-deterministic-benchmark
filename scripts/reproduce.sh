#!/usr/bin/env bash
#
# Reproduction script for the Velocity ARMv8 memory bus evaluation harness.
#
# This script records the host environment, builds the C baselines and the
# Rust harness, and runs them. It prints only what the tools output. It does
# not interpret results, compare them to previously published figures, or
# report success or failure of the measurements themselves.
#
# Usage:  ./scripts/reproduce.sh
#
# Requires: rustc/cargo, gcc, taskset (util-linux). sudo is used to pin to an
# isolated core and to set the cpufreq governor; both degrade gracefully.

set -uo pipefail

CORE=3
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

rule() { printf '%s\n' "----------------------------------------------------------------------"; }

rule
echo "Velocity ARMv8 memory bus evaluation harness - reproduction run"
rule
echo "date (UTC)   : $(date -u '+%Y-%m-%d %H:%M:%S')"
echo "machine      : $(uname -m)"
echo "kernel       : $(uname -r)"
echo "repo root    : $REPO_ROOT"
echo "git commit   : $(git rev-parse --short HEAD 2>/dev/null || echo 'not a git checkout')"
echo "pinned core  : $CORE"

if command -v gcc >/dev/null 2>&1; then
  echo "gcc          : $(gcc --version | head -1)"
fi
if command -v cargo >/dev/null 2>&1; then
  echo "cargo        : $(cargo --version)"
fi

rule
echo "CPU and thermal state before run"
rule

GOV_PATH="/sys/devices/system/cpu/cpu${CORE}/cpufreq/scaling_governor"
FREQ_PATH="/sys/devices/system/cpu/cpu${CORE}/cpufreq/scaling_cur_freq"

if [ -e "$GOV_PATH" ]; then
  echo "governor (before) : $(cat "$GOV_PATH")"
  if sudo -n true 2>/dev/null || sudo -v 2>/dev/null; then
    echo performance | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor >/dev/null 2>&1 \
      && echo "governor (set)    : performance" \
      || echo "governor (set)    : failed, leaving as-is"
  else
    echo "governor (set)    : skipped, no sudo"
  fi
  echo "governor (now)    : $(cat "$GOV_PATH")"
else
  echo "governor          : cpufreq not exposed on this host"
fi

if [ -e "$FREQ_PATH" ]; then
  echo "cpu${CORE} frequency   : $(cat "$FREQ_PATH") kHz"
fi

if command -v vcgencmd >/dev/null 2>&1; then
  echo "soc temperature   : $(vcgencmd measure_temp)"
elif [ -e /sys/class/thermal/thermal_zone0/temp ]; then
  echo "soc temperature   : $(( $(cat /sys/class/thermal/thermal_zone0/temp) / 1000 )) C"
fi

# ---------------------------------------------------------------- C baselines

run_pinned() {
  if command -v taskset >/dev/null 2>&1; then
    sudo taskset -c "$CORE" "$@" 2>/dev/null || taskset -c "$CORE" "$@"
  else
    echo "(taskset unavailable - running unpinned)"
    "$@"
  fi
}

for SRC in null_test.c null_test2.c; do
  if [ -f "$SRC" ]; then
    BIN="/tmp/$(basename "$SRC" .c)"
    rule
    echo "C baseline: $SRC"
    rule
    if gcc -O2 -o "$BIN" "$SRC"; then
      run_pinned "$BIN"
    else
      echo "compilation of $SRC failed"
    fi
    echo
  fi
done

# --------------------------------------------------------------- Rust harness

rule
echo "Rust harness"
rule

if command -v cargo >/dev/null 2>&1; then
  if cargo build --release; then
    BIN_NAME="$(grep -m1 '^name' Cargo.toml | cut -d'"' -f2)"
    if [ -x "target/release/$BIN_NAME" ]; then
      run_pinned "./target/release/$BIN_NAME"
    else
      echo "release binary not found at target/release/$BIN_NAME"
    fi
  else
    echo "cargo build --release failed"
  fi
else
  echo "cargo not found in PATH"
fi

rule
echo "Run complete. Output above is the raw measurement from this host."
echo "Published figures in README.md were measured on the hardware described"
echo "in section 3; results on other hardware will differ."
rule

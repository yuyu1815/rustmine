#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
HARNESS="$ROOT/oracle/harness/java"
JAVA_BIN="${JAVA:-$ROOT/_tools/java/jdk-25-full/Contents/Home/bin/java}"
SUN_MISC_UNSAFE_MEMORY_ACCESS="${SUN_MISC_UNSAFE_MEMORY_ACCESS:-allow}"
CP_FILE="$HARNESS/build/classpath.txt"
CLASSES="$HARNESS/build/classes"

if [ "$#" -ne 1 ]; then
  echo "usage: $0 oracle/cases/775/<case>.json" >&2
  exit 2
fi

if [ ! -d "$CLASSES" ] || [ ! -f "$CP_FILE" ]; then
  "$HARNESS/scripts/compile.sh" >/dev/null
fi

"$JAVA_BIN" \
  "--sun-misc-unsafe-memory-access=$SUN_MISC_UNSAFE_MEMORY_ACCESS" \
  -cp "$CLASSES:$(cat "$CP_FILE")" \
  dev.rustmine.oracle.OracleHarness "$1"

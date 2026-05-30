#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
HARNESS="$ROOT/oracle/harness/java"
VERSION_JSON="$ROOT/_analysis/minecraft-26.1.2/26.1.2.json"
CLIENT_JAR="$ROOT/_analysis/minecraft-26.1.2/client.jar"
CACHE_DIR="$HARNESS/build/libraries"
CP_FILE="$HARNESS/build/classpath.txt"
CLASSES="$HARNESS/build/classes"

JAVAC_BIN="${JAVAC:-}"
if [ -z "$JAVAC_BIN" ] && [ -x "$ROOT/_tools/java/jdk-25-full/Contents/Home/bin/javac" ]; then
  JAVAC_BIN="$ROOT/_tools/java/jdk-25-full/Contents/Home/bin/javac"
fi
if [ -z "$JAVAC_BIN" ]; then
  JAVAC_BIN="$(command -v javac || true)"
fi

if [ -z "$JAVAC_BIN" ] || ! "$JAVAC_BIN" -version >/dev/null 2>&1; then
  echo "Missing working javac. Install/use JDK 25; the bundled _tools/java runtime is a JRE." >&2
  echo "Set JAVAC=/path/to/jdk-25/bin/javac before running this script." >&2
  exit 2
fi

python3 "$HARNESS/scripts/resolve_classpath.py" \
  --version-json "$VERSION_JSON" \
  --client-jar "$CLIENT_JAR" \
  --cache-dir "$CACHE_DIR" \
  --out "$CP_FILE" >/dev/null

mkdir -p "$CLASSES"
find "$HARNESS/src/main/java" -name '*.java' > "$HARNESS/build/sources.list"
"$JAVAC_BIN" --release 25 -cp "$(cat "$CP_FILE")" -d "$CLASSES" @"$HARNESS/build/sources.list"
echo "$CLASSES"

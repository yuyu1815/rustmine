#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PROTOCOL_VERSION="${1:-${ORACLE_PROTOCOL_VERSION:-775}}"
CASE_DIR="$ROOT/oracle/cases/$PROTOCOL_VERSION"
MANIFEST_DIR="$ROOT/oracle/test-manifests/$PROTOCOL_VERSION"
CONTRACT_DIR="$ROOT/oracle/contracts/$PROTOCOL_VERSION"

if ! command -v node >/dev/null 2>&1; then
  echo "FAIL: missing node; required to validate oracle metadata." >&2
  exit 2
fi

if [ ! -d "$CASE_DIR" ]; then
  echo "FAIL: missing oracle case directory: $CASE_DIR" >&2
  exit 2
fi
if [ ! -d "$MANIFEST_DIR" ]; then
  echo "FAIL: missing oracle test manifest directory: $MANIFEST_DIR" >&2
  exit 2
fi
if [ ! -d "$CONTRACT_DIR" ]; then
  echo "FAIL: missing oracle contract directory: $CONTRACT_DIR" >&2
  exit 2
fi

validate_artifact_graph() {
  local phase="$1"
  node - "$ROOT" "$PROTOCOL_VERSION" "$phase" <<'NODE'
const fs = require("fs");
const path = require("path");

const root = process.argv[2];
const version = process.argv[3];
const phase = process.argv[4];
const failures = [];

function repoPath(relPath) {
  return path.join(root, ...relPath.split("/"));
}

function fail(message) {
  failures.push(message);
}

function listJson(relDir) {
  const abs = repoPath(relDir);
  if (!fs.existsSync(abs)) {
    fail(`missing directory: ${relDir}`);
    return [];
  }
  return fs
    .readdirSync(abs)
    .filter((entry) => entry.endsWith(".json"))
    .sort()
    .map((entry) => `${relDir}/${entry}`);
}

function readJson(relPath, kind) {
  const abs = repoPath(relPath);
  let text;
  try {
    text = fs.readFileSync(abs, "utf8");
  } catch (error) {
    fail(`missing ${kind}: ${relPath}: ${error.message}`);
    return null;
  }
  try {
    const parsed = JSON.parse(text);
    if (!parsed || typeof parsed !== "object" || Array.isArray(parsed)) {
      fail(`${kind} is not a JSON object: ${relPath}`);
      return null;
    }
    return parsed;
  } catch (error) {
    fail(`malformed ${kind}: ${relPath}: ${error.message}`);
    return null;
  }
}

function requireString(ownerRel, owner, key) {
  const value = owner ? owner[key] : undefined;
  if (typeof value !== "string" || value.length === 0) {
    fail(`${ownerRel} missing non-empty string ${key}`);
    return "";
  }
  return value;
}

function requireRepoRel(ownerRel, owner, key, prefix) {
  const value = requireString(ownerRel, owner, key);
  if (!value) {
    return value;
  }
  if (path.posix.isAbsolute(value) || value.includes("\\") || value.split("/").includes("..")) {
    fail(`${ownerRel} ${key} must be a repository-relative POSIX path: ${value}`);
  }
  if (prefix && !value.startsWith(prefix)) {
    fail(`${ownerRel} ${key} must stay under ${prefix}: ${value}`);
  }
  return value;
}

function putUnique(map, key, value, label) {
  if (!key) {
    return;
  }
  if (map.has(key)) {
    fail(`duplicate ${label}: ${key}`);
    return;
  }
  map.set(key, value);
}

console.log(`SCOPE: ${phase} oracle artifact graph validation for protocol ${version}`);

const caseRels = listJson(`oracle/cases/${version}`);
const manifestRels = listJson(`oracle/test-manifests/${version}`);
const contractRels = listJson(`oracle/contracts/${version}`);
const casesById = new Map();
const manifestsByCaseId = new Map();
const contractsByRel = new Map();
const contractsByCaseId = new Map();
const contractIds = new Map();
const referencedContractRels = new Set();
const answerOwners = new Map();
const rustTestsByCaseId = new Map();

for (const rel of caseRels) {
  const artifact = readJson(rel, "oracle case");
  if (!artifact) {
    continue;
  }
  const caseId = requireString(rel, artifact, "case_id");
  if (!Number.isInteger(artifact.protocol_version) || String(artifact.protocol_version) !== version) {
    fail(`${rel} protocol_version must match ${version}`);
  }
  const answerPath = requireRepoRel(
    rel,
    artifact,
    "answer_path",
    `oracle/answers/${version}/`
  );
  putUnique(casesById, caseId, { rel, artifact, answerPath }, "case_id");
  if (answerPath) {
    if (answerOwners.has(answerPath)) {
      fail(`${rel} answer_path duplicates ${answerOwners.get(answerPath)}: ${answerPath}`);
    } else {
      answerOwners.set(answerPath, rel);
    }
  }
}

for (const rel of contractRels) {
  const artifact = readJson(rel, "oracle contract");
  if (!artifact) {
    continue;
  }
  const contractId = requireString(rel, artifact, "contract_id");
  const caseId = requireString(rel, artifact, "case_id");
  const answerPath = requireRepoRel(
    rel,
    artifact,
    "answer_path",
    `oracle/answers/${version}/`
  );
  putUnique(contractIds, contractId, rel, "contract_id");
  contractsByRel.set(rel, { rel, artifact, contractId, caseId, answerPath });
  putUnique(contractsByCaseId, caseId, { rel, artifact, contractId, caseId, answerPath }, "contract case_id");
}

for (const rel of manifestRels) {
  const artifact = readJson(rel, "oracle test manifest");
  if (!artifact) {
    continue;
  }
  const caseId = requireString(rel, artifact, "case_id");
  const answerPath = requireRepoRel(
    rel,
    artifact,
    "answer_path",
    `oracle/answers/${version}/`
  );
  const contractPath = requireRepoRel(
    rel,
    artifact,
    "contract_path",
    `oracle/contracts/${version}/`
  );
  const rustTestTarget = requireRepoRel(
    rel,
    artifact,
    "rust_test_target",
    "oracle/rust-tests/"
  );
  const rustTestName = requireString(rel, artifact, "rust_test_name");
  putUnique(manifestsByCaseId, caseId, { rel, artifact, answerPath, contractPath }, "manifest case_id");
  putUnique(rustTestsByCaseId, caseId, { rustTestTarget, rustTestName }, "manifest Rust test case_id");
}

if (caseRels.length === 0) {
  fail(`no oracle cases found in oracle/cases/${version}`);
}

for (const [caseId, manifest] of manifestsByCaseId) {
  const oracleCase = casesById.get(caseId);
  if (!oracleCase) {
    fail(`${manifest.rel} names missing case_id ${caseId}`);
    continue;
  }
  if (manifest.answerPath !== oracleCase.answerPath) {
    fail(`${manifest.rel} answer_path ${manifest.answerPath} does not match ${oracleCase.rel} ${oracleCase.answerPath}`);
  }

  const contract = contractsByRel.get(manifest.contractPath);
  if (!contract) {
    fail(`${manifest.rel} contract_path is missing or not loaded: ${manifest.contractPath}`);
  } else {
    referencedContractRels.add(manifest.contractPath);
    if (contract.caseId !== caseId) {
      fail(`${manifest.rel} case_id ${caseId} does not match ${manifest.contractPath} case_id ${contract.caseId}`);
    }
    if (contract.answerPath !== manifest.answerPath) {
      fail(`${manifest.contractPath} answer_path ${contract.answerPath} does not match ${manifest.rel} ${manifest.answerPath}`);
    }
  }

  const rustTest = rustTestsByCaseId.get(caseId);
  if (rustTest && !fs.existsSync(repoPath(rustTest.rustTestTarget))) {
    fail(`${manifest.rel} rust_test_target file is missing: ${rustTest.rustTestTarget}`);
  }
}

for (const [caseId, oracleCase] of casesById) {
  if (!manifestsByCaseId.has(caseId)) {
    fail(`${oracleCase.rel} has no matching test manifest for case_id ${caseId}`);
  }
  const contract = contractsByCaseId.get(caseId);
  if (contract && contract.answerPath !== oracleCase.answerPath) {
    fail(`${contract.rel} answer_path ${contract.answerPath} does not match ${oracleCase.rel} ${oracleCase.answerPath}`);
  }
}

for (const [rel, contract] of contractsByRel) {
  if (!casesById.has(contract.caseId)) {
    fail(`${rel} names missing case_id ${contract.caseId}`);
  }
  if (!referencedContractRels.has(rel)) {
    fail(`${rel} is not referenced by a test manifest contract_path`);
  }
}

if (failures.length > 0) {
  for (const message of failures) {
    console.error(`FAIL: ${message}`);
  }
  process.exit(2);
}

console.log(
  `PASS: artifact links validated cases=${casesById.size} manifests=${manifestsByCaseId.size} contracts=${contractsByRel.size}`
);
NODE
}

list_cases() {
  node - "$ROOT" "$PROTOCOL_VERSION" <<'NODE'
const fs = require("fs");
const path = require("path");

const root = process.argv[2];
const version = process.argv[3];
const relDir = `oracle/cases/${version}`;
const absDir = path.join(root, ...relDir.split("/"));

for (const entry of fs.readdirSync(absDir).filter((name) => name.endsWith(".json")).sort()) {
  const rel = `${relDir}/${entry}`;
  const artifact = JSON.parse(fs.readFileSync(path.join(absDir, entry), "utf8"));
  process.stdout.write(
    `${JSON.stringify({
      case_id: artifact.case_id,
      case_path: rel,
      answer_path: artifact.answer_path,
    })}\n`
  );
}
NODE
}

list_manifests() {
  node - "$ROOT" "$PROTOCOL_VERSION" <<'NODE'
const fs = require("fs");
const path = require("path");

const root = process.argv[2];
const version = process.argv[3];
const relDir = `oracle/test-manifests/${version}`;
const absDir = path.join(root, ...relDir.split("/"));

for (const entry of fs.readdirSync(absDir).filter((name) => name.endsWith(".json")).sort()) {
  const rel = `${relDir}/${entry}`;
  const artifact = JSON.parse(fs.readFileSync(path.join(absDir, entry), "utf8"));
  process.stdout.write(
    `${JSON.stringify({
      manifest_path: rel,
      case_id: artifact.case_id,
      rust_test_target: artifact.rust_test_target,
      rust_test_name: artifact.rust_test_name,
    })}\n`
  );
}
NODE
}

json_field() {
  local row="$1"
  local field="$2"
  node -e 'const row = JSON.parse(process.argv[1]); console.log(row[process.argv[2]]);' "$row" "$field"
}

rust_test_binary_for_target() {
  local rust_test_target="$1"

  case "$rust_test_target" in
    oracle/rust-tests/tests/*.rs)
      basename "${rust_test_target%.rs}"
      ;;
    *)
      echo "FAIL: unsupported Rust oracle test target for exact execution: $rust_test_target" >&2
      exit 2
      ;;
  esac
}

run_rust_manifest_test() {
  local manifest_path="$1"
  local case_id="$2"
  local rust_test_target="$3"
  local rust_test_name="$4"
  local rust_test_binary
  local cargo_target_dir
  local output_file

  rust_test_binary="$(rust_test_binary_for_target "$rust_test_target")"
  cargo_target_dir="${ORACLE_CARGO_TARGET_DIR:-$BACKUP_DIR/cargo-target}"
  output_file="$(mktemp "$BACKUP_DIR/cargo-test.XXXXXX")"

  echo "SCOPE: Rust oracle test $rust_test_name for manifest $manifest_path"
  if ! (
    cd "$ROOT"
    CARGO_TARGET_DIR="$cargo_target_dir" \
      ORACLE_EXPECTED_MANIFEST="$manifest_path" \
      ORACLE_EXPECTED_CASE_ID="$case_id" \
      ORACLE_EXPECTED_RUST_TEST_TARGET="$rust_test_target" \
      ORACLE_EXPECTED_RUST_TEST_NAME="$rust_test_name" \
      cargo test --manifest-path oracle/rust-tests/Cargo.toml --test "$rust_test_binary" "$rust_test_name" -- --exact
  ) >"$output_file" 2>&1; then
    cat "$output_file" >&2
    echo "FAIL: exact Rust oracle test failed $rust_test_name for manifest $manifest_path" >&2
    exit 2
  fi

  if ! node - "$output_file" "$rust_test_name" "$manifest_path" <<'NODE'
const fs = require("fs");

const outputPath = process.argv[2];
const testName = process.argv[3];
const manifestPath = process.argv[4];

function escapeRegExp(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

const output = fs
  .readFileSync(outputPath, "utf8")
  .replace(/\x1b\[[0-9;]*m/g, "");
const testLine = new RegExp(`^test ${escapeRegExp(testName)} \\.\\.\\. ok$`, "m");

if (!testLine.test(output)) {
  console.error(
    `FAIL: exact Rust oracle test was filtered out or not executed: ${testName} for manifest ${manifestPath}`
  );
  process.exit(2);
}
NODE
  then
    cat "$output_file" >&2
    exit 2
  fi

  echo "PASS: exact Rust oracle test executed $rust_test_name for manifest $manifest_path"
}

validate_answer() {
  local case_id="$1"
  local answer_path="$2"
  node - "$ROOT" "$case_id" "$answer_path" <<'NODE'
const fs = require("fs");
const path = require("path");

const root = process.argv[2];
const caseId = process.argv[3];
const answerPath = process.argv[4];
const fullPath = path.join(root, ...answerPath.split("/"));
const failures = [];

function fail(message) {
  failures.push(message);
}

if (!fs.existsSync(fullPath)) {
  fail(`answer file missing: ${answerPath}`);
} else if (!fs.statSync(fullPath).isFile()) {
  fail(`answer path is not a regular file: ${answerPath}`);
} else {
  const text = fs.readFileSync(fullPath, "utf8");
  if (text.length === 0) {
    fail(`answer file is empty: ${answerPath}`);
  } else {
    const rows = [];
    const lines = text.split(/\r?\n/);
    for (let index = 0; index < lines.length; index += 1) {
      const line = lines[index];
      if (line.trim().length === 0) {
        continue;
      }
      try {
        const parsed = JSON.parse(line);
        if (!parsed || typeof parsed !== "object" || Array.isArray(parsed)) {
          fail(`JSONL row ${index + 1} is not an object in ${answerPath}`);
        } else {
          rows.push({ index: index + 1, parsed });
        }
      } catch (error) {
        fail(`malformed JSONL row ${index + 1} in ${answerPath}: ${error.message}`);
      }
    }

    if (rows.length !== 1) {
      fail(`expected exactly one non-empty JSONL row for ${caseId} in ${answerPath}; found ${rows.length}`);
    }
    for (const row of rows) {
      if (row.parsed.case_id !== caseId) {
        fail(`row ${row.index} case_id ${row.parsed.case_id} does not match ${caseId}`);
      }
      if (!row.parsed.generated_by || typeof row.parsed.generated_by !== "object") {
        fail(`row ${row.index} missing generated_by object`);
      }
      if (!row.parsed.official_source || typeof row.parsed.official_source !== "object") {
        fail(`row ${row.index} missing official_source object`);
      }
      if (!row.parsed.answer || typeof row.parsed.answer !== "object") {
        fail(`row ${row.index} missing answer object`);
      }
    }
  }
}

if (failures.length > 0) {
  for (const message of failures) {
    console.error(`FAIL: ${message}`);
  }
  process.exit(2);
}

console.log(`PASS: answer regenerated ${caseId} -> ${answerPath}`);
NODE
}

BACKUP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/rustmine-oracle-answers.XXXXXX")"
declare -a ANSWER_BACKUPS=()

restore_answers_on_failure() {
  local exit_status=$?
  if [ "$exit_status" -ne 0 ]; then
    for spec in "${ANSWER_BACKUPS[@]}"; do
      IFS=$'\t' read -r answer_path backup_path existed <<<"$spec"
      local answer_full="$ROOT/$answer_path"
      if [ "$existed" = "yes" ]; then
        mkdir -p "$(dirname "$answer_full")"
        cp "$backup_path" "$answer_full"
      else
        rm -f "$answer_full"
      fi
    done
  fi
  rm -rf "$BACKUP_DIR"
  return "$exit_status"
}
trap restore_answers_on_failure EXIT

backup_and_clear_answer() {
  local case_id="$1"
  local answer_path="$2"
  local answer_full="$ROOT/$answer_path"
  local backup_path="$BACKUP_DIR/$case_id.answer.jsonl"

  mkdir -p "$(dirname "$answer_full")"
  if [ -e "$answer_full" ]; then
    if [ ! -f "$answer_full" ]; then
      echo "FAIL: answer path is not a regular file: $answer_path" >&2
      exit 2
    fi
    cp "$answer_full" "$backup_path"
    ANSWER_BACKUPS+=("${answer_path}"$'\t'"${backup_path}"$'\t'"yes")
  else
    ANSWER_BACKUPS+=("${answer_path}"$'\t'"${backup_path}"$'\t'"no")
  fi
  rm -f "$answer_full"
}

validate_artifact_graph "pre-generation"

declare -a CASE_IDS=()
declare -a CASE_PATHS=()
declare -a ANSWER_PATHS=()

while IFS= read -r case_row; do
  case_id="$(json_field "$case_row" case_id)"
  case_path="$(json_field "$case_row" case_path)"
  answer_path="$(json_field "$case_row" answer_path)"

  backup_and_clear_answer "$case_id" "$answer_path"
  CASE_IDS+=("$case_id")
  CASE_PATHS+=("$case_path")
  ANSWER_PATHS+=("$answer_path")
done < <(list_cases)

echo "SCOPE: generating ${#CASE_PATHS[@]} oracle answers with one Java harness process"
"$ROOT/oracle/harness/java/scripts/run_case.sh" "${CASE_PATHS[@]}"
echo
for index in "${!CASE_PATHS[@]}"; do
  validate_answer "${CASE_IDS[$index]}" "${ANSWER_PATHS[$index]}"
done

validate_artifact_graph "post-generation"

while IFS= read -r manifest_row; do
  manifest_path="$(json_field "$manifest_row" manifest_path)"
  case_id="$(json_field "$manifest_row" case_id)"
  rust_test_target="$(json_field "$manifest_row" rust_test_target)"
  rust_test_name="$(json_field "$manifest_row" rust_test_name)"

  run_rust_manifest_test "$manifest_path" "$case_id" "$rust_test_target" "$rust_test_name"
done < <(list_manifests)

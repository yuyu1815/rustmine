---
name: stevenarella-oracle-workbench
description: Build versioned oracle cases from official Minecraft client/server jars, generate answer artifacts, and write oracle tests that compare Stevenarella against official behavior. Use when reading jars, creating protocol contracts, producing why/what/answer failures, or adding compatibility tests. Protocol 775 is the current populated example.
---

# Stevenarella Oracle Workbench

## Owner

```text
official jar/decompiled source
  -> active version
  -> oracle case
  -> contract artifact
  -> official answer artifact
  -> test manifest
  -> oracle test
  -> traceability row
  -> why/what/answer failure
```

This skill may write `oracle/**`, `.codex/skills/stevenarella-oracle-workbench/schemas/**`, and oracle tests. It must
not edit Stevenarella Rust implementation.

For client-load claims, read `.codex/skills/client-load-compatibility/SKILL.md`
first. Protocol packet work for the active version is one load surface, not the
whole client-load definition.

## Workflow

1. Choose the active version and one bounded case; default to the current populated protocol manifest only when the task does not name a version.
2. Read `oracle/versions/<version>.toml`.
3. Read official jar/decompiled source first.
4. Use reference repositories only as witnesses.
5. Write or update one `oracle/cases/<protocol_version>/*.json` case.
6. Write or update one `oracle/contracts/<protocol_version>/*.contract.json` contract.
7. Generate one `oracle/answers/<protocol_version>/*.answer.jsonl` artifact from an official function.
8. Write or update one `oracle/test-manifests/<protocol_version>/*.test-manifest.json` manifest.
9. Write or update a Rust oracle test that reads the answer artifact and manifest.
10. Update `docs/analysis/protocol/versions/<protocol_version>/traceability.md` and the relevant `docs/analysis/protocol/versions/<protocol_version>/cases/*.md` note.
11. Run the helper/oracle test and read each `PASS:` line as scoped evidence for the named case, artifact link, answer regeneration, structural cross-reference check, or exact Rust test execution only.
12. If the case/test is wrong, fix only oracle-owned files.
13. If Stevenarella is wrong, emit a `why/what/answer` failure and a `rust-fix-task` packet.

## Source Policy

Read `references/source-policy.md` before touching any jar, source, or reference
repository.

## Failure Format

Read `references/failure-format.md` before reporting a failed oracle test.

## Test Generation

Read `references/test-generation.md` before creating a new jar-backed oracle
test, extractor, test manifest, or Rust oracle test surface.

## Direct Official Calls

Use the lightweight Java harness first:

```text
oracle/harness/java/scripts/compile.sh
oracle/harness/java/scripts/run_case.sh oracle/cases/<protocol_version>/<case>.json
```

To verify every existing jar-backed case plus the Rust oracle contract surface,
run:

```text
bash oracle/scripts/run_jar_backed_oracle_tests.sh
```

The runner is a scoped integrity check, not a broad compatibility verdict. It
must perform structural case/contract/manifest cross-reference validation,
remove the target answer before jar generation, validate the regenerated JSONL
answer with bespoke runner checks, and execute each manifest-declared Rust
oracle test by exact name before Cargo output is treated as evidence.

For the current populated packet-table example, call:

```text
ConfigurationProtocols.SERVERBOUND.details().listPackets(...)
<Packet>.STREAM_CODEC.encode(...)
<Packet>.STREAM_CODEC.decode(...)
```

For registry, gameplay, or initialized client/server behavior, create an
explicit initialized-harness follow-up such as Fabric Loader JUnit or GameTest.
Do not fake initialized Minecraft state.

## Stop

Stop if the expected value cannot be produced by an official function or a
generated answer artifact. Do not hand-write expected packet bytes.

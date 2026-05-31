---
name: stevenarella-oracle-case-builder
description: Build or update one bounded Stevenarella oracle case package from official Minecraft jars, including case, contract, generated answer, test manifest, Rust oracle test, traceability row, and why/what/answer failure when Stevenarella differs.
---

# Stevenarella Oracle Case Builder

Use this only after the active task names a bounded protocol version and oracle
case target, or after `stevenarella-oracle-workbench` routes here.

## Owner

```text
active version
  -> one bounded case
    -> official jar function
      -> oracle case
        -> contract artifact
          -> official answer artifact
            -> test manifest
              -> Rust oracle test
                -> traceability row
                  -> why/what/answer failure if Stevenarella differs
```

This skill may write `oracle/**`, project-level oracle tests, protocol
traceability notes, case notes, and oracle schemas when explicitly needed. It
must not edit Stevenarella Rust implementation.

## Read Only When Needed

| Need | Read |
|---|---|
| Any jar, source, or reference repository access | `.codex/skills/stevenarella-oracle-workbench/references/source-policy.md` |
| New or changed Rust oracle test, answer factory, manifest, or traceability row | `.codex/skills/stevenarella-oracle-workbench/references/test-generation.md` |
| Reporting a Stevenarella mismatch | `.codex/skills/stevenarella-oracle-workbench/references/failure-format.md` |
| Schema validation or packet shape | the exact schema under `.codex/skills/stevenarella-oracle-workbench/schemas/` named by the task |

## Workflow

1. Choose the active version and one bounded case; default to the current
   populated protocol manifest only when the task does not name a version.
2. Read `oracle/versions/<version>.toml`.
3. Read official jar/decompiled source first.
4. Use reference repositories only as witnesses.
5. Write or update one `oracle/cases/<protocol_version>/*.json` case.
6. Write or update one `oracle/contracts/<protocol_version>/*.contract.json`
   contract.
7. Generate one `oracle/answers/<protocol_version>/*.answer.jsonl` artifact from
   an official function.
8. Write or update one
   `oracle/test-manifests/<protocol_version>/*.test-manifest.json` manifest.
9. Write or update a Rust oracle test that reads the answer artifact and
   manifest.
10. Update `docs/analysis/protocol/versions/<protocol_version>/traceability.md`
    and the relevant `docs/analysis/protocol/versions/<protocol_version>/cases/*.md`
    note.
11. Run the helper/oracle test and read each `PASS:` line as scoped evidence for
    the named case, artifact link, answer regeneration, structural
    cross-reference check, or exact Rust test execution only.
12. If the case/test is wrong, fix only oracle-owned files.
13. If Stevenarella is wrong, emit a `why/what/answer` failure and a
    `rust-fix-task` packet.

## Commands

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

The runner is a scoped integrity check, not a broad compatibility verdict.

## Stop

Stop if the expected value cannot be produced by an official function or a
generated answer artifact. Do not hand-write expected packet bytes.

For registry, gameplay, or initialized client/server behavior, create an
explicit initialized-harness follow-up such as Fabric Loader JUnit or GameTest.
Do not fake initialized Minecraft state.

## Output Shape

```text
scope:
files_changed:
official_evidence:
map:
test_or_answer:
failure_packet:
next_or_blocker:
```

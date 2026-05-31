# Oracle Factory

Purpose: define ownership for jar-backed answers, manifests, and reset-proof
compatibility tests.

## No-Flow Boundary

This file records ownership for oracle surfaces. It must not become the
case-building manual. Oracle instructions live in the relevant skill or helper
script.

| Instruction type | Owner |
|---|---|
| Oracle routing and source-policy selection | `.codex/skills/stevenarella-oracle-workbench/SKILL.md` |
| Bounded case package instructions | `.codex/skills/stevenarella-oracle-case-builder/SKILL.md` |
| Case/contract/answer/manifest/test validation commands | oracle helper scripts and the case-builder skill |

## Responsibilities

| Surface | Owner/scope | Stop boundary |
|---|---|---|
| `.codex/skills/stevenarella-oracle-workbench/SKILL.md` | lightweight router for oracle work, source policy, failure format, model lanes, and schemas | does not perform case generation by itself |
| `.codex/skills/stevenarella-oracle-case-builder/SKILL.md` | bounded case package instructions for case/contract/answer/manifest/Rust oracle test/traceability/failure packet | does not edit Rust implementation |
| `oracle/cases/`, `oracle/contracts/`, `oracle/answers/` | machine-readable oracle inputs and generated official answers | expected values are not hand-written |
| `oracle/test-manifests/` | mapping from answer artifacts to project-level Rust test surfaces | does not store generated answers |
| `oracle/rust-tests/` | reset-proof project-level oracle/compatibility contract tests | does not mirror `stevenarella/src/` tests |
| `oracle/scripts/run_jar_backed_oracle_tests.sh` | scoped helper validation for case/contract/answer/manifest/test linkage and regenerated jar answers | does not prove broad compatibility beyond named cases |
| `docs/analysis/protocol/versions/*/` | human-readable traceability and case notes | does not replace machine-readable oracle artifacts |

## Proof Location

For the current proven slices and their proof status, read
`docs/analysis/current-evidence/` and the relevant
`docs/analysis/protocol/versions/*/traceability.md` shard. Keep duplicated
current-state summaries out of this ownership file.

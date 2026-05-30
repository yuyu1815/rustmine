# Oracle Factory

Purpose: define ownership for jar-backed answers, manifests, and reset-proof
compatibility tests.

## Map

```text
official jar/source
  -> oracle case
    -> contract
      -> generated answer
        -> test manifest
          -> helper cross-reference validation
            -> project-level Rust oracle test
              -> traceability row
```

## Responsibilities

| Surface | Owner/scope | Stop boundary |
|---|---|---|
| `.codex/skills/stevenarella-oracle-workbench/` | fixed oracle workflow, source policy, schemas | does not edit Rust implementation |
| `oracle/cases/`, `oracle/contracts/`, `oracle/answers/` | machine-readable oracle inputs and generated official answers | expected values are not hand-written |
| `oracle/test-manifests/` | mapping from answer artifacts to project-level Rust test surfaces | does not store generated answers |
| `oracle/rust-tests/` | reset-proof project-level oracle/compatibility contract tests | does not mirror `stevenarella/src/` tests |
| `oracle/scripts/run_jar_backed_oracle_tests.sh` | scoped helper validation for case/contract/answer/manifest/test linkage and regenerated jar answers | does not prove broad compatibility beyond named cases |
| `docs/analysis/protocol/versions/*/` | human-readable traceability and case notes | does not replace machine-readable oracle artifacts |

## Current Proof Route

For the current proven slices and their proof status, read
`docs/analysis/current-evidence/` and the relevant
`docs/analysis/protocol/versions/*/traceability.md` shard. Keep duplicated
current-state summaries out of this ownership file.

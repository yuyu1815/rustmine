---
name: stevenarella-oracle-workbench
description: Route Stevenarella oracle work to the smallest needed oracle skill or reference. Use when reading official jars, creating oracle cases/contracts/answers/tests, producing why/what/answer failures, or deciding whether packet/state evidence belongs to oracle work.
---

# Stevenarella Oracle Workbench Router

This is the lightweight entry point for oracle work. It chooses the smallest
oracle route and avoids loading the heavy generation workflow unless the active
task needs it.

## Owner

```text
official jar/decompiled source
  -> bounded oracle route
    -> case builder / source policy / failure format / model lane
```

This skill may route work under `oracle/**`, oracle tests,
`.codex/skills/stevenarella-oracle-workbench/schemas/**`, and protocol
traceability. It must not edit Stevenarella Rust implementation.

For client-load claims, read `.codex/skills/client-load-compatibility/SKILL.md`
only when the task is about loading, playable readiness, registry/world/render
readiness, or phase selection. Protocol packet work is one load surface, not the
whole client-load definition.

## Route

| Need | Read |
|---|---|
| Create or update a jar-backed case, contract, answer, manifest, Rust oracle test, traceability row, or failure packet | `.codex/skills/stevenarella-oracle-case-builder/SKILL.md` |
| Decide official jar vs reference witness policy | `.codex/skills/stevenarella-oracle-workbench/references/source-policy.md` |
| Report an oracle-vs-Rust mismatch | `.codex/skills/stevenarella-oracle-workbench/references/failure-format.md` |
| Choose model lane or worker capacity | `.codex/skills/stevenarella-oracle-workbench/references/model-lanes.toml` |
| Validate schema shape | `.codex/skills/stevenarella-oracle-workbench/schemas/*.schema.json` named by the task |

## Hard Rules

| Rule | Meaning |
|---|---|
| Official answer first | Expected values come from official jars/functions or generated oracle answer artifacts. |
| No Rust implementation edits | Oracle work may emit a failure packet for Rust, but does not patch Stevenarella. |
| References are witnesses | Reference repositories may explain or cross-check; official jars win. |
| One bounded case | Do not broaden a case into general protocol repair. |
| Mutable proof state lives lower | Put evidence and proof status in `docs/analysis/` or `oracle/`, not in this router. |

## Output Shape

```text
scope:
route:
evidence_needed:
stop_boundary:
next_or_blocker:
```

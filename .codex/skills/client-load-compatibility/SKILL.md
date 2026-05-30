---
name: client-load-compatibility
description: Map Stevenarella client-load compatibility into load phases, oracle surfaces, reset-proof tests, and done conditions beyond active protocol packet bytes. Use when the user mentions client loading, playable readiness, official-jar equivalence beyond packet codecs, phase maps, registry/world/entity/render readiness, or choosing the right oracle/test surface.
---

# Client Load Compatibility

## Owner

```text
client load concern
  -> current evidence shard
  -> structural scan
  -> default earliest unproven load phase or explicit justified target
  -> official evidence surface
  -> oracle artifact or smoke proof
  -> reset-proof project test/probe
  -> client-load shard row
```

This skill owns the map from "the client loads" to testable compatibility
surfaces. It does not replace the versioned Oracle Workbench; it decides
when protocol oracle work is only one slice of a larger load concern.

## Workflow

1. Name the load concern in user terms.
2. Read `docs/analysis/current-evidence/client-load.md`.
3. Run or request `bash oracle/scripts/scan_current_load_surfaces.sh` after any reset.
4. Read `docs/analysis/current-evidence/structural-scan.md`.
5. Read `docs/analysis/client-load/README.md` and the relevant `docs/analysis/client-load/phases/*.md` detail.
6. By default, pick the earliest load phase whose proof is missing or failing.
7. If the task packet names a later phase or cross-phase target and supplies evidence or a stop boundary, keep that explicit target instead of re-routing it to the earliest phase.
8. Use `references/load-phase-lens.md` to choose boundaries and done condition.
9. Use `references/oracle-surfaces.md` to choose the evidence/test surface.
10. If the surface is packet/state/registry protocol evidence, route to `.codex/skills/stevenarella-oracle-workbench/SKILL.md`.
11. If the surface is Rust implementation, require an oracle artifact, smoke proof, or explicit allowed write scope before changing `stevenarella/`.
12. Put persistent tests and evidence outside reset-prone `stevenarella/`.
13. Update `docs/analysis/current-evidence/client-load.md`, `docs/analysis/client-load/README.md`, and the relevant phase detail when proof state changes.

## Phase Map

```text
local boot/resources
  -> network login/configuration
    -> registry hydration
      -> play entry
        -> world hydration
          -> entity/player hydration
            -> render ready
              -> control/interact ready
```

## Hard Rules

| Rule | Meaning |
|---|---|
| Protocol is not the whole load | Active protocol-version work is one evidence surface, not the product goal. |
| Same root, different tests | Official equivalence needs several tests tied to one evidence chain, not one universal test. |
| Reset-proof proof | Canonical tests, manifests, and phase evidence shards live at project root. |
| No guessed readiness | A load phase is done only with named evidence and a done condition. |
| Observed is not done | File/path/module existence is structural observation only, never compatibility proof. |
| Mutable facts live in shards | Keep current proof state in `docs/analysis/client-load/` and `docs/analysis/current-evidence/`, not in this fixed workflow. |

## Output Shape

```text
scope:
evidence_read:
load_phase_or_target:
official_evidence:
test_surface:
done_condition:
shard_update:
next_or_blocker:
```

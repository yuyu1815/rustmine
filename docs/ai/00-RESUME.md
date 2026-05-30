# AI Resume Card

Purpose: restore the current location and next action after compaction, fresh
session, or handoff. This file is a recovery pointer only.

## Current Location

| Field | Value |
|---|---|
| Current location | Protocol 775 `configuration_keepalive_clientbound_framed_dispatch` oracle package and Rust fix are in place; official jar answer generation succeeds and all four current Rust oracle tests pass |
| Last touched area | `_analysis/minecraft-26.1.2/`, `_tools/java/jdk-25-full`, `oracle/cases/775/`, `oracle/contracts/775/`, `oracle/answers/775/`, `oracle/test-manifests/775/`, `oracle/failures/775/`, `oracle/harness/java/`, `oracle/rust-tests/`, `docs/analysis/protocol/versions/775/`, `docs/ai/00-RESUME.md` |
| Next read entry | `docs/ai/README.md`, `CONTEXT.md` for project terms, then `docs/analysis/responsibility/README.md` and the shard named by the active task |
| Explicit uncertainty | The new proof is packet/framing only. It does not prove runtime keep-alive echo behavior, runtime Configuration-to-Play transition, Play readiness, registry hydration, world hydration, or client load completion. Decompiled source paths referenced by the cases are not restored in this checkout, but the official jar-backed answers are generated from the client jar. |

## Recovery Flow

```text
Read AGENTS.md
  -> read docs/ai/README.md
  -> read CONTEXT.md when project vocabulary is unclear or being sharpened
  -> read docs/analysis/README.md for domain routing
  -> read only the active shard, workflow, agent role, or task artifact
  -> update this card only for current location, next action, or recovery route
```

## Next Action

```text
For future work:
  start from docs/ai/README.md
    -> choose the owning responsibility shard for the active task
      -> keep parent Codex responsible for user answers, route decisions,
         final summaries, and recovery pointer updates
        -> use subagents only for bounded evidence, mapping, review, or
           implementation work packages
          -> for client-load/playability, continue from
             network_login_configuration
            -> next likely target: design the runtime keep-alive echo harness
               now that serverbound and clientbound Configuration keep_alive
               framed dispatch are proven as packet-level prerequisites
```

## Stop Boundary

Do not use this card as a durable evidence ledger or decision record.

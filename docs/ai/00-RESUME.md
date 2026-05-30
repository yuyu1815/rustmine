# AI Resume Card

Purpose: restore the current location and next action after compaction, fresh
session, or handoff. This file is a recovery pointer only.

## Current Location

| Field | Value |
|---|---|
| Current location | Protocol 775 `configuration_custom_payload_framed_dispatch` oracle package and Rust fix are in place; direct jar-backed runner passes and all current Rust oracle tests pass |
| Last touched area | `_analysis/minecraft-26.1.2/`, `_tools/java/jdk-25-full`, `oracle/cases/775/`, `oracle/contracts/775/`, `oracle/answers/775/`, `oracle/test-manifests/775/`, `oracle/failures/775/`, `oracle/harness/java/`, `oracle/rust-tests/`, `docs/analysis/protocol/versions/775/`, `docs/analysis/client-load/`, `docs/analysis/current-evidence/client-load.md`, `docs/ai/00-RESUME.md` |
| Next read entry | `docs/ai/README.md`, `CONTEXT.md` for project terms, then `docs/analysis/responsibility/README.md` and the shard named by the active task |
| Explicit uncertainty | The new custom_payload proof is packet framing and dispatch/decode for one BrandPayload fixture only. The current public Packet compatibility alias exposes channel `minecraft:brand` and payload bytes, not arbitrary payload routing policy. It does not prove arbitrary plugin-channel handling, payload routing policy, Configuration completion, Play readiness, world hydration, or client load completion. Decompiled source paths referenced by the cases are not restored in this checkout, but the official jar-backed answers are generated from the client jar. |

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
            -> next likely target: choose the next network_login_configuration
               runtime gap, such as full spawn_reader keep-alive reaction or
               runtime Configuration-to-Play transition, because the current
               Protocol 775 Configuration serverbound packet table is now
               covered by jar-backed packet-support proofs
```

## Stop Boundary

Do not use this card as a durable evidence ledger or decision record.

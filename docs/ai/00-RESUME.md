# AI Resume Card

Purpose: restore the current location and next action after compaction, fresh
session, or handoff. This file is a recovery pointer only.

## Current Location

| Field | Value |
|---|---|
| Current location | Repository root is staged for the first snapshot commit; `stevenarella/` is Leafish source imported as normal files |
| Last touched area | `stevenarella/`, `.gitignore`, `_research/protocol/versions/775/witnesses/`, `_tools/java/`, `docs/ai/00-RESUME.md` |
| Next read entry | `docs/ai/README.md`, `CONTEXT.md` for project terms, then `docs/analysis/responsibility/README.md` and the shard named by the active task |
| Explicit uncertainty | No build or compatibility test has been run after replacing `stevenarella/` with Leafish |

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
      -> if working on the client checkout, inspect stevenarella/ as Leafish
         source imported into the parent repository, not as an embedded Git repo
        -> run build or compatibility proof before claiming playability status
```

## Stop Boundary

Do not use this card as a durable evidence ledger or decision record.

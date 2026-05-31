# AI Resume Card

Purpose: restore the current location and next action after compaction, fresh
session, or handoff. This file is a recovery pointer only.

## Current Location

| Field | Value |
|---|---|
| Current location | Protocol 775 `login_cookie_response_serverbound_framed_dispatch` packet-support package now passes: the exact oracle test validates the official Login serverbound `minecraft:cookie_response` answer for one Identifier-key/non-null-payload fixture and decodes it through `packet::packet_by_id(775, State::Login, Direction::Serverbound, official id, body)` with full body consumption. The generated 26.1.2 Configuration clientbound and serverbound packet tables are complete through their current official rows; the Handshaking serverbound table has its official `minecraft:intention` / `0x00` row; Login serverbound packet-support is complete through current official rows: `minecraft:hello` / `0x00`, `minecraft:key` / `0x01`, `minecraft:custom_query_answer` / `0x02`, `minecraft:login_acknowledged` / `0x03`, and `minecraft:cookie_response` / `0x04`. |
| Last touched area | `oracle/cases/775/`, `oracle/contracts/775/`, `oracle/answers/775/`, `oracle/test-manifests/775/`, `oracle/failures/775/`, `oracle/rust-tests/`, `oracle/harness/java/`, `stevenarella/protocol/src/protocol/{mod.rs,packet.rs,versions/v26_1_2.rs}`, `docs/analysis/protocol/versions/775/`, `docs/analysis/client-load/`, `docs/analysis/current-evidence/client-load.md`, `docs/ai/00-RESUME.md` |
| Next read entry | `docs/ai/README.md`, `CONTEXT.md` for project terms, then `docs/analysis/responsibility/README.md` and the shard named by the active task |
| Explicit uncertainty | `login_cookie_response_serverbound_framed_dispatch` proves only Login serverbound cookie_response packet id/body dispatch for one official non-null payload fixture. It does not prove cookie storage policy, cookie request/response runtime behavior, Configuration entry, state transition handling, runtime Configuration-to-Play transition, Play readiness, world load, render readiness, or client load completion. |

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
          -> for packet-support loop, network_login_configuration now has
             Configuration clientbound/serverbound table proof through current
             official rows, Handshaking serverbound proof for
             minecraft:intention / 0x00, and Login serverbound proof for
             minecraft:hello / 0x00, minecraft:key / 0x01,
             minecraft:custom_query_answer / 0x02,
             minecraft:login_acknowledged / 0x03, and
             minecraft:cookie_response / 0x04
            -> Login serverbound packet-support is complete through current
               official rows; next packet-support target is Login clientbound
               minecraft:login_disconnect / 0x00, using
               LoginProtocols.CLIENTBOUND_TEMPLATE official table evidence first
              -> do not move to Play packet work until Handshake/Login packet
                 table gaps are audited or explicitly bounded
```

## Stop Boundary

Do not use this card as a durable evidence ledger or decision record.

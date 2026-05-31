# AI Resume Card

Purpose: restore the current location and next action after compaction, fresh
session, or handoff. This file is a recovery pointer only.

## Current Location

| Field | Value |
|---|---|
| Current location | Protocol 775 Play clientbound packet-support now has a safe GREEN/BLUE batch beyond `minecraft:custom_payload` / `0x18`. A deferred ledger parks `minecraft:damage_event` / `0x19` and intervening YELLOW/RED rows `0x1a`-`0x1f`, `0x21`-`0x22`, `0x24`, and `0x27`-`0x28` until official fixture evidence exists. The latest batch added jar-backed official answers and exact Rust oracle tests for `play_disconnect_clientbound_framed_dispatch` (`minecraft:disconnect` / `0x20`, body `080000`, framed `20080000`), `play_entity_position_sync_clientbound_framed_dispatch` (`minecraft:entity_position_sync` / `0x23`), `play_forget_level_chunk_clientbound_framed_dispatch` (`minecraft:forget_level_chunk` / `0x25`, body `fffffff90000000c`, framed `25fffffff90000000c`), `play_game_event_clientbound_framed_dispatch` (`minecraft:game_event` / `0x26`, body `013f000000`, framed `26013f000000`), and `play_mount_screen_open_clientbound_framed_dispatch` (`minecraft:mount_screen_open` / `0x29`, body `07050000007b`, framed `2907050000007b`). Play clientbound rows `minecraft:bundle_delimiter` / `0x00` through `minecraft:custom_payload` / `0x18` also pass where their case artifacts exist. The generated 26.1.2 Play table audit observed 141 clientbound rows and 69 serverbound rows from `GameProtocols.CLIENTBOUND_TEMPLATE` / `SERVERBOUND_TEMPLATE`; the next official Play clientbound row after this safe batch is `minecraft:hurt_animation` / `0x2a`. Configuration clientbound/serverbound, Handshaking serverbound, Login serverbound, and Login clientbound packet-support are complete through their current official rows. |
| Last touched area | `oracle/cases/775/`, `oracle/contracts/775/`, `oracle/answers/775/`, `oracle/test-manifests/775/`, `oracle/failures/775/`, `oracle/rust-tests/`, `oracle/harness/java/`, `stevenarella/protocol/src/protocol/{packet.rs,mapped_packet.rs,versions/v26_1_2.rs}`, `docs/analysis/protocol/versions/775/`, `docs/analysis/client-load/`, `docs/analysis/current-evidence/client-load.md`, `docs/ai/00-RESUME.md` |
| Next read entry | `docs/ai/README.md`, `CONTEXT.md` for project terms, then `docs/analysis/responsibility/README.md` and the shard named by the active task |
| Explicit uncertainty | The Play clientbound proofs through this safe batch are packet id/body dispatch evidence for one fixture per implemented row only. They do not prove item cooldown semantics, chat UI behavior, arbitrary plugin-channel handling, payload routing policy, UI disconnect handling, entity position semantics, chunk unload behavior, game event semantics, initialized Level/player/weather state, mount entity existence, initialized client/server state, runtime Configuration-to-Play transition, successful Play entry, world load, spawn readiness, render readiness, or client-load completion. Deferred rows are not rejected; they need official fixture evidence before implementation. |

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
          -> for packet-support loop, Play table audit now exists from
             GameProtocols.CLIENTBOUND_TEMPLATE / SERVERBOUND_TEMPLATE and
             Play clientbound minecraft:bundle_delimiter / 0x00 plus
             minecraft:add_entity / 0x01 plus minecraft:animate / 0x02
             plus minecraft:award_stats / 0x03 plus
             minecraft:block_changed_ack / 0x04 plus
             minecraft:block_destruction / 0x05 plus
             minecraft:block_entity_data / 0x06 plus
             minecraft:block_event / 0x07 plus
             minecraft:block_update / 0x08 plus
             minecraft:boss_event / 0x09 plus
             minecraft:change_difficulty / 0x0a plus
             minecraft:chunk_batch_finished / 0x0b plus
             minecraft:chunk_batch_start / 0x0c plus
             minecraft:chunks_biomes / 0x0d plus
             minecraft:clear_titles / 0x0e plus
             minecraft:command_suggestions / 0x0f plus
             minecraft:commands / 0x10 plus
             minecraft:container_close / 0x11 plus
             minecraft:container_set_content / 0x12 plus
             minecraft:container_set_data / 0x13 plus
             minecraft:container_set_slot / 0x14 plus
             minecraft:cookie_request / 0x15 plus
             minecraft:cooldown / 0x16 plus
             minecraft:custom_chat_completions / 0x17 plus
             minecraft:custom_payload / 0x18 plus safe GREEN/BLUE proofs for
             minecraft:disconnect / 0x20 plus
             minecraft:entity_position_sync / 0x23 plus
             minecraft:forget_level_chunk / 0x25 plus
             minecraft:game_event / 0x26 plus
             minecraft:mount_screen_open / 0x29 proofs pass; deferred ledger
             covers minecraft:damage_event / 0x19 and intervening YELLOW/RED
             rows until official fixture evidence exists
            -> next packet-support target by the same official Play
               clientbound table order is minecraft:hurt_animation / 0x2a
              -> first confirm official class/API/body shape and fixture
                 feasibility from the official jar before adding artifacts
```

## Stop Boundary

Do not use this card as a durable evidence ledger or decision record.

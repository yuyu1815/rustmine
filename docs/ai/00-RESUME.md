# AI Resume Card

Purpose: restore the current location and next action after compaction, fresh
session, or handoff. This file is a recovery pointer only.

## Current Location

| Field | Value |
|---|---|
| Current location | Protocol 775 Play clientbound packet-support now passes through `minecraft:custom_payload` / `0x18`. The latest batch added jar-backed official answers and exact Rust oracle tests for `play_cooldown_clientbound_framed_dispatch` (`minecraft:cooldown` / `0x16`, body `03613a617b`, framed `1603613a617b`), `play_custom_chat_completions_clientbound_framed_dispatch` (`minecraft:custom_chat_completions` / `0x17`, body `000105616c706861`, framed `17000105616c706861`), and `play_custom_payload_clientbound_framed_dispatch` (`minecraft:custom_payload` / `0x18`, BrandPayload body, framed `180f6d696e6563726166743a6272616e641a727573746d696e652d706c61792d6f7261636c652d6272616e64`). Play clientbound rows `minecraft:bundle_delimiter` / `0x00` through `minecraft:cookie_request` / `0x15` also pass. The generated 26.1.2 Play table audit observed 141 clientbound rows and 69 serverbound rows from `GameProtocols.CLIENTBOUND_TEMPLATE` / `SERVERBOUND_TEMPLATE`; the next official Play clientbound row is `minecraft:damage_event` / `0x19`. Configuration clientbound/serverbound, Handshaking serverbound, Login serverbound, and Login clientbound packet-support are complete through their current official rows. |
| Last touched area | `oracle/cases/775/`, `oracle/contracts/775/`, `oracle/answers/775/`, `oracle/test-manifests/775/`, `oracle/failures/775/`, `oracle/rust-tests/`, `oracle/harness/java/`, `stevenarella/protocol/src/protocol/{packet.rs,mapped_packet.rs,versions/v26_1_2.rs}`, `docs/analysis/protocol/versions/775/`, `docs/analysis/client-load/`, `docs/analysis/current-evidence/client-load.md`, `docs/ai/00-RESUME.md` |
| Next read entry | `docs/ai/README.md`, `CONTEXT.md` for project terms, then `docs/analysis/responsibility/README.md` and the shard named by the active task |
| Explicit uncertainty | The Play clientbound proofs through `minecraft:custom_payload` / `0x18` are packet id/body dispatch evidence for one fixture per row only. They do not prove item cooldown semantics, chat UI behavior, arbitrary plugin-channel handling, payload routing policy, initialized client/server state, runtime Configuration-to-Play transition, successful Play entry, world load, spawn readiness, render readiness, or client-load completion. |

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
             minecraft:custom_payload / 0x18 proofs pass
            -> next packet-support target by the same official Play
               clientbound table order is minecraft:damage_event / 0x19
              -> first determine whether a smallest official damage_event
                 fixture can be generated without initialized registry/game
                 state; stop with a registry/initialized-harness blocker if it
                 cannot
```

## Stop Boundary

Do not use this card as a durable evidence ledger or decision record.

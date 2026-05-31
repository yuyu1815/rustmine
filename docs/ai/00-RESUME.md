# AI Resume Card

Purpose: restore the current location and next action after compaction, fresh
session, or handoff. This file is a recovery pointer only.

## Current Location

| Field | Value |
|---|---|
| Current location | Protocol 775 `play_commands_clientbound_framed_dispatch` packet-support package now passes: the exact oracle test validates the official Play clientbound `minecraft:commands` / `0x10` answer for the official `ClientboundCommandsPacket(RootCommandNode<S>, NodeInspector<S>)` empty root-only command tree fixture, body `01000000`, framed bytes `1001000000`, then decodes it through `packet::packet_by_id(775, State::Play, Direction::Clientbound, official id, body)` with full body consumption. Play clientbound rows `minecraft:bundle_delimiter` / `0x00` through `minecraft:command_suggestions` / `0x0f` also pass. The generated 26.1.2 Play table audit observed 141 clientbound rows and 69 serverbound rows from `GameProtocols.CLIENTBOUND_TEMPLATE` / `SERVERBOUND_TEMPLATE`; first Play clientbound rows through the next target are `minecraft:bundle_delimiter` / `0x00`, `minecraft:add_entity` / `0x01`, `minecraft:animate` / `0x02`, `minecraft:award_stats` / `0x03`, `minecraft:block_changed_ack` / `0x04`, `minecraft:block_destruction` / `0x05`, `minecraft:block_entity_data` / `0x06`, `minecraft:block_event` / `0x07`, `minecraft:block_update` / `0x08`, `minecraft:boss_event` / `0x09`, `minecraft:change_difficulty` / `0x0a`, `minecraft:chunk_batch_finished` / `0x0b`, `minecraft:chunk_batch_start` / `0x0c`, `minecraft:chunks_biomes` / `0x0d`, `minecraft:clear_titles` / `0x0e`, `minecraft:command_suggestions` / `0x0f`, `minecraft:commands` / `0x10`, and `minecraft:container_close` / `0x11`. Configuration clientbound/serverbound, Handshaking serverbound, Login serverbound, and Login clientbound packet-support are complete through their current official rows. |
| Last touched area | `oracle/cases/775/`, `oracle/contracts/775/`, `oracle/answers/775/`, `oracle/test-manifests/775/`, `oracle/failures/775/`, `oracle/rust-tests/`, `oracle/harness/java/`, `stevenarella/protocol/src/protocol/{packet.rs,mapped_packet.rs,versions/v26_1_2.rs}`, `docs/analysis/protocol/versions/775/`, `docs/analysis/client-load/`, `docs/analysis/current-evidence/client-load.md`, `docs/ai/00-RESUME.md` |
| Next read entry | `docs/ai/README.md`, `CONTEXT.md` for project terms, then `docs/analysis/responsibility/README.md` and the shard named by the active task |
| Explicit uncertainty | `play_commands_clientbound_framed_dispatch` proves only Play clientbound commands packet id/body dispatch for one official empty root-only command tree fixture. It does not prove literal/argument node payloads, redirects, custom suggestions, restricted flags, Brigadier command semantics, command context behavior, command UI behavior, initialized game state, runtime Configuration-to-Play transition, successful Play entry, world load, spawn readiness, render readiness, or client-load completion. |

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
             minecraft:commands / 0x10 proofs pass
            -> next packet-support target by the same official Play
               clientbound table order is minecraft:container_close / 0x11
              -> first determine whether a smallest official container_close fixture
                 can be generated without initialized Minecraft/game state;
                 stop with an initialized-harness blocker if it cannot
```

## Stop Boundary

Do not use this card as a durable evidence ledger or decision record.

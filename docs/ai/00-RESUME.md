# AI Resume Card

Purpose: restore the current location and next action after compaction, fresh
session, or handoff. This file is a recovery pointer only.

## Current Location

| Field | Value |
|---|---|
| Current location | Protocol 775 Play clientbound packet-support now has safe GREEN/BLUE proofs through the selected `0x3e`-started batch: `play_pong_response_clientbound_framed_dispatch` (`minecraft:pong_response` / `0x3e`, body `0102030405060708`, framed `3e0102030405060708`), `play_player_abilities_clientbound_framed_dispatch` (`minecraft:player_abilities` / `0x40`, body `053d4ccccd3dcccccd`, framed `40053d4ccccd3dcccccd`), `play_player_combat_end_clientbound_framed_dispatch` (`minecraft:player_combat_end` / `0x42`, body `7b`, framed `427b`), `play_player_combat_enter_clientbound_framed_dispatch` (`minecraft:player_combat_enter` / `0x43`, empty body, framed `43`), and `play_remove_entities_clientbound_framed_dispatch` (`minecraft:remove_entities` / `0x4d`, body `027bd723`, framed `4d027bd723`). Earlier Play clientbound packet-support passes include `minecraft:bundle_delimiter` / `0x00` through `minecraft:custom_payload` / `0x18`, plus safe rows `0x20`, `0x23`, `0x25`, `0x26`, `0x29`, `0x2a`-`0x2c`, `0x2e`, `0x32`, `0x35`, `0x36`, `0x38`, `0x39`, `0x3a`, and `0x3d` where their case artifacts exist. The deferred ledger parks evidence-dependent YELLOW/RED rows including `0x19`-`0x1f`, `0x21`-`0x22`, `0x24`, `0x27`-`0x28`, `0x2d`, `0x2f`-`0x31`, `0x33`-`0x34`, `0x37`, `0x3b`-`0x3c`, `0x3f`, `0x41`, `0x44`, `0x46`-`0x4c`, and `0x4e`-`0x51` until official fixture evidence exists. The generated 26.1.2 Play table audit observed 141 clientbound rows and 69 serverbound rows from `GameProtocols.CLIENTBOUND_TEMPLATE` / `SERVERBOUND_TEMPLATE`; the next official Play clientbound row after this safe batch is `minecraft:respawn` / `0x52`. Configuration clientbound/serverbound, Handshaking serverbound, Login serverbound, and Login clientbound packet-support are complete through their current official rows. |
| Last touched area | `oracle/cases/775/`, `oracle/contracts/775/`, `oracle/answers/775/`, `oracle/test-manifests/775/`, `oracle/failures/775/`, `oracle/rust-tests/`, `oracle/harness/java/`, `stevenarella/protocol/src/protocol/{packet.rs,mapped_packet.rs,versions/v26_1_2.rs}`, `docs/analysis/protocol/versions/775/`, `docs/analysis/client-load/`, `docs/analysis/current-evidence/client-load.md`, `docs/ai/00-RESUME.md` |
| Next read entry | `docs/ai/README.md`, `CONTEXT.md` for project terms, then `docs/analysis/responsibility/README.md` and the shard named by the active task |
| Explicit uncertainty | The Play clientbound proofs through this safe batch are packet id/body dispatch evidence for one fixture per implemented row only. They do not prove item cooldown semantics, chat UI behavior, arbitrary plugin-channel handling, payload routing policy, UI disconnect handling, entity position semantics, chunk unload behavior, game event semantics, initialized Level/player/weather state, mount or vehicle existence, book UI behavior, runtime pong response behavior, initialized client/server state, runtime Configuration-to-Play transition, successful Play entry, world load, spawn readiness, render readiness, or client-load completion. Deferred rows are not rejected; they need official fixture evidence before implementation. |

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
	             minecraft:mount_screen_open / 0x29 plus
	             minecraft:hurt_animation / 0x2a plus
	             minecraft:initialize_border / 0x2b plus
	             minecraft:keep_alive / 0x2c plus
	             minecraft:level_event / 0x2e plus
	             minecraft:low_disk_space_warning / 0x32 plus
	             minecraft:move_entity_pos / 0x35 plus
	             minecraft:move_entity_pos_rot / 0x36 plus
	             minecraft:move_entity_rot / 0x38 plus
	             minecraft:move_vehicle / 0x39 plus
	             minecraft:open_book / 0x3a plus
	             minecraft:ping / 0x3d plus
	             minecraft:pong_response / 0x3e plus
	             minecraft:player_abilities / 0x40 plus
	             minecraft:player_combat_end / 0x42 plus
	             minecraft:player_combat_enter / 0x43 plus
	             minecraft:remove_entities / 0x4d proofs pass; deferred ledger
	             covers minecraft:damage_event / 0x19 and intervening YELLOW/RED
	             rows until official fixture evidence exists
	            -> next packet-support target by the same official Play
	               clientbound safe-batch cartography is
	               minecraft:respawn / 0x52
	              -> first confirm official class/API/body shape and fixture
	                 feasibility from the official jar before adding artifacts
```

## Stop Boundary

Do not use this card as a durable evidence ledger or decision record.

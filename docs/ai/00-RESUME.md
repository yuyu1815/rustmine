# AI Resume Card

Purpose: restore the current location and next action after compaction, fresh
session, or handoff. This file is a recovery pointer only.

## Current Location

| Field | Value |
|---|---|
| Current location | Protocol 775 Play clientbound packet-support now has safe GREEN/BLUE proofs through the selected `0x52`-started cartography batch plus the previously confirmed `0x45` row: `play_player_info_remove_clientbound_framed_dispatch` (`minecraft:player_info_remove` / `0x45`, body `02123e4567e89b12d3a45642661417404500000000000000000000000000000045`, framed `4502123e4567e89b12d3a45642661417404500000000000000000000000000000045`), `play_rotate_head_clientbound_framed_dispatch` (`minecraft:rotate_head` / `0x53`, body `7b40`, framed `537b40`), `play_select_advancements_tab_clientbound_framed_dispatch` (`minecraft:select_advancements_tab` / `0x55`, body `01146d696e6563726166743a73746f72792f726f6f74`, framed `5501146d696e6563726166743a73746f72792f726f6f74`), `play_set_border_center_clientbound_framed_dispatch` (`minecraft:set_border_center` / `0x58`, body `4029000000000000c041600000000000`, framed `584029000000000000c041600000000000`), `play_set_border_lerp_size_clientbound_framed_dispatch` (`minecraft:set_border_lerp_size` / `0x59`, body `4059000000000000406f500000000000b960`, framed `594059000000000000406f500000000000b960`), `play_set_border_size_clientbound_framed_dispatch` (`minecraft:set_border_size` / `0x5a`, body `4080020000000000`, framed `5a4080020000000000`), `play_set_border_warning_delay_clientbound_framed_dispatch` (`minecraft:set_border_warning_delay` / `0x5b`, body `2a`, framed `5b2a`), `play_set_border_warning_distance_clientbound_framed_dispatch` (`minecraft:set_border_warning_distance` / `0x5c`, body `07`, framed `5c07`), `play_set_chunk_cache_center_clientbound_framed_dispatch` (`minecraft:set_chunk_cache_center` / `0x5e`, body `07fdffffff0f`, framed `5e07fdffffff0f`), and `play_set_chunk_cache_radius_clientbound_framed_dispatch` (`minecraft:set_chunk_cache_radius` / `0x5f`, body `0c`, framed `5f0c`). Earlier Play clientbound packet-support passes include `minecraft:bundle_delimiter` / `0x00` through `minecraft:custom_payload` / `0x18`, plus safe rows `0x20`, `0x23`, `0x25`, `0x26`, `0x29`, `0x2a`-`0x2c`, `0x2e`, `0x32`, `0x35`, `0x36`, `0x38`, `0x39`, `0x3a`, `0x3d`, `0x3e`, `0x40`, `0x42`, `0x43`, and `0x4d` where their case artifacts exist. The deferred ledger parks evidence-dependent YELLOW/RED rows including `0x19`-`0x1f`, `0x21`-`0x22`, `0x24`, `0x27`-`0x28`, `0x2d`, `0x2f`-`0x31`, `0x33`-`0x34`, `0x37`, `0x3b`-`0x3c`, `0x3f`, `0x41`, `0x44`, `0x46`-`0x4c`, `0x4e`-`0x52`, `0x54`, `0x56`-`0x57`, `0x5d`, and `0x60`-`0x64` until official fixture evidence exists; `minecraft:set_entity_motion` / `0x65` remains safe BLUE deferred by batch cap, not rejected. The generated 26.1.2 Play table audit observed 141 clientbound rows and 69 serverbound rows from `GameProtocols.CLIENTBOUND_TEMPLATE` / `SERVERBOUND_TEMPLATE`; the next official Play clientbound row after this safe batch is `minecraft:set_cursor_item` / `0x60`, with `minecraft:set_entity_motion` / `0x65` as a safe follow-up candidate. Configuration clientbound/serverbound, Handshaking serverbound, Login serverbound, and Login clientbound packet-support are complete through their current official rows. |
| Last touched area | `oracle/cases/775/`, `oracle/contracts/775/`, `oracle/answers/775/`, `oracle/test-manifests/775/`, `oracle/failures/775/`, `oracle/rust-tests/`, `oracle/harness/java/`, `stevenarella/protocol/src/protocol/{packet.rs,mapped_packet.rs,versions/v26_1_2.rs}`, `docs/analysis/protocol/versions/775/`, `docs/analysis/client-load/`, `docs/analysis/current-evidence/client-load.md`, `docs/ai/00-RESUME.md` |
| Next read entry | `docs/ai/README.md`, `CONTEXT.md` for project terms, then `docs/analysis/responsibility/README.md` and the shard named by the active task |
| Explicit uncertainty | The Play clientbound proofs through this safe batch are packet id/body dispatch evidence for one fixture per implemented row only. They do not prove item cooldown semantics, chat UI behavior, arbitrary plugin-channel handling, payload routing policy, UI disconnect handling, entity position semantics, chunk unload behavior, game event semantics, initialized Level/player/weather state, mount or vehicle existence, book UI behavior, world-border runtime behavior, warning UI behavior, runtime pong response behavior, initialized client/server state, runtime Configuration-to-Play transition, successful Play entry, world load, spawn readiness, render readiness, or client-load completion. Deferred rows are not rejected; they need official fixture evidence before implementation. |

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
	             minecraft:player_info_remove / 0x45 plus
	             minecraft:remove_entities / 0x4d plus
	             minecraft:rotate_head / 0x53 plus
	             minecraft:select_advancements_tab / 0x55 plus
	             minecraft:set_border_center / 0x58 through
	             minecraft:set_border_warning_distance / 0x5c plus
	             minecraft:set_chunk_cache_center / 0x5e plus
	             minecraft:set_chunk_cache_radius / 0x5f proofs pass; deferred ledger
	             covers minecraft:damage_event / 0x19 and intervening YELLOW/RED
	             rows until official fixture evidence exists
	            -> next packet-support target by the same official Play
	               clientbound safe-batch cartography is
	               minecraft:set_entity_motion / 0x65; the next official row
	               after the latest proven batch is minecraft:set_cursor_item / 0x60
	              -> first confirm official class/API/body shape and fixture
	                 feasibility from the official jar before adding artifacts
```

## Stop Boundary

Do not use this card as a durable evidence ledger or decision record.

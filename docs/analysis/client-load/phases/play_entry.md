# play_entry

| Field | Value |
|---|---|
| Lens position | 4 of 8 |
| Load claim | Client enters Play with enough state to receive spawn/world packets. |
| Evidence surface | Official state transition plus smoke milestone |
| Proof label | `partial` |
| Current proof | Packet-support proofs are recorded case-by-case in `docs/analysis/protocol/versions/775/traceability.md`; latest selected safe GREEN/BLUE Play rows include `minecraft:pong_response` / `0x3e`, `minecraft:player_abilities` / `0x40`, `minecraft:player_combat_end` / `0x42`, `minecraft:player_combat_enter` / `0x43`, and `minecraft:remove_entities` / `0x4d` while skipped YELLOW rows remain deferred. |
| Project-level test/probe | Exact Rust oracle tests under `oracle/rust-tests/tests/oracle_contracts.rs`, named in each `oracle/test-manifests/775/play_*_clientbound_framed_dispatch.test-manifest.json`. |
| Candidate checkout owner under test | login/configuration/play handoff |
| Candidate evidence gap | Continue safe official Play table packet-support at `minecraft:respawn` / `0x52`, or return to deferred rows only after official fixture evidence; then define transition answer and smoke milestone. |

## Boundary

Codec proof in Configuration does not prove Play entry. This phase needs a
state-transition or smoke milestone proof tied to a root-owned artifact.
The current Play packet-support proofs only show that Stevenarella dispatches
the official Play clientbound `minecraft:bundle_delimiter` / `0x00` empty-body
fixture and one official Play clientbound `minecraft:add_entity` / `0x01`
built-in EntityType.PIG zero-movement fixture, plus one official Play
clientbound `minecraft:animate` / `0x02` entity id 123 and SWING_MAIN_HAND
fixture, plus one official Play clientbound `minecraft:award_stats` / `0x03`
empty-stats fixture, plus one official Play clientbound
`minecraft:block_changed_ack` / `0x04` sequence fixture, plus one official Play
clientbound `minecraft:block_destruction` / `0x05` breaker id, block position,
and progress fixture, plus one official Play clientbound
`minecraft:block_entity_data` / `0x06` block position, built-in chest block
entity type, and empty tag fixture, plus one official Play clientbound
`minecraft:block_event` / `0x07` block position, built-in note block, event
type, and event data fixture, plus one official Play clientbound
`minecraft:block_update` / `0x08` block position and built-in stone default
block-state fixture, plus one official Play clientbound `minecraft:boss_event`
/ `0x09` UUID remove-operation fixture, plus one official Play clientbound
`minecraft:change_difficulty` / `0x0a` difficulty/locked fixture, plus one
official Play clientbound `minecraft:chunk_batch_finished` / `0x0b` batchSize
fixture, plus one official Play clientbound `minecraft:chunk_batch_start` /
`0x0c` singleton empty-body fixture, plus one official Play clientbound
`minecraft:chunks_biomes` / `0x0d` empty chunkBiomeData-list fixture, plus one
official Play clientbound `minecraft:clear_titles` / `0x0e` resetTimes=true
boolean fixture, plus one official Play clientbound
`minecraft:command_suggestions` / `0x0f` command id/range/empty-suggestions
fixture, plus one official Play clientbound `minecraft:commands` / `0x10` empty
root-only command tree fixture, plus one official Play clientbound
`minecraft:container_close` / `0x11` container id fixture, plus one official
Play clientbound `minecraft:container_set_content` / `0x12` empty item list and
empty carried stack fixture, plus one official Play clientbound
`minecraft:container_set_data` / `0x13` numeric fixture, plus one official
Play clientbound `minecraft:container_set_slot` / `0x14` empty ItemStack
fixture, plus one official Play clientbound `minecraft:cookie_request` /
`0x15` Identifier key fixture, plus one official Play clientbound
`minecraft:cooldown` / `0x16` Identifier/duration fixture, plus one official
Play clientbound `minecraft:custom_chat_completions` / `0x17` ADD/string-list
fixture, plus one official Play clientbound `minecraft:custom_payload` /
`0x18` BrandPayload fixture, plus one official Play clientbound
`minecraft:disconnect` / `0x20` empty Component fixture, plus one official Play
clientbound `minecraft:entity_position_sync` / `0x23` primitive
position/movement fixture, plus one official Play clientbound
`minecraft:forget_level_chunk` / `0x25` chunk position fixture, plus one
official Play clientbound `minecraft:game_event` / `0x26` START_RAINING/0.5
fixture, plus one official Play clientbound `minecraft:mount_screen_open` /
`0x29` container/inventory/entity id fixture; they do not prove non-empty
ItemStack/component registry handling, menu property semantics, menu lifecycle
behavior, screen close behavior, inventory state, cookie storage policy, cookie
request/response runtime behavior, item cooldown semantics, item registry
contents, chat UI behavior, command context behavior, arbitrary plugin-channel
handling, payload routing policy, UI disconnect handling, entity position
semantics, chunk unload behavior, game event semantics, initialized
Level/player/weather state, mount entity existence, screen behavior, or runtime
entry into Play. The deferred ledger parks `minecraft:damage_event` / `0x19`
and the intervening YELLOW/RED rows until official fixture evidence exists.

The latest packet-support batch adds selected safe GREEN/BLUE rows
`minecraft:level_event` / `0x2e`, `minecraft:low_disk_space_warning` / `0x32`,
`minecraft:move_entity_pos` / `0x35`,
`minecraft:move_entity_pos_rot` / `0x36`, `minecraft:move_entity_rot` /
`0x38`, `minecraft:move_vehicle` / `0x39`, `minecraft:open_book` / `0x3a`,
`minecraft:ping` / `0x3d`, `minecraft:pong_response` / `0x3e`,
`minecraft:player_abilities` / `0x40`,
`minecraft:player_combat_end` / `0x42`,
`minecraft:player_combat_enter` / `0x43`, and
`minecraft:remove_entities` / `0x4d`. These remain framed dispatch/decode
proofs only. They do not prove skipped recipe/chat/player-info/world/UI rows,
entity existence, combat runtime behavior, player movement/ability semantics,
resource-pack behavior, render readiness, or client-load completion. The next
official Play clientbound row after this safe batch is
`minecraft:respawn` / `0x52`.

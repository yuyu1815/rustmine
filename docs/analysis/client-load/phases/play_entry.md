# play_entry

| Field | Value |
|---|---|
| Lens position | 4 of 8 |
| Load claim | Client enters Play with enough state to receive spawn/world packets. |
| Evidence surface | Official state transition plus smoke milestone |
| Proof label | `partial` |
| Current proof | `play_bundle_delimiter_clientbound_framed_dispatch`; `play_add_entity_clientbound_framed_dispatch`; `play_animate_clientbound_framed_dispatch`; `play_award_stats_clientbound_framed_dispatch`; `play_block_changed_ack_clientbound_framed_dispatch`; `play_block_destruction_clientbound_framed_dispatch`; `play_block_entity_data_clientbound_framed_dispatch`; `play_block_event_clientbound_framed_dispatch`; `play_block_update_clientbound_framed_dispatch`; `play_boss_event_clientbound_framed_dispatch`; `play_change_difficulty_clientbound_framed_dispatch`; `play_chunk_batch_finished_clientbound_framed_dispatch`; `play_chunk_batch_start_clientbound_framed_dispatch`; `play_chunks_biomes_clientbound_framed_dispatch`; `play_clear_titles_clientbound_framed_dispatch`; `play_command_suggestions_clientbound_framed_dispatch`; `play_commands_clientbound_framed_dispatch`; `play_container_close_clientbound_framed_dispatch`; `play_container_set_content_clientbound_framed_dispatch` |
| Project-level test/probe | `oracle/rust-tests/tests/oracle_contracts.rs::play_bundle_delimiter_clientbound_framed_dispatch_matches_official_oracle_answer`; `oracle/rust-tests/tests/oracle_contracts.rs::play_add_entity_clientbound_framed_dispatch_matches_official_oracle_answer`; `oracle/rust-tests/tests/oracle_contracts.rs::play_animate_clientbound_framed_dispatch_matches_official_oracle_answer`; `oracle/rust-tests/tests/oracle_contracts.rs::play_award_stats_clientbound_framed_dispatch_matches_official_oracle_answer`; `oracle/rust-tests/tests/oracle_contracts.rs::play_block_changed_ack_clientbound_framed_dispatch_matches_official_oracle_answer`; `oracle/rust-tests/tests/oracle_contracts.rs::play_block_destruction_clientbound_framed_dispatch_matches_official_oracle_answer`; `oracle/rust-tests/tests/oracle_contracts.rs::play_block_entity_data_clientbound_framed_dispatch_matches_official_oracle_answer`; `oracle/rust-tests/tests/oracle_contracts.rs::play_block_event_clientbound_framed_dispatch_matches_official_oracle_answer`; `oracle/rust-tests/tests/oracle_contracts.rs::play_block_update_clientbound_framed_dispatch_matches_official_oracle_answer`; `oracle/rust-tests/tests/oracle_contracts.rs::play_boss_event_clientbound_framed_dispatch_matches_official_oracle_answer`; `oracle/rust-tests/tests/oracle_contracts.rs::play_change_difficulty_clientbound_framed_dispatch_matches_official_oracle_answer`; `oracle/rust-tests/tests/oracle_contracts.rs::play_chunk_batch_finished_clientbound_framed_dispatch_matches_official_oracle_answer`; `oracle/rust-tests/tests/oracle_contracts.rs::play_chunk_batch_start_clientbound_framed_dispatch_matches_official_oracle_answer`; `oracle/rust-tests/tests/oracle_contracts.rs::play_chunks_biomes_clientbound_framed_dispatch_matches_official_oracle_answer`; `oracle/rust-tests/tests/oracle_contracts.rs::play_clear_titles_clientbound_framed_dispatch_matches_official_oracle_answer`; `oracle/rust-tests/tests/oracle_contracts.rs::play_command_suggestions_clientbound_framed_dispatch_matches_official_oracle_answer`; `oracle/rust-tests/tests/oracle_contracts.rs::play_commands_clientbound_framed_dispatch_matches_official_oracle_answer`; `oracle/rust-tests/tests/oracle_contracts.rs::play_container_close_clientbound_framed_dispatch_matches_official_oracle_answer`; `oracle/rust-tests/tests/oracle_contracts.rs::play_container_set_content_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Candidate checkout owner under test | login/configuration/play handoff |
| Candidate evidence gap | Continue official Play table packet-support in order at `minecraft:container_set_data` / `0x13`, then define transition answer and smoke milestone. |

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
empty carried stack fixture; they do not prove non-empty ItemStack/component
registry handling, menu lifecycle behavior, screen close behavior, inventory
state, or runtime entry into Play.

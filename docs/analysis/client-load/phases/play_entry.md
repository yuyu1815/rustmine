# play_entry

| Field | Value |
|---|---|
| Lens position | 4 of 8 |
| Load claim | Client enters Play with enough state to receive spawn/world packets. |
| Evidence surface | Official state transition plus smoke milestone |
| Proof label | `partial` |
| Current proof | `play_bundle_delimiter_clientbound_framed_dispatch`; `play_add_entity_clientbound_framed_dispatch` |
| Project-level test/probe | `oracle/rust-tests/tests/oracle_contracts.rs::play_bundle_delimiter_clientbound_framed_dispatch_matches_official_oracle_answer`; `oracle/rust-tests/tests/oracle_contracts.rs::play_add_entity_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Candidate checkout owner under test | login/configuration/play handoff |
| Candidate evidence gap | Continue official Play table packet-support in order, then define transition answer and smoke milestone. |

## Boundary

Codec proof in Configuration does not prove Play entry. This phase needs a
state-transition or smoke milestone proof tied to a root-owned artifact.
The current Play packet-support proofs only show that Stevenarella dispatches
the official Play clientbound `minecraft:bundle_delimiter` / `0x00` empty-body
fixture and one official Play clientbound `minecraft:add_entity` / `0x01`
built-in EntityType.PIG zero-movement fixture; they do not prove runtime entry
into Play.

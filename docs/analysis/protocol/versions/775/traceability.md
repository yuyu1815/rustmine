# Protocol 775 Traceability

Purpose: stop repeated packet reinterpretation by keeping the official source,
oracle artifact, project-level test surface, and checkout owner on one
reset-proof map.

## Rules

| Rule | Meaning |
|---|---|
| Official first | Packet facts must come from `client.jar`, `server.jar`, or a generated oracle answer. |
| Reset-proof test surface | Canonical oracle tests live outside `stevenarella/`. |
| Manifest before Rust work | A Rust fix needs an answer path and test manifest, not chat memory. |
| Mapping stays visible | Official names and internal names are mapped here instead of flattened. |

## Traceability Rows

| Case | Detail | Corridor | Official source | Oracle artifacts | Project-level Rust proof | Checkout owner under test | Evidence snapshot | Stop boundary |
|---|---|---|---|---|---|---|---|---|
| `configuration_keepalive_codec` | [cases/configuration-keepalive-codec.md](cases/configuration-keepalive-codec.md) | `Login -> Configuration -> Play` | `client.jar` `ServerboundKeepAlivePacket.STREAM_CODEC`; recorded source label `ConfigurationProtocols.SERVERBOUND.details().listPackets(...)` | `oracle/cases/775/configuration_keepalive_codec.json`; `oracle/contracts/775/configuration_keepalive_codec.contract.json`; `oracle/answers/775/configuration_keepalive_codec.answer.jsonl`; `oracle/test-manifests/775/configuration_keepalive_codec.test-manifest.json` | `oracle/rust-tests/tests/oracle_contracts.rs::configuration_keepalive_matches_official_oracle_answer` | `stevenarella/protocol/src/protocol/packet.rs`; `packet::configuration::serverbound::ConfigurationKeepAliveServerbound_i64`; `PacketType::packet_id(775)`; `stevenarella/protocol/src/protocol/versions/v26_1_2.rs` | Official answer regenerated and exact project-level Rust oracle test executed against the current Leafish checkout | Codec and packet-id only; does not prove Configuration packet dispatch/decode or runtime echo behavior |

## Candidate Rows for Future Evidence

These candidates are not a fixed live plan. Add a row when a task packet,
official answer, or probe creates reusable evidence for the case.

| Candidate | Why it may be useful | Required official proof |
|---|---|---|
| Configuration keep-alive runtime reaction | Moves from codec proof to client/server response behavior | Official server/client initialized behavior or source-backed harness call |
| Configuration finish path | Proves `Configuration -> Play` transition contract | Official protocol state transition source plus answer artifact |
| Registry data handling | Blocks Play/spawn correctness | Heavy harness or official registry-state proof; do not fake initialized state |

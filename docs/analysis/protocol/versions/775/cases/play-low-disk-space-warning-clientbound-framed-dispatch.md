# play_low_disk_space_warning_clientbound_framed_dispatch

| Field | Value |
|---|---|
| Packet | `minecraft:low_disk_space_warning` / `0x32` |
| Official class | `net.minecraft.network.protocol.game.ClientboundLowDiskSpaceWarningPacket` |
| Official body | singleton unit codec, empty body |
| Fixture | `ClientboundLowDiskSpaceWarningPacket.INSTANCE` |
| Answer | `oracle/answers/775/play_low_disk_space_warning_clientbound_framed_dispatch.answer.jsonl` |
| Rust proof | `oracle/rust-tests/tests/oracle_contracts.rs::play_low_disk_space_warning_clientbound_framed_dispatch_matches_official_oracle_answer` |

This is packet-support evidence only. It proves official Play clientbound
framing, empty body, dispatch, and full body consumption for the singleton
fixture. It does not prove disk warning UI behavior, client storage state,
render readiness, or client-load completion.

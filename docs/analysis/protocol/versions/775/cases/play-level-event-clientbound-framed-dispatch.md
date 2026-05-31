# play_level_event_clientbound_framed_dispatch

| Field | Value |
|---|---|
| Packet | `minecraft:level_event` / `0x2e` |
| Official class | `net.minecraft.network.protocol.game.ClientboundLevelEventPacket` |
| Official body | `int type`, `BlockPos`, `int data`, `boolean globalEvent` |
| Fixture | `type=2001`, `BlockPos(1,64,-2)`, `data=1`, `globalEvent=false` |
| Answer | `oracle/answers/775/play_level_event_clientbound_framed_dispatch.answer.jsonl` |
| Rust proof | `oracle/rust-tests/tests/oracle_contracts.rs::play_level_event_clientbound_framed_dispatch_matches_official_oracle_answer` |

This is packet-support evidence only. It proves official Play clientbound
framing, body bytes, dispatch, and full body consumption for one primitive
fixture. It does not prove level event semantics, sound/particle behavior,
initialized `Level` state, render readiness, or client-load completion.

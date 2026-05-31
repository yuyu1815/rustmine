# play_rotate_head_clientbound_framed_dispatch

## Spatial Map

```text
client.jar ClientboundRotateHeadPacket.STREAM_CODEC
  -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x53
    -> oracle/answers/775/play_rotate_head_clientbound_framed_dispatch.answer.jsonl
      -> oracle/rust-tests/tests/oracle_contracts.rs exact test
```

| Field | Value |
|---|---|
| Packet | `minecraft:rotate_head` / `0x53` |
| Official class | `net.minecraft.network.protocol.game.ClientboundRotateHeadPacket` |
| Official body | entity id VarInt, signed head-rotation byte |
| Fixture | entity id `123`, head-rotation byte `64` |
| Answer | `oracle/answers/775/play_rotate_head_clientbound_framed_dispatch.answer.jsonl` |
| Rust proof | `oracle/rust-tests/tests/oracle_contracts.rs::play_rotate_head_clientbound_framed_dispatch_matches_official_oracle_answer` |

## Stop Boundary

This is packet-support evidence only. It proves official Play clientbound
framing, body bytes, dispatch, and full body consumption for one primitive
entity id/head-rotation fixture. It does not prove entity existence, Level
state, render readiness, or client-load completion.

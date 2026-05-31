# play_pong_response_clientbound_framed_dispatch

## Spatial Map

```text
client.jar ClientboundPongResponsePacket(long)
  -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x3e
    -> oracle/answers/775/play_pong_response_clientbound_framed_dispatch.answer.jsonl
      -> oracle/rust-tests/tests/oracle_contracts.rs exact test
```

| Field | Value |
|---|---|
| Packet | `minecraft:pong_response` / `0x3e` |
| Official class | `net.minecraft.network.protocol.ping.ClientboundPongResponsePacket` |
| Official body | one signed `long` time |
| Fixture | primitive time `72623859790382856` |
| Answer | `oracle/answers/775/play_pong_response_clientbound_framed_dispatch.answer.jsonl` |
| Rust proof | `oracle/rust-tests/tests/oracle_contracts.rs::play_pong_response_clientbound_framed_dispatch_matches_official_oracle_answer` |

## Stop Boundary

This is packet-support evidence only. It proves official Play clientbound
framing, body bytes, dispatch, and full body consumption for one primitive
time fixture. It does not prove runtime ping/pong latency behavior, Play
entry, render readiness, or client-load completion.

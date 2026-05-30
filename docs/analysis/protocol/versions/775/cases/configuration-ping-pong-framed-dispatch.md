# configuration_ping_pong_framed_dispatch

## Map

```text
official client.jar
  -> ConfigurationProtocols.CLIENTBOUND.codec().encode/decode(ClientboundPingPacket)
  -> ConfigurationProtocols.SERVERBOUND.codec().encode/decode(ServerboundPongPacket)
  -> ConfigurationProtocols.{CLIENTBOUND,SERVERBOUND}_TEMPLATE.details().listPackets(...)
  -> oracle/answers/775/configuration_ping_pong_framed_dispatch.answer.jsonl
  -> oracle/rust-tests/tests/oracle_contracts.rs
    -> packet::packet_by_id(775, State::Configuration, Direction::{Clientbound,Serverbound}, id, body)
```

## Official Evidence

The generated answer is backed by direct official `client.jar` calls:

| Flow | Official packet class | Official packet type | Official table id | Body source |
|---|---|---|---|---|
| Clientbound | `net.minecraft.network.protocol.common.ClientboundPingPacket` | `minecraft:ping` | answer artifact row for `minecraft:ping` | `ClientboundPingPacket.STREAM_CODEC` via framed codec |
| Serverbound | `net.minecraft.network.protocol.common.ServerboundPongPacket` | `minecraft:pong` | answer artifact row for `minecraft:pong` | `ServerboundPongPacket.STREAM_CODEC` via framed codec |

The decompiled protocol paths are not restored in this checkout; they remain
witness labels only. The official jar call and generated answer artifact are
the expected-value source.

## Test Surface

```text
oracle/test-manifests/775/configuration_ping_pong_framed_dispatch.test-manifest.json
  -> configuration_ping_pong_framed_dispatch_matches_official_oracle_answer
```

Current result: passing against the current Stevenarella checkout. The test
dispatches both Configuration clientbound `minecraft:ping` packet id `0x05`
and Configuration serverbound `minecraft:pong` packet id `0x05`, preserving the
official packet identity and payload id.

## Stop Boundary

This case proves packet framing and dispatch/decode only. It does not prove
runtime ping response behavior, runtime keep-alive echo behavior,
Configuration completion, registry hydration, or Play readiness.

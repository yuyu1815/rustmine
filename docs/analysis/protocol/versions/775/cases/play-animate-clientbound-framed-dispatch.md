# play_animate_clientbound_framed_dispatch

Purpose: preserve the Protocol 775 Play clientbound `minecraft:animate`
packet id/body contract as a reset-proof packet-support slice.

```text
client.jar ClientboundAnimatePacket.STREAM_CODEC decode fixture
  -> entity id 123 and SWING_MAIN_HAND action
    -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x02
      -> GameProtocols.CLIENTBOUND codec encodes/decodes official frame
        -> oracle/answers/775/play_animate_clientbound_framed_dispatch.answer.jsonl
          -> oracle/test-manifests/775/play_animate_clientbound_framed_dispatch.test-manifest.json
            -> oracle/rust-tests/tests/oracle_contracts.rs
              -> stevenarella/protocol/src/protocol/versions/v26_1_2.rs
```

| Field | Value |
|---|---|
| Case id | `play_animate_clientbound_framed_dispatch` |
| Corridor | `Configuration -> Play` |
| Official source | `client.jar` `ClientboundAnimatePacket.STREAM_CODEC`; private `ClientboundAnimatePacket(FriendlyByteBuf)`; private `write(FriendlyByteBuf)`; `ClientboundAnimatePacket.SWING_MAIN_HAND`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(...)`; `ClientGamePacketListener.handleAnimate(ClientboundAnimatePacket)` |
| Bytecode witness | `CP="_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath "$CP" -c -p -verbose net.minecraft.network.protocol.game.ClientboundAnimatePacket net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientGamePacketListener` |
| Generated answer | `oracle/answers/775/play_animate_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/play_animate_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_animate_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Failure packet | `oracle/failures/775/play_animate_clientbound_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/play_animate_clientbound_framed_dispatch.rust-fix-task.json` |

## Official Body Shape

The official fixture uses `ClientboundAnimatePacket.STREAM_CODEC` to decode an
entity id and the official `SWING_MAIN_HAND` constant, then re-encodes through
the official Play clientbound protocol codec. It does not require initialized
`Entity`, `Level`, or game state.

| Order | Field | Fixture value |
|---|---|---|
| 1 | entity id VarInt | `123` |
| 2 | action unsigned byte | `0` / `SWING_MAIN_HAND` |

The generated official frame is:

```text
027b00
```

## Official Table

The generated answer observed 141 Play clientbound rows. The first rows are:

| Packet id | Packet type |
|---|---|
| `0x00` | `minecraft:bundle_delimiter` |
| `0x01` | `minecraft:add_entity` |
| `0x02` | `minecraft:animate` |
| `0x03` | `minecraft:award_stats` |
| `0x04` | `minecraft:block_changed_ack` |

## Stop Boundary

This is packet framing/dispatch/decode evidence for one official
`STREAM_CODEC` decode fixture with entity id `123` and `SWING_MAIN_HAND` only.
It does not prove entity existence, animation semantics, initialized
`Entity`/`Level` behavior, spawn readiness, world load, render readiness, or
client-load completion.

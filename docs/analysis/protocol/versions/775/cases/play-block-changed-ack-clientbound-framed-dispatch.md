# play_block_changed_ack_clientbound_framed_dispatch

Purpose: preserve the Protocol 775 Play clientbound
`minecraft:block_changed_ack` packet id/body contract as a reset-proof
packet-support slice.

```text
client.jar ClientboundBlockChangedAckPacket sequence fixture
  -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x04
    -> GameProtocols.CLIENTBOUND codec encodes/decodes official frame
      -> oracle/answers/775/play_block_changed_ack_clientbound_framed_dispatch.answer.jsonl
        -> oracle/test-manifests/775/play_block_changed_ack_clientbound_framed_dispatch.test-manifest.json
          -> oracle/rust-tests/tests/oracle_contracts.rs
            -> stevenarella/protocol/src/protocol/versions/v26_1_2.rs
```

| Field | Value |
|---|---|
| Case id | `play_block_changed_ack_clientbound_framed_dispatch` |
| Corridor | `Configuration -> Play` |
| Official source | `client.jar` `ClientboundBlockChangedAckPacket(int)`; `ClientboundBlockChangedAckPacket.STREAM_CODEC`; private `ClientboundBlockChangedAckPacket(FriendlyByteBuf)`; private `write(FriendlyByteBuf)`; `FriendlyByteBuf.readVarInt/writeVarInt`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(...)`; `ClientGamePacketListener.handleBlockChangedAck(ClientboundBlockChangedAckPacket)` |
| Bytecode witness | `CP="_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath "$CP" -c -p -verbose net.minecraft.network.protocol.game.ClientboundBlockChangedAckPacket net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientGamePacketListener` |
| Generated answer | `oracle/answers/775/play_block_changed_ack_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/play_block_changed_ack_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_block_changed_ack_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Failure packet | `oracle/failures/775/play_block_changed_ack_clientbound_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/play_block_changed_ack_clientbound_framed_dispatch.rust-fix-task.json` |

## Official Body Shape

The official fixture uses `ClientboundBlockChangedAckPacket.STREAM_CODEC` with
a public constructor sequence value, then re-encodes through the official Play
clientbound protocol codec. It does not require initialized Minecraft/game
state.

| Order | Field | Fixture value |
|---|---|---|
| 1 | sequence VarInt | `12345` |

The generated official frame is:

```text
04b960
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
| `0x05` | `minecraft:block_destruction` |

## Stop Boundary

This is packet framing/dispatch/decode evidence for one official sequence
fixture only. It does not prove block prediction semantics, client world
correction behavior, initialized game state, spawn readiness, world load,
render readiness, or client-load completion.

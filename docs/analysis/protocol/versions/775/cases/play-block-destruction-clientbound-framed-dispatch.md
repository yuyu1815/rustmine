# play_block_destruction_clientbound_framed_dispatch

Purpose: preserve the Protocol 775 Play clientbound
`minecraft:block_destruction` packet id/body contract as a reset-proof
packet-support slice.

```text
client.jar ClientboundBlockDestructionPacket breaker/BlockPos/progress fixture
  -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x05
    -> GameProtocols.CLIENTBOUND codec encodes/decodes official frame
      -> oracle/answers/775/play_block_destruction_clientbound_framed_dispatch.answer.jsonl
        -> oracle/test-manifests/775/play_block_destruction_clientbound_framed_dispatch.test-manifest.json
          -> oracle/rust-tests/tests/oracle_contracts.rs
            -> stevenarella/protocol/src/protocol/versions/v26_1_2.rs
```

| Field | Value |
|---|---|
| Case id | `play_block_destruction_clientbound_framed_dispatch` |
| Corridor | `Configuration -> Play` |
| Official source | `client.jar` `ClientboundBlockDestructionPacket(int, BlockPos, int)`; `ClientboundBlockDestructionPacket.STREAM_CODEC`; private `ClientboundBlockDestructionPacket(FriendlyByteBuf)`; private `write(FriendlyByteBuf)`; `FriendlyByteBuf.readVarInt/writeVarInt`; `FriendlyByteBuf.readBlockPos/writeBlockPos`; `FriendlyByteBuf.readUnsignedByte/writeByte`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(...)`; `ClientGamePacketListener.handleBlockDestruction(ClientboundBlockDestructionPacket)` |
| Bytecode witness | `CP="_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath "$CP" -c -p -verbose net.minecraft.network.protocol.game.ClientboundBlockDestructionPacket net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientGamePacketListener` |
| Generated answer | `oracle/answers/775/play_block_destruction_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/play_block_destruction_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_block_destruction_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Failure packet | `oracle/failures/775/play_block_destruction_clientbound_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/play_block_destruction_clientbound_framed_dispatch.rust-fix-task.json` |

## Official Body Shape

The official fixture uses `ClientboundBlockDestructionPacket.STREAM_CODEC` with
a public constructor breaker id, `BlockPos`, and progress value, then
re-encodes through the official Play clientbound protocol codec. It does not
require initialized Minecraft/game state.

| Order | Field | Fixture value |
|---|---|---|
| 1 | breaker id VarInt | `123` |
| 2 | block position | `x=12`, `y=64`, `z=-7` |
| 3 | progress unsigned byte | `5` |

The generated official frame is:

```text
057b0000033fffff904005
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
| `0x06` | `minecraft:block_entity_data` |

## Stop Boundary

This is packet framing/dispatch/decode evidence for one official breaker id,
block position, and progress fixture only. It does not prove block break
animation semantics, entity existence, client world state, initialized game
state, spawn readiness, world load, render readiness, or client-load
completion.

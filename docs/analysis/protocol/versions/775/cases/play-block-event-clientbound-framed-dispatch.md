# play_block_event_clientbound_framed_dispatch

Purpose: preserve the Protocol 775 Play clientbound
`minecraft:block_event` packet id/body contract as a reset-proof
packet-support slice.

```text
client.jar ClientboundBlockEventPacket BlockPos/block/event fixture
  -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x07
    -> GameProtocols.CLIENTBOUND codec encodes/decodes official frame
      -> oracle/answers/775/play_block_event_clientbound_framed_dispatch.answer.jsonl
        -> oracle/test-manifests/775/play_block_event_clientbound_framed_dispatch.test-manifest.json
          -> oracle/rust-tests/tests/oracle_contracts.rs
            -> stevenarella/protocol/src/protocol/versions/v26_1_2.rs
```

| Field | Value |
|---|---|
| Case id | `play_block_event_clientbound_framed_dispatch` |
| Corridor | `Configuration -> Play` |
| Official source | `client.jar` `ClientboundBlockEventPacket(BlockPos, Block, int, int)`; `ClientboundBlockEventPacket.STREAM_CODEC`; private `ClientboundBlockEventPacket(RegistryFriendlyByteBuf)`; private `write(RegistryFriendlyByteBuf)`; `RegistryFriendlyByteBuf.readBlockPos/writeBlockPos`; `RegistryFriendlyByteBuf.readUnsignedByte/writeByte`; `ByteBufCodecs.registry(Registries.BLOCK)`; `BuiltInRegistries.BLOCK`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.fromRegistryOfRegistries(BuiltInRegistries.REGISTRY))).codec().encode/decode(...)`; `ClientGamePacketListener.handleBlockEvent(ClientboundBlockEventPacket)` |
| Bytecode witness | `CP="_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath "$CP" -c -p -verbose net.minecraft.network.protocol.game.ClientboundBlockEventPacket net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientGamePacketListener net.minecraft.world.level.block.Block net.minecraft.world.level.block.Blocks net.minecraft.core.registries.BuiltInRegistries` |
| Generated answer | `oracle/answers/775/play_block_event_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/play_block_event_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_block_event_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Failure packet | `oracle/failures/775/play_block_event_clientbound_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/play_block_event_clientbound_framed_dispatch.rust-fix-task.json` |

## Official Body Shape

The official fixture uses `ClientboundBlockEventPacket.STREAM_CODEC` with a
public official constructor, a `BlockPos`, built-in `Blocks.NOTE_BLOCK`, event
type `1`, and event data `2`, then re-encodes through the official Play
clientbound protocol codec. It requires bootstrapped built-in registries for
the block registry codec but does not require initialized `Level`,
`BlockEntity`, or game state.

| Order | Field | Fixture value |
|---|---|---|
| 1 | block position | `x=12`, `y=64`, `z=-7` |
| 2 | event type | `1` |
| 3 | event data | `2` |
| 4 | block registry entry | `minecraft:note_block` / registry id `109` |

The generated official frame is:

```text
070000033fffff904001026d
```

## Official Table

The generated answer observes 141 Play clientbound rows. The first rows are:

| Packet id | Packet type |
|---|---|
| `0x00` | `minecraft:bundle_delimiter` |
| `0x01` | `minecraft:add_entity` |
| `0x02` | `minecraft:animate` |
| `0x03` | `minecraft:award_stats` |
| `0x04` | `minecraft:block_changed_ack` |
| `0x05` | `minecraft:block_destruction` |
| `0x06` | `minecraft:block_entity_data` |
| `0x07` | `minecraft:block_event` |
| `0x08` | `minecraft:block_update` |

## Stop Boundary

This is packet framing/dispatch/decode evidence for one official block
position, built-in note block, event type, and event data fixture only. It
does not prove block event semantics, note block behavior, world/chunk state,
initialized game state, spawn readiness, world load, render readiness, or
client-load completion.

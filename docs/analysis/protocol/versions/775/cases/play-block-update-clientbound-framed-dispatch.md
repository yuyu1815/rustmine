# play_block_update_clientbound_framed_dispatch

Purpose: preserve the Protocol 775 Play clientbound
`minecraft:block_update` packet id/body contract as a reset-proof
packet-support slice.

```text
client.jar ClientboundBlockUpdatePacket BlockPos/block-state fixture
  -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x08
    -> GameProtocols.CLIENTBOUND codec encodes/decodes official frame
      -> oracle/answers/775/play_block_update_clientbound_framed_dispatch.answer.jsonl
        -> oracle/test-manifests/775/play_block_update_clientbound_framed_dispatch.test-manifest.json
          -> oracle/rust-tests/tests/oracle_contracts.rs
            -> stevenarella/protocol/src/protocol/versions/v26_1_2.rs
```

| Field | Value |
|---|---|
| Case id | `play_block_update_clientbound_framed_dispatch` |
| Corridor | `Configuration -> Play` |
| Official source | `client.jar` `ClientboundBlockUpdatePacket(BlockPos, BlockState)`; `ClientboundBlockUpdatePacket.STREAM_CODEC`; `BlockPos.STREAM_CODEC`; `ByteBufCodecs.idMapper(Block.BLOCK_STATE_REGISTRY)`; `Blocks.STONE.defaultBlockState()`; `Block.getId(BlockState)`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(...)`; `ClientGamePacketListener.handleBlockUpdate(ClientboundBlockUpdatePacket)` |
| Bytecode witness | `CP="_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath "$CP" -c -p net.minecraft.network.protocol.game.ClientboundBlockUpdatePacket net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientGamePacketListener net.minecraft.world.level.block.state.BlockState net.minecraft.world.level.block.Blocks net.minecraft.world.level.block.Block` |
| Generated answer | `oracle/answers/775/play_block_update_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/play_block_update_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_block_update_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Failure packet | `oracle/failures/775/play_block_update_clientbound_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/play_block_update_clientbound_framed_dispatch.rust-fix-task.json` |

## Official Body Shape

The official fixture uses `ClientboundBlockUpdatePacket.STREAM_CODEC` with a
public official constructor, a `BlockPos`, and built-in
`Blocks.STONE.defaultBlockState()`, then re-encodes through the official Play
clientbound protocol codec. It requires the bootstrapped built-in block state
registry but does not require initialized `Level` or game state.

| Order | Field | Fixture value |
|---|---|---|
| 1 | block position | `x=12`, `y=64`, `z=-7` |
| 2 | block state | `minecraft:stone` / block-state registry id `1` |

The generated official frame is:

```text
080000033fffff904001
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
| `0x09` | `minecraft:boss_event` |

## Stop Boundary

This is packet framing/dispatch/decode evidence for one official block
position and built-in stone default block-state fixture only. It does not prove
world/chunk state, block update semantics, initialized game state, spawn
readiness, world load, render readiness, or client-load completion.

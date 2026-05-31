# play_block_entity_data_clientbound_framed_dispatch

Purpose: preserve the Protocol 775 Play clientbound
`minecraft:block_entity_data` packet id/body contract as a reset-proof
packet-support slice.

```text
client.jar ClientboundBlockEntityDataPacket BlockPos/type/tag fixture
  -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x06
    -> GameProtocols.CLIENTBOUND codec encodes/decodes official frame
      -> oracle/answers/775/play_block_entity_data_clientbound_framed_dispatch.answer.jsonl
        -> oracle/test-manifests/775/play_block_entity_data_clientbound_framed_dispatch.test-manifest.json
          -> oracle/rust-tests/tests/oracle_contracts.rs
            -> stevenarella/protocol/src/protocol/versions/v26_1_2.rs
```

| Field | Value |
|---|---|
| Case id | `play_block_entity_data_clientbound_framed_dispatch` |
| Corridor | `Configuration -> Play` |
| Official source | `client.jar` private `ClientboundBlockEntityDataPacket(BlockPos, BlockEntityType<?>, CompoundTag)`; `ClientboundBlockEntityDataPacket.STREAM_CODEC`; `BlockPos.STREAM_CODEC`; `ByteBufCodecs.registry(Registries.BLOCK_ENTITY_TYPE)`; `ByteBufCodecs.TRUSTED_COMPOUND_TAG`; `BuiltInRegistries.BLOCK_ENTITY_TYPE`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.fromRegistryOfRegistries(BuiltInRegistries.REGISTRY))).codec().encode/decode(...)`; `ClientGamePacketListener.handleBlockEntityData(ClientboundBlockEntityDataPacket)` |
| Bytecode witness | `CP="_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath "$CP" -c -p -verbose net.minecraft.network.protocol.game.ClientboundBlockEntityDataPacket net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientGamePacketListener net.minecraft.world.level.block.entity.BlockEntityType net.minecraft.core.registries.BuiltInRegistries net.minecraft.nbt.CompoundTag` |
| Generated answer | `oracle/answers/775/play_block_entity_data_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/play_block_entity_data_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_block_entity_data_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Failure packet | `oracle/failures/775/play_block_entity_data_clientbound_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/play_block_entity_data_clientbound_framed_dispatch.rust-fix-task.json` |

## Official Body Shape

The official fixture uses `ClientboundBlockEntityDataPacket.STREAM_CODEC` with
a private official constructor, a `BlockPos`, built-in `BlockEntityType.CHEST`,
and an empty `CompoundTag`, then re-encodes through the official Play
clientbound protocol codec. It requires bootstrapped built-in registries for
the registry codec but does not require initialized `Level`, `BlockEntity`, or
game state.

| Order | Field | Fixture value |
|---|---|---|
| 1 | block position | `x=12`, `y=64`, `z=-7` |
| 2 | block entity type registry entry | `minecraft:chest` |
| 3 | trusted compound tag | empty compound |

The generated official frame is:

```text
060000033fffff9040010a00
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

## Stop Boundary

This is packet framing/dispatch/decode evidence for one official block
position, built-in chest block entity type, and empty tag fixture only. It does
not prove block entity semantics, NBT schema, world/chunk state, initialized
game state, spawn readiness, world load, render readiness, or client-load
completion.

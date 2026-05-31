# play_chunk_batch_start_clientbound_framed_dispatch

Purpose: preserve the Protocol 775 Play clientbound
`minecraft:chunk_batch_start` packet id/body contract as a reset-proof
packet-support slice.

```text
client.jar ClientboundChunkBatchStartPacket singleton fixture
  -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x0c
    -> GameProtocols.CLIENTBOUND codec encodes/decodes official frame
      -> oracle/answers/775/play_chunk_batch_start_clientbound_framed_dispatch.answer.jsonl
        -> oracle/test-manifests/775/play_chunk_batch_start_clientbound_framed_dispatch.test-manifest.json
          -> oracle/rust-tests/tests/oracle_contracts.rs
            -> stevenarella/protocol/src/protocol/versions/v26_1_2.rs
```

| Field | Value |
|---|---|
| Case id | `play_chunk_batch_start_clientbound_framed_dispatch` |
| Corridor | `Configuration -> Play` |
| Official source | `client.jar` `ClientboundChunkBatchStartPacket.INSTANCE`; `ClientboundChunkBatchStartPacket.STREAM_CODEC`; `StreamCodec.unit(INSTANCE)`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(...)`; `ClientGamePacketListener.handleChunkBatchStart(ClientboundChunkBatchStartPacket)` |
| Bytecode witness | `CP="_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath "$CP" -c -p net.minecraft.network.protocol.game.ClientboundChunkBatchStartPacket net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientGamePacketListener` |
| Generated answer | `oracle/answers/775/play_chunk_batch_start_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/play_chunk_batch_start_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_chunk_batch_start_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Failure packet | `oracle/failures/775/play_chunk_batch_start_clientbound_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/play_chunk_batch_start_clientbound_framed_dispatch.rust-fix-task.json` |

## Official Body Shape

The official fixture uses `ClientboundChunkBatchStartPacket.INSTANCE` and
re-encodes through the official Play clientbound protocol codec. The singleton
is context-free: it does not require an initialized `Level`, chunk storage, or
game state.

| Order | Field | Fixture value |
|---|---|---|
| 1 | empty body | `StreamCodec.unit(INSTANCE)` |

The generated official frame is:

```text
0c
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
| `0x0a` | `minecraft:change_difficulty` |
| `0x0b` | `minecraft:chunk_batch_finished` |
| `0x0c` | `minecraft:chunk_batch_start` |
| `0x0d` | `minecraft:chunks_biomes` |

## Stop Boundary

This is packet framing/dispatch/decode evidence for the official singleton
empty-body fixture only. It does not prove chunk batch pacing behavior,
chunk-load start semantics, chunk-load completion, initialized game state,
spawn readiness, world load, render readiness, or client-load completion.

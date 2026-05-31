# play_clear_titles_clientbound_framed_dispatch

Purpose: preserve the Protocol 775 Play clientbound `minecraft:clear_titles`
packet id/body contract as a reset-proof packet-support slice.

```text
client.jar ClientboundClearTitlesPacket resetTimes=true fixture
  -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x0e
    -> GameProtocols.CLIENTBOUND codec encodes/decodes official frame
      -> oracle/answers/775/play_clear_titles_clientbound_framed_dispatch.answer.jsonl
        -> oracle/test-manifests/775/play_clear_titles_clientbound_framed_dispatch.test-manifest.json
          -> oracle/rust-tests/tests/oracle_contracts.rs
            -> stevenarella/protocol/src/protocol/versions/v26_1_2.rs
```

| Field | Value |
|---|---|
| Case id | `play_clear_titles_clientbound_framed_dispatch` |
| Corridor | `Configuration -> Play` |
| Official source | `client.jar` `ClientboundClearTitlesPacket(boolean)`; `ClientboundClearTitlesPacket.STREAM_CODEC`; private `ClientboundClearTitlesPacket(FriendlyByteBuf)`; private `write(FriendlyByteBuf)`; `FriendlyByteBuf.readBoolean/writeBoolean`; `ClientboundClearTitlesPacket.shouldResetTimes()`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(...)`; `ClientGamePacketListener.handleTitlesClear(ClientboundClearTitlesPacket)` |
| Bytecode witness | `CP="_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath "$CP" -c -p net.minecraft.network.protocol.game.ClientboundClearTitlesPacket net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientGamePacketListener` |
| Generated answer | `oracle/answers/775/play_clear_titles_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/play_clear_titles_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_clear_titles_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Failure packet | `oracle/failures/775/play_clear_titles_clientbound_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/play_clear_titles_clientbound_framed_dispatch.rust-fix-task.json` |

## Official Body Shape

The official fixture uses `ClientboundClearTitlesPacket(boolean)` with
`resetTimes=true`. This is context-free: it does not require initialized
client title state, screen, `Level`, or game state.

| Order | Field | Fixture value |
|---|---|---|
| 1 | `resetTimes` boolean | `true` |

The generated official frame is:

```text
0e01
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
| `0x0e` | `minecraft:clear_titles` |
| `0x0f` | `minecraft:command_suggestions` |

## Stop Boundary

This is packet framing/dispatch/decode evidence for one official
`resetTimes=true` boolean fixture only. It does not prove title UI clearing
behavior, title timing reset behavior, initialized client screen/title state,
runtime Play entry, world load, render readiness, or client-load completion.

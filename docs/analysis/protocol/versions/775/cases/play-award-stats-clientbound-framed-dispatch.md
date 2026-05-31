# play_award_stats_clientbound_framed_dispatch

Purpose: preserve the Protocol 775 Play clientbound `minecraft:award_stats`
packet id/body contract as a reset-proof packet-support slice.

```text
client.jar ClientboundAwardStatsPacket empty stats map fixture
  -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x03
    -> GameProtocols.CLIENTBOUND codec encodes/decodes official frame
      -> oracle/answers/775/play_award_stats_clientbound_framed_dispatch.answer.jsonl
        -> oracle/test-manifests/775/play_award_stats_clientbound_framed_dispatch.test-manifest.json
          -> oracle/rust-tests/tests/oracle_contracts.rs
            -> stevenarella/protocol/src/protocol/versions/v26_1_2.rs
```

| Field | Value |
|---|---|
| Case id | `play_award_stats_clientbound_framed_dispatch` |
| Corridor | `Configuration -> Play` |
| Official source | `client.jar` `ClientboundAwardStatsPacket(Object2IntMap<Stat<?>>)`; `ClientboundAwardStatsPacket.STREAM_CODEC`; `ClientboundAwardStatsPacket.STAT_VALUES_STREAM_CODEC`; `Stat.STREAM_CODEC`; `ByteBufCodecs.VAR_INT`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(...)`; `ClientGamePacketListener.handleAwardStats(ClientboundAwardStatsPacket)` |
| Bytecode witness | `CP="_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath "$CP" -c -p -verbose net.minecraft.network.protocol.game.ClientboundAwardStatsPacket net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientGamePacketListener net.minecraft.stats.Stat net.minecraft.stats.StatType` |
| Generated answer | `oracle/answers/775/play_award_stats_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/play_award_stats_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_award_stats_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Failure packet | `oracle/failures/775/play_award_stats_clientbound_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/play_award_stats_clientbound_framed_dispatch.rust-fix-task.json` |

## Official Body Shape

The official fixture uses `ClientboundAwardStatsPacket.STREAM_CODEC` with an
empty `Object2IntOpenHashMap<Stat<?>>`, then re-encodes through the official
Play clientbound protocol codec. Because the map has zero entries, it does not
require initialized Minecraft/game state or stat registry entries.

| Order | Field | Fixture value |
|---|---|---|
| 1 | stats map count VarInt | `0` |
| per entry | `Stat.STREAM_CODEC` key | none in this fixture |
| per entry | stat value VarInt | none in this fixture |

The generated official frame is:

```text
0300
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

This is packet framing/dispatch/decode evidence for one official empty-stats
fixture only. It does not prove non-empty Stat registry entry decoding, stat
semantics, UI/stat screen behavior, spawn readiness, world load, render
readiness, or client-load completion.

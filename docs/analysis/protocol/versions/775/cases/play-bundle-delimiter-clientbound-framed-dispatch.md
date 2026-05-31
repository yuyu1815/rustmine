# play_bundle_delimiter_clientbound_framed_dispatch

Purpose: preserve the Protocol 775 Play clientbound
`minecraft:bundle_delimiter` packet id/body contract as a reset-proof
packet-support slice.

```text
client.jar GameProtocols.CLIENTBOUND_TEMPLATE table
  -> official bundle_delimiter id 0x00
    -> GameProtocols.CLIENTBOUND codec decodes registered delimiter singleton
      -> GameProtocols.CLIENTBOUND codec re-encodes official frame
        -> oracle/answers/775/play_bundle_delimiter_clientbound_framed_dispatch.answer.jsonl
          -> oracle/test-manifests/775/play_bundle_delimiter_clientbound_framed_dispatch.test-manifest.json
            -> oracle/rust-tests/tests/oracle_contracts.rs
              -> stevenarella/protocol/src/protocol/versions/v26_1_2.rs
```

| Field | Value |
|---|---|
| Case id | `play_bundle_delimiter_clientbound_framed_dispatch` |
| Corridor | `Configuration -> Play` |
| Official source | `client.jar` `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().decode/encode(...)`; `ClientboundBundleDelimiterPacket.type()`; `BundleDelimiterPacket`; `ClientGamePacketListener` |
| Bytecode witness | `CP="_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath "$CP" -c -p net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket net.minecraft.network.protocol.game.ClientGamePacketListener net.minecraft.network.protocol.BundleDelimiterPacket net.minecraft.network.protocol.BundlerInfo net.minecraft.network.ProtocolInfo` |
| Generated answer | `oracle/answers/775/play_bundle_delimiter_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/play_bundle_delimiter_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_bundle_delimiter_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Failure packet | `oracle/failures/775/play_bundle_delimiter_clientbound_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/play_bundle_delimiter_clientbound_framed_dispatch.rust-fix-task.json` |

## Official Body Shape

The official `ClientboundBundleDelimiterPacket` is the Play clientbound bundle
delimiter singleton. The official framed fixture has no packet body bytes after
the VarInt packet id:

| Order | Field |
|---|---|
| none | empty body |

The harness does not encode a freshly constructed delimiter instance because
the official unit codec checks the registered singleton identity. It obtains
the registered singleton by decoding the official table id, then re-encodes
that official packet through the Play clientbound codec.

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
| `0x07` | `minecraft:block_event` |

The generated answer also observed 69 Play serverbound rows. The first rows are
`minecraft:accept_teleportation` / `0x00`, `minecraft:attack` / `0x01`,
`minecraft:block_entity_tag_query` / `0x02`,
`minecraft:bundle_item_selected` / `0x03`, and
`minecraft:change_difficulty` / `0x04`.

## Stop Boundary

This is packet framing/dispatch/decode evidence for the official Play
clientbound bundle_delimiter registered singleton only. It does not prove
bundle grouping behavior, Play state transition handling, world load, spawn
readiness, render readiness, or client-load completion.

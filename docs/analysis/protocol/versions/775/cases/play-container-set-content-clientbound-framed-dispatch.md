# play_container_set_content_clientbound_framed_dispatch

Purpose: preserve the Protocol 775 Play clientbound
`minecraft:container_set_content` packet id/body contract as a reset-proof
packet-support slice.

```text
client.jar ClientboundContainerSetContentPacket empty-items fixture
  -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x12
    -> GameProtocols.CLIENTBOUND codec encodes/decodes official frame
      -> oracle/answers/775/play_container_set_content_clientbound_framed_dispatch.answer.jsonl
        -> oracle/test-manifests/775/play_container_set_content_clientbound_framed_dispatch.test-manifest.json
          -> oracle/rust-tests/tests/oracle_contracts.rs
            -> stevenarella/protocol/src/protocol/versions/v26_1_2.rs
```

| Field | Value |
|---|---|
| Case id | `play_container_set_content_clientbound_framed_dispatch` |
| Corridor | `Configuration -> Play` |
| Official source | `client.jar` `ClientboundContainerSetContentPacket(int, int, List<ItemStack>, ItemStack)`; `ClientboundContainerSetContentPacket.STREAM_CODEC`; `ByteBufCodecs.CONTAINER_ID`; `ByteBufCodecs.VAR_INT`; `ItemStack.OPTIONAL_LIST_STREAM_CODEC`; `ItemStack.OPTIONAL_STREAM_CODEC`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(...)`; `ClientGamePacketListener.handleContainerContent(ClientboundContainerSetContentPacket)` |
| Bytecode witness | `CP="_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath "$CP" -c -p net.minecraft.network.protocol.game.ClientboundContainerSetContentPacket 'net.minecraft.world.item.ItemStack$1' net.minecraft.world.item.ItemStack net.minecraft.network.codec.ByteBufCodecs net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.ClientGamePacketListener` |
| Generated answer | `oracle/answers/775/play_container_set_content_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/play_container_set_content_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_container_set_content_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Failure packet | `oracle/failures/775/play_container_set_content_clientbound_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/play_container_set_content_clientbound_framed_dispatch.rust-fix-task.json` |

## Official Body Shape

The official fixture uses
`ClientboundContainerSetContentPacket(int, int, List<ItemStack>, ItemStack)`
with `containerId=7`, `stateId=123`, an empty item list, and
`ItemStack.EMPTY` as the carried item. This is context-free: it does not
require an initialized `Menu`, screen, client `Level`, inventory, item registry
entry, or game state.

| Order | Field | Fixture value |
|---|---|---|
| 1 | containerId via `ByteBufCodecs.CONTAINER_ID` | `7` |
| 2 | stateId via `ByteBufCodecs.VAR_INT` | `123` |
| 3 | items via `ItemStack.OPTIONAL_LIST_STREAM_CODEC` | `0` entries |
| 4 | carriedItem via `ItemStack.OPTIONAL_STREAM_CODEC` | `ItemStack.EMPTY` |

The generated official frame is:

```text
12077b0000
```

## Official Table

The generated answer observes 141 Play clientbound rows. The local packet
support route has now proven rows through:

| Packet id | Packet type |
|---|---|
| `0x11` | `minecraft:container_close` |
| `0x12` | `minecraft:container_set_content` |

The next official Play clientbound row is `minecraft:container_set_data` /
`0x13`.

## Stop Boundary

This is packet framing/dispatch/decode evidence for one official empty item
list and empty carried stack fixture only. It does not prove non-empty
ItemStack/component registry handling, menu lifecycle behavior, inventory
state, runtime Play entry, world load, render readiness, or client-load
completion.

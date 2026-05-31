# play_container_set_data_clientbound_framed_dispatch

Purpose: preserve the Protocol 775 Play clientbound
`minecraft:container_set_data` packet id/body contract as a reset-proof
packet-support slice.

```text
client.jar ClientboundContainerSetDataPacket numeric fixture
  -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x13
    -> GameProtocols.CLIENTBOUND codec encodes/decodes official frame
      -> oracle/answers/775/play_container_set_data_clientbound_framed_dispatch.answer.jsonl
        -> oracle/test-manifests/775/play_container_set_data_clientbound_framed_dispatch.test-manifest.json
          -> oracle/rust-tests/tests/oracle_contracts.rs
            -> stevenarella/protocol/src/protocol/versions/v26_1_2.rs
```

| Field | Value |
|---|---|
| Case id | `play_container_set_data_clientbound_framed_dispatch` |
| Corridor | `Configuration -> Play` |
| Official source | `client.jar` `ClientboundContainerSetDataPacket(int, int, int)`; `ClientboundContainerSetDataPacket.STREAM_CODEC`; `ClientboundContainerSetDataPacket(FriendlyByteBuf)`; `ClientboundContainerSetDataPacket.write(FriendlyByteBuf)`; `FriendlyByteBuf.readContainerId/writeContainerId`; `FriendlyByteBuf.readShort/writeShort`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(...)`; `ClientGamePacketListener.handleContainerSetData(ClientboundContainerSetDataPacket)` |
| Bytecode witness | `CP="_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath "$CP" -c -p net.minecraft.network.protocol.game.ClientboundContainerSetDataPacket net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.ClientGamePacketListener net.minecraft.network.codec.ByteBufCodecs` |
| Generated answer | `oracle/answers/775/play_container_set_data_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/play_container_set_data_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_container_set_data_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Failure packet | `oracle/failures/775/play_container_set_data_clientbound_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/play_container_set_data_clientbound_framed_dispatch.rust-fix-task.json` |

## Official Body Shape

The official fixture uses `ClientboundContainerSetDataPacket(int, int, int)`
with `containerId=7`, `id=2`, and `value=300`. This is context-free: it does
not require an initialized `Menu`, screen, client `Level`, inventory, or game
state.

| Order | Field | Fixture value |
|---|---|---|
| 1 | containerId via `FriendlyByteBuf.writeContainerId` | `7` |
| 2 | id via `FriendlyByteBuf.writeShort` | `2` |
| 3 | value via `FriendlyByteBuf.writeShort` | `300` |

The generated official frame is:

```text
13070002012c
```

## Official Table

The generated answer observes 141 Play clientbound rows. The local packet
support route has now proven rows through:

| Packet id | Packet type |
|---|---|
| `0x12` | `minecraft:container_set_content` |
| `0x13` | `minecraft:container_set_data` |

The next official Play clientbound row is `minecraft:container_set_slot` /
`0x14`.

## Stop Boundary

This is packet framing/dispatch/decode evidence for one official numeric
container id, data id, and value fixture only. It does not prove menu property
semantics, inventory state, initialized game state, runtime Play entry, world
load, render readiness, or client-load completion.

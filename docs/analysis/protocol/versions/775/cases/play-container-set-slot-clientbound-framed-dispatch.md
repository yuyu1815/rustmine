# play_container_set_slot_clientbound_framed_dispatch

Purpose: preserve the Protocol 775 Play clientbound
`minecraft:container_set_slot` packet id/body contract as a reset-proof
packet-support slice.

```text
client.jar ClientboundContainerSetSlotPacket empty ItemStack fixture
  -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x14
    -> GameProtocols.CLIENTBOUND codec encodes/decodes official frame
      -> oracle/answers/775/play_container_set_slot_clientbound_framed_dispatch.answer.jsonl
        -> oracle/test-manifests/775/play_container_set_slot_clientbound_framed_dispatch.test-manifest.json
          -> oracle/rust-tests/tests/oracle_contracts.rs
            -> stevenarella/protocol/src/protocol/versions/v26_1_2.rs
```

| Field | Value |
|---|---|
| Case id | `play_container_set_slot_clientbound_framed_dispatch` |
| Corridor | `Configuration -> Play` |
| Official source | `client.jar` `ClientboundContainerSetSlotPacket(int, int, int, ItemStack)`; `ClientboundContainerSetSlotPacket.STREAM_CODEC`; `ClientboundContainerSetSlotPacket(RegistryFriendlyByteBuf)`; `ClientboundContainerSetSlotPacket.write(RegistryFriendlyByteBuf)`; `RegistryFriendlyByteBuf.readContainerId/writeContainerId`; `RegistryFriendlyByteBuf.readVarInt/writeVarInt`; `RegistryFriendlyByteBuf.readShort/writeShort`; `ItemStack.OPTIONAL_STREAM_CODEC`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(...)`; `ClientGamePacketListener.handleContainerSetSlot(ClientboundContainerSetSlotPacket)` |
| Bytecode witness | `CP="_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath "$CP" -c -p net.minecraft.network.protocol.game.ClientboundContainerSetSlotPacket net.minecraft.world.item.ItemStack net.minecraft.network.codec.ByteBufCodecs net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.ClientGamePacketListener` |
| Generated answer | `oracle/answers/775/play_container_set_slot_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/play_container_set_slot_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_container_set_slot_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Failure packet | `oracle/failures/775/play_container_set_slot_clientbound_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/play_container_set_slot_clientbound_framed_dispatch.rust-fix-task.json` |

## Official Body Shape

The official fixture uses `ClientboundContainerSetSlotPacket(int, int, int,
ItemStack)` with `containerId=7`, `stateId=123`, `slot=4`, and
`ItemStack.EMPTY`. This is context-free: it does not require an initialized
`Menu`, screen, client `Level`, inventory, item registry entry, component
registry, or game state.

| Order | Field | Fixture value |
|---|---|---|
| 1 | containerId via `RegistryFriendlyByteBuf.writeContainerId` | `7` |
| 2 | stateId via `RegistryFriendlyByteBuf.writeVarInt` | `123` |
| 3 | slot via `RegistryFriendlyByteBuf.writeShort` | `4` |
| 4 | itemStack via `ItemStack.OPTIONAL_STREAM_CODEC` | empty marker |

The generated official frame is:

```text
14077b000400
```

## Official Table

The generated answer observes 141 Play clientbound rows. The local packet
support route has now proven rows through:

| Packet id | Packet type |
|---|---|
| `0x13` | `minecraft:container_set_data` |
| `0x14` | `minecraft:container_set_slot` |

The next official Play clientbound row is `minecraft:cookie_request` / `0x15`.

## Stop Boundary

This is packet framing/dispatch/decode evidence for one official empty
ItemStack fixture only. It does not prove non-empty ItemStack/component
registry handling, menu lifecycle behavior, inventory state, initialized game
state, runtime Play entry, world load, render readiness, or client-load
completion.

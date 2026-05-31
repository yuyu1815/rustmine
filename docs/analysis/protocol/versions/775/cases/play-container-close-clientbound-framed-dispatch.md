# play_container_close_clientbound_framed_dispatch

Purpose: preserve the Protocol 775 Play clientbound `minecraft:container_close`
packet id/body contract as a reset-proof packet-support slice.

```text
client.jar ClientboundContainerClosePacket container id fixture
  -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x11
    -> GameProtocols.CLIENTBOUND codec encodes/decodes official frame
      -> oracle/answers/775/play_container_close_clientbound_framed_dispatch.answer.jsonl
        -> oracle/test-manifests/775/play_container_close_clientbound_framed_dispatch.test-manifest.json
          -> oracle/rust-tests/tests/oracle_contracts.rs
            -> stevenarella/protocol/src/protocol/versions/v26_1_2.rs
```

| Field | Value |
|---|---|
| Case id | `play_container_close_clientbound_framed_dispatch` |
| Corridor | `Configuration -> Play` |
| Official source | `client.jar` `ClientboundContainerClosePacket(int)`; `ClientboundContainerClosePacket.STREAM_CODEC`; `ClientboundContainerClosePacket(FriendlyByteBuf)`; `ClientboundContainerClosePacket.write(FriendlyByteBuf)`; `FriendlyByteBuf.readContainerId/writeContainerId`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(...)`; `ClientGamePacketListener.handleContainerClose(ClientboundContainerClosePacket)` |
| Bytecode witness | `CP="_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath "$CP" -c -p net.minecraft.network.protocol.game.ClientboundContainerClosePacket net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.ClientGamePacketListener` |
| Generated answer | `oracle/answers/775/play_container_close_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/play_container_close_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_container_close_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Failure packet | `oracle/failures/775/play_container_close_clientbound_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/play_container_close_clientbound_framed_dispatch.rust-fix-task.json` |

## Official Body Shape

The official fixture uses `ClientboundContainerClosePacket(int)` with
`containerId=7`. This is context-free: it does not require an initialized
`Menu`, screen, client `Level`, or game state.

| Order | Field | Fixture value |
|---|---|---|
| 1 | containerId via `FriendlyByteBuf.writeContainerId` | `7` |

The generated official frame is:

```text
1107
```

## Official Table

The generated answer observes 141 Play clientbound rows. The local packet
support route has now proven rows through:

| Packet id | Packet type |
|---|---|
| `0x10` | `minecraft:commands` |
| `0x11` | `minecraft:container_close` |
| `0x12` | `minecraft:container_set_content` |

## Stop Boundary

This is packet framing/dispatch/decode evidence for one official container id
fixture only. It does not prove menu lifecycle behavior, screen close behavior,
inventory state, runtime Play entry, world load, render readiness, or
client-load completion.

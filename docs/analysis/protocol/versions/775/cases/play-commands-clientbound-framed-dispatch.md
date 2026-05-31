# play_commands_clientbound_framed_dispatch

Purpose: preserve the Protocol 775 Play clientbound `minecraft:commands`
packet id/body contract as a reset-proof packet-support slice.

```text
client.jar ClientboundCommandsPacket root-only fixture
  -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x10
    -> GameProtocols.CLIENTBOUND codec encodes/decodes official frame
      -> oracle/answers/775/play_commands_clientbound_framed_dispatch.answer.jsonl
        -> oracle/test-manifests/775/play_commands_clientbound_framed_dispatch.test-manifest.json
          -> oracle/rust-tests/tests/oracle_contracts.rs
            -> stevenarella/protocol/src/protocol/versions/v26_1_2.rs
```

| Field | Value |
|---|---|
| Case id | `play_commands_clientbound_framed_dispatch` |
| Corridor | `Configuration -> Play` |
| Official source | `client.jar` `ClientboundCommandsPacket(RootCommandNode<S>, NodeInspector<S>)`; `ClientboundCommandsPacket.STREAM_CODEC`; `ClientboundCommandsPacket(FriendlyByteBuf)`; `ClientboundCommandsPacket.write(FriendlyByteBuf)`; `Entry.write(FriendlyByteBuf)`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(...)`; `ClientGamePacketListener.handleCommands(ClientboundCommandsPacket)` |
| Bytecode witness | `CP="_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath "$CP" -c -p net.minecraft.network.protocol.game.ClientboundCommandsPacket 'net.minecraft.network.protocol.game.ClientboundCommandsPacket$Entry' 'net.minecraft.network.protocol.game.ClientboundCommandsPacket$NodeInspector' 'net.minecraft.network.protocol.game.ClientboundCommandsPacket$NodeStub' com.mojang.brigadier.CommandDispatcher com.mojang.brigadier.tree.RootCommandNode net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientGamePacketListener` |
| Generated answer | `oracle/answers/775/play_commands_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/play_commands_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_commands_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Failure packet | `oracle/failures/775/play_commands_clientbound_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/play_commands_clientbound_framed_dispatch.rust-fix-task.json` |

## Official Body Shape

The official fixture uses `ClientboundCommandsPacket(RootCommandNode<S>,
NodeInspector<S>)` with an empty Brigadier `CommandDispatcher` root. This is
context-free: it does not require argument nodes, an argument registry payload,
command context, client `Level`, or initialized game state.

| Order | Field | Fixture value |
|---|---|---|
| 1 | node count VarInt | `1` |
| 2 | root node flags byte | `0` |
| 3 | root child-index array VarInt length | `0` |
| 4 | root index VarInt | `0` |

The generated official frame is:

```text
1001000000
```

## Official Table

The generated answer observes 141 Play clientbound rows. The local packet
support route has now proven rows through:

| Packet id | Packet type |
|---|---|
| `0x0f` | `minecraft:command_suggestions` |
| `0x10` | `minecraft:commands` |
| `0x11` | `minecraft:container_close` |

## Stop Boundary

This is packet framing/dispatch/decode evidence for one official empty
root-only command tree fixture only. It does not prove literal/argument node
payloads, redirects, custom suggestions, restricted flags, Brigadier command
semantics, command context behavior, command UI behavior, runtime Play entry,
world load, render readiness, or client-load completion.

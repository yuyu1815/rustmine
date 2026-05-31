# play_command_suggestions_clientbound_framed_dispatch

Purpose: preserve the Protocol 775 Play clientbound
`minecraft:command_suggestions` packet id/body contract as a reset-proof
packet-support slice.

```text
client.jar ClientboundCommandSuggestionsPacket empty-suggestions fixture
  -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x0f
    -> GameProtocols.CLIENTBOUND codec encodes/decodes official frame
      -> oracle/answers/775/play_command_suggestions_clientbound_framed_dispatch.answer.jsonl
        -> oracle/test-manifests/775/play_command_suggestions_clientbound_framed_dispatch.test-manifest.json
          -> oracle/rust-tests/tests/oracle_contracts.rs
            -> stevenarella/protocol/src/protocol/versions/v26_1_2.rs
```

| Field | Value |
|---|---|
| Case id | `play_command_suggestions_clientbound_framed_dispatch` |
| Corridor | `Configuration -> Play` |
| Official source | `client.jar` `ClientboundCommandSuggestionsPacket(int, Suggestions)`; `ClientboundCommandSuggestionsPacket.STREAM_CODEC`; `ClientboundCommandSuggestionsPacket(int, int, int, List<Entry>)`; `ClientboundCommandSuggestionsPacket.toSuggestions()`; `ClientboundCommandSuggestionsPacket.Entry.STREAM_CODEC`; `ByteBufCodecs.VAR_INT`; `ByteBufCodecs.STRING_UTF8`; `ComponentSerialization.TRUSTED_OPTIONAL_STREAM_CODEC`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(...)`; `ClientGamePacketListener.handleCommandSuggestions(ClientboundCommandSuggestionsPacket)` |
| Bytecode witness | `CP="_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath "$CP" -c -p net.minecraft.network.protocol.game.ClientboundCommandSuggestionsPacket 'net.minecraft.network.protocol.game.ClientboundCommandSuggestionsPacket$Entry' com.mojang.brigadier.suggestion.Suggestions com.mojang.brigadier.context.StringRange net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientGamePacketListener` |
| Generated answer | `oracle/answers/775/play_command_suggestions_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/play_command_suggestions_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_command_suggestions_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Failure packet | `oracle/failures/775/play_command_suggestions_clientbound_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/play_command_suggestions_clientbound_framed_dispatch.rust-fix-task.json` |

## Official Body Shape

The official fixture uses `ClientboundCommandSuggestionsPacket(int, Suggestions)`
with command id `123`, `StringRange.between(1, 4)`, and an empty suggestion
list. This is context-free: it does not require an initialized command tree,
command context, client `Level`, or game state.

| Order | Field | Fixture value |
|---|---|---|
| 1 | command id VarInt | `123` |
| 2 | range start VarInt | `1` |
| 3 | range length VarInt | `3` |
| 4 | suggestion count VarInt | `0` |
| 5 | Entry records | none |

For non-empty lists, the official `Entry` shape is text `STRING_UTF8` plus an
optional trusted `Component` tooltip. This fixture has zero entries, so it does
not prove Entry text/tooltip handling.

The generated official frame is:

```text
0f7b010300
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
| `0x10` | `minecraft:commands` |

## Stop Boundary

This is packet framing/dispatch/decode evidence for one official
command id/range/empty-suggestions fixture only. It does not prove non-empty
suggestion Entry text/tooltip handling, Brigadier command tree semantics,
command context behavior, command UI behavior, runtime Play entry, world load,
render readiness, or client-load completion.

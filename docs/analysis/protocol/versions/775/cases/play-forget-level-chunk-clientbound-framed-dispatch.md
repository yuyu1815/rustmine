# play_forget_level_chunk_clientbound_framed_dispatch

Purpose: preserve the Protocol 775 Play clientbound
`minecraft:forget_level_chunk` packet id/body contract as a reset-proof
packet-support slice.

```text
client.jar ChunkPos(12, -7)
  -> ClientboundForgetLevelChunkPacket(ChunkPos)
    -> ClientboundForgetLevelChunkPacket.STREAM_CODEC
      -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x25
        -> GameProtocols.CLIENTBOUND codec encodes/decodes official frame
          -> oracle/answers/775/play_forget_level_chunk_clientbound_framed_dispatch.answer.jsonl
            -> oracle/test-manifests/775/play_forget_level_chunk_clientbound_framed_dispatch.test-manifest.json
              -> oracle/rust-tests/tests/oracle_contracts.rs
                -> stevenarella/protocol/src/protocol/versions/v26_1_2.rs
```

| Field | Value |
|---|---|
| Case id | `play_forget_level_chunk_clientbound_framed_dispatch` |
| Corridor | `Configuration -> Play` |
| Official source | `client.jar` `ChunkPos(int, int)`; `ClientboundForgetLevelChunkPacket(ChunkPos)`; `ClientboundForgetLevelChunkPacket.STREAM_CODEC`; `FriendlyByteBuf.readChunkPos/writeChunkPos`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundForgetLevelChunkPacket)` |
| Generated answer | `oracle/answers/775/play_forget_level_chunk_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/play_forget_level_chunk_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_forget_level_chunk_clientbound_framed_dispatch_matches_official_oracle_answer` |

## Official Body Shape

| Order | Field | Fixture value |
|---|---|---|
| 1 | `pos` via `FriendlyByteBuf.writeChunkPos` | `x=12`, `z=-7` |

The generated official frame is:

```text
25fffffff90000000c
```

## Stop Boundary

This is packet framing/dispatch/decode evidence for one official
`ClientboundForgetLevelChunkPacket` fixture only. It does not prove chunk
unload behavior, client world state, world load, render readiness, or
client-load completion.

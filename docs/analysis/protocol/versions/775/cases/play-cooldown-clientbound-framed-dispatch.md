# play_cooldown_clientbound_framed_dispatch

Purpose: preserve the Protocol 775 Play clientbound `minecraft:cooldown`
packet id/body contract as a reset-proof packet-support slice.

```text
client.jar Identifier.parse("a:a")
  -> ClientboundCooldownPacket(Identifier, int)
    -> ClientboundCooldownPacket.STREAM_CODEC
      -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x16
        -> GameProtocols.CLIENTBOUND codec encodes/decodes official frame
          -> oracle/answers/775/play_cooldown_clientbound_framed_dispatch.answer.jsonl
            -> oracle/test-manifests/775/play_cooldown_clientbound_framed_dispatch.test-manifest.json
              -> oracle/rust-tests/tests/oracle_contracts.rs
                -> stevenarella/protocol/src/protocol/versions/v26_1_2.rs
```

| Field | Value |
|---|---|
| Case id | `play_cooldown_clientbound_framed_dispatch` |
| Corridor | `Configuration -> Play` |
| Official source | `client.jar` `Identifier.parse(String)`; `ClientboundCooldownPacket(Identifier, int)`; `ClientboundCooldownPacket.STREAM_CODEC`; `Identifier.STREAM_CODEC`; `ByteBufCodecs.VAR_INT`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundCooldownPacket)` |
| Generated answer | `oracle/answers/775/play_cooldown_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/play_cooldown_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_cooldown_clientbound_framed_dispatch_matches_official_oracle_answer` |

## Official Body Shape

| Order | Field | Fixture value |
|---|---|---|
| 1 | `cooldownGroup` via `Identifier.STREAM_CODEC` | `a:a` |
| 2 | `duration` via `ByteBufCodecs.VAR_INT` | `123` |

The generated official frame is:

```text
1603613a617b
```

## Stop Boundary

This is packet framing/dispatch/decode evidence for one official
`ClientboundCooldownPacket` fixture only. It does not prove item cooldown
semantics, item registry contents, UI cooldown behavior, runtime Play entry,
world load, spawn readiness, render readiness, or client-load completion.

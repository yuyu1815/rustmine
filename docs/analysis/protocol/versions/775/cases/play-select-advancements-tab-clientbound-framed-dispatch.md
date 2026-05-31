# play_select_advancements_tab_clientbound_framed_dispatch

## Spatial Map

```text
client.jar ClientboundSelectAdvancementsTabPacket(Identifier)
  -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x55
    -> oracle/answers/775/play_select_advancements_tab_clientbound_framed_dispatch.answer.jsonl
      -> oracle/rust-tests/tests/oracle_contracts.rs exact test
```

| Field | Value |
|---|---|
| Packet | `minecraft:select_advancements_tab` / `0x55` |
| Official class | `net.minecraft.network.protocol.game.ClientboundSelectAdvancementsTabPacket` |
| Official body | nullable Identifier |
| Fixture | non-null Identifier `minecraft:story/root` |
| Answer | `oracle/answers/775/play_select_advancements_tab_clientbound_framed_dispatch.answer.jsonl` |
| Rust proof | `oracle/rust-tests/tests/oracle_contracts.rs::play_select_advancements_tab_clientbound_framed_dispatch_matches_official_oracle_answer` |

## Stop Boundary

This is packet-support evidence only. It proves official Play clientbound
framing, body bytes, dispatch, and full body consumption for one non-null
Identifier fixture. It does not prove advancement tree state, UI behavior,
Play entry, render readiness, or client-load completion.

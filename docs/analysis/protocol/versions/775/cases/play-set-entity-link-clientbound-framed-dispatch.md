# play_set_entity_link_clientbound_framed_dispatch

Purpose: prove Protocol 775 Play clientbound `minecraft:set_entity_link`
framed dispatch/decode for one initialized official GameTest entity fixture.

```text
client.jar GameTestMainUtil server
  -> GameTestHelper.spawn(PIG) + spawn(ARMOR_STAND) in ServerLevel
    -> ClientboundSetEntityLinkPacket(Entity, Entity)
      -> GameProtocols.CLIENTBOUND_TEMPLATE assigns 0x64
        -> oracle answer frame/body
          -> Stevenarella packet_by_id Play Clientbound
```

| Field | Value |
|---|---|
| Case id | `play_set_entity_link_clientbound_framed_dispatch` |
| Official source | `client.jar` `GameTestMainUtil.runGameTestServer(...)`; `TestFunctionLoader.registerLoader(...)`; `GameTestHelper.spawn(EntityType.PIG/ARMOR_STAND, Vec3)`; `ClientboundSetEntityLinkPacket(Entity, Entity)`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)` |
| Generated answer | `oracle/answers/775/play_set_entity_link_clientbound_framed_dispatch.answer.jsonl` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_set_entity_link_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Internal owner | `stevenarella/protocol/src/protocol/versions/v26_1_2.rs` Play clientbound `0x64` mapping |

Stop boundary: this is packet framing and body-shape evidence only for source
pig id `1` and destination armor stand id `2` from the official GameTest
fixture. Other entity ids are rejected before broader entity, leash, world, or
client-load semantics.

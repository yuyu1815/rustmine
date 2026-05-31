# play_set_passengers_clientbound_framed_dispatch

Purpose: prove Protocol 775 Play clientbound `minecraft:set_passengers`
framed dispatch/decode for one initialized official GameTest relationship
fixture.

```text
client.jar GameTestMainUtil server
  -> GameTestHelper.spawn(MINECART) + spawn(PIG) in ServerLevel
    -> passenger.startRiding(vehicle)
      -> ClientboundSetPassengersPacket(Entity)
        -> GameProtocols.CLIENTBOUND_TEMPLATE assigns 0x6b
          -> Stevenarella packet_by_id Play Clientbound
```

| Field | Value |
|---|---|
| Case id | `play_set_passengers_clientbound_framed_dispatch` |
| Official source | `client.jar` `GameTestMainUtil.runGameTestServer(...)`; `TestFunctionLoader.registerLoader(...)`; `GameTestHelper.spawn(EntityType.MINECART/PIG, Vec3)`; `Entity.startRiding(Entity)`; `ClientboundSetPassengersPacket(Entity)`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)` |
| Generated answer | `oracle/answers/775/play_set_passengers_clientbound_framed_dispatch.answer.jsonl` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_set_passengers_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Internal owner | `stevenarella/protocol/src/protocol/versions/v26_1_2.rs` Play clientbound `0x6b` mapping |

Stop boundary: this is packet framing and body-shape evidence only for
minecart vehicle id `3` with one passenger pig id `4`. Other passenger
topologies are rejected before broader entity graph, world, or client-load
semantics.

# play_add_entity_clientbound_framed_dispatch

Purpose: preserve the Protocol 775 Play clientbound `minecraft:add_entity`
packet id/body contract as a reset-proof packet-support slice.

```text
client.jar ClientboundAddEntityPacket value constructor
  -> bootstrapped built-in EntityType.PIG and Vec3.ZERO fixture
    -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x01
      -> GameProtocols.CLIENTBOUND codec encodes/decodes official frame
        -> oracle/answers/775/play_add_entity_clientbound_framed_dispatch.answer.jsonl
          -> oracle/test-manifests/775/play_add_entity_clientbound_framed_dispatch.test-manifest.json
            -> oracle/rust-tests/tests/oracle_contracts.rs
              -> stevenarella/protocol/src/protocol/versions/v26_1_2.rs
```

| Field | Value |
|---|---|
| Case id | `play_add_entity_clientbound_framed_dispatch` |
| Corridor | `Configuration -> Play` |
| Official source | `client.jar` `ClientboundAddEntityPacket(int, UUID, double, double, double, float, float, EntityType<?>, int, Vec3, double)`; `ClientboundAddEntityPacket.STREAM_CODEC`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.fromRegistryOfRegistries(BuiltInRegistries.REGISTRY))).codec().encode/decode(...)`; `EntityType.PIG`; `BuiltInRegistries.ENTITY_TYPE`; `Vec3.LP_STREAM_CODEC`; `ClientGamePacketListener.handleAddEntity(ClientboundAddEntityPacket)` |
| Bytecode witness | `CP="_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath "$CP" -c -p net.minecraft.network.protocol.game.ClientboundAddEntityPacket net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientGamePacketListener net.minecraft.core.RegistryAccess net.minecraft.core.registries.BuiltInRegistries net.minecraft.world.entity.EntityType net.minecraft.world.phys.Vec3` |
| Generated answer | `oracle/answers/775/play_add_entity_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/play_add_entity_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_add_entity_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Failure packet | `oracle/failures/775/play_add_entity_clientbound_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/play_add_entity_clientbound_framed_dispatch.rust-fix-task.json` |

## Official Body Shape

The official fixture uses the public value constructor, bootstrapped built-in
`EntityType.PIG`, and `Vec3.ZERO`, so it does not require initialized world,
`Entity`, or `ServerEntity` state.

| Order | Field | Fixture value |
|---|---|---|
| 1 | entity id VarInt | `123` |
| 2 | UUID | `00000000-0000-0000-0000-000000000001` |
| 3 | entity type registry id | `100` / `minecraft:pig` |
| 4 | x double | `1.25` |
| 5 | y double | `64.0` |
| 6 | z double | `-2.5` |
| 7 | `Vec3.LP_STREAM_CODEC` movement | `00` for `Vec3.ZERO` |
| 8 | x rotation byte | `32` |
| 9 | y rotation byte | `64` |
| 10 | y-head rotation byte | `-128` |
| 11 | data VarInt | `7` |

The generated official frame is:

```text
017b00000000000000000000000000000001643ff40000000000004050000000000000c0040000000000000020408007
```

## Official Table

The generated answer observed 141 Play clientbound rows. The first rows are:

| Packet id | Packet type |
|---|---|
| `0x00` | `minecraft:bundle_delimiter` |
| `0x01` | `minecraft:add_entity` |
| `0x02` | `minecraft:animate` |
| `0x03` | `minecraft:award_stats` |
| `0x04` | `minecraft:block_changed_ack` |

## Stop Boundary

This is packet framing/dispatch/decode evidence for one direct official
constructor fixture with built-in `EntityType.PIG` and zero movement only. It
does not prove arbitrary entity registry contents, initialized
`Entity`/`ServerEntity` behavior, spawn readiness, world load, render readiness,
or client-load completion.

# login_finished_clientbound_framed_dispatch

Purpose: preserve the Protocol 775 Login clientbound
`minecraft:login_finished` packet id/body contract as a reset-proof
packet-support slice.

```text
client.jar ClientboundLoginFinishedPacket(GameProfile)
  -> ClientboundLoginFinishedPacket.STREAM_CODEC
    -> ByteBufCodecs.GAME_PROFILE
      -> LoginProtocols.CLIENTBOUND codec
        -> LoginProtocols.CLIENTBOUND_TEMPLATE table
          -> oracle/answers/775/login_finished_clientbound_framed_dispatch.answer.jsonl
            -> oracle/test-manifests/775/login_finished_clientbound_framed_dispatch.test-manifest.json
              -> oracle/rust-tests/tests/oracle_contracts.rs
                -> stevenarella/protocol/src/protocol/versions/v26_1_2.rs
```

| Field | Value |
|---|---|
| Case id | `login_finished_clientbound_framed_dispatch` |
| Corridor | `Handshake -> Login -> Configuration` |
| Official source | `client.jar` `GameProfile(UUID, String)`; `ClientboundLoginFinishedPacket(GameProfile)`; `ClientboundLoginFinishedPacket.STREAM_CODEC`; `ByteBufCodecs.GAME_PROFILE`; `LoginProtocols.CLIENTBOUND.codec().encode/decode(ClientboundLoginFinishedPacket)`; `LoginProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `ClientboundLoginFinishedPacket.gameProfile()`; `ClientboundLoginFinishedPacket.isTerminal()`; `ClientLoginPacketListener.handleLoginFinished(ClientboundLoginFinishedPacket)` |
| Bytecode witness | `CP="_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath "$CP" -c -p net.minecraft.network.protocol.login.LoginProtocols net.minecraft.network.protocol.login.LoginPacketTypes net.minecraft.network.protocol.login.ClientboundLoginFinishedPacket net.minecraft.network.protocol.login.ClientLoginPacketListener net.minecraft.network.codec.ByteBufCodecs 'net.minecraft.network.codec.ByteBufCodecs$32' com.mojang.authlib.GameProfile com.mojang.authlib.properties.PropertyMap` |
| Generated answer | `oracle/answers/775/login_finished_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/login_finished_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::login_finished_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Failure packet | `oracle/failures/775/login_finished_clientbound_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/login_finished_clientbound_framed_dispatch.rust-fix-task.json` |

## Official Body Shape

The official `ClientboundLoginFinishedPacket` bytecode writes:

| Order | Field |
|---|---|
| 1 | `GameProfile.id` as UUID |
| 2 | `GameProfile.name` as `PLAYER_NAME` / `stringUtf8(16)` |
| 3 | `GameProfile.properties` as property count plus property entries |

This fixture uses `GameProfile(UUID, String)`, which official authlib bytecode
delegates to `PropertyMap.EMPTY`, so the generated body has property count
`0`.

## Official Table

The generated answer observed this Login clientbound table:

| Packet id | Packet type |
|---|---|
| `0x00` | `minecraft:login_disconnect` |
| `0x01` | `minecraft:hello` |
| `0x02` | `minecraft:login_finished` |
| `0x03` | `minecraft:login_compression` |
| `0x04` | `minecraft:custom_query` |
| `0x05` | `minecraft:cookie_request` |

## Stop Boundary

This is packet framing/dispatch/decode evidence for one official Login
clientbound login_finished fixture with a zero UUID, empty profile name, and
empty profile properties only. It does not prove authentication success,
Login-to-Configuration state transition handling, profile property semantics,
skin/session trust, Configuration entry, Play readiness, or client-load
completion.

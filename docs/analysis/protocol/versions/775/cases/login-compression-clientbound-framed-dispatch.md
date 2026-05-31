# login_compression_clientbound_framed_dispatch

Purpose: preserve the Protocol 775 Login clientbound
`minecraft:login_compression` packet id/body contract as a reset-proof
packet-support slice.

```text
client.jar ClientboundLoginCompressionPacket(int)
  -> ClientboundLoginCompressionPacket.STREAM_CODEC
    -> FriendlyByteBuf.writeVarInt/readVarInt
      -> LoginProtocols.CLIENTBOUND codec
        -> LoginProtocols.CLIENTBOUND_TEMPLATE table
          -> oracle/answers/775/login_compression_clientbound_framed_dispatch.answer.jsonl
            -> oracle/test-manifests/775/login_compression_clientbound_framed_dispatch.test-manifest.json
              -> oracle/rust-tests/tests/oracle_contracts.rs
                -> stevenarella/protocol/src/protocol/versions/v26_1_2.rs
```

| Field | Value |
|---|---|
| Case id | `login_compression_clientbound_framed_dispatch` |
| Corridor | `Handshake -> Login -> Configuration` |
| Official source | `client.jar` `ClientboundLoginCompressionPacket(int)`; `ClientboundLoginCompressionPacket.STREAM_CODEC`; `FriendlyByteBuf.readVarInt/writeVarInt`; `LoginProtocols.CLIENTBOUND.codec().encode/decode(ClientboundLoginCompressionPacket)`; `LoginProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `ClientboundLoginCompressionPacket.getCompressionThreshold()`; `ClientLoginPacketListener.handleCompression(ClientboundLoginCompressionPacket)` |
| Bytecode witness | `CP="_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath "$CP" -c -p net.minecraft.network.protocol.login.LoginProtocols net.minecraft.network.protocol.login.LoginPacketTypes net.minecraft.network.protocol.login.ClientboundLoginCompressionPacket net.minecraft.network.protocol.login.ClientLoginPacketListener` |
| Generated answer | `oracle/answers/775/login_compression_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/login_compression_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::login_compression_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Failure packet | `oracle/failures/775/login_compression_clientbound_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/login_compression_clientbound_framed_dispatch.rust-fix-task.json` |

## Official Body Shape

The official `ClientboundLoginCompressionPacket` bytecode writes:

| Order | Field |
|---|---|
| 1 | `compressionThreshold` as `FriendlyByteBuf.writeVarInt(int)` |

This fixture uses threshold `0`, the smallest integer accepted by the official
constructor and VarInt codec without initialized Minecraft state.

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
clientbound login_compression fixture with `compressionThreshold=0` only. It
does not prove compression negotiation policy, connection compression
activation, Login-to-Configuration state transition handling, Configuration
entry, Play readiness, or client-load completion.

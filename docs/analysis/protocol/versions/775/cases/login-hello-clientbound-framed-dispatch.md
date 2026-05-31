# login_hello_clientbound_framed_dispatch

Purpose: preserve the Protocol 775 Login clientbound `minecraft:hello`
packet id/body contract as a reset-proof packet-support slice.

```text
client.jar ClientboundHelloPacket(String, byte[], byte[], boolean)
  -> ClientboundHelloPacket.STREAM_CODEC
    -> LoginProtocols.CLIENTBOUND codec
      -> LoginProtocols.CLIENTBOUND_TEMPLATE table
        -> oracle/answers/775/login_hello_clientbound_framed_dispatch.answer.jsonl
          -> oracle/test-manifests/775/login_hello_clientbound_framed_dispatch.test-manifest.json
            -> oracle/rust-tests/tests/oracle_contracts.rs
              -> stevenarella/protocol/src/protocol/versions/v26_1_2.rs
```

| Field | Value |
|---|---|
| Case id | `login_hello_clientbound_framed_dispatch` |
| Corridor | `Handshake -> Login -> Configuration` |
| Official source | `client.jar` `ClientboundHelloPacket(String, byte[], byte[], boolean)`; `ClientboundHelloPacket.STREAM_CODEC`; `LoginProtocols.CLIENTBOUND.codec().encode/decode(ClientboundHelloPacket)`; `LoginProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `ClientboundHelloPacket.getServerId()`; `ClientboundHelloPacket.getChallenge()`; `ClientboundHelloPacket.shouldAuthenticate()`; private `publicKey` field; `ClientLoginPacketListener.handleHello(ClientboundHelloPacket)` |
| Bytecode witness | `_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.login.LoginProtocols net.minecraft.network.protocol.login.LoginPacketTypes net.minecraft.network.protocol.login.ClientboundHelloPacket net.minecraft.network.protocol.login.ClientLoginPacketListener` |
| Generated answer | `oracle/answers/775/login_hello_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/login_hello_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::login_hello_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Failure packet | `oracle/failures/775/login_hello_clientbound_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/login_hello_clientbound_framed_dispatch.rust-fix-task.json` |

## Official Body Shape

The official `ClientboundHelloPacket` bytecode writes:

| Order | Field |
|---|---|
| 1 | `serverId` as String |
| 2 | `publicKey` as VarInt-prefixed byte array |
| 3 | `challenge` as VarInt-prefixed byte array |
| 4 | `shouldAuthenticate` as boolean |

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
clientbound hello fixture with empty `serverId`, empty `publicKey`, empty
`challenge`, and `shouldAuthenticate=false` only. It does not prove encryption
success, authentication success, key validation, login state transition
handling, Configuration entry, Play readiness, or client-load completion.

# login_custom_query_clientbound_framed_dispatch

Purpose: preserve the Protocol 775 Login clientbound
`minecraft:custom_query` packet id/body contract as a reset-proof
packet-support slice.

```text
client.jar Identifier.parse("a:a")
  -> DiscardedQueryPayload(Identifier)
    -> ClientboundCustomQueryPacket(int, CustomQueryPayload)
      -> ClientboundCustomQueryPacket.STREAM_CODEC
        -> LoginProtocols.CLIENTBOUND codec
          -> LoginProtocols.CLIENTBOUND_TEMPLATE table
            -> oracle/answers/775/login_custom_query_clientbound_framed_dispatch.answer.jsonl
              -> oracle/test-manifests/775/login_custom_query_clientbound_framed_dispatch.test-manifest.json
                -> oracle/rust-tests/tests/oracle_contracts.rs
                  -> stevenarella/protocol/src/protocol/versions/v26_1_2.rs
```

| Field | Value |
|---|---|
| Case id | `login_custom_query_clientbound_framed_dispatch` |
| Corridor | `Handshake -> Login -> Configuration` |
| Official source | `client.jar` `Identifier.parse(String)`; `DiscardedQueryPayload(Identifier)`; `ClientboundCustomQueryPacket(int, CustomQueryPayload)`; `ClientboundCustomQueryPacket.STREAM_CODEC`; `FriendlyByteBuf.readVarInt/writeVarInt`; `FriendlyByteBuf.readIdentifier/writeIdentifier`; `LoginProtocols.CLIENTBOUND.codec().encode/decode(ClientboundCustomQueryPacket)`; `LoginProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `ClientboundCustomQueryPacket.transactionId()`; `ClientboundCustomQueryPacket.payload()`; `DiscardedQueryPayload.id()`; `CustomQueryPayload.write(FriendlyByteBuf)`; `ClientLoginPacketListener.handleCustomQuery(ClientboundCustomQueryPacket)` |
| Bytecode witness | `CP="_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath "$CP" -c -p net.minecraft.network.protocol.login.ClientboundCustomQueryPacket net.minecraft.network.protocol.login.LoginProtocols net.minecraft.network.protocol.login.LoginPacketTypes net.minecraft.network.protocol.login.ClientLoginPacketListener net.minecraft.network.protocol.login.custom.CustomQueryPayload net.minecraft.network.protocol.login.custom.DiscardedQueryPayload` |
| Generated answer | `oracle/answers/775/login_custom_query_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/login_custom_query_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::login_custom_query_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Failure packet | `oracle/failures/775/login_custom_query_clientbound_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/login_custom_query_clientbound_framed_dispatch.rust-fix-task.json` |

## Official Body Shape

The official `ClientboundCustomQueryPacket` bytecode writes:

| Order | Field |
|---|---|
| 1 | `transactionId` as `FriendlyByteBuf.writeVarInt(int)` |
| 2 | `payload.id()` as `FriendlyByteBuf.writeIdentifier(Identifier)` |
| 3 | `payload.write(FriendlyByteBuf)` |

This fixture uses transaction id `0`, payload id `a:a`, and
`DiscardedQueryPayload`, the smallest VarInt and one-character namespace/path
Identifier fixture accepted by the official constructor and codec without
initialized Minecraft state. `DiscardedQueryPayload.write(...)` writes an empty
payload body.

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
clientbound custom_query fixture with `transactionId=0`, `payloadId=a:a`, and
an empty `DiscardedQueryPayload` only. It does not prove plugin channel
handling, custom query semantics, login acknowledgement behavior,
Login-to-Configuration state transition handling, Configuration entry, Play
readiness, or client-load completion.

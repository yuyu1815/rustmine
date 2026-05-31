# login_cookie_response_serverbound_framed_dispatch

Purpose: preserve the Protocol 775 Login serverbound
`minecraft:cookie_response` packet id/body contract as a reset-proof
packet-support slice.

```text
client.jar ServerboundCookieResponsePacket(Identifier, byte[])
  -> ServerboundCookieResponsePacket.STREAM_CODEC
    -> LoginProtocols.SERVERBOUND codec
      -> LoginProtocols.SERVERBOUND_TEMPLATE table
        -> oracle/answers/775/login_cookie_response_serverbound_framed_dispatch.answer.jsonl
          -> oracle/test-manifests/775/login_cookie_response_serverbound_framed_dispatch.test-manifest.json
            -> oracle/rust-tests/tests/oracle_contracts.rs
              -> stevenarella/protocol/src/protocol/versions/v26_1_2.rs
```

| Field | Value |
|---|---|
| Case id | `login_cookie_response_serverbound_framed_dispatch` |
| Corridor | `Handshake -> Login -> Configuration` |
| Official source | `client.jar` `Identifier.parse(String)`; `ServerboundCookieResponsePacket(Identifier, byte[])`; `ServerboundCookieResponsePacket.STREAM_CODEC`; `LoginProtocols.SERVERBOUND.codec().encode/decode(ServerboundCookieResponsePacket)`; `LoginProtocols.SERVERBOUND_TEMPLATE.details().listPackets(...)`; `ServerboundCookieResponsePacket.key()`; `ServerboundCookieResponsePacket.payload()`; `ServerLoginPacketListener extends ServerCookiePacketListener` |
| Bytecode witness | `_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.login.LoginProtocols net.minecraft.network.protocol.login.LoginPacketTypes net.minecraft.network.protocol.cookie.ServerboundCookieResponsePacket` |
| Generated answer | `oracle/answers/775/login_cookie_response_serverbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/login_cookie_response_serverbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::login_cookie_response_serverbound_framed_dispatch_matches_official_oracle_answer` |
| Failure packet | `oracle/failures/775/login_cookie_response_serverbound_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/login_cookie_response_serverbound_framed_dispatch.rust-fix-task.json` |

## Stop Boundary

This is packet framing/dispatch/decode evidence for one official Login
serverbound cookie_response non-null payload fixture only. It does not prove
cookie storage policy, cookie request/response runtime behavior, Configuration
entry, Play readiness, or client-load completion.

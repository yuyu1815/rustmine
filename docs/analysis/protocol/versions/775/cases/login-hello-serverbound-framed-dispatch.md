# login_hello_serverbound_framed_dispatch

Purpose: preserve the official Protocol 775 Login serverbound
`minecraft:hello` packet id/body contract as a reset-proof packet-support
case.

```text
Login
  -> official client.jar ServerboundHelloPacket fixture
    -> LoginProtocols.SERVERBOUND codec
      -> LoginProtocols.SERVERBOUND_TEMPLATE table
        -> oracle/answers/775/login_hello_serverbound_framed_dispatch.answer.jsonl
          -> oracle/test-manifests/775/login_hello_serverbound_framed_dispatch.test-manifest.json
            -> oracle/rust-tests/tests/oracle_contracts.rs
```

| Field | Value |
|---|---|
| Case id | `login_hello_serverbound_framed_dispatch` |
| Corridor | `Handshake -> Login -> Configuration` |
| Official source | `client.jar` `ServerboundHelloPacket(String, UUID)`; `ServerboundHelloPacket.STREAM_CODEC`; `LoginProtocols.SERVERBOUND.codec().encode/decode(ServerboundHelloPacket)`; `LoginProtocols.SERVERBOUND_TEMPLATE.details().listPackets(...)`; decoded field accessors |
| Bytecode witness | `_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.login.LoginProtocols net.minecraft.network.protocol.login.LoginPacketTypes net.minecraft.network.protocol.login.ServerboundHelloPacket` |
| Generated answer | `oracle/answers/775/login_hello_serverbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/login_hello_serverbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::login_hello_serverbound_framed_dispatch_matches_official_oracle_answer` |
| Failure packet | `oracle/failures/775/login_hello_serverbound_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/login_hello_serverbound_framed_dispatch.rust-fix-task.json` |

## Stop Boundary

This is packet framing and dispatch/decode proof for one official Login
serverbound hello fixture only. It does not prove authentication success,
encryption/key exchange, login acknowledgement, Configuration entry, runtime
state transition, or client-load completion.

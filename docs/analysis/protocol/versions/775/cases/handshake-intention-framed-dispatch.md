# handshake_intention_framed_dispatch

Purpose: preserve the official Protocol 775 Handshaking serverbound
`minecraft:intention` packet id/body contract as a reset-proof packet-support
case.

```text
Handshake
  -> official client.jar ClientIntentionPacket fixture
    -> HandshakeProtocols.SERVERBOUND codec
      -> HandshakeProtocols.SERVERBOUND_TEMPLATE table
        -> oracle/answers/775/handshake_intention_framed_dispatch.answer.jsonl
          -> oracle/test-manifests/775/handshake_intention_framed_dispatch.test-manifest.json
            -> oracle/rust-tests/tests/oracle_contracts.rs
```

| Field | Value |
|---|---|
| Case id | `handshake_intention_framed_dispatch` |
| Corridor | `Handshake -> Login` |
| Official source | `client.jar` `ClientIntentionPacket(int, String, int, ClientIntent)`; `ClientIntentionPacket.STREAM_CODEC`; `HandshakeProtocols.SERVERBOUND.codec().encode/decode(ClientIntentionPacket)`; `HandshakeProtocols.SERVERBOUND_TEMPLATE.details().listPackets(...)`; decoded field accessors and terminal flag |
| Bytecode witness | `_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.handshake.ClientIntentionPacket net.minecraft.network.protocol.handshake.HandshakeProtocols net.minecraft.network.protocol.handshake.HandshakePacketTypes net.minecraft.network.protocol.handshake.ClientIntent` |
| Generated answer | `oracle/answers/775/handshake_intention_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/handshake_intention_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::handshake_intention_framed_dispatch_matches_official_oracle_answer` |
| Failure packet | `oracle/failures/775/handshake_intention_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/handshake_intention_framed_dispatch.rust-fix-task.json` |

## Stop Boundary

This is packet framing and dispatch/decode proof for one official LOGIN-intent
fixture only. It does not prove Login authentication, Configuration entry,
runtime state transition, or client-load completion.

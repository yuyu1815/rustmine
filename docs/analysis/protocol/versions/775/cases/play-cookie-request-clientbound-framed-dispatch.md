# play_cookie_request_clientbound_framed_dispatch

Purpose: preserve the Protocol 775 Play clientbound
`minecraft:cookie_request` packet id/body contract as a reset-proof
packet-support slice.

```text
client.jar Identifier.parse("a:a")
  -> ClientboundCookieRequestPacket(Identifier)
    -> ClientboundCookieRequestPacket.STREAM_CODEC
      -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x15
        -> GameProtocols.CLIENTBOUND codec encodes/decodes official frame
          -> oracle/answers/775/play_cookie_request_clientbound_framed_dispatch.answer.jsonl
            -> oracle/test-manifests/775/play_cookie_request_clientbound_framed_dispatch.test-manifest.json
              -> oracle/rust-tests/tests/oracle_contracts.rs
                -> stevenarella/protocol/src/protocol/versions/v26_1_2.rs
```

| Field | Value |
|---|---|
| Case id | `play_cookie_request_clientbound_framed_dispatch` |
| Corridor | `Configuration -> Play` |
| Official source | `client.jar` `Identifier.parse(String)`; `ClientboundCookieRequestPacket(Identifier)`; `ClientboundCookieRequestPacket.STREAM_CODEC`; `FriendlyByteBuf.readIdentifier/writeIdentifier`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundCookieRequestPacket)`; `ClientboundCookieRequestPacket.key()`; `ClientGamePacketListener extends ClientCommonPacketListener extends ClientCookiePacketListener` |
| Bytecode witness | `CP="_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath "$CP" -c -p net.minecraft.network.protocol.cookie.ClientboundCookieRequestPacket net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientGamePacketListener net.minecraft.network.protocol.common.ClientCommonPacketListener net.minecraft.network.protocol.cookie.ClientCookiePacketListener net.minecraft.network.protocol.cookie.CookiePacketTypes` |
| Generated answer | `oracle/answers/775/play_cookie_request_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/play_cookie_request_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_cookie_request_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Failure packet | `oracle/failures/775/play_cookie_request_clientbound_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/play_cookie_request_clientbound_framed_dispatch.rust-fix-task.json` |

## Official Body Shape

The official `ClientboundCookieRequestPacket` bytecode reads and writes one
field:

| Order | Field | Fixture value |
|---|---|---|
| 1 | `key` via `FriendlyByteBuf.writeIdentifier(Identifier)` | `a:a` |

This fixture uses key `a:a`, the smallest one-character namespace/path
Identifier fixture accepted by `Identifier.parse(String)` without initialized
Minecraft state.

The generated official frame is:

```text
1503613a61
```

## Official Table

The generated answer observes 141 Play clientbound rows. The local packet
support route has now proven rows through:

| Packet id | Packet type |
|---|---|
| `0x14` | `minecraft:container_set_slot` |
| `0x15` | `minecraft:cookie_request` |

The next official Play clientbound row is `minecraft:cooldown` / `0x16`.

## Stop Boundary

This is packet framing/dispatch/decode evidence for one official
`ClientboundCookieRequestPacket` fixture with key `a:a` only. It does not prove
cookie storage policy, cookie request/response runtime behavior, initialized
client/server state, runtime Play entry, world load, spawn readiness, render
readiness, or client-load completion.

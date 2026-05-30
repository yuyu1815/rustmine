# configuration_cookie_request_framed_dispatch

Purpose: document the Protocol 775 jar-backed proof package for Configuration
clientbound `minecraft:cookie_request` framed dispatch/decode without expanding
it into cookie storage policy, cookie request/response runtime behavior,
Configuration completion, or Play readiness.

## Evidence Map

```text
client.jar ClientboundCookieRequestPacket(Identifier)
  -> ClientboundCookieRequestPacket.STREAM_CODEC
  -> ConfigurationProtocols.CLIENTBOUND.codec().encode/decode
  -> ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets
    -> oracle/answers/775/configuration_cookie_request_framed_dispatch.answer.jsonl
      -> oracle/test-manifests/775/configuration_cookie_request_framed_dispatch.test-manifest.json
        -> oracle/rust-tests/tests/oracle_contracts.rs
          -> packet::packet_by_id(775, Configuration, Clientbound, official id, official body)
            -> current result: matching cookie_request identity and consumed body
```

## Facts

| Field | Value |
|---|---|
| Case id | `configuration_cookie_request_framed_dispatch` |
| Corridor | `Login -> Configuration -> Play` |
| Official source | `client.jar` `Identifier.parse(String)`; `ClientboundCookieRequestPacket(Identifier)`; `ClientboundCookieRequestPacket.STREAM_CODEC`; `ConfigurationProtocols.CLIENTBOUND.codec().encode/decode(ClientboundCookieRequestPacket)`; `ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `ClientboundCookieRequestPacket.key()` |
| Bytecode witness | `_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.cookie.ClientboundCookieRequestPacket` shows `readIdentifier`, `writeIdentifier`, and `key()` |
| Generated answer | `oracle/answers/775/configuration_cookie_request_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/configuration_cookie_request_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::configuration_cookie_request_framed_dispatch_matches_official_oracle_answer` |
| Internal owner under test | `stevenarella/protocol/src/protocol/mod.rs`; `packet::packet_by_id(775, State::Configuration, Direction::Clientbound, packet id, body)` |
| Failure packet | `oracle/failures/775/configuration_cookie_request_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/configuration_cookie_request_framed_dispatch.rust-fix-task.json` |
| Traceability row | `docs/analysis/protocol/versions/775/traceability.md` |

## Proves

The official jar emits the full Configuration clientbound
`minecraft:cookie_request` frame for `ClientboundCookieRequestPacket`,
dispatches that frame back to `ClientboundCookieRequestPacket`, preserves the
official `Identifier` key, and leaves no unread bytes after official decode.

The generated answer, not this note, owns the exact packet id, frame bytes,
body bytes, decoded packet class, key bytes/string, and remaining decode byte
count.

## Current Rust Result

The manifest-declared Rust oracle test reads the generated answer and calls
Stevenarella `packet::packet_by_id` with the official packet id and body. In
the current checkout, the test passes and maps the official
`minecraft:cookie_request` packet to the current compatibility alias
`Packet::PluginMessageClientbound` with channel `CookieRequest` while consuming
the full official key body.

Regression packets kept for traceability:

```text
oracle/failures/775/configuration_cookie_request_framed_dispatch.why-what-answer.jsonl
oracle/failures/775/configuration_cookie_request_framed_dispatch.rust-fix-task.json
```

## Does Not Prove

This does not prove cookie storage policy, cookie request/response runtime
behavior, Configuration completion, Play entry, registry hydration, or any
later client-load phase.

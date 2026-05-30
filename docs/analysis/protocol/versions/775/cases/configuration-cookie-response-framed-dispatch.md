# configuration_cookie_response_framed_dispatch

Purpose: document the Protocol 775 jar-backed proof package for Configuration
serverbound `minecraft:cookie_response` framed dispatch/decode without
expanding it into cookie storage policy, request/response runtime behavior,
Configuration completion, or Play readiness.

## Evidence Map

```text
client.jar ServerboundCookieResponsePacket(Identifier, byte[])
  -> ServerboundCookieResponsePacket.STREAM_CODEC
  -> ConfigurationProtocols.SERVERBOUND.codec().encode/decode
  -> ConfigurationProtocols.SERVERBOUND_TEMPLATE.details().listPackets
    -> oracle/answers/775/configuration_cookie_response_framed_dispatch.answer.jsonl
      -> oracle/test-manifests/775/configuration_cookie_response_framed_dispatch.test-manifest.json
        -> oracle/rust-tests/tests/oracle_contracts.rs
          -> packet::packet_by_id(775, Configuration, Serverbound, official id, official body)
            -> current result: current public Packet alias channel CookieResponse
               with official payload bytes and consumed key/payload body
```

## Facts

| Field | Value |
|---|---|
| Case id | `configuration_cookie_response_framed_dispatch` |
| Corridor | `Login -> Configuration -> Play` |
| Official source | `client.jar` `Identifier.parse(String)`; `ServerboundCookieResponsePacket(Identifier, byte[])`; `ServerboundCookieResponsePacket.STREAM_CODEC`; `ConfigurationProtocols.SERVERBOUND.codec().encode/decode(ServerboundCookieResponsePacket)`; `ConfigurationProtocols.SERVERBOUND_TEMPLATE.details().listPackets(...)`; `ServerboundCookieResponsePacket.key()`; `ServerboundCookieResponsePacket.payload()` |
| Generated answer | `oracle/answers/775/configuration_cookie_response_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/configuration_cookie_response_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::configuration_cookie_response_framed_dispatch_matches_official_oracle_answer` |
| Internal owner under test | `stevenarella/protocol/src/protocol/mod.rs`; `packet::packet_by_id(775, State::Configuration, Direction::Serverbound, packet id, body)` |
| Failure packet | `oracle/failures/775/configuration_cookie_response_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/configuration_cookie_response_framed_dispatch.rust-fix-task.json` |
| Traceability row | `docs/analysis/protocol/versions/775/traceability.md` |

## Proves

The official jar emits the full Configuration serverbound
`minecraft:cookie_response` frame for `ServerboundCookieResponsePacket`,
dispatches that frame back to `ServerboundCookieResponsePacket`, preserves the
official `Identifier` key and non-null payload bytes, and leaves no unread
bytes after official decode.

The generated answer, not this note, owns the exact packet id, frame bytes,
body bytes, decoded packet class, key bytes/string, nullable payload marker,
payload length, and payload bytes.

## Current Rust Result

The manifest-declared Rust oracle test reads the generated answer and calls
Stevenarella `packet::packet_by_id` with the official packet id and body. In
the current checkout, that test passes: Protocol 775 Configuration serverbound
id `0x01` dispatches to the current public `Packet::PluginMessageServerbound`
compatibility alias with channel `CookieResponse`, carries the official
non-null payload bytes, and consumes the official key/payload body.

The regression packets remain as traceability for the fix:

```text
oracle/failures/775/configuration_cookie_response_framed_dispatch.why-what-answer.jsonl
oracle/failures/775/configuration_cookie_response_framed_dispatch.rust-fix-task.json
```

## Does Not Prove

This does not prove cookie storage policy, cookie request/response runtime
behavior, Configuration completion, Play entry, registry hydration, or any
later client-load phase.

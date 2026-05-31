# configuration_store_cookie_clientbound_framed_dispatch

Purpose: document the Protocol 775 jar-backed proof package for Configuration
clientbound `minecraft:store_cookie` framed dispatch/decode without expanding
it into cookie storage policy, persistence, cookie request/response runtime
behavior, Configuration completion, Play entry, world load, render readiness,
or runtime behavior.

## Evidence Map

```text
client.jar ClientboundStoreCookiePacket(Identifier, byte[])
  -> ClientboundStoreCookiePacket.STREAM_CODEC
    -> ClientboundStoreCookiePacket.PAYLOAD_STREAM_CODEC
      -> ConfigurationProtocols.CLIENTBOUND.codec().encode/decode
        -> ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets
          -> oracle/answers/775/configuration_store_cookie_clientbound_framed_dispatch.answer.jsonl
            -> oracle/test-manifests/775/configuration_store_cookie_clientbound_framed_dispatch.test-manifest.json
              -> oracle/rust-tests/tests/oracle_contracts.rs
                -> packet::packet_by_id(775, Configuration, Clientbound, official id, official body)
                  -> current result: StoreCookie compatibility identity, preserved payload, and consumed body
```

## Facts

| Field | Value |
|---|---|
| Case id | `configuration_store_cookie_clientbound_framed_dispatch` |
| Corridor | `Login -> Configuration -> Play` |
| Official source | `client.jar` `Identifier.parse(String)`; `ClientboundStoreCookiePacket(Identifier, byte[])`; `ClientboundStoreCookiePacket.STREAM_CODEC`; `ClientboundStoreCookiePacket.PAYLOAD_STREAM_CODEC`; `ConfigurationProtocols.CLIENTBOUND.codec().encode/decode(ClientboundStoreCookiePacket)`; `ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `ClientboundStoreCookiePacket.key()`; `payload()` |
| Bytecode witness | `_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.common.ClientboundStoreCookiePacket net.minecraft.network.protocol.configuration.ConfigurationProtocols` |
| Generated answer | `oracle/answers/775/configuration_store_cookie_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/configuration_store_cookie_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::configuration_store_cookie_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Internal owner under test | `stevenarella/protocol/src/protocol/mod.rs`; `packet::packet_by_id(775, State::Configuration, Direction::Clientbound, packet id, body)` |
| Failure packet | `oracle/failures/775/configuration_store_cookie_clientbound_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/configuration_store_cookie_clientbound_framed_dispatch.rust-fix-task.json` |
| Traceability row | `docs/analysis/protocol/versions/775/traceability.md` |

## Proves

The official jar emits the full Configuration clientbound
`minecraft:store_cookie` frame for one key/payload fixture, dispatches that
frame back to `ClientboundStoreCookiePacket`, preserves the Identifier key and
payload bytes, and leaves no unread bytes after official decode.

The generated answer, not this note, owns the exact packet id, frame bytes,
body bytes, decoded packet class, decoded fields, and remaining decode byte
count.

## Current Rust Result

The manifest-declared Rust oracle test reads the generated answer and calls
Stevenarella `packet::packet_by_id` with the official packet id and body. In
the current checkout, the test passes and maps the official
`minecraft:store_cookie` packet to the current compatibility alias
`Packet::PluginMessageClientbound` with channel `StoreCookie`, preserving the
official payload bytes while consuming the official body bytes.

```text
oracle/failures/775/configuration_store_cookie_clientbound_framed_dispatch.why-what-answer.jsonl
oracle/failures/775/configuration_store_cookie_clientbound_framed_dispatch.rust-fix-task.json
```

## Does Not Prove

This does not prove cookie storage policy, persistence, cookie
request/response runtime behavior, Configuration completion, Play entry, world
load, render readiness, or any later client-load phase.

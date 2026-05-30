# network_login_configuration

| Field | Value |
|---|---|
| Lens position | 2 of 8 |
| Load claim | Client can satisfy official login/configuration wire contracts. |
| Evidence surface | Official jar codec/state evidence |
| Proof label | `partial` |
| Current proof | `configuration_client_information_framed_dispatch`, `configuration_cookie_response_framed_dispatch`, `configuration_keepalive_codec`, `configuration_keepalive_framed_dispatch`, `configuration_keepalive_clientbound_framed_dispatch`, `configuration_ping_pong_framed_dispatch`, `configuration_finish_framed_terminal`, `configuration_resource_pack_response_framed_dispatch`, `configuration_select_known_packs_framed_dispatch`, `configuration_custom_click_action_framed_dispatch`, and `configuration_accept_code_of_conduct_framed_dispatch` have regenerated jar-backed answers for Configuration client_information, cookie_response key/nullable-payload, keep-alive body/table-id, serverbound/clientbound keep_alive framed dispatch/decode, clientbound ping/serverbound pong framed dispatch/decode, finish_configuration framed/terminal fields, serverbound resource_pack UUID/action response dispatch/decode, serverbound select_known_packs known-pack list dispatch/decode, serverbound custom_click_action identifier/optional-payload dispatch/decode, and serverbound accept_code_of_conduct empty-body dispatch/decode. Those exact reset-proof Rust oracle tests pass against the current Leafish checkout. `configuration_keepalive_runtime_send_helper` proves `packet::send_keep_alive` writes the official Configuration serverbound keep_alive frame in Configuration state; `configuration_keepalive_runtime_protocol_echo` proves the protocol crate can read an official Configuration clientbound keep_alive frame, map it to `MappedPacket::KeepAliveClientbound(id)`, and send the official Configuration serverbound keep_alive response. |
| Project-level test/probe | `oracle/rust-tests/tests/oracle_contracts.rs` |
| Candidate checkout owner under test | `stevenarella/protocol/src/protocol/packet.rs` |
| Candidate evidence gap | Add the next Configuration packet-support proof, full `spawn_reader` keep-alive reaction, and runtime Configuration-to-Play transition proof. |

## Proven Slice

The currently proven compatibility slices are
`configuration_client_information_framed_dispatch`,
`configuration_cookie_response_framed_dispatch`,
`configuration_keepalive_codec`, `configuration_keepalive_framed_dispatch`,
`configuration_keepalive_clientbound_framed_dispatch`,
`configuration_ping_pong_framed_dispatch`,
`configuration_finish_framed_terminal`,
`configuration_resource_pack_response_framed_dispatch`,
`configuration_select_known_packs_framed_dispatch`,
`configuration_custom_click_action_framed_dispatch`, and
`configuration_accept_code_of_conduct_framed_dispatch`, backed by official jar
output and stored outside the reset-prone checkout. For those slices, the
official answers were regenerated and the manifest-declared Rust oracle tests
passed:

```text
oracle/answers/775/configuration_keepalive_codec.answer.jsonl
oracle/answers/775/configuration_keepalive_framed_dispatch.answer.jsonl
oracle/answers/775/configuration_keepalive_clientbound_framed_dispatch.answer.jsonl
oracle/answers/775/configuration_client_information_framed_dispatch.answer.jsonl
oracle/answers/775/configuration_ping_pong_framed_dispatch.answer.jsonl
oracle/answers/775/configuration_finish_framed_terminal.answer.jsonl
oracle/answers/775/configuration_resource_pack_response_framed_dispatch.answer.jsonl
oracle/answers/775/configuration_select_known_packs_framed_dispatch.answer.jsonl
oracle/answers/775/configuration_custom_click_action_framed_dispatch.answer.jsonl
oracle/answers/775/configuration_accept_code_of_conduct_framed_dispatch.answer.jsonl
oracle/test-manifests/775/configuration_keepalive_codec.test-manifest.json
oracle/test-manifests/775/configuration_keepalive_framed_dispatch.test-manifest.json
oracle/test-manifests/775/configuration_keepalive_clientbound_framed_dispatch.test-manifest.json
oracle/test-manifests/775/configuration_client_information_framed_dispatch.test-manifest.json
oracle/test-manifests/775/configuration_ping_pong_framed_dispatch.test-manifest.json
oracle/test-manifests/775/configuration_finish_framed_terminal.test-manifest.json
oracle/test-manifests/775/configuration_resource_pack_response_framed_dispatch.test-manifest.json
oracle/test-manifests/775/configuration_select_known_packs_framed_dispatch.test-manifest.json
oracle/test-manifests/775/configuration_custom_click_action_framed_dispatch.test-manifest.json
oracle/test-manifests/775/configuration_accept_code_of_conduct_framed_dispatch.test-manifest.json
oracle/rust-tests/tests/oracle_contracts.rs
```

The cookie-response regression packets remain as traceability for the fix:

```text
oracle/answers/775/configuration_cookie_response_framed_dispatch.answer.jsonl
oracle/test-manifests/775/configuration_cookie_response_framed_dispatch.test-manifest.json
oracle/failures/775/configuration_cookie_response_framed_dispatch.why-what-answer.jsonl
oracle/failures/775/configuration_cookie_response_framed_dispatch.rust-fix-task.json
```

The accept-code-of-conduct regression packets remain as traceability for the
fix:

```text
oracle/answers/775/configuration_accept_code_of_conduct_framed_dispatch.answer.jsonl
oracle/test-manifests/775/configuration_accept_code_of_conduct_framed_dispatch.test-manifest.json
oracle/failures/775/configuration_accept_code_of_conduct_framed_dispatch.why-what-answer.jsonl
oracle/failures/775/configuration_accept_code_of_conduct_framed_dispatch.rust-fix-task.json
```

The custom-click-action regression packets remain as traceability for the fix:

```text
oracle/answers/775/configuration_custom_click_action_framed_dispatch.answer.jsonl
oracle/test-manifests/775/configuration_custom_click_action_framed_dispatch.test-manifest.json
oracle/failures/775/configuration_custom_click_action_framed_dispatch.why-what-answer.jsonl
oracle/failures/775/configuration_custom_click_action_framed_dispatch.rust-fix-task.json
```

The resource-pack response regression packets remain as traceability for the
fix:

```text
oracle/answers/775/configuration_resource_pack_response_framed_dispatch.answer.jsonl
oracle/test-manifests/775/configuration_resource_pack_response_framed_dispatch.test-manifest.json
oracle/failures/775/configuration_resource_pack_response_framed_dispatch.why-what-answer.jsonl
oracle/failures/775/configuration_resource_pack_response_framed_dispatch.rust-fix-task.json
```

The select-known-packs regression packets remain as traceability for the fix:

```text
oracle/answers/775/configuration_select_known_packs_framed_dispatch.answer.jsonl
oracle/test-manifests/775/configuration_select_known_packs_framed_dispatch.test-manifest.json
oracle/failures/775/configuration_select_known_packs_framed_dispatch.why-what-answer.jsonl
oracle/failures/775/configuration_select_known_packs_framed_dispatch.rust-fix-task.json
```

## Current Client Information Packet Slice

```text
configuration_client_information_framed_dispatch
  -> official Configuration serverbound minecraft:client_information frame
    from oracle/answers/775/configuration_client_information_framed_dispatch.answer.jsonl
  -> packet::packet_by_id(775, State::Configuration, Direction::Serverbound, official id, body)
    -> current result: matching client_information identity and consumed body
```

This slice stops before runtime client settings send behavior.

## Current Cookie Response Packet Slice

```text
configuration_cookie_response_framed_dispatch
  -> official Configuration serverbound minecraft:cookie_response frame
    from oracle/answers/775/configuration_cookie_response_framed_dispatch.answer.jsonl
  -> packet::packet_by_id(775, State::Configuration, Direction::Serverbound, official id, body)
    -> current result: current public Packet alias channel CookieResponse,
       official non-null payload bytes, and consumed key/payload body
```

This slice stops before cookie storage policy, request/response runtime
behavior, Configuration completion, or Play entry.

## Current Resource Pack Response Packet Slice

```text
configuration_resource_pack_response_framed_dispatch
  -> official Configuration serverbound minecraft:resource_pack frame
    from oracle/answers/775/configuration_resource_pack_response_framed_dispatch.answer.jsonl
  -> packet::packet_by_id(775, State::Configuration, Direction::Serverbound, official id, body)
    -> current result: matching resource_pack identity and consumed body
```

This slice stops before runtime resource pack UI, accept/reject behavior,
download/reload behavior, Configuration completion, or Play entry.

## Current Select Known Packs Packet Slice

```text
configuration_select_known_packs_framed_dispatch
  -> official Configuration serverbound minecraft:select_known_packs frame
    from oracle/answers/775/configuration_select_known_packs_framed_dispatch.answer.jsonl
  -> packet::packet_by_id(775, State::Configuration, Direction::Serverbound, official id, body)
    -> current result: matching select_known_packs identity and consumed body
```

This slice stops before registry hydration, known-pack negotiation completion,
Configuration completion, or Play entry.

## Current Custom Click Action Packet Slice

```text
configuration_custom_click_action_framed_dispatch
  -> official Configuration serverbound minecraft:custom_click_action frame
    from oracle/answers/775/configuration_custom_click_action_framed_dispatch.answer.jsonl
  -> packet::packet_by_id(775, State::Configuration, Direction::Serverbound, official id, body)
    -> current result: matching custom_click_action identity and consumed body
```

This slice stops before UI behavior, command execution, interaction readiness,
Configuration completion, or Play entry.

## Current Accept Code Of Conduct Packet Slice

```text
configuration_accept_code_of_conduct_framed_dispatch
  -> official Configuration serverbound minecraft:accept_code_of_conduct frame
    from oracle/answers/775/configuration_accept_code_of_conduct_framed_dispatch.answer.jsonl
  -> packet::packet_by_id(775, State::Configuration, Direction::Serverbound, official id, body)
    -> current result: current public Packet alias channel AcceptCodeOfConduct and consumed empty body
```

This slice is packet framing/dispatch/decode only. It stops before UI consent
flow, legal acceptance semantics, Configuration completion, or Play entry.

## Current Ping/Pong Packet Slice

```text
configuration_ping_pong_framed_dispatch
  -> official Configuration clientbound minecraft:ping frame
    from oracle/answers/775/configuration_ping_pong_framed_dispatch.answer.jsonl
  -> packet::packet_by_id(775, State::Configuration, Direction::Clientbound, 0x05, body)
    -> current result: matching ping packet identity and payload id
  -> official Configuration serverbound minecraft:pong frame
    -> packet::packet_by_id(775, State::Configuration, Direction::Serverbound, 0x05, body)
      -> current result: matching pong packet identity and payload id
```

## Not Proven

This phase is still not complete. The proven slice does not prove full
login/configuration runtime behavior, runtime ping response behavior,
keep-alive response loop behavior, runtime Configuration-to-Play transition
behavior, or Play readiness.

## Current Runtime-Send Probe

```text
configuration_keepalive_runtime_send_helper
  -> packet::send_keep_alive(&mut Conn in State::Configuration, official id)
    -> expected: VarInt(length) + official Configuration serverbound keep_alive frame
      from oracle/answers/775/configuration_keepalive_framed_dispatch.answer.jsonl
    -> observed: matching frame on localhost listener
```

Regression packet kept for traceability:

```text
oracle/failures/775/configuration_keepalive_runtime_send_helper.why-what-answer.jsonl
```

## Current Protocol Echo Probe

```text
configuration_keepalive_runtime_protocol_echo
  -> official Configuration clientbound keep_alive network frame
    from oracle/answers/775/configuration_keepalive_clientbound_framed_dispatch.answer.jsonl
  -> Conn::read_packet() in State::Configuration
    -> Packet::map()
      -> MappedPacket::KeepAliveClientbound(id)
        -> packet::send_keep_alive(&mut Conn, id)
          -> expected: VarInt(length) + official Configuration serverbound keep_alive frame
            from oracle/answers/775/configuration_keepalive_framed_dispatch.answer.jsonl
          -> observed: matching response frame on localhost listener
```

This still stops before the full `spawn_reader` thread path.

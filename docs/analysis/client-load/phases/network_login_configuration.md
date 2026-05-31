# network_login_configuration

| Field | Value |
|---|---|
| Lens position | 2 of 8 |
| Load claim | Client can satisfy official login/configuration wire contracts. |
| Evidence surface | Official jar codec/state evidence |
| Proof label | `partial` |
| Current proof | `configuration_client_information_framed_dispatch`, `configuration_cookie_request_framed_dispatch`, `configuration_cookie_response_framed_dispatch`, `configuration_custom_payload_clientbound_framed_dispatch`, `configuration_custom_payload_framed_dispatch`, `configuration_disconnect_clientbound_framed_dispatch`, `configuration_reset_chat_clientbound_framed_dispatch`, `configuration_registry_data_clientbound_framed_dispatch`, `configuration_resource_pack_pop_clientbound_framed_dispatch`, `configuration_resource_pack_push_clientbound_framed_dispatch`, `configuration_keepalive_codec`, `configuration_keepalive_framed_dispatch`, `configuration_keepalive_clientbound_framed_dispatch`, `configuration_ping_pong_framed_dispatch`, `configuration_finish_framed_terminal`, `configuration_resource_pack_response_framed_dispatch`, `configuration_select_known_packs_framed_dispatch`, `configuration_custom_click_action_framed_dispatch`, and `configuration_accept_code_of_conduct_framed_dispatch` have regenerated jar-backed answers for Configuration client_information, clientbound cookie_request Identifier key, cookie_response key/nullable-payload, clientbound custom_payload BrandPayload, serverbound custom_payload BrandPayload, clientbound disconnect empty literal Component reason, clientbound reset_chat singleton empty body, clientbound registry_data DIMENSION_TYPE empty-entry fixture, clientbound resource_pack_pop present-UUID fixture, clientbound resource_pack_push no-prompt fixture, keep-alive body/table-id, serverbound/clientbound keep_alive framed dispatch/decode, clientbound ping/serverbound pong framed dispatch/decode, finish_configuration framed/terminal fields, serverbound resource_pack UUID/action response dispatch/decode, serverbound select_known_packs known-pack list dispatch/decode, serverbound custom_click_action identifier/optional-payload dispatch/decode, and serverbound accept_code_of_conduct empty-body dispatch/decode. Those exact reset-proof Rust oracle tests pass against the current Leafish checkout. `configuration_keepalive_runtime_send_helper` proves `packet::send_keep_alive` writes the official Configuration serverbound keep_alive frame in Configuration state; `configuration_keepalive_runtime_protocol_echo` proves the protocol crate can read an official Configuration clientbound keep_alive frame, map it to `MappedPacket::KeepAliveClientbound(id)`, and send the official Configuration serverbound keep_alive response. `configuration_keepalive_runtime_spawn_reader_reaction` proves the same official clientbound keep_alive frame drives the factored `Server::spawn_reader` keep_alive branch and observes the official Configuration serverbound keep_alive response frame. |
| Project-level test/probe | `oracle/rust-tests/tests/oracle_contracts.rs` |
| Candidate checkout owner under test | `stevenarella/protocol/src/protocol/packet.rs` |
| Candidate evidence gap | For packet-support loop, next missing Configuration clientbound packet proof is likely `minecraft:store_cookie` / `0x0a`; runtime Configuration-to-Play, registry hydration, and Play readiness remain later gaps. |

## Proven Slice

The currently proven compatibility slices are
`configuration_client_information_framed_dispatch`,
`configuration_cookie_request_framed_dispatch`,
`configuration_cookie_response_framed_dispatch`,
`configuration_custom_payload_clientbound_framed_dispatch`,
`configuration_custom_payload_framed_dispatch`,
`configuration_disconnect_clientbound_framed_dispatch`,
`configuration_reset_chat_clientbound_framed_dispatch`,
`configuration_registry_data_clientbound_framed_dispatch`,
`configuration_resource_pack_pop_clientbound_framed_dispatch`,
`configuration_resource_pack_push_clientbound_framed_dispatch`,
`configuration_keepalive_codec`, `configuration_keepalive_framed_dispatch`,
`configuration_keepalive_clientbound_framed_dispatch`,
`configuration_ping_pong_framed_dispatch`,
`configuration_finish_framed_terminal`,
`configuration_resource_pack_response_framed_dispatch`,
`configuration_select_known_packs_framed_dispatch`,
`configuration_custom_click_action_framed_dispatch`, and
`configuration_accept_code_of_conduct_framed_dispatch`, backed by official jar
output and stored outside the reset-prone checkout. For those passing slices,
the official answers were regenerated and the manifest-declared Rust oracle
tests passed:

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
oracle/answers/775/configuration_cookie_request_framed_dispatch.answer.jsonl
oracle/answers/775/configuration_cookie_response_framed_dispatch.answer.jsonl
oracle/answers/775/configuration_custom_payload_clientbound_framed_dispatch.answer.jsonl
oracle/answers/775/configuration_custom_payload_framed_dispatch.answer.jsonl
oracle/answers/775/configuration_disconnect_clientbound_framed_dispatch.answer.jsonl
oracle/answers/775/configuration_reset_chat_clientbound_framed_dispatch.answer.jsonl
oracle/answers/775/configuration_registry_data_clientbound_framed_dispatch.answer.jsonl
oracle/answers/775/configuration_resource_pack_pop_clientbound_framed_dispatch.answer.jsonl
oracle/answers/775/configuration_resource_pack_push_clientbound_framed_dispatch.answer.jsonl
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
oracle/test-manifests/775/configuration_cookie_request_framed_dispatch.test-manifest.json
oracle/test-manifests/775/configuration_cookie_response_framed_dispatch.test-manifest.json
oracle/test-manifests/775/configuration_custom_payload_clientbound_framed_dispatch.test-manifest.json
oracle/test-manifests/775/configuration_custom_payload_framed_dispatch.test-manifest.json
oracle/test-manifests/775/configuration_disconnect_clientbound_framed_dispatch.test-manifest.json
oracle/test-manifests/775/configuration_reset_chat_clientbound_framed_dispatch.test-manifest.json
oracle/test-manifests/775/configuration_registry_data_clientbound_framed_dispatch.test-manifest.json
oracle/test-manifests/775/configuration_resource_pack_pop_clientbound_framed_dispatch.test-manifest.json
oracle/test-manifests/775/configuration_resource_pack_push_clientbound_framed_dispatch.test-manifest.json
oracle/rust-tests/tests/oracle_contracts.rs
```

The custom-payload regression packets remain as traceability for the fix:

```text
oracle/answers/775/configuration_custom_payload_framed_dispatch.answer.jsonl
oracle/test-manifests/775/configuration_custom_payload_framed_dispatch.test-manifest.json
oracle/failures/775/configuration_custom_payload_framed_dispatch.why-what-answer.jsonl
oracle/failures/775/configuration_custom_payload_framed_dispatch.rust-fix-task.json
```

The clientbound custom-payload regression packets remain as traceability for
the fix:

```text
oracle/answers/775/configuration_custom_payload_clientbound_framed_dispatch.answer.jsonl
oracle/test-manifests/775/configuration_custom_payload_clientbound_framed_dispatch.test-manifest.json
oracle/failures/775/configuration_custom_payload_clientbound_framed_dispatch.why-what-answer.jsonl
oracle/failures/775/configuration_custom_payload_clientbound_framed_dispatch.rust-fix-task.json
```

The clientbound disconnect regression packets remain as traceability for the
fix:

```text
oracle/answers/775/configuration_disconnect_clientbound_framed_dispatch.answer.jsonl
oracle/test-manifests/775/configuration_disconnect_clientbound_framed_dispatch.test-manifest.json
oracle/failures/775/configuration_disconnect_clientbound_framed_dispatch.why-what-answer.jsonl
oracle/failures/775/configuration_disconnect_clientbound_framed_dispatch.rust-fix-task.json
```

The clientbound reset-chat regression packets remain as traceability for the
fix:

```text
oracle/answers/775/configuration_reset_chat_clientbound_framed_dispatch.answer.jsonl
oracle/test-manifests/775/configuration_reset_chat_clientbound_framed_dispatch.test-manifest.json
oracle/failures/775/configuration_reset_chat_clientbound_framed_dispatch.why-what-answer.jsonl
oracle/failures/775/configuration_reset_chat_clientbound_framed_dispatch.rust-fix-task.json
```

The clientbound registry-data regression packets remain as traceability for
the fix:

```text
oracle/answers/775/configuration_registry_data_clientbound_framed_dispatch.answer.jsonl
oracle/test-manifests/775/configuration_registry_data_clientbound_framed_dispatch.test-manifest.json
oracle/failures/775/configuration_registry_data_clientbound_framed_dispatch.why-what-answer.jsonl
oracle/failures/775/configuration_registry_data_clientbound_framed_dispatch.rust-fix-task.json
```

The clientbound resource-pack-pop regression packets remain as traceability
for the fix:

```text
oracle/answers/775/configuration_resource_pack_pop_clientbound_framed_dispatch.answer.jsonl
oracle/test-manifests/775/configuration_resource_pack_pop_clientbound_framed_dispatch.test-manifest.json
oracle/failures/775/configuration_resource_pack_pop_clientbound_framed_dispatch.why-what-answer.jsonl
oracle/failures/775/configuration_resource_pack_pop_clientbound_framed_dispatch.rust-fix-task.json
```

The clientbound resource-pack-push regression packets remain as traceability
for the fix:

```text
oracle/answers/775/configuration_resource_pack_push_clientbound_framed_dispatch.answer.jsonl
oracle/test-manifests/775/configuration_resource_pack_push_clientbound_framed_dispatch.test-manifest.json
oracle/failures/775/configuration_resource_pack_push_clientbound_framed_dispatch.why-what-answer.jsonl
oracle/failures/775/configuration_resource_pack_push_clientbound_framed_dispatch.rust-fix-task.json
```

The cookie-response regression packets remain as traceability for the fix:

```text
oracle/answers/775/configuration_cookie_response_framed_dispatch.answer.jsonl
oracle/test-manifests/775/configuration_cookie_response_framed_dispatch.test-manifest.json
oracle/failures/775/configuration_cookie_response_framed_dispatch.why-what-answer.jsonl
oracle/failures/775/configuration_cookie_response_framed_dispatch.rust-fix-task.json
```

The cookie-request regression packets remain as traceability for the fix:

```text
oracle/answers/775/configuration_cookie_request_framed_dispatch.answer.jsonl
oracle/test-manifests/775/configuration_cookie_request_framed_dispatch.test-manifest.json
oracle/failures/775/configuration_cookie_request_framed_dispatch.why-what-answer.jsonl
oracle/failures/775/configuration_cookie_request_framed_dispatch.rust-fix-task.json
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

## Current Cookie Request Packet Slice

```text
configuration_cookie_request_framed_dispatch
  -> official Configuration clientbound minecraft:cookie_request frame
    from oracle/answers/775/configuration_cookie_request_framed_dispatch.answer.jsonl
  -> packet::packet_by_id(775, State::Configuration, Direction::Clientbound, official id, body)
    -> current result: current public Packet alias channel CookieRequest
       and consumed Identifier key body
```

This slice stops before cookie storage policy, request/response runtime
behavior, Configuration completion, or Play entry.

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

## Current Custom Payload Packet Slice

```text
configuration_custom_payload_framed_dispatch
  -> official Configuration serverbound minecraft:custom_payload frame
    from oracle/answers/775/configuration_custom_payload_framed_dispatch.answer.jsonl
  -> packet::packet_by_id(775, State::Configuration, Direction::Serverbound, official id, body)
    -> current result: current public Packet alias channel minecraft:brand,
       official BrandPayload bytes, and consumed custom_payload body
```

This slice stops before arbitrary plugin-channel handling, payload routing
policy, Configuration completion, or Play entry.

## Current Clientbound Custom Payload Packet Slice

```text
configuration_custom_payload_clientbound_framed_dispatch
  -> official Configuration clientbound minecraft:custom_payload frame
    from oracle/answers/775/configuration_custom_payload_clientbound_framed_dispatch.answer.jsonl
  -> packet::packet_by_id(775, State::Configuration, Direction::Clientbound, official id, body)
    -> current result: current public Packet alias channel minecraft:brand,
       official BrandPayload bytes, and consumed custom_payload body
```

This slice stops before arbitrary plugin-channel handling, payload routing
policy, Configuration completion, registry hydration, Play entry, or runtime
behavior.

## Current Clientbound Disconnect Packet Slice

```text
configuration_disconnect_clientbound_framed_dispatch
  -> official Configuration clientbound minecraft:disconnect frame
    from oracle/answers/775/configuration_disconnect_clientbound_framed_dispatch.answer.jsonl
  -> packet::packet_by_id(775, State::Configuration, Direction::Clientbound, official id, body)
    -> current result: Packet::Disconnect with official empty reason text
       and consumed Component reason body
```

This slice stops before UI disconnect handling, screen flow, Configuration
completion, registry hydration, Play entry, or runtime behavior.

## Current Clientbound Reset Chat Packet Slice

```text
configuration_reset_chat_clientbound_framed_dispatch
  -> official Configuration clientbound minecraft:reset_chat frame
    from oracle/answers/775/configuration_reset_chat_clientbound_framed_dispatch.answer.jsonl
  -> packet::packet_by_id(775, State::Configuration, Direction::Clientbound, official id, body)
    -> current result: current public Packet alias channel ResetChat
       and consumed empty body
```

This slice stops before chat UI reset behavior, screen flow, Configuration
completion, registry hydration, Play entry, or runtime behavior.

## Current Clientbound Registry Data Packet Slice

```text
configuration_registry_data_clientbound_framed_dispatch
  -> official Configuration clientbound minecraft:registry_data frame
    from oracle/answers/775/configuration_registry_data_clientbound_framed_dispatch.answer.jsonl
  -> packet::packet_by_id(775, State::Configuration, Direction::Clientbound, official id, body)
    -> current result: current public Packet alias channel RegistryData
       and consumed registry-key plus empty entry-list body
```

This slice stops before real registry contents,
`RegistrySynchronization.packRegistries` output, registry hydration,
Configuration completion, Play entry, world load, render readiness, or runtime
behavior.

## Current Clientbound Resource Pack Pop Packet Slice

```text
configuration_resource_pack_pop_clientbound_framed_dispatch
  -> official Configuration clientbound minecraft:resource_pack_pop frame
    from oracle/answers/775/configuration_resource_pack_pop_clientbound_framed_dispatch.answer.jsonl
  -> packet::packet_by_id(775, State::Configuration, Direction::Clientbound, official id, body)
    -> current result: current public Packet alias channel ResourcePackPop
       and consumed optional UUID body
```

This slice stops before resource-pack UI behavior, pack removal policy,
download/reload behavior, Configuration completion, Play entry, world load,
render readiness, or runtime behavior.

## Current Clientbound Resource Pack Push Packet Slice

```text
configuration_resource_pack_push_clientbound_framed_dispatch
  -> official Configuration clientbound minecraft:resource_pack_push frame
    from oracle/answers/775/configuration_resource_pack_push_clientbound_framed_dispatch.answer.jsonl
  -> packet::packet_by_id(775, State::Configuration, Direction::Clientbound, official id, body)
    -> current result: current public Packet alias channel ResourcePackPush
       and consumed UUID/URL/hash/required/prompt-presence body
```

This slice stops before resource-pack UI behavior, pack
download/reload/application behavior, Configuration completion, Play entry,
world load, render readiness, or runtime behavior.

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
runtime Configuration-to-Play transition behavior, or Play readiness.

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

## Current Spawn Reader Reaction Probe

```text
configuration_keepalive_runtime_spawn_reader_reaction
  -> official Configuration clientbound keep_alive network frame
    from oracle/answers/775/configuration_keepalive_clientbound_framed_dispatch.answer.jsonl
  -> runtime owner:
    stevenarella/src/server/mod.rs Server::spawn_reader keep_alive branch
      -> Conn::read_packet()
        -> Packet::map()
          -> MappedPacket::KeepAliveClientbound(id)
            -> packet::send_keep_alive(server.conn.write().as_mut().unwrap(), id)
              -> expected: VarInt(length) + official Configuration serverbound keep_alive frame
                from oracle/answers/775/configuration_keepalive_framed_dispatch.answer.jsonl
              -> observed: matching response frame on localhost listener
```

Current result: the exact Rust oracle test validates both official answer
artifacts, then runs a crate-local probe through
`Server::handle_next_reader_packet_for_oracle`, the same factored keep_alive
branch used by `Server::spawn_reader`, and observes the official Configuration
serverbound keep_alive network frame.

Regression packets kept for traceability:

```text
oracle/failures/775/configuration_keepalive_runtime_spawn_reader_reaction.why-what-answer.jsonl
oracle/failures/775/configuration_keepalive_runtime_spawn_reader_reaction.rust-fix-task.json
```

This still stops before Configuration completion, runtime Configuration-to-Play
transition behavior, registry hydration, Play entry, world load, or render
readiness.

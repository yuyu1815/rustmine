# network_login_configuration

| Field | Value |
|---|---|
| Lens position | 2 of 8 |
| Load claim | Client can satisfy official login/configuration wire contracts. |
| Evidence surface | Official jar codec/state evidence |
| Proof label | `partial` |
| Current proof | `configuration_keepalive_codec` and `configuration_keepalive_framed_dispatch` have regenerated jar-backed answers for Configuration serverbound keep-alive body/table-id and framed dispatch/decode, and the exact reset-proof Rust oracle tests pass against the current Leafish checkout. |
| Project-level test/probe | `oracle/rust-tests/tests/oracle_contracts.rs` |
| Candidate checkout owner under test | `stevenarella/protocol/src/protocol/packet.rs` |
| Candidate evidence gap | Add runtime reaction and Configuration completion cases. |

## Proven Slice

The currently proven compatibility slices are `configuration_keepalive_codec`
and `configuration_keepalive_framed_dispatch`, backed by official jar output
and stored outside the reset-prone checkout. In the current run, the official
answers were regenerated and the manifest-declared Rust oracle tests passed:

```text
oracle/answers/775/configuration_keepalive_codec.answer.jsonl
oracle/answers/775/configuration_keepalive_framed_dispatch.answer.jsonl
oracle/test-manifests/775/configuration_keepalive_codec.test-manifest.json
oracle/test-manifests/775/configuration_keepalive_framed_dispatch.test-manifest.json
oracle/rust-tests/tests/oracle_contracts.rs
```

## Not Proven

This phase is still not complete. The proven slice does not prove full
login/configuration runtime behavior, keep-alive response loop behavior,
Configuration completion, or `Configuration -> Play` transition behavior.

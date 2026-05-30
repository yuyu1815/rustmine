# network_login_configuration

| Field | Value |
|---|---|
| Lens position | 2 of 8 |
| Load claim | Client can satisfy official login/configuration wire contracts. |
| Evidence surface | Official jar codec/state evidence |
| Proof label | `partial` |
| Current proof | `configuration_keepalive_codec`, `configuration_keepalive_framed_dispatch`, and `configuration_finish_framed_terminal` have regenerated jar-backed answers for Configuration keep-alive body/table-id, framed dispatch/decode, and finish_configuration framed/terminal fields; the exact reset-proof Rust oracle tests pass against the current Leafish checkout. |
| Project-level test/probe | `oracle/rust-tests/tests/oracle_contracts.rs` |
| Candidate checkout owner under test | `stevenarella/protocol/src/protocol/packet.rs` |
| Candidate evidence gap | Add runtime keep-alive reaction and runtime Configuration-to-Play transition proof. |

## Proven Slice

The currently proven compatibility slices are `configuration_keepalive_codec`,
`configuration_keepalive_framed_dispatch`, and
`configuration_finish_framed_terminal`, backed by official jar output and
stored outside the reset-prone checkout. In the current run, the official
answers were regenerated and the manifest-declared Rust oracle tests passed:

```text
oracle/answers/775/configuration_keepalive_codec.answer.jsonl
oracle/answers/775/configuration_keepalive_framed_dispatch.answer.jsonl
oracle/answers/775/configuration_finish_framed_terminal.answer.jsonl
oracle/test-manifests/775/configuration_keepalive_codec.test-manifest.json
oracle/test-manifests/775/configuration_keepalive_framed_dispatch.test-manifest.json
oracle/test-manifests/775/configuration_finish_framed_terminal.test-manifest.json
oracle/rust-tests/tests/oracle_contracts.rs
```

## Not Proven

This phase is still not complete. The proven slice does not prove full
login/configuration runtime behavior, keep-alive response loop behavior,
runtime Configuration-to-Play transition behavior, or Play readiness.

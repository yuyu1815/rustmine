# network_login_configuration

| Field | Value |
|---|---|
| Lens position | 2 of 8 |
| Load claim | Client can satisfy official login/configuration wire contracts. |
| Evidence surface | Official jar codec/state evidence |
| Proof label | `partial` |
| Current proof | `configuration_keepalive_codec` has a regenerated jar-backed answer for one Configuration serverbound packet id/body, and the exact reset-proof Rust oracle test passes against the current Leafish checkout. |
| Project-level test/probe | `oracle/rust-tests/tests/oracle_contracts.rs` |
| Candidate checkout owner under test | `stevenarella/protocol/src/protocol/packet.rs` |
| Candidate evidence gap | Add Configuration packet dispatch/decode, runtime reaction, and Configuration completion cases. |

## Proven Slice

The only currently proven compatibility slice is
`configuration_keepalive_codec`, backed by official jar output and stored
outside the reset-prone checkout. In the current run, the official answer was
regenerated and the manifest-declared Rust oracle test passed:

```text
oracle/answers/775/configuration_keepalive_codec.answer.jsonl
oracle/test-manifests/775/configuration_keepalive_codec.test-manifest.json
oracle/rust-tests/tests/oracle_contracts.rs
```

## Not Proven

This phase is still not complete. The proven slice does not prove full
login/configuration runtime behavior, Configuration packet dispatch/decode,
keep-alive response loop behavior, Configuration completion, or
`Configuration -> Play` transition behavior.

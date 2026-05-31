# Play Update Tags Clientbound Framed Dispatch

| Field | Value |
|---|---|
| Case | `play_update_tags_clientbound_framed_dispatch` |
| Packet | `minecraft:update_tags` / `0x86` |
| Fixture | Empty registry tag map |
| Official body | Zero registry payload map count |
| Rust surface | `packet::packet_by_id(775, Play, Clientbound, 0x86, body)` |

This proof is packet-support only. It proves the official empty-map Play
fixture and Stevenarella dispatch/decode for that body. It does not prove
registry keys, tag payload entries, registry reload behavior, or tag semantics.

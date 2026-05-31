# Play Set Score Clientbound Framed Dispatch

| Field | Value |
|---|---|
| Case | `play_set_score_clientbound_framed_dispatch` |
| Packet | `minecraft:set_score` / `0x6e` |
| Fixture | Owner string, objective string, score `42`, absent optional display, absent optional number format |
| Official body | Owner string, objective string, score VarInt, false display marker, false number-format marker |
| Rust surface | `packet::packet_by_id(775, Play, Clientbound, 0x6e, body)` |

This proof is packet-support only. It proves the official no-optional
scoreboard field fixture and Stevenarella dispatch/decode for that body. It
does not prove scoreboard lifecycle, optional Component display bytes,
number-format semantics, or UI behavior.

# Play Set Time Clientbound Framed Dispatch

| Field | Value |
|---|---|
| Case | `play_set_time_clientbound_framed_dispatch` |
| Packet | `minecraft:set_time` / `0x71` |
| Fixture | `gameTime = 123456789`, empty clock update map |
| Official body | Big-endian long gameTime, then zero map count |
| Rust surface | `packet::packet_by_id(775, Play, Clientbound, 0x71, body)` |

This proof is packet-support only. It proves the official primitive-plus-empty
map fixture and Stevenarella dispatch/decode for that body. It does not prove
WorldClock entries, ClockNetworkState entries, or time runtime behavior.

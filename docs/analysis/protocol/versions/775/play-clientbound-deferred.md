# Protocol 775 Play Clientbound Deferred Rows

Purpose: make skipped Play clientbound rows explicit while the packet-support
proof loop takes safe GREEN/BLUE batches.

```text
0x18 custom_payload passes
  -> defer evidence-dependent rows
    -> prove context-free GREEN/BLUE rows
      -> return to deferred rows only with official fixture evidence
```

| Row | Packet | Status | Official-evidence reason | Stop boundary |
|---|---|---|---|---|
| `0x19` | `minecraft:damage_event` | deferred | Requires damage type/source registry semantics before a safe fixture can be named. | Do not invent registry ids, damage source fields, or initialized damage context. |
| `0x1a` | `minecraft:debug/block_value` | deferred | Debug block value generator path is not established. | Do not guess debug payload fields from names. |
| `0x1b` | `minecraft:debug/chunk_value` | deferred | Debug chunk value generator path is not established. | Do not guess debug payload fields from names. |
| `0x1c` | `minecraft:debug/entity_value` | deferred | Requires entity/debug fixture dependency evidence. | Do not fake entity/debug runtime state. |
| `0x1d` | `minecraft:debug/event` | deferred | Debug event fixture path is not established. | Do not guess debug event semantics. |
| `0x1e` | `minecraft:debug_sample` | deferred | Sample/debug semantics need official evidence before fixture selection. | Do not infer sample type or payload meaning from row name. |
| `0x1f` | `minecraft:delete_chat` | deferred | Requires chat signature/deletion context evidence. | Do not invent chat signature or deletion context. |
| `0x21` | `minecraft:disguised_chat` | deferred | Requires chat type/component context evidence. | Do not invent chat type, registry, or display context. |
| `0x22` | `minecraft:entity_event` | deferred | Requires entity-dependent official construction evidence. | Do not fake initialized entity existence or entity event semantics. |
| `0x24` | `minecraft:explode` | deferred | Requires particle/sound/world payload dependency evidence. | Do not invent particle, sound, or world payload fields. |
| `0x27` | `minecraft:game_rule_values` | deferred | Requires game rule table/state dependency evidence. | Do not invent game rule table entries or state. |
| `0x28` | `minecraft:game_test_highlight_pos` | deferred | Requires game-test/debug display semantics evidence. | Do not guess game-test display semantics. |

These rows are not rejected. They are parked until an official jar-backed
fixture or initialized harness route can name the packet body without guessing.

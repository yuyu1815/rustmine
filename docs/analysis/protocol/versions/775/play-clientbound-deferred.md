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
| `0x2d` | `minecraft:level_chunk_with_light` | deferred YELLOW | Requires chunk, light, heightmap, block entity, and world data fixture evidence before a safe body can be named. | Do not fake initialized chunk/world/light state or copy a body from a previous version. |
| `0x2f` | `minecraft:level_particles` | deferred YELLOW | Requires particle registry/options and world effect fixture evidence. | Do not invent particle option payloads or registry ids. |
| `0x30` | `minecraft:light_update` | deferred YELLOW | Requires chunk position, light update data, bitsets, and light array fixture evidence. | Do not guess sky/block light array layout or initialized lighting state. |
| `0x31` | `minecraft:login` | deferred YELLOW | Requires dimension, registry, game mode, spawn, and world context evidence. | Do not infer join-game body fields or registry-backed values from names. |
| `0x33` | `minecraft:map_item_data` | deferred YELLOW | Requires map id/decorations/color patch fixture evidence. | Do not invent map decoration or color-patch semantics. |
| `0x34` | `minecraft:merchant_offers` | deferred YELLOW | Requires `MerchantOffers`, `ItemStack`, trade, and component/registry fixture evidence. | Do not fake non-empty item or trade payloads. |
| `0x37` | `minecraft:move_minecart_along_track` | deferred YELLOW | Requires minecart step/entity/track fixture evidence. | Do not guess minecart interpolation or track-step body semantics. |
| `0x3b` | `minecraft:open_screen` | deferred YELLOW | Requires menu registry/type and trusted Component fixture evidence. | Do not invent menu registry ids or title component bytes. |
| `0x3c` | `minecraft:open_sign_editor` | deferred YELLOW | Requires sign/block position and front-text behavior evidence. | Do not infer sign state or world block semantics from previous-version witnesses. |

These rows are not rejected. They are parked until an official jar-backed
fixture or initialized harness route can name the packet body without guessing.

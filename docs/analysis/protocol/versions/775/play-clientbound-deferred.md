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
| `0x3f` | `minecraft:place_ghost_recipe` | deferred YELLOW | Official codec uses `RecipeDisplay`, which can carry recipe/item display data. | Do not invent recipe display payloads, item contents, or registry-backed recipe semantics. |
| `0x41` | `minecraft:player_chat` | deferred YELLOW | Official codec uses message signatures, signed message body, optional Component, filter mask, and bound chat type. | Do not invent chat signatures, chat type context, signed body, or component bytes. |
| `0x44` | `minecraft:player_combat_kill` | deferred YELLOW | Official codec uses player id plus trusted Component death message. | Do not infer entity/player death context or message component bytes. |
| `0x46` | `minecraft:player_info_update` | deferred YELLOW | Official codec uses action sets and player info entries backed by profile, listed, latency, game mode, display name, chat session, and list order data. | Do not invent player list entries, profile data, chat session data, or display components. |
| `0x47` | `minecraft:player_look_at` | deferred YELLOW | Although a coordinate-only constructor exists, official body can include entity targeting and anchor semantics; fixture policy needs a specific no-entity proof before implementation. | Do not infer entity targeting or anchor behavior from previous-version witnesses. |
| `0x48` | `minecraft:player_position` | deferred YELLOW | Official codec uses `PositionMoveRotation` plus relative flags and teleport id, which affects player/world movement state. | Do not infer teleport/movement semantics or relative flag policy from packet name. |
| `0x49` | `minecraft:player_rotation` | deferred YELLOW | Official codec uses rotation floats plus relative flags, which affects player movement state. | Do not infer movement/rotation semantics from previous-version witnesses. |
| `0x4a` | `minecraft:recipe_book_add` | deferred YELLOW | Official codec uses recipe book entries and recipe display data. | Do not invent recipe display ids, recipe contents, or notification/highlight flags. |
| `0x4b` | `minecraft:recipe_book_remove` | deferred YELLOW | Official codec uses recipe display ids. | Do not invent recipe display ids or recipe book state. |
| `0x4c` | `minecraft:recipe_book_settings` | deferred YELLOW | Official codec uses `RecipeBookSettings` state. | Do not infer recipe book category/settings bits without official fixture evidence. |
| `0x4e` | `minecraft:remove_mob_effect` | deferred YELLOW | Official codec uses entity id plus `MobEffect` registry holder. | Do not invent mob-effect registry ids or initialized entity/effect state. |
| `0x4f` | `minecraft:reset_score` | deferred YELLOW | Official codec uses owner string plus nullable objective name, which is scoreboard state. | Do not infer scoreboard owner/objective semantics without a named fixture policy. |
| `0x50` | `minecraft:resource_pack_pop` | deferred YELLOW | Official common packet codec is context-free, but Play-phase resource-pack stack behavior needs a policy separate from Configuration proof reuse. | Do not reuse Configuration resource-pack proof as Play runtime behavior. |
| `0x51` | `minecraft:resource_pack_push` | deferred YELLOW | Official common packet codec includes UUID, URL, hash, required flag, and optional trusted Component prompt. | Do not invent prompt component bytes, URL/hash policy, or Play resource-pack UI behavior. |
| `0x52` | `minecraft:respawn` | deferred YELLOW | Official codec uses `CommonPlayerSpawnInfo` plus data-to-keep flags, requiring dimension/world/game mode/spawn context evidence. | Do not invent dimension, registry, spawn, or respawn state. |
| `0x54` | `minecraft:section_blocks_update` | deferred YELLOW | Official codec uses section position plus block-state update data from chunk section/block-state context. | Do not fake chunk section contents or block-state ids. |
| `0x56` | `minecraft:server_data` | deferred YELLOW | Official codec uses trusted Component MOTD and optional icon bytes, which needs UI/content fixture policy. | Do not invent MOTD component or icon payload policy. |
| `0x57` | `minecraft:set_action_bar_text` | deferred YELLOW | Official codec uses trusted Component text. | Do not invent action-bar component bytes or UI behavior. |
| `0x5d` | `minecraft:set_camera` | deferred YELLOW | Official public constructor is entity-backed and the packet changes camera target state. | Do not fake camera entity existence or spectate state. |
| `0x60` | `minecraft:set_cursor_item` | promoted BLUE | Official `ClientboundSetCursorItemPacket(ItemStack.EMPTY)` generated a jar-backed Play answer with one empty ItemStack VarInt count marker `0`; official `ItemStack$1` only enters item registry/component patch decoding for positive counts. | Promoted only for `ItemStack.EMPTY`; reject positive ItemStack counts before item registry/component bytes. |
| `0x61` | `minecraft:set_default_spawn_position` | deferred YELLOW | Official codec uses `LevelData.RespawnData` with position/angle spawn semantics. | Do not infer spawn state or compass/player behavior. |
| `0x62` | `minecraft:set_display_objective` | promoted BLUE | Official `ClientboundSetDisplayObjectivePacket(DisplaySlot.LIST, null)` generated a jar-backed Play answer with a clear-slot body: display slot id plus empty objective name. | Promoted only for the null Objective clear-slot fixture; do not infer scoreboard Objective state or display behavior. |
| `0x63` | `minecraft:set_entity_data` | deferred YELLOW | Official codec uses SynchedEntityData values and serializers. | Do not invent entity metadata serializers or initialized entity state. |
| `0x64` | `minecraft:set_entity_link` | deferred YELLOW | Official public constructor is entity-backed and represents entity link/leash relationship state. | Do not fake linked entity existence or relationship semantics. |
| `0x66` | `minecraft:set_equipment` | promoted BLUE | Official `ClientboundSetEquipmentPacket(123, List.of(Pair.of(EquipmentSlot.MAINHAND, ItemStack.EMPTY)))` generated a jar-backed Play answer with entity id, one MAINHAND slot byte, and one empty ItemStack VarInt count marker `0`. | Promoted only for the one-entry MAINHAND `ItemStack.EMPTY` fixture; reject multi-entry continuation and positive ItemStack counts. Do not infer entity existence or equipment behavior. |
| `0x6a` | `minecraft:set_objective` | deferred YELLOW | Official public constructor is Objective-backed and add/change paths use trusted Component and optional number format. | Do not invent Objective construction, display Component bytes, render type, number format, or scoreboard lifecycle. |
| `0x6b` | `minecraft:set_passengers` | deferred YELLOW | Official public constructor is entity-backed and writes vehicle id plus passenger id array. | Do not fake vehicle/passenger entity existence or relationship semantics. |
| `0x6c` | `minecraft:set_player_inventory` | promoted BLUE | Official `ClientboundSetPlayerInventoryPacket(7, ItemStack.EMPTY)` generated a jar-backed Play answer with slot VarInt plus one empty ItemStack VarInt count marker `0`; official `ItemStack$1` only enters item registry/component patch decoding for positive counts. | Promoted only for slot 7 plus `ItemStack.EMPTY`; reject positive ItemStack counts before item registry/component bytes. Do not infer player inventory state. |
| `0x6d` | `minecraft:set_player_team` | deferred YELLOW | Official codec uses team actions, optional parameters, player collections, Components, colors, and visibility/collision names. | Do not invent team lifecycle, player membership, Component bytes, colors, visibility, or collision policy. |
| `0x6e` | `minecraft:set_score` | promoted BLUE | Official `ClientboundSetScorePacket(owner, objective, score, Optional.empty(), Optional.empty())` generated a jar-backed Play answer with plain strings, score VarInt, and two false optional markers. | Promoted only for the no-optional fixture; do not infer scoreboard lifecycle, optional Component display, or number-format semantics. |
| `0x70` | `minecraft:set_subtitle_text` | promoted BLUE | Official `ClientboundSetSubtitleTextPacket(Component.literal("rustmine subtitle"))` generated a jar-backed Play answer with a simple NBT string Component body. | Promoted only for the `Component.literal(String)` simple string fixture; do not infer rich Component, subtitle UI, or title runtime behavior. |
| `0x71` | `minecraft:set_time` | promoted BLUE | Official `ClientboundSetTimePacket(long, Map.of())` generated a jar-backed Play answer with one long and a zero clock-update map count. | Promoted only for the empty clock-update map fixture; do not infer `WorldClock`, `ClockNetworkState`, or time runtime semantics. |
| `0x72` | `minecraft:set_title_text` | promoted BLUE | Official `ClientboundSetTitleTextPacket(Component.literal("rustmine title"))` generated a jar-backed Play answer with a simple NBT string Component body. | Promoted only for the `Component.literal(String)` simple string fixture; do not infer rich Component, title UI, or title runtime behavior. |
| `0x74` | `minecraft:sound_entity` | deferred YELLOW | Official `ClientboundSoundEntityPacket` uses `SoundEvent.STREAM_CODEC`, `SoundSource`, entity id, volume, pitch, and seed. | Do not invent sound registry holder values or entity runtime context. |
| `0x75` | `minecraft:sound` | deferred YELLOW | Official `ClientboundSoundPacket` uses `SoundEvent.STREAM_CODEC`, `SoundSource`, position ints, volume, pitch, and seed. | Do not invent sound registry holder values or world sound context. |
| `0x77` | `minecraft:stop_sound` | promoted GREEN | Official `ClientboundStopSoundPacket(null, null)` generated a jar-backed Play answer with one flags byte `0`; no registry holder was required for that fixture. | Promoted only for the null/null fixture; do not infer named source or named sound behavior. |
| `0x78` | `minecraft:store_cookie` | promoted BLUE | Official Play row uses common `ClientboundStoreCookiePacket` with Identifier plus bounded byte-array payload; a Play-specific answer and Rust mapping now prove one fixture. | Do not infer runtime cookie storage policy from this packet-support proof. |
| `0x79` | `minecraft:system_chat` | promoted BLUE | Official `ClientboundSystemChatPacket(Component.literal("rustmine system chat"), false)` generated a jar-backed Play answer with a simple NBT string Component body plus false overlay byte. | Promoted only for the `Component.literal(String)` plus `overlay=false` fixture; do not infer rich Component, signed chat, chat HUD, or overlay-true behavior. |
| `0x7a` | `minecraft:tab_list` | promoted BLUE | Official `ClientboundTabListPacket(Component.literal(header), Component.literal(footer))` generated a jar-backed Play answer with two simple NBT string Component bodies. | Promoted only for simple `Component.literal(String)` header/footer fixtures; do not infer rich Component or player-list UI behavior. |
| `0x7b` | `minecraft:tag_query` | deferred YELLOW | Official codec uses VarInt transaction id plus NBT via `readNbt`/`writeNbt`. | Do not invent NBT payload policy or query context. |
| `0x7d` | `minecraft:teleport_entity` | deferred YELLOW | Official codec uses entity id, `PositionMoveRotation.STREAM_CODEC`, relative flags, and onGround. | Do not infer entity teleport/movement semantics or relative flag policy. |
| `0x7e` | `minecraft:test_instance_block_status` | deferred YELLOW | Official codec uses Component status plus optional `Vec3i` size for test-instance/debug behavior. | Do not invent Component bytes or game-test block semantics. |
| `0x7f` | `minecraft:ticking_state` | promoted GREEN | Official `ClientboundTickingStatePacket(float, boolean)` generated a jar-backed Play answer with one float and one boolean. | Promoted only as primitive packet support; do not claim world ticking runtime behavior. |
| `0x80` | `minecraft:ticking_step` | promoted GREEN | Official `ClientboundTickingStepPacket(int)` generated a jar-backed Play answer with one VarInt tick step count. | Promoted only as primitive packet support; do not claim tick-manager runtime behavior. |
| `0x81` | `minecraft:transfer` | promoted BLUE | Official Play row uses common `ClientboundTransferPacket` with host string and port VarInt; a Play-specific answer and Rust mapping now prove one fixture. | Do not infer runtime transfer/reconnect handling from this packet-support proof. |
| `0x82` | `minecraft:update_advancements` | deferred YELLOW | Official codec uses advancement holders, identifiers, advancement progress, and show/reset flags. | Do not invent advancement tree/progress semantics. |
| `0x83` | `minecraft:update_attributes` | deferred YELLOW | Official codec uses entity id plus attribute snapshots with attribute registry holders and modifiers. | Do not invent attribute registry ids or initialized entity attributes. |
| `0x84` | `minecraft:update_mob_effect` | deferred YELLOW | Official codec uses entity id plus `MobEffect.STREAM_CODEC`, amplifier, duration, and flags. | Do not invent mob-effect registry holders or entity/effect state. |
| `0x85` | `minecraft:update_recipes` | deferred YELLOW | Official codec uses recipe property sets and selectable recipe data. | Do not invent recipe/item display contents. |
| `0x86` | `minecraft:update_tags` | promoted BLUE | Official Play row uses common `ClientboundUpdateTagsPacket`; `Map.of()` generated a jar-backed Play answer with a zero registry payload count. | Promoted only for the empty registry tag map fixture; do not infer registry keys, tag payloads, or registry reload behavior. |
| `0x87` | `minecraft:projectile_power` | deferred YELLOW | Official `ClientboundProjectilePowerPacket(int, double)` is primitive on the wire, but the packet is entity/projectile targeted. | Do not infer projectile entity runtime context or acceleration behavior from a primitive body alone. |
| `0x88` | `minecraft:custom_report_details` | promoted BLUE | Official Play row uses common `ClientboundCustomReportDetailsPacket`; the empty map fixture generated a jar-backed Play answer with a zero count body. | Promoted only for the empty map fixture; do not infer non-empty report detail entry semantics. |
| `0x89` | `minecraft:server_links` | promoted BLUE | Official Play row uses common `ClientboundServerLinksPacket`; the empty list fixture generated a jar-backed Play answer with a zero count body. | Promoted only for the empty list fixture; do not infer non-empty server link entry semantics or UI behavior. |
| `0x8a` | `minecraft:waypoint` | deferred YELLOW | Official `ClientboundTrackedWaypointPacket` uses an operation plus `TrackedWaypoint.STREAM_CODEC` with waypoint/world data. | Do not invent waypoint operation, icon, UUID, position, chunk, or azimuth semantics. |
| `0x8b` | `minecraft:clear_dialog` | promoted BLUE | Official Play row uses common `ClientboundClearDialogPacket.INSTANCE`; singleton fixture generated a jar-backed Play answer with an empty body. | Promoted only as packet support; do not infer dialog UI behavior or `show_dialog` semantics. |
| `0x8c` | `minecraft:show_dialog` | deferred YELLOW | Official `ClientboundShowDialogPacket` uses a dialog holder/stream codec and context-free dialog codec, requiring dialog fixture policy. | Do not invent dialog holder, dialog contents, or UI behavior. |

These rows are not rejected. They are parked until an official jar-backed
fixture or initialized harness route can name the packet body without guessing.
The former safe BLUE border rows `0x58`-`0x5c` have been promoted to
jar-backed packet-support proofs and are no longer deferred. Safe BLUE row
`0x65` has also been promoted to a jar-backed packet-support proof package; its
current status is a Stevenarella dispatch mismatch, not a deferred fixture gap.

## Batch Confirmation

The `0x2e` / `0x32` / `0x35` / `0x36` / `0x38` safe batch did not implement
the skipped YELLOW rows. `0x2d`, `0x2f`, `0x30`, `0x31`, `0x33`, `0x34`, and
`0x37` remain deferred for the reasons above. No packet ids, registry-backed
payloads, chunk/light/world fixtures, map/trade/item payloads, or minecart
track-step bodies were inferred while crossing those rows.

The `0x39` / `0x3a` / `0x3d` safe batch did not implement the skipped YELLOW
rows `0x3b` and `0x3c`. `0x3b` remains parked because its official codec uses
the menu registry plus trusted `Component` title data; `0x3c` remains parked
because its official fixture needs sign/block position and front-text behavior
evidence. No menu registry id, title component bytes, sign state, block
semantics, or world state were inferred while crossing those rows.

The `0x3e` / `0x40` / `0x42` / `0x43` / `0x4d` safe batch did not implement
the skipped YELLOW rows `0x3f`, `0x41`, `0x44`, `0x46`-`0x4c`, and
`0x4e`-`0x51`. These rows remain parked for the official-evidence reasons
above. No recipe display, chat signature, player-info entry, player movement,
scoreboard, mob-effect registry, resource-pack prompt, or Play resource-pack
runtime behavior was inferred while crossing those rows.

The `0x45` / `0x53` / `0x55` / `0x5e` / `0x5f` safe batch implemented the
previously confirmed GREEN `player_info_remove` row plus selected GREEN/BLUE
rows from the `0x52`-started cartography pass. The follow-up border batch
promoted `0x58`-`0x5c` from safe BLUE deferred rows to official jar-backed
packet-support proofs. The skipped YELLOW rows `0x52`, `0x54`, `0x56`-`0x57`,
`0x5d`, and `0x60`-`0x64` remain parked for the official-evidence reasons
above. Safe BLUE row `0x65` has now moved out of this deferred list into an
official answer plus Rust mismatch package. No respawn, component UI,
chunk-section, camera, item stack, spawn, scoreboard, entity metadata,
entity-link, world-border runtime, warning UI, or entity-motion runtime
behavior was inferred while crossing those rows.

The `0x65` / `0x67` / `0x68` / `0x69` / `0x6f` safe batch promoted only those
five named rows into jar-backed packet-support packages. The skipped rows
`0x60`-`0x64`, `0x66`, and `0x70`-`0x72` were not implemented or promoted in
this batch. No item stack, spawn, scoreboard, entity metadata, entity link,
equipment, recipe, custom sound, or entity-sound fixture was inferred while
crossing the batch boundary.

The `0x73` / `0x76` / `0x7c` safe batch promoted only
`set_titles_animation`, `start_configuration`, and `take_item_entity` into
jar-backed packet-support packages. The skipped rows `0x74`-`0x75`,
`0x77`-`0x7b`, `0x7d`-`0x86` were classified from explicit official bytecode
evidence, but only GREEN rows selected for this batch were implemented. No
sound registry holder, Component, NBT, entity teleport, game-test, ticking
runtime, common-packet cross-state ownership, advancement, attribute, mob
effect, recipe, or tag semantics were inferred while crossing this batch.

The `0x77` / `0x78` / `0x7f` / `0x80` / `0x81` safe follow-up batch promoted
only `stop_sound` null/null, `store_cookie`, `ticking_state`, `ticking_step`,
and `transfer` into jar-backed packet-support packages. The skipped rows
`0x79`-`0x7b`, `0x7d`-`0x7e`, and `0x82`-`0x86` remain parked for the
official-evidence reasons above. No named source/sound, Component, NBT, entity
teleport, game-test, advancement, attribute, mob-effect, recipe, tag, cookie
storage, transfer runtime, or tick-manager runtime behavior was inferred while
crossing this batch.

The `0x88` / `0x89` / `0x8b` safe post-`0x86` batch promoted only
`custom_report_details` empty map, `server_links` empty list, and
`clear_dialog` singleton into jar-backed packet-support packages. The skipped
rows `0x87`, `0x8a`, and `0x8c` remain parked for the official-evidence
reasons above. The official Play clientbound table currently ends at `0x8c`,
so future packet-support work should either return to parked rows with exact
official fixture evidence or move to another route. No projectile entity,
waypoint, dialog holder, non-empty report details, non-empty server links, or
dialog UI behavior was inferred while crossing this batch.

## Parked Row Dependency Buckets

This map is for returning to YELLOW rows after the first pass reached the
official Play clientbound table end at `0x8c`.

| Bucket | Rows | Fixture-policy option | Current route |
|---|---|---|---|
| Empty map / absent optional / clear field | `0x62`, `0x6e`, `0x71`, `0x86` | Official constructor must generate an empty/absent body branch and Rust must reject unsupported non-empty variants. | Promoted in the parked-row follow-up batch. |
| ItemStack / item component | recipe-bearing `0x85` | Simple empty ItemStack rows `0x60`, `0x66`, and `0x6c` have been promoted only for `ItemStack.EMPTY` fixtures. `0x85` is not an optional-ItemStack row; it needs recipe-map/selectable-recipe fixture policy. | Partly promoted; recipe row still parked. |
| Scoreboard and teams beyond no-optional field packets | `0x6a`, `0x6d` | Needs Objective/Team construction policy without inventing Components, number formats, colors, visibility, or player memberships. | Still parked. |
| Entity relationship / metadata / movement / projectile | `0x63`, `0x64`, `0x6b`, `0x74`, `0x7d`, `0x83`, `0x84`, `0x87` | Needs entity-id-only policy or initialized entity/runtime fixture evidence per row. | Still parked. |
| Trusted Component / NBT / UI text | `0x7b`, `0x7e`, `0x8c` | Needs NBT/query, game-test, or dialog fixture policy, not just row names. Simple `Component.literal(String)` rows `0x70`, `0x72`, `0x79`, and `0x7a` have been promoted only for bounded simple string fixtures. | Partly promoted; NBT/dialog rows still parked. |
| Registry / world / game data | `0x61`, `0x75`, `0x82`, `0x85`, `0x8a` | Needs registry/world/advancement/recipe/waypoint fixture policy or initialized harness. | Still parked. |

The parked-row follow-up batch promoted only `0x62` clear
`set_display_objective`, `0x6e` no-optional `set_score`, `0x71` empty-map
`set_time`, and `0x86` empty-map `update_tags` into jar-backed
packet-support packages. The next Component text follow-up promoted only
`0x70` simple `set_subtitle_text`, `0x72` simple `set_title_text`, `0x79`
simple `system_chat` with false overlay, and `0x7a` simple `tab_list`
header/footer into jar-backed packet-support packages. No ItemStack,
equipment, rich Component, NBT query payload, entity metadata, sound registry,
advancement, recipe, waypoint, non-empty tag, WorldClock, ClockNetworkState,
scoreboard Objective, scoreboard Team, or dialog semantics were inferred while
crossing these batches.

The empty ItemStack follow-up promoted only `0x60` `set_cursor_item`,
`0x66` one-entry `set_equipment`, and `0x6c` `set_player_inventory` into
jar-backed packet-support packages. These fixtures use official
`ItemStack.EMPTY`, whose official optional stream codec writes a VarInt count
marker `0`; positive counts enter item registry and component patch decoding
and are rejected by the scoped Rust mapping. No non-empty ItemStack, item
registry, component patch, equipment runtime, entity existence, player
inventory state, recipe, or client-load behavior was inferred.

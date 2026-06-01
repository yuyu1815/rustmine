use super::*;
use crate::protocol::mapped_packet::handshake::serverbound::Handshake;
use crate::protocol::mapped_packet::login::clientbound::{
    EncryptionRequest, EncryptionRequest_ShouldAuthenticate, LoginCookieRequest, LoginDisconnect,
    LoginPluginRequest, LoginSuccess_String, LoginSuccess_UUID, SetInitialCompression,
};
use crate::protocol::mapped_packet::login::serverbound::{
    EncryptionResponse, LoginAcknowledged, LoginCookieResponse, LoginPluginResponse, LoginStart,
};
use crate::protocol::mapped_packet::play::clientbound::{
    AcknowledgePlayerDigging, Advancements, Animation, BlockAction, BlockBreakAnimation,
    BlockChange, BossBar, BundleDelimiterClientbound, Camera, ChangeGameState, ChunkData,
    ChunkDataBulk, ChunkDataBulk_17, ChunkData_17, ChunkData_Biomes3D, ChunkData_Biomes3D_bool,
    ChunkData_Biomes3D_i32, ChunkData_HeightMap, ChunkData_NoEntities, ChunkData_NoEntities_u16,
    ChunkUnload, CoFHLib_SendUUID, CollectItem, CombatEvent, ConfirmTransaction,
    CraftRecipeResponse, DeclareCommands, DeclareRecipes, Disconnect, Effect, Entity, EntityAction,
    EntityAttach, EntityDestroy, EntityEffect, EntityEquipment_Array, EntityEquipment_Single,
    EntityHeadLook, EntityLook, EntityLookAndMove, EntityMetadata, EntityMove, EntityProperties,
    EntityRemoveEffect, EntitySoundEffect, EntityStatus, EntityTeleport, EntityUpdateNBT,
    EntityUsedBed, EntityVelocity, Explosion, FacePlayer, JoinGame, KeepAliveClientbound, Maps,
    MultiBlockChange, NBTQueryResponse, NamedSoundEffect, OpenBook, Particle,
    PlayAddEntityClientbound, PlayAnimateClientbound, PlayAwardStatsClientbound,
    PlayBlockChangedAckClientbound, PlayBlockDestructionClientbound,
    PlayBlockEntityDataClientbound, PlayBlockEventClientbound, PlayBlockUpdateClientbound,
    PlayChunkBatchFinishedClientbound, PlayChunkBatchStartClientbound, PlayChunksBiomesClientbound,
    PlayClearDialogClientbound, PlayClearTitlesClientbound, PlayCommandSuggestionsClientbound,
    PlayContainerSetContentClientbound, PlayContainerSetSlotClientbound,
    PlayCookieRequestClientbound, PlayCooldownClientbound, PlayCustomChatCompletionsClientbound,
    PlayCustomReportDetailsClientbound, PlayEntityPositionSyncClientbound,
    PlayForgetLevelChunkClientbound, PlayGameEventClientbound, PlayHurtAnimationClientbound,
    PlayInitializeBorderClientbound, PlayLowDiskSpaceWarningClientbound,
    PlayMountScreenOpenClientbound, PlayPingClientbound, PlayPlayerCombatEndClientbound,
    PlayPlayerCombatEnterClientbound, PlayPlayerInfoRemoveClientbound, PlayPongResponseClientbound,
    PlayProjectilePowerClientbound, PlayServerLinksClientbound, PlaySetBorderCenterClientbound,
    PlaySetBorderLerpSizeClientbound, PlaySetBorderSizeClientbound,
    PlaySetBorderWarningDelayClientbound, PlaySetBorderWarningDistanceClientbound,
    PlaySetCursorItemClientbound, PlaySetDefaultSpawnPositionClientbound,
    PlaySetDisplayObjectiveClientbound, PlaySetEntityDataClientbound, PlaySetEquipmentClientbound,
    PlaySetObjectiveClientbound, PlaySetPlayerInventoryClientbound, PlaySetPlayerTeamClientbound,
    PlaySetScoreClientbound, PlaySetSubtitleTextClientbound, PlaySetTimeClientbound,
    PlaySetTitleTextClientbound, PlaySetTitlesAnimationClientbound,
    PlayStartConfigurationClientbound, PlayStoreCookieClientbound, PlaySystemChatClientbound,
    PlayTabListClientbound, PlayTagQueryClientbound, PlayTeleportEntityClientbound,
    PlayTestInstanceBlockStatusClientbound, PlayTickingStateClientbound,
    PlayTickingStepClientbound, PlayTransferClientbound, PlayUpdateAdvancementsClientbound,
    PlayUpdateAttributesClientbound, PlayUpdateRecipesClientbound, PlayUpdateTagsClientbound,
    PlayWaypointClientbound, PlayerAbilities, PlayerInfo, PlayerInfo_String,
    PlayerListHeaderFooter, PluginMessageClientbound, ResourcePackSend, Respawn, ScoreboardDisplay,
    ScoreboardObjective, SelectAdvancementTab, ServerDifficulty, ServerMessage, SetCompression,
    SetCooldown, SetCurrentHotbarSlot, SetExperience, SetPassengers, SignEditorOpen, SoundEffect,
    SpawnExperienceOrb, SpawnGlobalEntity, SpawnMob, SpawnObject, SpawnPainting, SpawnPlayer,
    SpawnPosition, Statistics, StopSound, TabCompleteReply, Tags, Teams, TeleportPlayer,
    TimeUpdate, Title, TradeList, UnlockRecipes, UpdateBlockEntity, UpdateHealth, UpdateLight,
    UpdateScore, UpdateSign, UpdateViewDistance, UpdateViewPosition, VehicleTeleport, WindowClose,
    WindowItems, WindowOpen, WindowOpenHorse, WindowProperty, WindowSetSlot, WorldBorder,
};
use crate::protocol::mapped_packet::play::serverbound::{
    AdvancementTab, ArmSwing, ChatMessage, ClickWindow, ClickWindowButton, ClientAbilities,
    ClientSettings, ClientStatus, CloseWindow, ConfirmTransactionServerbound, CraftRecipeRequest,
    CraftingBookData, CreativeInventoryAction, EditBook, EnchantItem, GenerateStructure,
    HeldItemChange, KeepAliveServerbound, LockDifficulty, NameItem, PickItem, Player, PlayerAction,
    PlayerBlockPlacement, PlayerDigging, PlayerLook, PlayerPosition, PlayerPositionLook,
    PluginMessageServerbound, QueryBlockNBT, QueryEntityNBT, ResourcePackStatus, SelectTrade,
    SetBeaconEffect, SetDifficulty, SetDisplayedRecipe, SetRecipeBookState, SetSign,
    SpectateTeleport, SteerBoat, SteerVehicle, TabComplete, TeleportConfirm, UpdateCommandBlock,
    UpdateCommandBlockMinecart, UpdateJigsawBlock_Joint, UpdateJigsawBlock_Type,
    UpdateStructureBlock, UseEntity, UseItem, VehicleMove,
};
use crate::protocol::mapped_packet::status::clientbound::{StatusPong, StatusResponse};
use crate::protocol::mapped_packet::status::serverbound::{StatusPing, StatusRequest};
use crate::protocol::packet::Hand;
use crate::protocol::{mapped_packet, packet};
use std::io::Cursor;

pub trait MappablePacket {
    fn map(self) -> MappedPacket;
}

impl MappablePacket for packet::Packet {
    fn map(self) -> MappedPacket {
        match self {
            packet::Packet::BundleDelimiterClientbound(delimiter) => {
                mapped_packet::MappedPacket::BundleDelimiterClientbound(
                    BundleDelimiterClientbound {
                        empty: delimiter.empty,
                    },
                )
            }
            packet::Packet::PlayAddEntityClientbound(add_entity) => {
                mapped_packet::MappedPacket::PlayAddEntityClientbound(PlayAddEntityClientbound {
                    entity_id: add_entity.entity_id.0,
                    uuid: add_entity.uuid,
                    ty: add_entity.ty.0,
                    x: add_entity.x,
                    y: add_entity.y,
                    z: add_entity.z,
                    movement_lp_zero: add_entity.movement_lp_zero.0,
                    x_rot: add_entity.x_rot,
                    y_rot: add_entity.y_rot,
                    y_head_rot: add_entity.y_head_rot,
                    data: add_entity.data.0,
                })
            }
            packet::Packet::PlayAnimateClientbound(animate) => {
                mapped_packet::MappedPacket::PlayAnimateClientbound(PlayAnimateClientbound {
                    entity_id: animate.entity_id.0,
                    action: animate.action,
                })
            }
            packet::Packet::PlayAwardStatsClientbound(stats) => {
                mapped_packet::MappedPacket::PlayAwardStatsClientbound(PlayAwardStatsClientbound {
                    stat_count: stats.stat_count.0,
                })
            }
            packet::Packet::PlayBlockChangedAckClientbound(ack) => {
                mapped_packet::MappedPacket::PlayBlockChangedAckClientbound(
                    PlayBlockChangedAckClientbound {
                        sequence: ack.sequence.0,
                    },
                )
            }
            packet::Packet::PlayBlockDestructionClientbound(destruction) => {
                mapped_packet::MappedPacket::PlayBlockDestructionClientbound(
                    PlayBlockDestructionClientbound {
                        breaker_id: destruction.breaker_id.0,
                        location: destruction.location,
                        progress: destruction.progress,
                    },
                )
            }
            packet::Packet::PlayBlockEntityDataClientbound(block_entity_data) => {
                mapped_packet::MappedPacket::PlayBlockEntityDataClientbound(
                    PlayBlockEntityDataClientbound {
                        location: block_entity_data.location,
                        block_entity_type: block_entity_data.block_entity_type.0,
                        nbt_tag_type: block_entity_data.nbt_tag_type,
                        tag: block_entity_data.tag,
                    },
                )
            }
            packet::Packet::PlayBlockEventClientbound(block_event) => {
                mapped_packet::MappedPacket::PlayBlockEventClientbound(PlayBlockEventClientbound {
                    location: block_event.location,
                    event_type: block_event.event_type,
                    event_data: block_event.event_data,
                    block: block_event.block.0,
                })
            }
            packet::Packet::PlayBlockUpdateClientbound(block_update) => {
                mapped_packet::MappedPacket::PlayBlockUpdateClientbound(
                    PlayBlockUpdateClientbound {
                        location: block_update.location,
                        block_state: block_update.block_state.0,
                    },
                )
            }
            packet::Packet::PlayChunkBatchFinishedClientbound(chunk_batch_finished) => {
                mapped_packet::MappedPacket::PlayChunkBatchFinishedClientbound(
                    PlayChunkBatchFinishedClientbound {
                        batch_size: chunk_batch_finished.batch_size.0,
                    },
                )
            }
            packet::Packet::PlayChunkBatchStartClientbound(chunk_batch_start) => {
                mapped_packet::MappedPacket::PlayChunkBatchStartClientbound(
                    PlayChunkBatchStartClientbound {
                        empty: chunk_batch_start.empty,
                    },
                )
            }
            packet::Packet::PlayChunksBiomesClientbound(chunks_biomes) => {
                mapped_packet::MappedPacket::PlayChunksBiomesClientbound(
                    PlayChunksBiomesClientbound {
                        chunk_biome_data: chunks_biomes.chunk_biome_data.data,
                    },
                )
            }
            packet::Packet::PlayClearTitlesClientbound(clear_titles) => {
                mapped_packet::MappedPacket::PlayClearTitlesClientbound(
                    PlayClearTitlesClientbound {
                        reset_times: clear_titles.reset_times,
                    },
                )
            }
            packet::Packet::PlayCommandSuggestionsClientbound(command_suggestions) => {
                mapped_packet::MappedPacket::PlayCommandSuggestionsClientbound(
                    PlayCommandSuggestionsClientbound {
                        id: command_suggestions.id.0,
                        start: command_suggestions.start.0,
                        length: command_suggestions.length.0,
                        suggestion_count: command_suggestions.suggestion_count.0,
                    },
                )
            }
            packet::Packet::PlayContainerSetContentClientbound(set_content) => {
                mapped_packet::MappedPacket::PlayContainerSetContentClientbound(
                    PlayContainerSetContentClientbound {
                        container_id: set_content.container_id.0,
                        state_id: set_content.state_id.0,
                        items: set_content.items.data,
                        carried_item: set_content.carried_item,
                    },
                )
            }
            packet::Packet::PlayContainerSetSlotClientbound(set_slot) => {
                mapped_packet::MappedPacket::PlayContainerSetSlotClientbound(
                    PlayContainerSetSlotClientbound {
                        container_id: set_slot.container_id.0,
                        state_id: set_slot.state_id.0,
                        slot: set_slot.slot,
                        item: set_slot.item,
                    },
                )
            }
            packet::Packet::PlayCookieRequestClientbound(cookie_request) => {
                mapped_packet::MappedPacket::PlayCookieRequestClientbound(
                    PlayCookieRequestClientbound {
                        key: cookie_request.key,
                    },
                )
            }
            packet::Packet::PlayCooldownClientbound(cooldown) => {
                mapped_packet::MappedPacket::PlayCooldownClientbound(PlayCooldownClientbound {
                    cooldown_group: cooldown.cooldown_group,
                    duration: cooldown.duration.0,
                })
            }
            packet::Packet::PlayCustomChatCompletionsClientbound(completions) => {
                mapped_packet::MappedPacket::PlayCustomChatCompletionsClientbound(
                    PlayCustomChatCompletionsClientbound {
                        action: completions.action.0,
                        entries: completions.entries.data,
                    },
                )
            }
            packet::Packet::PlayEntityPositionSyncClientbound(sync) => {
                mapped_packet::MappedPacket::PlayEntityPositionSyncClientbound(
                    PlayEntityPositionSyncClientbound {
                        entity_id: sync.entity_id.0,
                        x: sync.x,
                        y: sync.y,
                        z: sync.z,
                        delta_x: sync.delta_x,
                        delta_y: sync.delta_y,
                        delta_z: sync.delta_z,
                        y_rot: sync.y_rot,
                        x_rot: sync.x_rot,
                        on_ground: sync.on_ground,
                    },
                )
            }
            packet::Packet::PlayForgetLevelChunkClientbound(forget) => {
                mapped_packet::MappedPacket::PlayForgetLevelChunkClientbound(
                    PlayForgetLevelChunkClientbound {
                        chunk_pos: forget.chunk_pos,
                    },
                )
            }
            packet::Packet::PlayGameEventClientbound(game_event) => {
                mapped_packet::MappedPacket::PlayGameEventClientbound(PlayGameEventClientbound {
                    event: game_event.event,
                    param: game_event.param,
                })
            }
            packet::Packet::PlayMountScreenOpenClientbound(mount) => {
                mapped_packet::MappedPacket::PlayMountScreenOpenClientbound(
                    PlayMountScreenOpenClientbound {
                        container_id: mount.container_id.0,
                        inventory_columns: mount.inventory_columns.0,
                        entity_id: mount.entity_id,
                    },
                )
            }
            packet::Packet::PlayHurtAnimationClientbound(hurt) => {
                mapped_packet::MappedPacket::PlayHurtAnimationClientbound(
                    PlayHurtAnimationClientbound {
                        entity_id: hurt.entity_id.0,
                        yaw: hurt.yaw,
                    },
                )
            }
            packet::Packet::PlayInitializeBorderClientbound(border) => {
                mapped_packet::MappedPacket::PlayInitializeBorderClientbound(
                    PlayInitializeBorderClientbound {
                        new_center_x: border.new_center_x,
                        new_center_z: border.new_center_z,
                        old_size: border.old_size,
                        new_size: border.new_size,
                        lerp_time: border.lerp_time.0,
                        new_absolute_max_size: border.new_absolute_max_size.0,
                        warning_blocks: border.warning_blocks.0,
                        warning_time: border.warning_time.0,
                    },
                )
            }
            packet::Packet::PlaySetBorderCenterClientbound(border) => {
                mapped_packet::MappedPacket::PlaySetBorderCenterClientbound(
                    PlaySetBorderCenterClientbound {
                        new_center_x: border.new_center_x,
                        new_center_z: border.new_center_z,
                    },
                )
            }
            packet::Packet::PlaySetBorderLerpSizeClientbound(border) => {
                mapped_packet::MappedPacket::PlaySetBorderLerpSizeClientbound(
                    PlaySetBorderLerpSizeClientbound {
                        old_size: border.old_size,
                        new_size: border.new_size,
                        lerp_time: border.lerp_time.0,
                    },
                )
            }
            packet::Packet::PlaySetBorderSizeClientbound(border) => {
                mapped_packet::MappedPacket::PlaySetBorderSizeClientbound(
                    PlaySetBorderSizeClientbound { size: border.size },
                )
            }
            packet::Packet::PlaySetBorderWarningDelayClientbound(border) => {
                mapped_packet::MappedPacket::PlaySetBorderWarningDelayClientbound(
                    PlaySetBorderWarningDelayClientbound {
                        warning_delay: border.warning_delay.0,
                    },
                )
            }
            packet::Packet::PlaySetBorderWarningDistanceClientbound(border) => {
                mapped_packet::MappedPacket::PlaySetBorderWarningDistanceClientbound(
                    PlaySetBorderWarningDistanceClientbound {
                        warning_blocks: border.warning_blocks.0,
                    },
                )
            }
            packet::Packet::PlayLowDiskSpaceWarningClientbound(low_disk) => {
                mapped_packet::MappedPacket::PlayLowDiskSpaceWarningClientbound(
                    PlayLowDiskSpaceWarningClientbound {
                        empty: low_disk.empty,
                    },
                )
            }
            packet::Packet::PlayPongResponseClientbound_i64(pong) => {
                mapped_packet::MappedPacket::PlayPongResponseClientbound(
                    PlayPongResponseClientbound { time: pong.time },
                )
            }
            packet::Packet::PlayPlayerCombatEndClientbound(combat_end) => {
                mapped_packet::MappedPacket::PlayPlayerCombatEndClientbound(
                    PlayPlayerCombatEndClientbound {
                        duration: combat_end.duration.0,
                    },
                )
            }
            packet::Packet::PlayPlayerCombatEnterClientbound(combat_enter) => {
                mapped_packet::MappedPacket::PlayPlayerCombatEnterClientbound(
                    PlayPlayerCombatEnterClientbound {
                        empty: combat_enter.empty,
                    },
                )
            }
            packet::Packet::PlayPlayerInfoRemoveClientbound(player_info_remove) => {
                mapped_packet::MappedPacket::PlayPlayerInfoRemoveClientbound(
                    PlayPlayerInfoRemoveClientbound {
                        profile_ids: player_info_remove.profile_ids.data,
                    },
                )
            }
            packet::Packet::PlaySetDisplayObjectiveClientbound(display_objective) => {
                mapped_packet::MappedPacket::PlaySetDisplayObjectiveClientbound(
                    PlaySetDisplayObjectiveClientbound {
                        slot: display_objective.slot.0,
                        objective_name: display_objective.objective_name,
                    },
                )
            }
            packet::Packet::PlaySetScoreClientbound(score) => {
                mapped_packet::MappedPacket::PlaySetScoreClientbound(PlaySetScoreClientbound {
                    owner: score.owner,
                    objective_name: score.objective_name,
                    score: score.score.0,
                    display_present: score.display_present,
                    number_format_present: score.number_format_present,
                })
            }
            packet::Packet::PlaySetPassengersClientbound(passengers) => {
                mapped_packet::MappedPacket::SetPassengers(SetPassengers {
                    entity_id: passengers.vehicle_entity_id.0,
                    passengers: passengers
                        .passenger_entity_ids
                        .data
                        .iter()
                        .map(|id| id.0)
                        .collect(),
                })
            }
            packet::Packet::PlaySetCursorItemClientbound(cursor_item) => {
                mapped_packet::MappedPacket::PlaySetCursorItemClientbound(
                    PlaySetCursorItemClientbound {
                        item: cursor_item.item,
                    },
                )
            }
            packet::Packet::PlaySetDefaultSpawnPositionClientbound(spawn) => {
                mapped_packet::MappedPacket::PlaySetDefaultSpawnPositionClientbound(
                    PlaySetDefaultSpawnPositionClientbound {
                        dimension: spawn.dimension,
                        location: spawn.location,
                        yaw: spawn.yaw,
                        pitch: spawn.pitch,
                    },
                )
            }
            packet::Packet::PlaySetEntityDataClientbound(entity_data) => {
                mapped_packet::MappedPacket::PlaySetEntityDataClientbound(
                    PlaySetEntityDataClientbound {
                        entity_id: entity_data.entity_id.0,
                        packed_item_count: entity_data.packed_item_count.0,
                    },
                )
            }
            packet::Packet::PlaySetObjectiveClientbound(objective) => {
                mapped_packet::MappedPacket::PlaySetObjectiveClientbound(
                    PlaySetObjectiveClientbound {
                        objective_name: objective.objective_name,
                        method: objective.method,
                    },
                )
            }
            packet::Packet::PlaySetEquipmentClientbound(equipment) => {
                mapped_packet::MappedPacket::PlaySetEquipmentClientbound(
                    PlaySetEquipmentClientbound {
                        entity_id: equipment.entity_id.0,
                        equipment_slot: equipment.equipment_slot,
                        item: equipment.item,
                    },
                )
            }
            packet::Packet::PlaySetEntityLinkClientbound(link) => {
                mapped_packet::MappedPacket::EntityAttach(EntityAttach {
                    entity_id: link.source_entity_id,
                    vehicle: link.destination_entity_id,
                    leash: Some(true),
                })
            }
            packet::Packet::PlaySetPlayerInventoryClientbound(inventory) => {
                mapped_packet::MappedPacket::PlaySetPlayerInventoryClientbound(
                    PlaySetPlayerInventoryClientbound {
                        slot: inventory.slot.0,
                        item: inventory.item,
                    },
                )
            }
            packet::Packet::PlaySetPlayerTeamClientbound(team) => {
                mapped_packet::MappedPacket::PlaySetPlayerTeamClientbound(
                    PlaySetPlayerTeamClientbound {
                        team_name: team.team_name,
                        method: team.method,
                    },
                )
            }
            packet::Packet::PlaySetSubtitleTextClientbound(subtitle) => {
                mapped_packet::MappedPacket::PlaySetSubtitleTextClientbound(
                    PlaySetSubtitleTextClientbound {
                        text: subtitle.text,
                    },
                )
            }
            packet::Packet::PlaySetTimeClientbound(time) => {
                mapped_packet::MappedPacket::PlaySetTimeClientbound(PlaySetTimeClientbound {
                    game_time: time.game_time,
                    clock_update_count: time.clock_update_count.0,
                })
            }
            packet::Packet::PlaySetTitleTextClientbound(title) => {
                mapped_packet::MappedPacket::PlaySetTitleTextClientbound(
                    PlaySetTitleTextClientbound { text: title.text },
                )
            }
            packet::Packet::PlaySetTitlesAnimationClientbound(set_titles_animation) => {
                mapped_packet::MappedPacket::PlaySetTitlesAnimationClientbound(
                    PlaySetTitlesAnimationClientbound {
                        fade_in: set_titles_animation.fade_in,
                        stay: set_titles_animation.stay,
                        fade_out: set_titles_animation.fade_out,
                    },
                )
            }
            packet::Packet::PlayStartConfigurationClientbound(start_configuration) => {
                mapped_packet::MappedPacket::PlayStartConfigurationClientbound(
                    PlayStartConfigurationClientbound {
                        empty: start_configuration.empty,
                    },
                )
            }
            packet::Packet::PlayStoreCookieClientbound(store_cookie) => {
                mapped_packet::MappedPacket::PlayStoreCookieClientbound(
                    PlayStoreCookieClientbound {
                        key: store_cookie.key,
                        payload: store_cookie.payload.data,
                    },
                )
            }
            packet::Packet::PlaySystemChatClientbound(system_chat) => {
                mapped_packet::MappedPacket::PlaySystemChatClientbound(PlaySystemChatClientbound {
                    content: system_chat.content,
                    overlay: system_chat.overlay,
                })
            }
            packet::Packet::PlayTabListClientbound(tab_list) => {
                mapped_packet::MappedPacket::PlayTabListClientbound(PlayTabListClientbound {
                    header: tab_list.header,
                    footer: tab_list.footer,
                })
            }
            packet::Packet::PlayTagQueryClientbound(tag_query) => {
                mapped_packet::MappedPacket::PlayTagQueryClientbound(PlayTagQueryClientbound {
                    transaction_id: tag_query.transaction_id.0,
                    nbt_tag_type: tag_query.nbt_tag_type,
                    tag: tag_query.tag,
                })
            }
            packet::Packet::PlayTeleportEntityClientbound(teleport_entity) => {
                mapped_packet::MappedPacket::PlayTeleportEntityClientbound(
                    PlayTeleportEntityClientbound {
                        entity_id: teleport_entity.entity_id.0,
                        position_x: teleport_entity.position_x,
                        position_y: teleport_entity.position_y,
                        position_z: teleport_entity.position_z,
                        delta_x: teleport_entity.delta_x,
                        delta_y: teleport_entity.delta_y,
                        delta_z: teleport_entity.delta_z,
                        y_rot: teleport_entity.y_rot,
                        x_rot: teleport_entity.x_rot,
                        relative_mask: teleport_entity.relative_mask,
                        on_ground: teleport_entity.on_ground,
                    },
                )
            }
            packet::Packet::PlayTestInstanceBlockStatusClientbound(status) => {
                mapped_packet::MappedPacket::PlayTestInstanceBlockStatusClientbound(
                    PlayTestInstanceBlockStatusClientbound {
                        status: status.status,
                        size_present: status.size_present,
                    },
                )
            }
            packet::Packet::PlayTickingStateClientbound(ticking_state) => {
                mapped_packet::MappedPacket::PlayTickingStateClientbound(
                    PlayTickingStateClientbound {
                        tick_rate: ticking_state.tick_rate,
                        frozen: ticking_state.frozen,
                    },
                )
            }
            packet::Packet::PlayTickingStepClientbound(ticking_step) => {
                mapped_packet::MappedPacket::PlayTickingStepClientbound(
                    PlayTickingStepClientbound {
                        tick_steps: ticking_step.tick_steps.0,
                    },
                )
            }
            packet::Packet::PlayTransferClientbound(transfer) => {
                mapped_packet::MappedPacket::PlayTransferClientbound(PlayTransferClientbound {
                    host: transfer.host,
                    port: transfer.port.0,
                })
            }
            packet::Packet::PlayCustomReportDetailsClientbound(custom_report_details) => {
                mapped_packet::MappedPacket::PlayCustomReportDetailsClientbound(
                    PlayCustomReportDetailsClientbound {
                        detail_count: custom_report_details.detail_count.0,
                    },
                )
            }
            packet::Packet::PlayServerLinksClientbound(server_links) => {
                mapped_packet::MappedPacket::PlayServerLinksClientbound(
                    PlayServerLinksClientbound {
                        link_count: server_links.link_count.0,
                    },
                )
            }
            packet::Packet::PlayClearDialogClientbound(clear_dialog) => {
                mapped_packet::MappedPacket::PlayClearDialogClientbound(
                    PlayClearDialogClientbound {
                        empty: clear_dialog.empty,
                    },
                )
            }
            packet::Packet::PlayUpdateAttributesClientbound(update_attributes) => {
                mapped_packet::MappedPacket::PlayUpdateAttributesClientbound(
                    PlayUpdateAttributesClientbound {
                        entity_id: update_attributes.entity_id.0,
                        attribute_count: update_attributes.attribute_count.0,
                    },
                )
            }
            packet::Packet::PlayUpdateAdvancementsClientbound(update_advancements) => {
                mapped_packet::MappedPacket::PlayUpdateAdvancementsClientbound(
                    PlayUpdateAdvancementsClientbound {
                        reset: update_advancements.reset,
                        added_count: update_advancements.added_count.0,
                        removed_count: update_advancements.removed_count.0,
                        progress_count: update_advancements.progress_count.0,
                        show_advancements: update_advancements.show_advancements,
                    },
                )
            }
            packet::Packet::PlayUpdateRecipesClientbound(update_recipes) => {
                mapped_packet::MappedPacket::PlayUpdateRecipesClientbound(
                    PlayUpdateRecipesClientbound {
                        item_set_count: update_recipes.item_set_count.0,
                        stonecutter_recipe_count: update_recipes.stonecutter_recipe_count.0,
                    },
                )
            }
            packet::Packet::PlayUpdateTagsClientbound(update_tags) => {
                mapped_packet::MappedPacket::PlayUpdateTagsClientbound(PlayUpdateTagsClientbound {
                    registry_payload_count: update_tags.registry_payload_count.0,
                })
            }
            packet::Packet::PlayProjectilePowerClientbound(projectile_power) => {
                mapped_packet::MappedPacket::PlayProjectilePowerClientbound(
                    PlayProjectilePowerClientbound {
                        entity_id: projectile_power.entity_id.0,
                        acceleration_power: projectile_power.acceleration_power,
                    },
                )
            }
            packet::Packet::PlayWaypointClientbound(waypoint) => {
                mapped_packet::MappedPacket::PlayWaypointClientbound(PlayWaypointClientbound {
                    operation_id: waypoint.operation_id.0,
                    waypoint_payload: waypoint.waypoint_payload,
                })
            }
            packet::Packet::Advancements(advancements) => {
                mapped_packet::MappedPacket::Advancements(Advancements {
                    data: advancements.data,
                })
            }
            packet::Packet::AcknowledgePlayerDigging(digging) => {
                mapped_packet::MappedPacket::AcknowledgePlayerDigging(AcknowledgePlayerDigging {
                    location: digging.location,
                    block: digging.block.0,
                    status: digging.status.0,
                    successful: digging.successful,
                })
            }
            packet::Packet::AdvancementTab(advancement) => {
                mapped_packet::MappedPacket::AdvancementTab(AdvancementTab {
                    action: advancement.action.0,
                    tab_id: advancement.tab_id,
                })
            }
            packet::Packet::Animation(animation) => {
                mapped_packet::MappedPacket::Animation(Animation {
                    entity_id: animation.entity_id.0,
                    animation_id: animation.animation_id,
                })
            }
            packet::Packet::ArmSwing(arm_swing) => {
                mapped_packet::MappedPacket::ArmSwing(ArmSwing {
                    hand: Some(Hand::from(arm_swing.hand.0)),
                    entity_id: None,
                    animation: None,
                })
            }
            packet::Packet::ArmSwing_Handsfree(_arm_swing) => {
                mapped_packet::MappedPacket::ArmSwing(ArmSwing {
                    hand: None,
                    entity_id: None,
                    animation: None,
                })
            }
            packet::Packet::ArmSwing_Handsfree_ID(arm_swing) => {
                mapped_packet::MappedPacket::ArmSwing(ArmSwing {
                    hand: None,
                    entity_id: Some(arm_swing.entity_id),
                    animation: Some(arm_swing.animation),
                })
            }
            packet::Packet::BlockAction(block_action) => {
                mapped_packet::MappedPacket::BlockAction(BlockAction {
                    location: block_action.location,
                    byte1: block_action.byte1,
                    byte2: block_action.byte2,
                    block_type: block_action.block_type.0,
                })
            }
            packet::Packet::BlockAction_u16(block_action) => {
                mapped_packet::MappedPacket::BlockAction(BlockAction {
                    location: Position::new(block_action.x, block_action.y as i32, block_action.z),
                    byte1: block_action.byte1,
                    byte2: block_action.byte2,
                    block_type: block_action.block_type.0,
                })
            }
            packet::Packet::BlockBreakAnimation(break_animation) => {
                mapped_packet::MappedPacket::BlockBreakAnimation(BlockBreakAnimation {
                    entity_id: break_animation.entity_id.0,
                    location: break_animation.location,
                    stage: break_animation.stage,
                })
            }
            packet::Packet::BlockBreakAnimation_i32(break_animation) => {
                mapped_packet::MappedPacket::BlockBreakAnimation(BlockBreakAnimation {
                    entity_id: break_animation.entity_id.0,
                    location: Position::new(
                        break_animation.x,
                        break_animation.y,
                        break_animation.z,
                    ),
                    stage: break_animation.stage,
                })
            }
            packet::Packet::BlockChange_u8(block_change) => {
                mapped_packet::MappedPacket::BlockChange(BlockChange {
                    location: Position::new(block_change.x, block_change.y as i32, block_change.z),
                    block_id: (block_change.block_id.0 << 4) | (block_change.block_metadata as i32),
                })
            }
            packet::Packet::BlockChange_VarInt(block_change) => {
                mapped_packet::MappedPacket::BlockChange(BlockChange {
                    location: block_change.location,
                    block_id: block_change.block_id.0,
                })
            }
            packet::Packet::BossBar(boss_bar) => mapped_packet::MappedPacket::BossBar(BossBar {
                uuid: boss_bar.uuid,
                action: boss_bar.action.0,
                title: boss_bar.title,
                health: boss_bar.health,
                color: boss_bar.color.0,
                style: boss_bar.style.0,
                flags: boss_bar.flags,
            }),
            packet::Packet::ChatMessage(chat_msg) => {
                mapped_packet::MappedPacket::ChatMessage(ChatMessage {
                    message: chat_msg.message,
                })
            }
            packet::Packet::ChangeGameState(change_game_state) => {
                mapped_packet::MappedPacket::ChangeGameState(ChangeGameState {
                    reason: change_game_state.reason,
                    value: change_game_state.value,
                })
            }
            packet::Packet::ClientStatus(client_status) => {
                mapped_packet::MappedPacket::ClientStatus(ClientStatus {
                    action_id: client_status.action_id.0,
                })
            }
            packet::Packet::ClientStatus_u8(client_status) => {
                mapped_packet::MappedPacket::ClientStatus(ClientStatus {
                    action_id: client_status.action_id as i32,
                })
            }
            packet::Packet::ClientSettings(client_settings) => {
                mapped_packet::MappedPacket::ClientSettings(ClientSettings {
                    locale: client_settings.locale,
                    view_distance: client_settings.view_distance,
                    chat_mode: client_settings.chat_mode.0,
                    chat_colors: client_settings.chat_colors,
                    difficulty: None,
                    displayed_skin_parts: client_settings.displayed_skin_parts,
                    main_hand: Some(Hand::from(client_settings.main_hand.0)),
                })
            }
            packet::Packet::ClientSettings_u8(client_settings) => {
                mapped_packet::MappedPacket::ClientSettings(ClientSettings {
                    locale: client_settings.locale,
                    view_distance: client_settings.view_distance,
                    chat_mode: client_settings.chat_mode as i32,
                    chat_colors: client_settings.chat_colors,
                    difficulty: None,
                    displayed_skin_parts: client_settings.displayed_skin_parts,
                    main_hand: Some(Hand::from(client_settings.main_hand.0)),
                })
            }
            packet::Packet::ClientSettings_u8_Handsfree(client_settings) => {
                mapped_packet::MappedPacket::ClientSettings(ClientSettings {
                    locale: client_settings.locale,
                    view_distance: client_settings.view_distance,
                    chat_mode: client_settings.chat_mode as i32,
                    chat_colors: client_settings.chat_colors,
                    difficulty: None,
                    displayed_skin_parts: client_settings.displayed_skin_parts,
                    main_hand: None,
                })
            }
            packet::Packet::ClientSettings_u8_Handsfree_Difficulty(client_settings) => {
                mapped_packet::MappedPacket::ClientSettings(ClientSettings {
                    locale: client_settings.locale,
                    view_distance: client_settings.view_distance,
                    chat_mode: client_settings.chat_mode as i32,
                    chat_colors: client_settings.chat_colors,
                    difficulty: Some(client_settings.difficulty),
                    displayed_skin_parts: client_settings.displayed_skin_parts,
                    main_hand: None,
                })
            }
            packet::Packet::ConfirmTransactionServerbound(confirm_transaction) => {
                mapped_packet::MappedPacket::ConfirmTransactionServerbound(
                    ConfirmTransactionServerbound {
                        id: confirm_transaction.id,
                        action_number: confirm_transaction.action_number,
                        accepted: confirm_transaction.accepted,
                    },
                )
            }
            packet::Packet::ConfirmTransaction(confirm_transaction) => {
                mapped_packet::MappedPacket::ConfirmTransaction(ConfirmTransaction {
                    id: confirm_transaction.id,
                    action_number: confirm_transaction.action_number,
                    accepted: confirm_transaction.accepted,
                })
            }
            packet::Packet::ChunkUnload(chunk_unload) => {
                mapped_packet::MappedPacket::ChunkUnload(ChunkUnload {
                    x: chunk_unload.x,
                    z: chunk_unload.z,
                })
            }
            packet::Packet::ChunkData(chunk_data) => {
                mapped_packet::MappedPacket::ChunkData(ChunkData {
                    chunk_x: chunk_data.chunk_x,
                    chunk_z: chunk_data.chunk_z,
                    new: chunk_data.new,
                    bitmask: chunk_data.bitmask.0,
                    data: chunk_data.data.data,
                    block_entities: chunk_data.block_entities.data,
                })
            }
            packet::Packet::ChunkData_HeightMap(chunk_data) => {
                mapped_packet::MappedPacket::ChunkData_HeightMap(ChunkData_HeightMap {
                    chunk_x: chunk_data.chunk_x,
                    chunk_z: chunk_data.chunk_z,
                    new: chunk_data.new,
                    bitmask: chunk_data.bitmask.0,
                    heightmaps: chunk_data.heightmaps,
                    data: chunk_data.data.data,
                    block_entities: chunk_data.block_entities.data,
                })
            }
            packet::Packet::ChunkData_Biomes3D_VarInt(chunk_data) => {
                mapped_packet::MappedPacket::ChunkData_Biomes3D_i32(ChunkData_Biomes3D_i32 {
                    chunk_x: chunk_data.chunk_x,
                    chunk_z: chunk_data.chunk_z,
                    new: chunk_data.new,
                    bitmask: chunk_data.bitmask.0,
                    heightmaps: chunk_data.heightmaps,
                    biomes: chunk_data.biomes.data.iter().map(|x| x.0).collect(),
                    data: chunk_data.data.data,
                    block_entities: chunk_data.block_entities.data,
                })
            }
            packet::Packet::ChunkData_Biomes3D(chunk_data) => {
                mapped_packet::MappedPacket::ChunkData_Biomes3D(ChunkData_Biomes3D {
                    chunk_x: chunk_data.chunk_x,
                    chunk_z: chunk_data.chunk_z,
                    new: chunk_data.new,
                    bitmask: chunk_data.bitmask.0,
                    heightmaps: chunk_data.heightmaps,
                    biomes: chunk_data.biomes,
                    data: chunk_data.data.data,
                    block_entities: chunk_data.block_entities.data,
                })
            }
            packet::Packet::ChunkData_Biomes3D_bool(chunk_data) => {
                mapped_packet::MappedPacket::ChunkData_Biomes3D_bool(ChunkData_Biomes3D_bool {
                    chunk_x: chunk_data.chunk_x,
                    chunk_z: chunk_data.chunk_z,
                    new: chunk_data.new,
                    ignore_old_data: chunk_data.ignore_old_data,
                    bitmask: chunk_data.bitmask.0,
                    heightmaps: chunk_data.heightmaps,
                    biomes: chunk_data.biomes,
                    data: chunk_data.data.data,
                    block_entities: chunk_data.block_entities.data,
                })
            }
            packet::Packet::ChunkData_17(chunk_data) => {
                mapped_packet::MappedPacket::ChunkData_17(ChunkData_17 {
                    chunk_x: chunk_data.chunk_x,
                    chunk_z: chunk_data.chunk_z,
                    new: chunk_data.new,
                    bitmask: chunk_data.bitmask,
                    add_bitmask: chunk_data.add_bitmask,
                    compressed_data: chunk_data.compressed_data.data,
                })
            }
            packet::Packet::ChunkData_NoEntities(chunk_data) => {
                mapped_packet::MappedPacket::ChunkData_NoEntities(ChunkData_NoEntities {
                    chunk_x: chunk_data.chunk_x,
                    chunk_z: chunk_data.chunk_z,
                    new: chunk_data.new,
                    bitmask: chunk_data.bitmask.0,
                    data: chunk_data.data.data,
                })
            }
            packet::Packet::ChunkData_NoEntities_u16(chunk_data) => {
                mapped_packet::MappedPacket::ChunkData_NoEntities_u16(ChunkData_NoEntities_u16 {
                    chunk_x: chunk_data.chunk_x,
                    chunk_z: chunk_data.chunk_z,
                    new: chunk_data.new,
                    bitmask: chunk_data.bitmask,
                    data: chunk_data.data.data,
                })
            }
            packet::Packet::ChunkDataBulk_17(chunk_data) => {
                mapped_packet::MappedPacket::ChunkDataBulk_17(ChunkDataBulk_17 {
                    chunk_column_count: chunk_data.chunk_column_count,
                    data_length: chunk_data.data_length,
                    skylight: chunk_data.skylight,
                    chunk_data_and_meta: chunk_data.chunk_data_and_meta,
                })
            }
            packet::Packet::ChunkDataBulk(chunk_data) => {
                mapped_packet::MappedPacket::ChunkDataBulk(ChunkDataBulk {
                    skylight: chunk_data.skylight,
                    chunk_meta: chunk_data.chunk_meta.data,
                    chunk_data: chunk_data.chunk_data,
                })
            }
            packet::Packet::Camera(camera) => mapped_packet::MappedPacket::Camera(Camera {
                target_id: camera.target_id.0,
            }),
            packet::Packet::ClickWindow(click_window) => {
                mapped_packet::MappedPacket::ClickWindow(ClickWindow {
                    id: click_window.id,
                    slot: click_window.slot,
                    button: click_window.button,
                    action_number: click_window.action_number,
                    mode: click_window.mode.0,
                    clicked_item: click_window.clicked_item,
                })
            }
            packet::Packet::ClickWindow_u8(click_window) => {
                mapped_packet::MappedPacket::ClickWindow(ClickWindow {
                    id: click_window.id,
                    slot: click_window.slot,
                    button: click_window.button,
                    action_number: click_window.action_number,
                    mode: click_window.mode as i32,
                    clicked_item: click_window.clicked_item,
                })
            }
            packet::Packet::ClickWindowButton(click_window_button) => {
                mapped_packet::MappedPacket::ClickWindowButton(ClickWindowButton {
                    id: click_window_button.id,
                    button: click_window_button.button,
                })
            }
            packet::Packet::ClientAbilities_f32(client_abilities) => {
                mapped_packet::MappedPacket::ClientAbilities(ClientAbilities {
                    flags: client_abilities.flags,
                    flying_speed: Some(client_abilities.flying_speed),
                    walking_speed: Some(client_abilities.walking_speed),
                })
            }
            packet::Packet::ClientAbilities_u8(client_abilities) => {
                mapped_packet::MappedPacket::ClientAbilities(ClientAbilities {
                    flags: client_abilities.flags,
                    flying_speed: None,
                    walking_speed: None,
                })
            }
            packet::Packet::CloseWindow(close_window) => {
                mapped_packet::MappedPacket::CloseWindow(CloseWindow {
                    id: close_window.id,
                })
            }
            packet::Packet::CoFHLib_SendUUID(send_uuid) => {
                mapped_packet::MappedPacket::CoFHLib_SendUUID(CoFHLib_SendUUID {
                    player_uuid: send_uuid.player_uuid,
                })
            }
            packet::Packet::CollectItem(collect_item) => {
                mapped_packet::MappedPacket::CollectItem(CollectItem {
                    collected_entity_id: collect_item.collected_entity_id.0,
                    collector_entity_id: collect_item.collector_entity_id.0,
                    number_of_items: Some(collect_item.number_of_items.0),
                })
            }
            packet::Packet::CollectItem_nocount(collect_item) => {
                mapped_packet::MappedPacket::CollectItem(CollectItem {
                    collected_entity_id: collect_item.collected_entity_id.0,
                    collector_entity_id: collect_item.collector_entity_id.0,
                    number_of_items: None,
                })
            }
            packet::Packet::CollectItem_nocount_i32(collect_item) => {
                mapped_packet::MappedPacket::CollectItem(CollectItem {
                    collected_entity_id: collect_item.collected_entity_id,
                    collector_entity_id: collect_item.collector_entity_id,
                    number_of_items: None,
                })
            }
            packet::Packet::CombatEvent(combat_event) => {
                mapped_packet::MappedPacket::CombatEvent(CombatEvent {
                    event: combat_event.event.0,
                    direction: combat_event.direction.map(|x| x.0),
                    player_id: combat_event.player_id.map(|x| x.0),
                    entity_id: combat_event.entity_id,
                    message: combat_event.message,
                })
            }
            packet::Packet::CraftingBookData(crafting_book) => {
                mapped_packet::MappedPacket::CraftingBookData(CraftingBookData {
                    action: crafting_book.action.0,
                    recipe_id: crafting_book.recipe_id,
                    crafting_book_open: crafting_book.crafting_book_open,
                    crafting_filter: crafting_book.crafting_filter,
                })
            }
            packet::Packet::CraftRecipeRequest(craft_recipe_request) => {
                mapped_packet::MappedPacket::CraftRecipeRequest(CraftRecipeRequest {
                    window_id: craft_recipe_request.window_id,
                    recipe: craft_recipe_request.recipe.0,
                    make_all: craft_recipe_request.make_all,
                })
            }
            packet::Packet::CraftRecipeResponse(craft_recipe_response) => {
                mapped_packet::MappedPacket::CraftRecipeResponse(CraftRecipeResponse {
                    window_id: craft_recipe_response.window_id,
                    recipe: craft_recipe_response.recipe.0,
                })
            }
            packet::Packet::CreativeInventoryAction(creative_inventory_action) => {
                mapped_packet::MappedPacket::CreativeInventoryAction(CreativeInventoryAction {
                    slot: creative_inventory_action.slot,
                    clicked_item: creative_inventory_action.clicked_item,
                })
            }
            packet::Packet::Disconnect(disconnect) => {
                mapped_packet::MappedPacket::Disconnect(Disconnect {
                    reason: disconnect.reason,
                })
            }
            packet::Packet::DeclareCommands(declare_commands) => {
                mapped_packet::MappedPacket::DeclareCommands(DeclareCommands {
                    nodes: declare_commands.nodes.data,
                    root_index: declare_commands.root_index.0,
                })
            }
            packet::Packet::DeclareRecipes(declare_recipes) => {
                mapped_packet::MappedPacket::DeclareRecipes(DeclareRecipes {
                    recipes: declare_recipes.recipes.data,
                })
            }
            packet::Packet::Entity(entity) => mapped_packet::MappedPacket::Entity(Entity {
                entity_id: entity.entity_id.0,
            }),
            packet::Packet::Entity_i32(entity) => mapped_packet::MappedPacket::Entity(Entity {
                entity_id: entity.entity_id,
            }),
            packet::Packet::EntityHeadLook(head_look) => {
                mapped_packet::MappedPacket::EntityHeadLook(EntityHeadLook {
                    entity_id: head_look.entity_id.0,
                    head_yaw: head_look.head_yaw,
                })
            }
            packet::Packet::EntityHeadLook_i32(head_look) => {
                mapped_packet::MappedPacket::EntityHeadLook(EntityHeadLook {
                    entity_id: head_look.entity_id,
                    head_yaw: head_look.head_yaw,
                })
            }
            packet::Packet::EntityVelocity(velocity) => {
                mapped_packet::MappedPacket::EntityVelocity(EntityVelocity {
                    entity_id: velocity.entity_id.0,
                    velocity_x: velocity.velocity_x,
                    velocity_y: velocity.velocity_y,
                    velocity_z: velocity.velocity_z,
                })
            }
            packet::Packet::EntityVelocity_i32(velocity) => {
                mapped_packet::MappedPacket::EntityVelocity(EntityVelocity {
                    entity_id: velocity.entity_id,
                    velocity_x: velocity.velocity_x,
                    velocity_y: velocity.velocity_y,
                    velocity_z: velocity.velocity_z,
                })
            }
            packet::Packet::EntityLookAndMove_i16(look_and_move) => {
                mapped_packet::MappedPacket::EntityLookAndMove(EntityLookAndMove {
                    entity_id: look_and_move.entity_id.0,
                    delta_x: From::from(look_and_move.delta_x),
                    delta_y: From::from(look_and_move.delta_y),
                    delta_z: From::from(look_and_move.delta_z),
                    yaw: look_and_move.yaw,
                    pitch: look_and_move.pitch,
                    on_ground: Some(look_and_move.on_ground),
                })
            }
            packet::Packet::EntityLookAndMove_i8(look_and_move) => {
                mapped_packet::MappedPacket::EntityLookAndMove(EntityLookAndMove {
                    entity_id: look_and_move.entity_id.0,
                    delta_x: From::from(look_and_move.delta_x),
                    delta_y: From::from(look_and_move.delta_y),
                    delta_z: From::from(look_and_move.delta_z),
                    yaw: look_and_move.yaw,
                    pitch: look_and_move.pitch,
                    on_ground: Some(look_and_move.on_ground),
                })
            }
            packet::Packet::EntityLookAndMove_i8_i32_NoGround(look_and_move) => {
                mapped_packet::MappedPacket::EntityLookAndMove(EntityLookAndMove {
                    entity_id: look_and_move.entity_id,
                    delta_x: From::from(look_and_move.delta_x),
                    delta_y: From::from(look_and_move.delta_y),
                    delta_z: From::from(look_and_move.delta_z),
                    yaw: look_and_move.yaw,
                    pitch: look_and_move.pitch,
                    on_ground: None,
                })
            }
            packet::Packet::EntityLook_i32_NoGround(look) => {
                mapped_packet::MappedPacket::EntityLook(EntityLook {
                    entity_id: look.entity_id,
                    yaw: look.yaw,
                    pitch: look.pitch,
                    on_ground: None,
                })
            }
            packet::Packet::EntityLook_VarInt(look) => {
                mapped_packet::MappedPacket::EntityLook(EntityLook {
                    entity_id: look.entity_id.0,
                    yaw: look.yaw,
                    pitch: look.pitch,
                    on_ground: Some(look.on_ground),
                })
            }
            packet::Packet::EntityTeleport_f64(teleport) => {
                mapped_packet::MappedPacket::EntityTeleport(EntityTeleport {
                    entity_id: teleport.entity_id.0,
                    x: teleport.x,
                    y: teleport.y,
                    z: teleport.z,
                    yaw: teleport.yaw,
                    pitch: teleport.pitch,
                    on_ground: Some(teleport.on_ground),
                })
            }
            packet::Packet::EntityTeleport_i32(teleport) => {
                mapped_packet::MappedPacket::EntityTeleport(EntityTeleport {
                    entity_id: teleport.entity_id.0,
                    x: From::from(teleport.x),
                    y: From::from(teleport.y),
                    z: From::from(teleport.z),
                    yaw: teleport.yaw,
                    pitch: teleport.pitch,
                    on_ground: Some(teleport.on_ground),
                })
            }
            packet::Packet::EntityTeleport_i32_i32_NoGround(teleport) => {
                mapped_packet::MappedPacket::EntityTeleport(EntityTeleport {
                    entity_id: teleport.entity_id,
                    x: From::from(teleport.x),
                    y: From::from(teleport.y),
                    z: From::from(teleport.z),
                    yaw: teleport.yaw,
                    pitch: teleport.pitch,
                    on_ground: None,
                })
            }
            packet::Packet::EntityMove_i16(entity_move) => {
                mapped_packet::MappedPacket::EntityMove(EntityMove {
                    entity_id: entity_move.entity_id.0,
                    delta_x: From::from(entity_move.delta_x),
                    delta_y: From::from(entity_move.delta_y),
                    delta_z: From::from(entity_move.delta_z),
                    on_ground: Some(entity_move.on_ground),
                })
            }
            packet::Packet::EntityMove_i8(entity_move) => {
                mapped_packet::MappedPacket::EntityMove(EntityMove {
                    entity_id: entity_move.entity_id.0,
                    delta_x: From::from(entity_move.delta_x),
                    delta_y: From::from(entity_move.delta_y),
                    delta_z: From::from(entity_move.delta_z),
                    on_ground: Some(entity_move.on_ground),
                })
            }
            packet::Packet::EntityMove_i8_i32_NoGround(entity_move) => {
                mapped_packet::MappedPacket::EntityMove(EntityMove {
                    entity_id: entity_move.entity_id,
                    delta_x: From::from(entity_move.delta_x),
                    delta_y: From::from(entity_move.delta_y),
                    delta_z: From::from(entity_move.delta_z),
                    on_ground: None,
                })
            }
            packet::Packet::EntityDestroy(destroy) => {
                mapped_packet::MappedPacket::EntityDestroy(EntityDestroy {
                    entity_ids: destroy.entity_ids.data.iter().map(|x| x.0).collect(),
                })
            }
            packet::Packet::EntityDestroy_u8(destroy) => {
                mapped_packet::MappedPacket::EntityDestroy(EntityDestroy {
                    entity_ids: destroy.entity_ids.data,
                })
            }
            packet::Packet::EditBook(edit_book) => {
                mapped_packet::MappedPacket::EditBook(EditBook {
                    new_book: edit_book.new_book,
                    is_signing: edit_book.is_signing,
                    hand: Hand::from(edit_book.hand.0),
                })
            }
            packet::Packet::Effect(effect) => mapped_packet::MappedPacket::Effect(Effect {
                effect_id: effect.effect_id,
                location: effect.location,
                data: effect.data,
                disable_relative: effect.disable_relative,
            }),
            packet::Packet::Effect_u8y(effect) => mapped_packet::MappedPacket::Effect(Effect {
                effect_id: effect.effect_id,
                location: Position::new(effect.x, effect.y as i32, effect.z),
                data: effect.data,
                disable_relative: effect.disable_relative,
            }),
            packet::Packet::EnchantItem(enchant_item) => {
                mapped_packet::MappedPacket::EnchantItem(EnchantItem {
                    id: enchant_item.id,
                    enchantment: enchant_item.enchantment,
                })
            }
            packet::Packet::EncryptionRequest(encryption_request) => {
                mapped_packet::MappedPacket::EncryptionRequest(EncryptionRequest {
                    server_id: encryption_request.server_id,
                    public_key: encryption_request.public_key.data,
                    verify_token: encryption_request.verify_token.data,
                })
            }
            packet::Packet::EncryptionRequest_ShouldAuthenticate(encryption_request) => {
                mapped_packet::MappedPacket::EncryptionRequest_ShouldAuthenticate(
                    EncryptionRequest_ShouldAuthenticate {
                        server_id: encryption_request.server_id,
                        public_key: encryption_request.public_key.data,
                        verify_token: encryption_request.verify_token.data,
                        should_authenticate: encryption_request.should_authenticate,
                    },
                )
            }
            packet::Packet::EncryptionRequest_i16(encryption_request) => {
                mapped_packet::MappedPacket::EncryptionRequest(EncryptionRequest {
                    server_id: encryption_request.server_id,
                    public_key: encryption_request.public_key.data,
                    verify_token: encryption_request.verify_token.data,
                })
            }
            packet::Packet::EncryptionResponse(encryption_response) => {
                mapped_packet::MappedPacket::EncryptionResponse(EncryptionResponse {
                    shared_secret: encryption_response.shared_secret.data,
                    verify_token: encryption_response.verify_token.data,
                })
            }
            packet::Packet::EncryptionResponse_i16(encryption_response) => {
                mapped_packet::MappedPacket::EncryptionResponse(EncryptionResponse {
                    shared_secret: encryption_response.shared_secret.data,
                    verify_token: encryption_response.verify_token.data,
                })
            }
            packet::Packet::EntityAction(action) => {
                mapped_packet::MappedPacket::EntityAction(EntityAction {
                    entity_id: action.entity_id,
                    action_id: action.action_id,
                })
            }
            packet::Packet::EntityAttach(attach) => {
                mapped_packet::MappedPacket::EntityAttach(EntityAttach {
                    entity_id: attach.entity_id,
                    vehicle: attach.vehicle,
                    leash: None,
                })
            }
            packet::Packet::EntityAttach_leashed(attach) => {
                mapped_packet::MappedPacket::EntityAttach(EntityAttach {
                    entity_id: attach.entity_id,
                    vehicle: attach.vehicle,
                    leash: Some(attach.leash),
                })
            }
            packet::Packet::EntityEffect(effect) => {
                mapped_packet::MappedPacket::EntityEffect(EntityEffect {
                    entity_id: effect.entity_id.0,
                    effect_id: effect.effect_id,
                    amplifier: effect.amplifier,
                    duration: effect.duration.0,
                    hide_particles: Some(effect.hide_particles),
                })
            }
            packet::Packet::EntityEffect_i32(effect) => {
                mapped_packet::MappedPacket::EntityEffect(EntityEffect {
                    entity_id: effect.entity_id,
                    effect_id: effect.effect_id,
                    amplifier: effect.amplifier,
                    duration: effect.duration as i32,
                    hide_particles: None,
                })
            }
            packet::Packet::PlayUpdateMobEffectClientbound(effect) => {
                mapped_packet::MappedPacket::EntityEffect(EntityEffect {
                    entity_id: effect.entity_id.0,
                    effect_id: effect.effect_holder_id.0 as i8,
                    amplifier: effect.amplifier.0 as i8,
                    duration: effect.duration.0,
                    hide_particles: Some(effect.flags & 0x02 == 0),
                })
            }
            packet::Packet::EntityEquipment_Array(equipment) => {
                mapped_packet::MappedPacket::EntityEquipment_Array(EntityEquipment_Array {
                    entity_id: equipment.entity_id.0,
                    equipments: equipment.equipments,
                })
            }
            packet::Packet::EntityEquipment_u16(equipment) => {
                mapped_packet::MappedPacket::EntityEquipment_Single(EntityEquipment_Single {
                    entity_id: equipment.entity_id.0,
                    slot: equipment.slot as i32,
                    item: equipment.item,
                })
            }
            packet::Packet::EntityEquipment_u16_i32(equipment) => {
                mapped_packet::MappedPacket::EntityEquipment_Single(EntityEquipment_Single {
                    entity_id: equipment.entity_id,
                    slot: equipment.slot as i32,
                    item: equipment.item,
                })
            }
            packet::Packet::EntityEquipment_VarInt(equipment) => {
                mapped_packet::MappedPacket::EntityEquipment_Single(EntityEquipment_Single {
                    entity_id: equipment.entity_id.0,
                    slot: equipment.slot.0,
                    item: equipment.item,
                })
            }
            packet::Packet::EntityMetadata(metadata) => {
                mapped_packet::MappedPacket::EntityMetadata(EntityMetadata {
                    entity_id: metadata.entity_id.0,
                    metadata: metadata.metadata,
                })
            }
            packet::Packet::EntityMetadata_i32(metadata) => {
                mapped_packet::MappedPacket::EntityMetadata(EntityMetadata {
                    entity_id: metadata.entity_id,
                    metadata: metadata.metadata,
                })
            }
            packet::Packet::EntityProperties(properties) => {
                mapped_packet::MappedPacket::EntityProperties(EntityProperties {
                    entity_id: properties.entity_id.0,
                    properties: properties
                        .properties
                        .data
                        .into_iter()
                        .map(|x| EntityProperty {
                            key: x.key,
                            value: x.value,
                            modifiers: x.modifiers.data,
                        })
                        .collect(),
                })
            }
            packet::Packet::EntityProperties_i32(properties) => {
                mapped_packet::MappedPacket::EntityProperties(EntityProperties {
                    entity_id: properties.entity_id,
                    properties: properties
                        .properties
                        .data
                        .into_iter()
                        .map(|x| EntityProperty {
                            key: x.key,
                            value: x.value,
                            modifiers: x.modifiers.data,
                        })
                        .collect(),
                })
            }
            packet::Packet::EntityRemoveEffect(remove_effect) => {
                mapped_packet::MappedPacket::EntityRemoveEffect(EntityRemoveEffect {
                    entity_id: remove_effect.entity_id.0,
                    effect_id: remove_effect.effect_id,
                })
            }
            packet::Packet::EntityRemoveEffect_i32(remove_effect) => {
                mapped_packet::MappedPacket::EntityRemoveEffect(EntityRemoveEffect {
                    entity_id: remove_effect.entity_id,
                    effect_id: remove_effect.effect_id,
                })
            }
            packet::Packet::EntitySoundEffect(sound_effect) => {
                mapped_packet::MappedPacket::EntitySoundEffect(EntitySoundEffect {
                    sound_id: sound_effect.sound_id.0,
                    sound_category: sound_effect.sound_category.0,
                    entity_id: sound_effect.entity_id.0,
                    volume: sound_effect.volume,
                    pitch: sound_effect.pitch,
                })
            }
            packet::Packet::EntityStatus(status) => {
                mapped_packet::MappedPacket::EntityStatus(EntityStatus {
                    entity_id: status.entity_id,
                    entity_status: status.entity_status,
                })
            }
            packet::Packet::EntityUpdateNBT(update_nbt) => {
                mapped_packet::MappedPacket::EntityUpdateNBT(EntityUpdateNBT {
                    entity_id: update_nbt.entity_id.0,
                    nbt: update_nbt.nbt,
                })
            }
            packet::Packet::EntityUsedBed(used_bed) => {
                mapped_packet::MappedPacket::EntityUsedBed(EntityUsedBed {
                    entity_id: used_bed.entity_id.0,
                    location: used_bed.location,
                })
            }
            packet::Packet::EntityUsedBed_i32(used_bed) => {
                mapped_packet::MappedPacket::EntityUsedBed(EntityUsedBed {
                    entity_id: used_bed.entity_id,
                    location: Position::new(used_bed.x, used_bed.y as i32, used_bed.z),
                })
            }
            packet::Packet::Explosion(explosion) => {
                mapped_packet::MappedPacket::Explosion(Explosion {
                    x: explosion.x,
                    y: explosion.y,
                    z: explosion.z,
                    radius: explosion.radius,
                    records: explosion.records.data,
                    velocity_x: explosion.velocity_x,
                    velocity_y: explosion.velocity_y,
                    velocity_z: explosion.velocity_z,
                })
            }
            packet::Packet::FacePlayer(face_player) => {
                mapped_packet::MappedPacket::FacePlayer(FacePlayer {
                    feet_eyes: face_player.feet_eyes.0,
                    target_x: face_player.target_x,
                    target_y: face_player.target_y,
                    target_z: face_player.target_z,
                    is_entity: face_player.is_entity,
                    entity_id: face_player.entity_id.map(|x| x.0),
                    entity_feet_eyes: face_player.entity_feet_eyes.map(|x| x.0),
                })
            }
            packet::Packet::GenerateStructure(generate_structure) => {
                mapped_packet::MappedPacket::GenerateStructure(GenerateStructure {
                    location: generate_structure.location,
                    levels: generate_structure.levels.0,
                    keep_jigsaws: generate_structure.keep_jigsaws,
                })
            }
            packet::Packet::HeldItemChange(held_item) => {
                mapped_packet::MappedPacket::HeldItemChange(HeldItemChange {
                    slot: held_item.slot,
                })
            }
            packet::Packet::Handshake(handshake) => {
                mapped_packet::MappedPacket::Handshake(Handshake {
                    protocol_version: handshake.protocol_version.0,
                    host: handshake.host,
                    port: handshake.port,
                    next: handshake.next.0,
                })
            }
            packet::Packet::JoinGame_i8(join_game) => {
                mapped_packet::MappedPacket::JoinGame(JoinGame {
                    entity_id: join_game.entity_id,
                    is_hardcore: None,
                    gamemode: join_game.gamemode,
                    previous_gamemode: None,
                    world_names: None,
                    dimension_codec: None,
                    dimension: None,
                    dimension_name: None,
                    dimension_id: Some(join_game.dimension as i32),
                    difficulty: Some(join_game.difficulty),
                    max_players: join_game.max_players as i32,
                    level_type: Some(join_game.level_type),
                    world_name: None,
                    reduced_debug_info: Some(join_game.reduced_debug_info),
                    enable_respawn_screen: None,
                    is_debug: None,
                    hashed_seed: None,
                    view_distance: None,
                    is_flat: None,
                })
            }
            packet::Packet::JoinGame_i8_NoDebug(join_game) => {
                mapped_packet::MappedPacket::JoinGame(JoinGame {
                    entity_id: join_game.entity_id,
                    is_hardcore: None,
                    gamemode: join_game.gamemode,
                    previous_gamemode: None,
                    world_names: None,
                    dimension_codec: None,
                    dimension: None,
                    dimension_name: None,
                    dimension_id: Some(join_game.dimension as i32),
                    difficulty: Some(join_game.difficulty),
                    max_players: join_game.max_players as i32,
                    level_type: Some(join_game.level_type),
                    world_name: None,
                    reduced_debug_info: None,
                    enable_respawn_screen: None,
                    is_debug: None,
                    hashed_seed: None,
                    view_distance: None,
                    is_flat: None,
                })
            }
            packet::Packet::JoinGame_i32(join_game) => {
                mapped_packet::MappedPacket::JoinGame(JoinGame {
                    entity_id: join_game.entity_id,
                    is_hardcore: None,
                    gamemode: join_game.gamemode,
                    previous_gamemode: None,
                    world_names: None,
                    dimension_codec: None,
                    dimension: None,
                    dimension_name: None,
                    dimension_id: Some(join_game.dimension),
                    difficulty: Some(join_game.difficulty),
                    max_players: join_game.max_players as i32,
                    level_type: Some(join_game.level_type),
                    world_name: None,
                    reduced_debug_info: Some(join_game.reduced_debug_info),
                    enable_respawn_screen: None,
                    is_debug: None,
                    hashed_seed: None,
                    view_distance: None,
                    is_flat: None,
                })
            }
            packet::Packet::JoinGame_i32_ViewDistance(join_game) => {
                mapped_packet::MappedPacket::JoinGame(JoinGame {
                    entity_id: join_game.entity_id,
                    is_hardcore: None,
                    gamemode: join_game.gamemode,
                    previous_gamemode: None,
                    world_names: None,
                    dimension_codec: None,
                    dimension: None,
                    dimension_name: None,
                    dimension_id: Some(join_game.dimension),
                    max_players: join_game.max_players as i32,
                    level_type: Some(join_game.level_type),
                    world_name: None,
                    view_distance: Some(join_game.view_distance.0),
                    reduced_debug_info: Some(join_game.reduced_debug_info),
                    enable_respawn_screen: None,
                    is_debug: None,
                    difficulty: None,
                    hashed_seed: None,
                    is_flat: None,
                })
            }
            packet::Packet::JoinGame_WorldNames(join_game) => {
                mapped_packet::MappedPacket::JoinGame(JoinGame {
                    entity_id: join_game.entity_id,
                    is_hardcore: None,
                    gamemode: join_game.gamemode,
                    previous_gamemode: Some(join_game.previous_gamemode),
                    world_names: Some(join_game.world_names.data),
                    dimension_codec: join_game.dimension_codec,
                    dimension: None,
                    dimension_name: Some(join_game.dimension),
                    dimension_id: None,
                    difficulty: None,
                    level_type: None,
                    world_name: Some(join_game.world_name),
                    hashed_seed: Some(join_game.hashed_seed),
                    max_players: join_game.max_players as i32,
                    view_distance: Some(join_game.view_distance.0),
                    reduced_debug_info: Some(join_game.reduced_debug_info),
                    enable_respawn_screen: Some(join_game.enable_respawn_screen),
                    is_debug: Some(join_game.is_debug),
                    is_flat: Some(join_game.is_flat),
                })
            }
            packet::Packet::JoinGame_WorldNames_IsHard(join_game) => {
                mapped_packet::MappedPacket::JoinGame(JoinGame {
                    entity_id: join_game.entity_id,
                    is_hardcore: Some(join_game.is_hardcore),
                    gamemode: join_game.gamemode,
                    previous_gamemode: Some(join_game.previous_gamemode),
                    world_names: Some(join_game.world_names.data),
                    dimension_codec: join_game.dimension_codec,
                    dimension: join_game.dimension,
                    dimension_name: None,
                    dimension_id: None,
                    difficulty: None,
                    level_type: None,
                    world_name: Some(join_game.world_name),
                    hashed_seed: Some(join_game.hashed_seed),
                    max_players: join_game.max_players.0,
                    view_distance: Some(join_game.view_distance.0),
                    reduced_debug_info: Some(join_game.reduced_debug_info),
                    enable_respawn_screen: Some(join_game.enable_respawn_screen),
                    is_debug: Some(join_game.is_debug),
                    is_flat: Some(join_game.is_flat),
                })
            }
            packet::Packet::JoinGame_HashedSeed_Respawn(join_game) => {
                mapped_packet::MappedPacket::JoinGame(JoinGame {
                    entity_id: join_game.entity_id,
                    is_hardcore: None,
                    gamemode: join_game.gamemode,
                    previous_gamemode: None,
                    world_names: None,
                    dimension_codec: None,
                    dimension: None,
                    dimension_name: None,
                    dimension_id: Some(join_game.dimension),
                    difficulty: None,
                    level_type: Some(join_game.level_type),
                    world_name: None,
                    hashed_seed: Some(join_game.hashed_seed),
                    max_players: join_game.max_players as i32,
                    view_distance: Some(join_game.view_distance.0),
                    reduced_debug_info: Some(join_game.reduced_debug_info),
                    enable_respawn_screen: Some(join_game.enable_respawn_screen),
                    is_debug: None,
                    is_flat: None,
                })
            }
            packet::Packet::KeepAliveClientbound_i32(keep_alive) => {
                mapped_packet::MappedPacket::KeepAliveClientbound(KeepAliveClientbound {
                    id: keep_alive.id as i64,
                })
            }
            packet::Packet::KeepAliveClientbound_i64(keep_alive) => {
                mapped_packet::MappedPacket::KeepAliveClientbound(KeepAliveClientbound {
                    id: keep_alive.id,
                })
            }
            packet::Packet::KeepAliveClientbound_VarInt(keep_alive) => {
                mapped_packet::MappedPacket::KeepAliveClientbound(KeepAliveClientbound {
                    id: keep_alive.id.0 as i64,
                })
            }
            packet::Packet::KeepAliveServerbound_i32(keep_alive) => {
                mapped_packet::MappedPacket::KeepAliveServerbound(KeepAliveServerbound {
                    id: keep_alive.id as i64,
                })
            }
            packet::Packet::KeepAliveServerbound_i64(keep_alive) => {
                mapped_packet::MappedPacket::KeepAliveServerbound(KeepAliveServerbound {
                    id: keep_alive.id,
                })
            }
            packet::Packet::KeepAliveServerbound_VarInt(keep_alive) => {
                mapped_packet::MappedPacket::KeepAliveServerbound(KeepAliveServerbound {
                    id: keep_alive.id.0 as i64,
                })
            }
            packet::Packet::LockDifficulty(lock_difficulty) => {
                mapped_packet::MappedPacket::LockDifficulty(LockDifficulty {
                    locked: lock_difficulty.locked,
                })
            }
            packet::Packet::LoginDisconnect(login_disconnect) => {
                mapped_packet::MappedPacket::LoginDisconnect(LoginDisconnect {
                    reason: login_disconnect.reason,
                })
            }
            packet::Packet::LoginPluginRequest(plugin_request) => {
                mapped_packet::MappedPacket::LoginPluginRequest(LoginPluginRequest {
                    message_id: plugin_request.message_id.0,
                    channel: plugin_request.channel,
                    data: plugin_request.data,
                })
            }
            packet::Packet::LoginCookieRequest(cookie_request) => {
                mapped_packet::MappedPacket::LoginCookieRequest(LoginCookieRequest {
                    key: cookie_request.key,
                })
            }
            packet::Packet::LoginPluginResponse(plugin_response) => {
                mapped_packet::MappedPacket::LoginPluginResponse(LoginPluginResponse {
                    message_id: plugin_response.message_id.0,
                    successful: plugin_response.successful,
                    data: plugin_response.data,
                })
            }
            packet::Packet::LoginAcknowledged(login_acknowledged) => {
                mapped_packet::MappedPacket::LoginAcknowledged(LoginAcknowledged {
                    empty: login_acknowledged.empty,
                })
            }
            packet::Packet::LoginCookieResponse(login_cookie_response) => {
                mapped_packet::MappedPacket::LoginCookieResponse(LoginCookieResponse {
                    key: login_cookie_response.key,
                    has_payload: login_cookie_response.has_payload,
                    payload: login_cookie_response.payload.data,
                })
            }
            packet::Packet::LoginStart(login_start) => {
                mapped_packet::MappedPacket::LoginStart(LoginStart {
                    username: login_start.username,
                })
            }
            packet::Packet::LoginSuccess_String(login_success) => {
                mapped_packet::MappedPacket::LoginSuccess_String(LoginSuccess_String {
                    uuid: login_success.uuid,
                    username: login_success.username,
                })
            }
            packet::Packet::LoginSuccess_UUID(login_success) => {
                mapped_packet::MappedPacket::LoginSuccess_UUID(LoginSuccess_UUID {
                    uuid: login_success.uuid,
                    username: login_success.username,
                })
            }
            packet::Packet::Maps(maps) => mapped_packet::MappedPacket::Maps(Maps {
                item_damage: maps.item_damage.0,
                scale: Some(maps.scale),
                tracking_position: Some(maps.tracking_position),
                locked: Some(maps.locked),
                icons: Some(maps.icons.data),
                columns: Some(maps.columns),
                rows: maps.rows,
                x: maps.x,
                z: maps.z,
                data: maps.data.map(|x| x.data),
            }),
            packet::Packet::Maps_NoLocked(maps) => mapped_packet::MappedPacket::Maps(Maps {
                item_damage: maps.item_damage.0,
                scale: Some(maps.scale),
                tracking_position: Some(maps.tracking_position),
                locked: None,
                icons: Some(maps.icons.data),
                columns: Some(maps.columns),
                rows: maps.rows,
                x: maps.x,
                z: maps.z,
                data: maps.data.map(|x| x.data),
            }),
            packet::Packet::Maps_NoTracking(maps) => mapped_packet::MappedPacket::Maps(Maps {
                item_damage: maps.item_damage.0,
                scale: Some(maps.scale),
                tracking_position: None,
                locked: None,
                icons: Some(maps.icons.data),
                columns: Some(maps.columns),
                rows: maps.rows,
                x: maps.x,
                z: maps.z,
                data: maps.data.map(|x| x.data),
            }),
            packet::Packet::Maps_NoTracking_Data(maps) => mapped_packet::MappedPacket::Maps(Maps {
                item_damage: maps.item_damage.0,
                scale: None,
                tracking_position: None,
                locked: None,
                icons: None,
                columns: None,
                rows: None,
                x: None,
                z: None,
                data: None,
            }),
            packet::Packet::MultiBlockChange_Packed(block_change) => {
                let sx = (block_change.chunk_section_pos >> 42) as i32;
                let sy = ((block_change.chunk_section_pos << 44) >> 44) as i32;
                let sz = ((block_change.chunk_section_pos << 22) >> 42) as i32;
                mapped_packet::MappedPacket::MultiBlockChange(MultiBlockChange {
                    chunk_x: sx,
                    chunk_y: Some(sy),
                    chunk_z: sz,
                    no_trust_edges: Some(block_change.no_trust_edges),
                    records: block_change
                        .records
                        .data
                        .iter()
                        .map(|record| {
                            let block_id = record.0 >> 12;
                            let z = (record.0 & 0xf) as u8;
                            let y = ((record.0 >> 4) & 0xf) as u8;
                            let x = ((record.0 >> 8) & 0xf) as u8;
                            let xz = (z & 0xF) | (x << 4);
                            BlockChangeRecord {
                                xz,
                                y,
                                block_id: block_id as i32,
                            }
                        })
                        .collect(),
                })
            }
            packet::Packet::MultiBlockChange_u16(block_change) => {
                let mut cursor = Cursor::new(block_change.data);
                let mut records = vec![];
                for _ in 0..block_change.record_count {
                    let record = cursor.read_u32::<BigEndian>().unwrap();

                    let id = record & 0x0000_ffff;
                    let y = ((record & 0x00ff_0000) >> 16) as u8;
                    let z = ((record & 0x0f00_0000) >> 24) as u8;
                    let x = ((record & 0xf000_0000) >> 28) as u8;
                    let xz = (z & 0xF) | (x << 4);
                    records.push(BlockChangeRecord {
                        xz,
                        y,
                        block_id: id as i32,
                    });
                }
                mapped_packet::MappedPacket::MultiBlockChange(MultiBlockChange {
                    chunk_x: block_change.chunk_x,
                    chunk_y: None,
                    chunk_z: block_change.chunk_z,
                    no_trust_edges: None,
                    records,
                })
            }
            packet::Packet::MultiBlockChange_VarInt(block_change) => {
                mapped_packet::MappedPacket::MultiBlockChange(MultiBlockChange {
                    chunk_x: block_change.chunk_x,
                    chunk_y: None,
                    chunk_z: block_change.chunk_z,
                    no_trust_edges: None,
                    records: block_change
                        .records
                        .data
                        .iter()
                        .map(|record| BlockChangeRecord {
                            xz: record.xz,
                            y: record.y,
                            block_id: record.block_id.0,
                        })
                        .collect(),
                })
            }
            packet::Packet::NamedSoundEffect(sound_effect) => {
                mapped_packet::MappedPacket::NamedSoundEffect(NamedSoundEffect {
                    name: sound_effect.name,
                    category: Some(sound_effect.category.0),
                    x: sound_effect.x,
                    y: sound_effect.y,
                    z: sound_effect.z,
                    volume: sound_effect.volume,
                    pitch: sound_effect.pitch,
                })
            }
            packet::Packet::NamedSoundEffect_u8(sound_effect) => {
                mapped_packet::MappedPacket::NamedSoundEffect(NamedSoundEffect {
                    name: sound_effect.name,
                    category: Some(sound_effect.category.0),
                    x: sound_effect.x,
                    y: sound_effect.y,
                    z: sound_effect.z,
                    volume: sound_effect.volume,
                    pitch: sound_effect.pitch as f32, // TODO: Conversion?
                })
            }
            packet::Packet::NamedSoundEffect_u8_NoCategory(sound_effect) => {
                mapped_packet::MappedPacket::NamedSoundEffect(NamedSoundEffect {
                    name: sound_effect.name,
                    category: None,
                    x: sound_effect.x,
                    y: sound_effect.y,
                    z: sound_effect.z,
                    volume: sound_effect.volume,
                    pitch: sound_effect.pitch as f32, // TODO: Conversion?
                })
            }
            packet::Packet::NameItem(name_item) => {
                mapped_packet::MappedPacket::NameItem(NameItem {
                    item_name: name_item.item_name,
                })
            }
            packet::Packet::NBTQueryResponse(nbt_query) => {
                mapped_packet::MappedPacket::NBTQueryResponse(NBTQueryResponse {
                    transaction_id: nbt_query.transaction_id.0,
                    nbt: nbt_query.nbt,
                })
            }
            packet::Packet::OpenBook(open_book) => {
                mapped_packet::MappedPacket::OpenBook(OpenBook {
                    hand: Hand::from(open_book.hand.0),
                })
            }
            packet::Packet::PlayPingClientbound_i32(ping) => {
                mapped_packet::MappedPacket::PlayPingClientbound(PlayPingClientbound {
                    id: ping.id,
                })
            }
            packet::Packet::Player(player) => mapped_packet::MappedPacket::Player(Player {
                on_ground: player.on_ground,
            }),
            packet::Packet::PlayerDigging(digging) => {
                mapped_packet::MappedPacket::PlayerDigging(PlayerDigging {
                    status: digging.status.0,
                    location: digging.location,
                    face: digging.face,
                })
            }
            packet::Packet::PlayerDigging_u8(digging) => {
                mapped_packet::MappedPacket::PlayerDigging(PlayerDigging {
                    status: digging.status as i32,
                    location: digging.location,
                    face: digging.face,
                })
            }
            packet::Packet::PlayerDigging_u8_u8y(digging) => {
                mapped_packet::MappedPacket::PlayerDigging(PlayerDigging {
                    status: digging.status as i32,
                    location: Position::new(digging.x, digging.y as i32, digging.z),
                    face: digging.face,
                })
            }
            packet::Packet::PlayerInfo_String(info) => {
                mapped_packet::MappedPacket::PlayerInfo_String(PlayerInfo_String {
                    name: info.name,
                    online: info.online,
                    ping: info.ping,
                })
            }
            packet::Packet::PlayerInfo(info) => {
                mapped_packet::MappedPacket::PlayerInfo(PlayerInfo { inner: info.inner })
            }
            packet::Packet::Particle_Data(particle) => {
                mapped_packet::MappedPacket::Particle(Particle {
                    particle_id: Some(particle.particle_id),
                    particle_name: None,
                    long_distance: Some(particle.long_distance),
                    x: particle.x as f64,
                    y: particle.y as f64,
                    z: particle.z as f64,
                    offset_x: particle.offset_x,
                    offset_y: particle.offset_y,
                    offset_z: particle.offset_z,
                    speed: particle.speed,
                    count: particle.count,
                    block_state: Some(particle.block_state.0),
                    red: Some(particle.red),
                    green: Some(particle.green),
                    blue: Some(particle.blue),
                    scale: Some(particle.scale),
                    item: particle.item,
                    data1: None,
                    data2: None,
                })
            }
            packet::Packet::Particle_Data13(particle) => {
                mapped_packet::MappedPacket::Particle(Particle {
                    particle_id: Some(particle.particle_id),
                    particle_name: None,
                    long_distance: Some(particle.long_distance),
                    x: particle.x as f64,
                    y: particle.y as f64,
                    z: particle.z as f64,
                    offset_x: particle.offset_x,
                    offset_y: particle.offset_y,
                    offset_z: particle.offset_z,
                    speed: particle.speed,
                    count: particle.count,
                    block_state: Some(particle.block_state.0),
                    red: Some(particle.red),
                    green: Some(particle.green),
                    blue: Some(particle.blue),
                    scale: Some(particle.scale),
                    item: particle.item,
                    data1: None,
                    data2: None,
                })
            }
            packet::Packet::Particle_f64(particle) => {
                mapped_packet::MappedPacket::Particle(Particle {
                    particle_id: Some(particle.particle_id),
                    particle_name: None,
                    long_distance: Some(particle.long_distance),
                    x: particle.x,
                    y: particle.y,
                    z: particle.z,
                    offset_x: particle.offset_x,
                    offset_y: particle.offset_y,
                    offset_z: particle.offset_z,
                    speed: particle.speed,
                    count: particle.count,
                    block_state: Some(particle.block_state.0),
                    red: Some(particle.red),
                    green: Some(particle.green),
                    blue: Some(particle.blue),
                    scale: Some(particle.scale),
                    item: particle.item,
                    data1: None,
                    data2: None,
                })
            }
            packet::Packet::Particle_Named(particle) => {
                mapped_packet::MappedPacket::Particle(Particle {
                    particle_id: None,
                    particle_name: Some(particle.particle_id),
                    long_distance: None,
                    x: particle.x as f64,
                    y: particle.y as f64,
                    z: particle.z as f64,
                    offset_x: particle.offset_x,
                    offset_y: particle.offset_y,
                    offset_z: particle.offset_z,
                    speed: particle.speed,
                    count: particle.count,
                    block_state: None,
                    red: None,
                    green: None,
                    blue: None,
                    scale: None,
                    item: None,
                    data1: None,
                    data2: None,
                })
            }
            packet::Packet::Particle_VarIntArray(particle) => {
                mapped_packet::MappedPacket::Particle(Particle {
                    particle_id: Some(particle.particle_id),
                    particle_name: None,
                    long_distance: Some(particle.long_distance),
                    x: particle.x as f64,
                    y: particle.y as f64,
                    z: particle.z as f64,
                    offset_x: particle.offset_x,
                    offset_y: particle.offset_y,
                    offset_z: particle.offset_z,
                    speed: particle.speed,
                    count: particle.count,
                    block_state: None,
                    red: None,
                    green: None,
                    blue: None,
                    scale: None,
                    item: None,
                    data1: Some(particle.data1.0),
                    data2: Some(particle.data2.0),
                })
            }
            packet::Packet::PickItem(pick_item) => {
                mapped_packet::MappedPacket::PickItem(PickItem {
                    slot_to_use: pick_item.slot_to_use.0,
                })
            }
            packet::Packet::PlayerAbilities(abilities) => {
                mapped_packet::MappedPacket::PlayerAbilities(PlayerAbilities {
                    flags: abilities.flags,
                    flying_speed: abilities.flying_speed,
                    walking_speed: abilities.walking_speed,
                })
            }
            packet::Packet::PlayerAction(action) => {
                mapped_packet::MappedPacket::PlayerAction(PlayerAction {
                    entity_id: action.entity_id.0,
                    action_id: action.action_id.0,
                    jump_boost: action.jump_boost.0,
                })
            }
            packet::Packet::PlayerAction_i32(action) => {
                mapped_packet::MappedPacket::PlayerAction(PlayerAction {
                    entity_id: action.entity_id,
                    action_id: action.action_id as i32,
                    jump_boost: action.jump_boost,
                })
            }
            packet::Packet::PlayerBlockPlacement_f32(block_placement) => {
                mapped_packet::MappedPacket::PlayerBlockPlacement(PlayerBlockPlacement {
                    location: block_placement.location,
                    face: block_placement.face.0,
                    hand: Some(block_placement.hand.0),
                    hand_item: None,
                    cursor_x: block_placement.cursor_x,
                    cursor_y: block_placement.cursor_y,
                    cursor_z: block_placement.cursor_z,
                    inside_block: None,
                })
            }
            packet::Packet::PlayerBlockPlacement_insideblock(block_placement) => {
                mapped_packet::MappedPacket::PlayerBlockPlacement(PlayerBlockPlacement {
                    location: block_placement.location,
                    face: block_placement.face.0,
                    hand: Some(block_placement.hand.0),
                    hand_item: None,
                    cursor_x: block_placement.cursor_x,
                    cursor_y: block_placement.cursor_y,
                    cursor_z: block_placement.cursor_z,
                    inside_block: Some(block_placement.inside_block),
                })
            }
            packet::Packet::PlayerBlockPlacement_u8(block_placement) => {
                mapped_packet::MappedPacket::PlayerBlockPlacement(PlayerBlockPlacement {
                    location: block_placement.location,
                    face: block_placement.face.0,
                    hand: Some(block_placement.hand.0),
                    hand_item: None,
                    cursor_x: block_placement.cursor_x as f32, // TODO: Map this properly!
                    cursor_y: block_placement.cursor_y as f32, // TODO: Map this properly!
                    cursor_z: block_placement.cursor_z as f32, // TODO: Map this properly!
                    inside_block: None,
                })
            }
            packet::Packet::PlayerBlockPlacement_u8_Item(block_placement) => {
                mapped_packet::MappedPacket::PlayerBlockPlacement(PlayerBlockPlacement {
                    location: block_placement.location,
                    face: block_placement.face as i32,
                    hand: None,
                    hand_item: block_placement.hand,
                    cursor_x: block_placement.cursor_x as f32, // TODO: Map this properly!
                    cursor_y: block_placement.cursor_y as f32, // TODO: Map this properly!
                    cursor_z: block_placement.cursor_z as f32, // TODO: Map this properly!
                    inside_block: None,
                })
            }
            packet::Packet::PlayerBlockPlacement_u8_Item_u8y(block_placement) => {
                mapped_packet::MappedPacket::PlayerBlockPlacement(PlayerBlockPlacement {
                    location: Position::new(
                        block_placement.x,
                        block_placement.y as i32,
                        block_placement.z,
                    ),
                    face: block_placement.face as i32,
                    hand: None,
                    hand_item: block_placement.hand,
                    cursor_x: block_placement.cursor_x as f32, // TODO: Map this properly!
                    cursor_y: block_placement.cursor_y as f32, // TODO: Map this properly!
                    cursor_z: block_placement.cursor_z as f32, // TODO: Map this properly!
                    inside_block: None,
                })
            }
            packet::Packet::PlayerListHeaderFooter(list_header_footer) => {
                mapped_packet::MappedPacket::PlayerListHeaderFooter(PlayerListHeaderFooter {
                    header: list_header_footer.header,
                    footer: list_header_footer.footer,
                })
            }
            packet::Packet::PlayerLook(look) => {
                mapped_packet::MappedPacket::PlayerLook(PlayerLook {
                    yaw: look.yaw,
                    pitch: look.pitch,
                    on_ground: look.on_ground,
                })
            }
            packet::Packet::PlayerPosition(position) => {
                mapped_packet::MappedPacket::PlayerPosition(PlayerPosition {
                    x: position.x,
                    y: Some(position.y),
                    z: position.z,
                    feet_y: None,
                    head_y: None,
                    on_ground: position.on_ground,
                })
            }
            packet::Packet::PlayerPosition_HeadY(position) => {
                mapped_packet::MappedPacket::PlayerPosition(PlayerPosition {
                    x: position.x,
                    y: None,
                    z: position.z,
                    feet_y: Some(position.feet_y),
                    head_y: Some(position.head_y),
                    on_ground: position.on_ground,
                })
            }
            packet::Packet::PlayerPositionLook(position_look) => {
                mapped_packet::MappedPacket::PlayerPositionLook(PlayerPositionLook {
                    x: position_look.x,
                    y: Some(position_look.y),
                    z: position_look.z,
                    feet_y: None,
                    head_y: None,
                    yaw: position_look.yaw,
                    pitch: position_look.pitch,
                    on_ground: position_look.on_ground,
                })
            }
            packet::Packet::PlayerPositionLook_HeadY(position_look) => {
                mapped_packet::MappedPacket::PlayerPositionLook(PlayerPositionLook {
                    x: position_look.x,
                    y: None,
                    z: position_look.z,
                    feet_y: Some(position_look.feet_y),
                    head_y: Some(position_look.head_y),
                    yaw: position_look.yaw,
                    pitch: position_look.pitch,
                    on_ground: position_look.on_ground,
                })
            }
            packet::Packet::PluginMessageClientbound(plugin_msg) => {
                mapped_packet::MappedPacket::PluginMessageClientbound(PluginMessageClientbound {
                    channel: plugin_msg.channel,
                    data: plugin_msg.data,
                })
            }
            packet::Packet::PlayShowDialogClientbound(show_dialog) => {
                mapped_packet::MappedPacket::PluginMessageClientbound(PluginMessageClientbound {
                    channel: "ShowDialog".to_owned(),
                    data: show_dialog.dialog_data,
                })
            }
            packet::Packet::PluginMessageClientbound_i16(plugin_msg) => {
                mapped_packet::MappedPacket::PluginMessageClientbound(PluginMessageClientbound {
                    channel: plugin_msg.channel,
                    data: plugin_msg.data.data,
                })
            }
            packet::Packet::PluginMessageServerbound(plugin_msg) => {
                mapped_packet::MappedPacket::PluginMessageServerbound(PluginMessageServerbound {
                    channel: plugin_msg.channel,
                    data: plugin_msg.data,
                })
            }
            packet::Packet::PluginMessageServerbound_i16(plugin_msg) => {
                mapped_packet::MappedPacket::PluginMessageServerbound(PluginMessageServerbound {
                    channel: plugin_msg.channel,
                    data: plugin_msg.data.data,
                })
            }
            packet::Packet::QueryBlockNBT(block_nbt) => {
                mapped_packet::MappedPacket::QueryBlockNBT(QueryBlockNBT {
                    transaction_id: block_nbt.transaction_id.0,
                    location: block_nbt.location,
                })
            }
            packet::Packet::QueryEntityNBT(entity_nbt) => {
                mapped_packet::MappedPacket::QueryEntityNBT(QueryEntityNBT {
                    transaction_id: entity_nbt.transaction_id.0,
                    entity_id: entity_nbt.entity_id.0,
                })
            }
            packet::Packet::Respawn_WorldName(respawn) => {
                mapped_packet::MappedPacket::Respawn(Respawn {
                    dimension_tag: None,
                    dimension_name: Some(respawn.dimension),
                    world_name: Some(respawn.world_name),
                    dimension: None,
                    hashed_seed: Some(respawn.hashed_seed),
                    difficulty: None,
                    gamemode: respawn.gamemode,
                    level_type: None,
                    previous_gamemode: Some(respawn.previous_gamemode),
                    is_debug: Some(respawn.is_debug),
                    is_flat: Some(respawn.is_flat),
                    copy_metadata: Some(respawn.copy_metadata),
                })
            }
            packet::Packet::Respawn_NBT(respawn) => mapped_packet::MappedPacket::Respawn(Respawn {
                dimension_tag: respawn.dimension,
                dimension_name: None,
                world_name: Some(respawn.world_name),
                dimension: None,
                hashed_seed: Some(respawn.hashed_seed),
                difficulty: None,
                gamemode: respawn.gamemode,
                level_type: None,
                previous_gamemode: Some(respawn.previous_gamemode),
                is_debug: Some(respawn.is_debug),
                is_flat: Some(respawn.is_flat),
                copy_metadata: Some(respawn.copy_metadata),
            }),
            packet::Packet::Respawn_HashedSeed(respawn) => {
                mapped_packet::MappedPacket::Respawn(Respawn {
                    dimension_tag: None,
                    dimension_name: None,
                    world_name: None,
                    dimension: Some(respawn.dimension),
                    hashed_seed: Some(respawn.hashed_seed),
                    difficulty: Some(respawn.difficulty),
                    gamemode: respawn.gamemode,
                    level_type: Some(respawn.level_type),
                    previous_gamemode: None,
                    is_debug: None,
                    is_flat: None,
                    copy_metadata: None,
                })
            }
            packet::Packet::Respawn_Gamemode(respawn) => {
                mapped_packet::MappedPacket::Respawn(Respawn {
                    dimension_tag: None,
                    dimension_name: None,
                    world_name: None,
                    dimension: Some(respawn.dimension),
                    hashed_seed: None,
                    difficulty: Some(respawn.difficulty),
                    gamemode: respawn.gamemode,
                    level_type: Some(respawn.level_type),
                    previous_gamemode: None,
                    is_debug: None,
                    is_flat: None,
                    copy_metadata: None,
                })
            }
            packet::Packet::ResourcePackSend(resource_pack) => {
                mapped_packet::MappedPacket::ResourcePackSend(ResourcePackSend {
                    url: resource_pack.url,
                    hash: resource_pack.hash,
                })
            }
            packet::Packet::ResourcePackStatus(resource_pack) => {
                mapped_packet::MappedPacket::ResourcePackStatus(ResourcePackStatus {
                    hash: None,
                    result: resource_pack.result.0,
                })
            }
            packet::Packet::ResourcePackStatus_hash(resource_pack) => {
                mapped_packet::MappedPacket::ResourcePackStatus(ResourcePackStatus {
                    hash: Some(resource_pack.hash),
                    result: resource_pack.result.0,
                })
            }
            packet::Packet::SpawnMob_WithMeta(spawn_mob) => {
                mapped_packet::MappedPacket::SpawnMob(SpawnMob {
                    entity_id: spawn_mob.entity_id.0,
                    uuid: Some(spawn_mob.uuid),
                    ty: spawn_mob.ty.0,
                    x: spawn_mob.x,
                    y: spawn_mob.y,
                    z: spawn_mob.z,
                    yaw: spawn_mob.yaw,
                    pitch: spawn_mob.pitch,
                    head_pitch: spawn_mob.head_pitch,
                    velocity_x: spawn_mob.velocity_x,
                    velocity_y: spawn_mob.velocity_y,
                    velocity_z: spawn_mob.velocity_z,
                    metadata: Some(spawn_mob.metadata),
                })
            }
            packet::Packet::SpawnMob_NoMeta(spawn_mob) => {
                mapped_packet::MappedPacket::SpawnMob(SpawnMob {
                    entity_id: spawn_mob.entity_id.0,
                    uuid: Some(spawn_mob.uuid),
                    ty: spawn_mob.ty.0,
                    x: spawn_mob.x,
                    y: spawn_mob.y,
                    z: spawn_mob.z,
                    yaw: spawn_mob.yaw,
                    pitch: spawn_mob.pitch,
                    head_pitch: spawn_mob.head_pitch,
                    velocity_x: spawn_mob.velocity_x,
                    velocity_y: spawn_mob.velocity_y,
                    velocity_z: spawn_mob.velocity_z,
                    metadata: None,
                })
            }
            packet::Packet::SpawnMob_u8(spawn_mob) => {
                mapped_packet::MappedPacket::SpawnMob(SpawnMob {
                    entity_id: spawn_mob.entity_id.0,
                    uuid: Some(spawn_mob.uuid),
                    ty: spawn_mob.ty as i32,
                    x: spawn_mob.x,
                    y: spawn_mob.y,
                    z: spawn_mob.z,
                    yaw: spawn_mob.yaw,
                    pitch: spawn_mob.pitch,
                    head_pitch: spawn_mob.head_pitch,
                    velocity_x: spawn_mob.velocity_x,
                    velocity_y: spawn_mob.velocity_y,
                    velocity_z: spawn_mob.velocity_z,
                    metadata: Some(spawn_mob.metadata),
                })
            }
            packet::Packet::SpawnMob_u8_i32(spawn_mob) => {
                mapped_packet::MappedPacket::SpawnMob(SpawnMob {
                    entity_id: spawn_mob.entity_id.0,
                    uuid: Some(spawn_mob.uuid),
                    ty: spawn_mob.ty as i32,
                    x: From::from(spawn_mob.x),
                    y: From::from(spawn_mob.y),
                    z: From::from(spawn_mob.z),
                    yaw: spawn_mob.yaw,
                    pitch: spawn_mob.pitch,
                    head_pitch: spawn_mob.head_pitch,
                    velocity_x: spawn_mob.velocity_x,
                    velocity_y: spawn_mob.velocity_y,
                    velocity_z: spawn_mob.velocity_z,
                    metadata: Some(spawn_mob.metadata),
                })
            }
            packet::Packet::SpawnMob_u8_i32_NoUUID(spawn_mob) => {
                mapped_packet::MappedPacket::SpawnMob(SpawnMob {
                    entity_id: spawn_mob.entity_id.0,
                    uuid: None,
                    ty: spawn_mob.ty as i32,
                    x: From::from(spawn_mob.x),
                    y: From::from(spawn_mob.y),
                    z: From::from(spawn_mob.z),
                    yaw: spawn_mob.yaw,
                    pitch: spawn_mob.pitch,
                    head_pitch: spawn_mob.head_pitch,
                    velocity_x: spawn_mob.velocity_x,
                    velocity_y: spawn_mob.velocity_y,
                    velocity_z: spawn_mob.velocity_z,
                    metadata: Some(spawn_mob.metadata),
                })
            }
            packet::Packet::SpawnObject(spawn_object) => {
                mapped_packet::MappedPacket::SpawnObject(SpawnObject {
                    entity_id: spawn_object.entity_id.0,
                    uuid: Some(spawn_object.uuid),
                    ty: spawn_object.ty as i32,
                    x: spawn_object.x,
                    y: spawn_object.y,
                    z: spawn_object.z,
                    pitch: spawn_object.pitch,
                    yaw: spawn_object.yaw,
                    data: spawn_object.data,
                    velocity_x: spawn_object.velocity_x,
                    velocity_y: spawn_object.velocity_y,
                    velocity_z: spawn_object.velocity_z,
                })
            }
            packet::Packet::SpawnObject_VarInt(spawn_object) => {
                mapped_packet::MappedPacket::SpawnObject(SpawnObject {
                    entity_id: spawn_object.entity_id.0,
                    uuid: Some(spawn_object.uuid),
                    ty: spawn_object.ty.0,
                    x: spawn_object.x,
                    y: spawn_object.y,
                    z: spawn_object.z,
                    pitch: spawn_object.pitch,
                    yaw: spawn_object.yaw,
                    data: spawn_object.data,
                    velocity_x: spawn_object.velocity_x,
                    velocity_y: spawn_object.velocity_y,
                    velocity_z: spawn_object.velocity_z,
                })
            }
            packet::Packet::SpawnObject_i32(spawn_object) => {
                mapped_packet::MappedPacket::SpawnObject(SpawnObject {
                    entity_id: spawn_object.entity_id.0,
                    uuid: Some(spawn_object.uuid),
                    ty: spawn_object.ty as i32,
                    x: From::from(spawn_object.x),
                    y: From::from(spawn_object.y),
                    z: From::from(spawn_object.z),
                    pitch: spawn_object.pitch,
                    yaw: spawn_object.yaw,
                    data: spawn_object.data,
                    velocity_x: spawn_object.velocity_x,
                    velocity_y: spawn_object.velocity_y,
                    velocity_z: spawn_object.velocity_z,
                })
            }
            packet::Packet::SpawnObject_i32_NoUUID(spawn_object) => {
                mapped_packet::MappedPacket::SpawnObject(SpawnObject {
                    entity_id: spawn_object.entity_id.0,
                    uuid: None,
                    ty: spawn_object.ty as i32,
                    x: From::from(spawn_object.x),
                    y: From::from(spawn_object.y),
                    z: From::from(spawn_object.z),
                    pitch: spawn_object.pitch,
                    yaw: spawn_object.yaw,
                    data: spawn_object.data,
                    velocity_x: spawn_object.velocity_x,
                    velocity_y: spawn_object.velocity_y,
                    velocity_z: spawn_object.velocity_z,
                })
            }
            packet::Packet::SetCurrentHotbarSlot(set_slot) => {
                mapped_packet::MappedPacket::SetCurrentHotbarSlot(SetCurrentHotbarSlot {
                    slot: set_slot.slot,
                })
            }
            packet::Packet::ServerMessage_Sender(server_msg) => {
                mapped_packet::MappedPacket::ServerMessage(ServerMessage {
                    message: server_msg.message,
                    position: Some(server_msg.position),
                    sender: Some(server_msg.sender),
                })
            }
            packet::Packet::ServerMessage_Position(server_msg) => {
                mapped_packet::MappedPacket::ServerMessage(ServerMessage {
                    message: server_msg.message,
                    position: Some(server_msg.position),
                    sender: None,
                })
            }
            packet::Packet::ServerMessage_NoPosition(server_msg) => {
                mapped_packet::MappedPacket::ServerMessage(ServerMessage {
                    message: server_msg.message,
                    position: None,
                    sender: None,
                })
            }
            packet::Packet::SpawnPlayer_f64(spawn_player) => {
                mapped_packet::MappedPacket::SpawnPlayer(SpawnPlayer {
                    entity_id: spawn_player.entity_id.0,
                    uuid: Some(spawn_player.uuid),
                    uuid_str: None,
                    name: None,
                    properties: None,
                    x: spawn_player.x,
                    y: spawn_player.y,
                    z: spawn_player.z,
                    yaw: spawn_player.yaw,
                    pitch: spawn_player.pitch,
                    current_item: None,
                    metadata: Some(spawn_player.metadata),
                })
            }
            packet::Packet::SpawnPlayer_f64_NoMeta(spawn_player) => {
                mapped_packet::MappedPacket::SpawnPlayer(SpawnPlayer {
                    entity_id: spawn_player.entity_id.0,
                    uuid: Some(spawn_player.uuid),
                    uuid_str: None,
                    name: None,
                    properties: None,
                    x: spawn_player.x,
                    y: spawn_player.y,
                    z: spawn_player.z,
                    yaw: spawn_player.yaw,
                    pitch: spawn_player.pitch,
                    current_item: None,
                    metadata: None,
                })
            }
            packet::Packet::SpawnPlayer_i32(spawn_player) => {
                mapped_packet::MappedPacket::SpawnPlayer(SpawnPlayer {
                    entity_id: spawn_player.entity_id.0,
                    uuid: Some(spawn_player.uuid),
                    uuid_str: None,
                    name: None,
                    properties: None,
                    x: From::from(spawn_player.x),
                    y: From::from(spawn_player.y),
                    z: From::from(spawn_player.z),
                    yaw: spawn_player.yaw,
                    pitch: spawn_player.pitch,
                    current_item: None,
                    metadata: Some(spawn_player.metadata),
                })
            }
            packet::Packet::SpawnPlayer_i32_HeldItem(spawn_player) => {
                mapped_packet::MappedPacket::SpawnPlayer(SpawnPlayer {
                    entity_id: spawn_player.entity_id.0,
                    uuid: Some(spawn_player.uuid),
                    uuid_str: None,
                    name: None,
                    properties: None,
                    x: From::from(spawn_player.x),
                    y: From::from(spawn_player.y),
                    z: From::from(spawn_player.z),
                    yaw: spawn_player.yaw,
                    pitch: spawn_player.pitch,
                    current_item: Some(spawn_player.current_item),
                    metadata: Some(spawn_player.metadata),
                })
            }
            packet::Packet::SpawnPlayer_i32_HeldItem_String(spawn_player) => {
                mapped_packet::MappedPacket::SpawnPlayer(SpawnPlayer {
                    entity_id: spawn_player.entity_id.0,
                    uuid: None,
                    uuid_str: Some(spawn_player.uuid),
                    name: Some(spawn_player.name),
                    properties: Some(spawn_player.properties.data),
                    x: From::from(spawn_player.x),
                    y: From::from(spawn_player.y),
                    z: From::from(spawn_player.z),
                    yaw: spawn_player.yaw,
                    pitch: spawn_player.pitch,
                    current_item: Some(spawn_player.current_item),
                    metadata: Some(spawn_player.metadata),
                })
            }
            packet::Packet::ScoreboardDisplay(display) => {
                mapped_packet::MappedPacket::ScoreboardDisplay(ScoreboardDisplay {
                    position: display.position,
                    name: display.name,
                })
            }
            packet::Packet::ScoreboardObjective(objective) => {
                mapped_packet::MappedPacket::ScoreboardObjective(ScoreboardObjective {
                    name: objective.name,
                    mode: Some(objective.mode),
                    value: objective.value,
                    ty_str: Some(objective.ty),
                    ty: None,
                })
            }
            packet::Packet::ScoreboardObjective_NoMode(objective) => {
                mapped_packet::MappedPacket::ScoreboardObjective(ScoreboardObjective {
                    name: objective.name,
                    mode: None,
                    value: objective.value,
                    ty_str: None,
                    ty: Some(objective.ty),
                })
            }
            packet::Packet::SelectAdvancementTab(advancements_tab) => {
                mapped_packet::MappedPacket::SelectAdvancementTab(SelectAdvancementTab {
                    has_id: advancements_tab.has_id,
                    tab_id: advancements_tab.tab_id,
                })
            }
            packet::Packet::SelectTrade(trade) => {
                mapped_packet::MappedPacket::SelectTrade(SelectTrade {
                    selected_slot: trade.selected_slot.0,
                })
            }
            packet::Packet::ServerDifficulty(difficulty) => {
                mapped_packet::MappedPacket::ServerDifficulty(ServerDifficulty {
                    difficulty: difficulty.difficulty,
                    locked: None,
                })
            }
            packet::Packet::ServerDifficulty_Locked(difficulty) => {
                mapped_packet::MappedPacket::ServerDifficulty(ServerDifficulty {
                    difficulty: difficulty.difficulty,
                    locked: Some(difficulty.locked),
                })
            }
            packet::Packet::SetBeaconEffect(beacon) => {
                mapped_packet::MappedPacket::SetBeaconEffect(SetBeaconEffect {
                    primary_effect: beacon.primary_effect.0,
                    secondary_effect: beacon.secondary_effect.0,
                })
            }
            packet::Packet::SetCompression(compression) => {
                mapped_packet::MappedPacket::SetCompression(SetCompression {
                    threshold: compression.threshold.0,
                })
            }
            packet::Packet::SetCooldown(cooldown) => {
                mapped_packet::MappedPacket::SetCooldown(SetCooldown {
                    item_id: cooldown.item_id.0,
                    ticks: cooldown.ticks.0,
                })
            }
            packet::Packet::SetDifficulty(difficulty) => {
                mapped_packet::MappedPacket::SetDifficulty(SetDifficulty {
                    new_difficulty: difficulty.new_difficulty,
                })
            }
            packet::Packet::SetDisplayedRecipe(displayed_recipe) => {
                mapped_packet::MappedPacket::SetDisplayedRecipe(SetDisplayedRecipe {
                    recipe_id: displayed_recipe.recipe_id,
                })
            }
            packet::Packet::SetExperience(set_exp) => {
                mapped_packet::MappedPacket::SetExperience(SetExperience {
                    experience_bar: set_exp.experience_bar,
                    level: set_exp.level.0,
                    total_experience: set_exp.total_experience.0,
                })
            }
            packet::Packet::SetExperience_i16(set_exp) => {
                mapped_packet::MappedPacket::SetExperience(SetExperience {
                    experience_bar: set_exp.experience_bar,
                    level: set_exp.level as i32,
                    total_experience: set_exp.total_experience as i32,
                })
            }
            packet::Packet::SetInitialCompression(init_comp) => {
                mapped_packet::MappedPacket::SetInitialCompression(SetInitialCompression {
                    threshold: init_comp.threshold.0,
                })
            }
            packet::Packet::SetPassengers(passengers) => {
                mapped_packet::MappedPacket::SetPassengers(SetPassengers {
                    entity_id: passengers.entity_id.0,
                    passengers: passengers.passengers.data.iter().map(|x| x.0).collect(),
                })
            }
            packet::Packet::SetRecipeBookState(recipe_book) => {
                mapped_packet::MappedPacket::SetRecipeBookState(SetRecipeBookState {
                    book_id: recipe_book.book_id.0,
                    book_open: recipe_book.book_open,
                    filter_active: recipe_book.filter_active,
                })
            }
            packet::Packet::SetSign(set_sign) => mapped_packet::MappedPacket::SetSign(SetSign {
                location: set_sign.location,
                line1: set_sign.line1,
                line2: set_sign.line2,
                line3: set_sign.line3,
                line4: set_sign.line4,
            }),
            packet::Packet::SetSign_i16y(set_sign) => {
                mapped_packet::MappedPacket::SetSign(SetSign {
                    location: Position::new(set_sign.x, set_sign.y as i32, set_sign.z),
                    line1: set_sign.line1,
                    line2: set_sign.line2,
                    line3: set_sign.line3,
                    line4: set_sign.line4,
                })
            }
            packet::Packet::SignEditorOpen(sign_editor) => {
                mapped_packet::MappedPacket::SignEditorOpen(SignEditorOpen {
                    location: sign_editor.location,
                })
            }
            packet::Packet::SignEditorOpen_i32(sign_editor) => {
                mapped_packet::MappedPacket::SignEditorOpen(SignEditorOpen {
                    location: Position::new(sign_editor.x, sign_editor.y, sign_editor.z),
                })
            }
            packet::Packet::SoundEffect(sound) => {
                mapped_packet::MappedPacket::SoundEffect(SoundEffect {
                    name: sound.name.0,
                    category: sound.category.0,
                    x: sound.x,
                    y: sound.y,
                    z: sound.z,
                    volume: sound.volume,
                    pitch: sound.pitch,
                })
            }
            packet::Packet::PlaySoundClientbound(sound) => {
                mapped_packet::MappedPacket::SoundEffect(SoundEffect {
                    name: sound.sound_holder_id.0,
                    category: sound.source.0,
                    x: sound.x,
                    y: sound.y,
                    z: sound.z,
                    volume: sound.volume,
                    pitch: sound.pitch,
                })
            }
            packet::Packet::PlaySoundEntityClientbound(sound) => {
                mapped_packet::MappedPacket::EntitySoundEffect(EntitySoundEffect {
                    sound_id: sound.sound_holder_id.0,
                    sound_category: sound.source.0,
                    entity_id: sound.entity_id.0,
                    volume: sound.volume,
                    pitch: sound.pitch,
                })
            }
            packet::Packet::SoundEffect_u8(sound) => {
                mapped_packet::MappedPacket::SoundEffect(SoundEffect {
                    name: sound.name.0,
                    category: sound.category.0,
                    x: sound.x,
                    y: sound.y,
                    z: sound.z,
                    volume: sound.volume,
                    pitch: sound.pitch as f32, // TODO: Convert this somehow?
                })
            }
            packet::Packet::SpawnExperienceOrb(exp_orb) => {
                mapped_packet::MappedPacket::SpawnExperienceOrb(SpawnExperienceOrb {
                    entity_id: exp_orb.entity_id.0,
                    x: exp_orb.x,
                    y: exp_orb.y,
                    z: exp_orb.z,
                    count: exp_orb.count,
                })
            }
            packet::Packet::SpawnExperienceOrb_i32(exp_orb) => {
                mapped_packet::MappedPacket::SpawnExperienceOrb(SpawnExperienceOrb {
                    entity_id: exp_orb.entity_id.0,
                    x: From::from(exp_orb.x),
                    y: From::from(exp_orb.y),
                    z: From::from(exp_orb.z),
                    count: exp_orb.count,
                })
            }
            packet::Packet::SpawnGlobalEntity(global) => {
                mapped_packet::MappedPacket::SpawnGlobalEntity(SpawnGlobalEntity {
                    entity_id: global.entity_id.0,
                    ty: global.ty,
                    x: global.x,
                    y: global.y,
                    z: global.z,
                })
            }
            packet::Packet::SpawnGlobalEntity_i32(global) => {
                mapped_packet::MappedPacket::SpawnGlobalEntity(SpawnGlobalEntity {
                    entity_id: global.entity_id.0,
                    ty: global.ty,
                    x: From::from(global.x),
                    y: From::from(global.y),
                    z: From::from(global.z),
                })
            }
            packet::Packet::SpawnPainting_NoUUID(painting) => {
                mapped_packet::MappedPacket::SpawnPainting(SpawnPainting {
                    entity_id: painting.entity_id.0,
                    uuid: None,
                    motive: None,
                    title: Some(painting.title),
                    location: painting.location,
                    direction: painting.direction as i32,
                })
            }
            packet::Packet::SpawnPainting_NoUUID_i32(painting) => {
                mapped_packet::MappedPacket::SpawnPainting(SpawnPainting {
                    entity_id: painting.entity_id.0,
                    uuid: None,
                    motive: None,
                    title: Some(painting.title),
                    location: Position::new(painting.x, painting.y, painting.z),
                    direction: painting.direction,
                })
            }
            packet::Packet::SpawnPainting_String(painting) => {
                mapped_packet::MappedPacket::SpawnPainting(SpawnPainting {
                    entity_id: painting.entity_id.0,
                    uuid: Some(painting.uuid),
                    motive: None,
                    title: Some(painting.title),
                    location: painting.location,
                    direction: painting.direction as i32,
                })
            }
            packet::Packet::SpawnPainting_VarInt(painting) => {
                mapped_packet::MappedPacket::SpawnPainting(SpawnPainting {
                    entity_id: painting.entity_id.0,
                    uuid: Some(painting.uuid),
                    motive: Some(painting.motive.0),
                    title: None,
                    location: painting.location,
                    direction: painting.direction as i32,
                })
            }
            packet::Packet::SpawnPosition(position) => {
                mapped_packet::MappedPacket::SpawnPosition(SpawnPosition {
                    location: position.location,
                })
            }
            packet::Packet::SpawnPosition_i32(position) => {
                mapped_packet::MappedPacket::SpawnPosition(SpawnPosition {
                    location: Position::new(position.x, position.y, position.z),
                })
            }
            packet::Packet::SpectateTeleport(teleport) => {
                mapped_packet::MappedPacket::SpectateTeleport(SpectateTeleport {
                    target: teleport.target,
                })
            }
            packet::Packet::Statistics(statistics) => {
                mapped_packet::MappedPacket::Statistics(Statistics {
                    statistices: statistics.statistices.data,
                })
            }
            packet::Packet::StatusPing(ping) => {
                mapped_packet::MappedPacket::StatusPing(StatusPing { ping: ping.ping })
            }
            packet::Packet::StatusPong(pong) => {
                mapped_packet::MappedPacket::StatusPong(StatusPong { ping: pong.ping })
            }
            packet::Packet::StatusRequest(_request) => {
                mapped_packet::MappedPacket::StatusRequest(StatusRequest { empty: () })
            }
            packet::Packet::StatusResponse(response) => {
                mapped_packet::MappedPacket::StatusResponse(StatusResponse {
                    status: response.status,
                })
            }
            packet::Packet::SteerBoat(steer_boat) => {
                mapped_packet::MappedPacket::SteerBoat(SteerBoat {
                    left_paddle_turning: steer_boat.left_paddle_turning,
                    right_paddle_turning: steer_boat.right_paddle_turning,
                })
            }
            packet::Packet::SteerVehicle(steer_vehicle) => {
                mapped_packet::MappedPacket::SteerVehicle(SteerVehicle {
                    sideways: steer_vehicle.sideways,
                    forward: steer_vehicle.forward,
                    flags: Some(steer_vehicle.flags),
                    jump: None,
                    unmount: None,
                })
            }
            packet::Packet::SteerVehicle_jump_unmount(steer_vehicle) => {
                mapped_packet::MappedPacket::SteerVehicle(SteerVehicle {
                    sideways: steer_vehicle.sideways,
                    forward: steer_vehicle.forward,
                    flags: None,
                    jump: Some(steer_vehicle.jump),
                    unmount: Some(steer_vehicle.unmount),
                })
            }
            packet::Packet::StopSound(stop_sound) => {
                mapped_packet::MappedPacket::StopSound(StopSound {
                    flags: stop_sound.flags,
                    source: stop_sound.source.map(|x| x.0),
                    sound: stop_sound.sound,
                })
            }
            packet::Packet::TimeUpdate(time_update) => {
                mapped_packet::MappedPacket::TimeUpdate(TimeUpdate {
                    world_age: time_update.world_age,
                    time_of_day: time_update.time_of_day,
                })
            }
            packet::Packet::TeleportConfirm(teleport_confirm) => {
                mapped_packet::MappedPacket::TeleportConfirm(TeleportConfirm {
                    teleport_id: teleport_confirm.teleport_id.0,
                })
            }
            packet::Packet::TeleportPlayer_OnGround(tp_player) => {
                mapped_packet::MappedPacket::TeleportPlayer(TeleportPlayer {
                    x: tp_player.x,
                    y: tp_player.eyes_y - 1.62,
                    z: tp_player.z,
                    yaw: tp_player.yaw,
                    pitch: tp_player.pitch,
                    flags: None,
                    teleport_id: None,
                    on_ground: Some(tp_player.on_ground),
                })
            }
            packet::Packet::TeleportPlayer_NoConfirm(tp_player) => {
                mapped_packet::MappedPacket::TeleportPlayer(TeleportPlayer {
                    x: tp_player.x,
                    y: tp_player.y,
                    z: tp_player.z,
                    yaw: tp_player.yaw,
                    pitch: tp_player.pitch,
                    flags: Some(tp_player.flags),
                    teleport_id: None,
                    on_ground: None,
                })
            }
            packet::Packet::TeleportPlayer_WithConfirm(tp_player) => {
                mapped_packet::MappedPacket::TeleportPlayer(TeleportPlayer {
                    x: tp_player.x,
                    y: tp_player.y,
                    z: tp_player.z,
                    yaw: tp_player.yaw,
                    pitch: tp_player.pitch,
                    flags: Some(tp_player.flags),
                    teleport_id: Some(tp_player.teleport_id.0),
                    on_ground: None,
                })
            }
            packet::Packet::TabComplete(tab_complete) => {
                mapped_packet::MappedPacket::TabComplete(TabComplete {
                    text: tab_complete.text,
                    assume_command: Some(tab_complete.assume_command),
                    has_target: Some(tab_complete.has_target),
                    target: tab_complete.target,
                })
            }
            packet::Packet::TabComplete_NoAssume(tab_complete) => {
                mapped_packet::MappedPacket::TabComplete(TabComplete {
                    text: tab_complete.text,
                    assume_command: None,
                    has_target: Some(tab_complete.has_target),
                    target: tab_complete.target,
                })
            }
            packet::Packet::TabComplete_NoAssume_NoTarget(tab_complete) => {
                mapped_packet::MappedPacket::TabComplete(TabComplete {
                    text: tab_complete.text,
                    assume_command: None,
                    has_target: None,
                    target: None,
                })
            }
            packet::Packet::TabCompleteReply(reply) => {
                mapped_packet::MappedPacket::TabCompleteReply(TabCompleteReply {
                    matches: reply.matches.data,
                })
            }
            packet::Packet::Tags(tags) => mapped_packet::MappedPacket::Tags(Tags {
                block_tags: tags.block_tags.data,
                item_tags: tags.item_tags.data,
                fluid_tags: tags.fluid_tags.data,
                entity_tags: None,
            }),
            packet::Packet::TagsWithEntities(tags) => mapped_packet::MappedPacket::Tags(Tags {
                block_tags: tags.block_tags.data,
                item_tags: tags.item_tags.data,
                fluid_tags: tags.fluid_tags.data,
                entity_tags: Some(tags.entity_tags.data),
            }),
            packet::Packet::Teams_u8(teams) => mapped_packet::MappedPacket::Teams(Teams {
                name: teams.name,
                mode: teams.mode,
                display_name: teams.display_name,
                flags: teams.flags,
                name_tag_visibility: teams.name_tag_visibility,
                collision_rule: teams.collision_rule,
                formatting: None,
                prefix: teams.prefix,
                suffix: teams.suffix,
                players: teams.players.map(|x| x.data),
                color: teams.color,
                data: Some(teams.data),
            }),
            packet::Packet::Teams_NoVisColor(teams) => mapped_packet::MappedPacket::Teams(Teams {
                name: teams.name,
                mode: teams.mode,
                display_name: teams.display_name,
                flags: teams.flags,
                name_tag_visibility: None,
                collision_rule: None,
                formatting: None,
                prefix: teams.prefix,
                suffix: teams.suffix,
                players: teams.players.map(|x| x.data),
                color: None,
                data: None,
            }),
            packet::Packet::Teams_VarInt(teams) => mapped_packet::MappedPacket::Teams(Teams {
                name: teams.name,
                mode: teams.mode,
                display_name: teams.display_name,
                flags: teams.flags,
                name_tag_visibility: teams.name_tag_visibility,
                collision_rule: teams.collision_rule,
                formatting: teams.formatting.map(|x| x.0),
                prefix: teams.prefix,
                suffix: teams.suffix,
                players: teams.players.map(|x| x.data),
                color: None,
                data: None,
            }),
            packet::Packet::Title(title) => mapped_packet::MappedPacket::Title(Title {
                action: title.action.0,
                title: title.title,
                sub_title: title.sub_title,
                action_bar_text: title.action_bar_text,
                fade_in: title.fade_in,
                fade_stay: title.fade_stay,
                fade_out: title.fade_out,
                fade_in_comp: None,
                fade_stay_comp: None,
                fade_out_comp: None,
            }),
            packet::Packet::Title_notext(title) => mapped_packet::MappedPacket::Title(Title {
                action: title.action.0,
                title: title.title,
                sub_title: title.sub_title,
                action_bar_text: None,
                fade_in: title.fade_in,
                fade_stay: title.fade_stay,
                fade_out: title.fade_out,
                fade_in_comp: None,
                fade_stay_comp: None,
                fade_out_comp: None,
            }),
            packet::Packet::Title_notext_component(title) => {
                mapped_packet::MappedPacket::Title(Title {
                    action: title.action.0,
                    title: title.title,
                    sub_title: title.sub_title,
                    action_bar_text: None,
                    fade_in: None,
                    fade_stay: None,
                    fade_out: None,
                    fade_in_comp: title.fade_in,
                    fade_stay_comp: title.fade_stay,
                    fade_out_comp: title.fade_out,
                })
            }
            packet::Packet::TradeList_WithoutRestock(trade_list) => {
                mapped_packet::MappedPacket::TradeList(TradeList {
                    id: trade_list.id.0,
                    trades: trade_list.trades.data,
                    villager_level: trade_list.villager_level.0,
                    experience: trade_list.experience.0,
                    is_regular_villager: trade_list.is_regular_villager,
                    can_restock: None,
                })
            }
            packet::Packet::TradeList_WithRestock(trade_list) => {
                mapped_packet::MappedPacket::TradeList(TradeList {
                    id: trade_list.id.0,
                    trades: trade_list.trades.data,
                    villager_level: trade_list.villager_level.0,
                    experience: trade_list.experience.0,
                    is_regular_villager: trade_list.is_regular_villager,
                    can_restock: Some(trade_list.can_restock),
                })
            }
            packet::Packet::UpdateHealth(health) => {
                mapped_packet::MappedPacket::UpdateHealth(UpdateHealth {
                    health: health.health,
                    food: health.food.0,
                    food_saturation: health.food_saturation,
                })
            }
            packet::Packet::UpdateHealth_u16(health) => {
                mapped_packet::MappedPacket::UpdateHealth(UpdateHealth {
                    health: health.health,
                    food: health.food as i32,
                    food_saturation: health.food_saturation,
                })
            }
            packet::Packet::UpdateLight_WithTrust(light) => {
                mapped_packet::MappedPacket::UpdateLight(UpdateLight {
                    chunk_x: light.chunk_x.0,
                    chunk_z: light.chunk_z.0,
                    trust_edges: Some(light.trust_edges),
                    sky_light_mask: light.sky_light_mask.0,
                    block_light_mask: light.block_light_mask.0,
                    empty_block_light_mask: light.empty_block_light_mask.0,
                    empty_sky_light_mask: light.empty_sky_light_mask.0,
                    light_arrays: light.light_arrays,
                })
            }
            packet::Packet::UpdateLight_NoTrust(light) => {
                mapped_packet::MappedPacket::UpdateLight(UpdateLight {
                    chunk_x: light.chunk_x.0,
                    chunk_z: light.chunk_z.0,
                    trust_edges: None,
                    sky_light_mask: light.sky_light_mask.0,
                    block_light_mask: light.block_light_mask.0,
                    empty_block_light_mask: light.empty_block_light_mask.0,
                    empty_sky_light_mask: light.empty_sky_light_mask.0,
                    light_arrays: light.light_arrays,
                })
            }
            packet::Packet::UpdateViewPosition(view_position) => {
                mapped_packet::MappedPacket::UpdateViewPosition(UpdateViewPosition {
                    chunk_x: view_position.chunk_x.0,
                    chunk_z: view_position.chunk_z.0,
                })
            }
            packet::Packet::UpdateBlockEntity(block_entity) => {
                mapped_packet::MappedPacket::UpdateBlockEntity(UpdateBlockEntity {
                    location: block_entity.location,
                    action: block_entity.action,
                    nbt: block_entity.nbt,
                    data_length: None,
                    gzipped_nbt: None,
                })
            }
            packet::Packet::UpdateBlockEntity_Data(block_entity) => {
                mapped_packet::MappedPacket::UpdateBlockEntity(UpdateBlockEntity {
                    location: Position::new(block_entity.x, block_entity.y as i32, block_entity.z),
                    action: block_entity.action,
                    nbt: None,
                    data_length: Some(block_entity.data_length),
                    gzipped_nbt: Some(block_entity.gzipped_nbt),
                })
            }
            packet::Packet::UpdateSign(sign) => {
                mapped_packet::MappedPacket::UpdateSign(UpdateSign {
                    location: sign.location,
                    line1: sign.line1,
                    line2: sign.line2,
                    line3: sign.line3,
                    line4: sign.line4,
                })
            }
            packet::Packet::UpdateSign_u16(sign) => {
                mapped_packet::MappedPacket::UpdateSign(UpdateSign {
                    location: Position::new(sign.x, sign.y as i32, sign.z),
                    line1: sign.line1,
                    line2: sign.line2,
                    line3: sign.line3,
                    line4: sign.line4,
                })
            }
            packet::Packet::UnlockRecipes_NoSmelting(recipes) => {
                mapped_packet::MappedPacket::UnlockRecipes(UnlockRecipes {
                    action: recipes.action.0,
                    crafting_book_open: recipes.crafting_book_open,
                    filtering_craftable: recipes.filtering_craftable,
                    smelting_book_open: None,
                    filtering_smeltable: None,
                    blast_furnace_open: None,
                    filtering_blast_furnace: None,
                    smoker_open: None,
                    filtering_smoker: None,
                    recipe_ids: Some(recipes.recipe_ids.data.iter().map(|x| x.0).collect()),
                    recipe_ids2: Some(recipes.recipe_ids2.data.iter().map(|x| x.0).collect()),
                    recipe_ids_str: None,
                    recipe_ids_str2: None,
                })
            }
            packet::Packet::UnlockRecipes_WithSmelting(recipes) => {
                mapped_packet::MappedPacket::UnlockRecipes(UnlockRecipes {
                    action: recipes.action.0,
                    crafting_book_open: recipes.crafting_book_open,
                    filtering_craftable: recipes.filtering_craftable,
                    smelting_book_open: Some(recipes.smelting_book_open),
                    filtering_smeltable: Some(recipes.filtering_smeltable),
                    blast_furnace_open: None,
                    filtering_blast_furnace: None,
                    smoker_open: None,
                    filtering_smoker: None,
                    recipe_ids: None,
                    recipe_ids2: None,
                    recipe_ids_str: Some(recipes.recipe_ids.data),
                    recipe_ids_str2: Some(recipes.recipe_ids2.data),
                })
            }
            packet::Packet::UnlockRecipes_WithBlastSmoker(recipes) => {
                mapped_packet::MappedPacket::UnlockRecipes(UnlockRecipes {
                    action: recipes.action.0,
                    crafting_book_open: recipes.crafting_book_open,
                    filtering_craftable: recipes.filtering_craftable,
                    smelting_book_open: Some(recipes.smelting_book_open),
                    filtering_smeltable: Some(recipes.filtering_smeltable),
                    blast_furnace_open: Some(recipes.blast_furnace_open),
                    filtering_blast_furnace: Some(recipes.filtering_blast_furnace),
                    smoker_open: Some(recipes.smoker_open),
                    filtering_smoker: Some(recipes.filtering_smoker),
                    recipe_ids: None,
                    recipe_ids2: None,
                    recipe_ids_str: Some(recipes.recipe_ids.data),
                    recipe_ids_str2: Some(recipes.recipe_ids2.data),
                })
            }
            packet::Packet::UpdateCommandBlock(command) => {
                mapped_packet::MappedPacket::UpdateCommandBlock(UpdateCommandBlock {
                    location: command.location,
                    command: command.command,
                    mode: command.mode.0,
                    flags: command.flags,
                })
            }
            packet::Packet::UpdateCommandBlockMinecart(command_minecart) => {
                mapped_packet::MappedPacket::UpdateCommandBlockMinecart(
                    UpdateCommandBlockMinecart {
                        entity_id: command_minecart.entity_id.0,
                        command: command_minecart.command,
                        track_output: command_minecart.track_output,
                    },
                )
            }
            packet::Packet::UpdateJigsawBlock_Joint(jigsaw) => {
                mapped_packet::MappedPacket::UpdateJigsawBlock_Joint(UpdateJigsawBlock_Joint {
                    location: jigsaw.location,
                    name: jigsaw.name,
                    target: jigsaw.target,
                    pool: jigsaw.pool,
                    final_state: jigsaw.final_state,
                    joint_type: jigsaw.joint_type,
                })
            }
            packet::Packet::UpdateJigsawBlock_Type(jigsaw) => {
                mapped_packet::MappedPacket::UpdateJigsawBlock_Type(UpdateJigsawBlock_Type {
                    location: jigsaw.location,
                    attachment_type: jigsaw.attachment_type,
                    target_pool: jigsaw.target_pool,
                    final_state: jigsaw.final_state,
                })
            }
            packet::Packet::UpdateScore(score) => {
                mapped_packet::MappedPacket::UpdateScore(UpdateScore {
                    name: score.name,
                    action: score.action,
                    object_name: score.object_name,
                    value: score.value.map(|x| x.0),
                })
            }
            packet::Packet::UpdateScore_i32(score) => {
                mapped_packet::MappedPacket::UpdateScore(UpdateScore {
                    name: score.name,
                    action: score.action,
                    object_name: score.object_name,
                    value: score.value,
                })
            }
            packet::Packet::UpdateStructureBlock(structure_block) => {
                mapped_packet::MappedPacket::UpdateStructureBlock(UpdateStructureBlock {
                    location: structure_block.location,
                    action: structure_block.action.0,
                    mode: structure_block.mode.0,
                    name: structure_block.name,
                    offset_x: structure_block.offset_x,
                    offset_y: structure_block.offset_y,
                    offset_z: structure_block.offset_z,
                    size_x: structure_block.size_x,
                    size_y: structure_block.size_y,
                    size_z: structure_block.size_z,
                    mirror: structure_block.mirror.0,
                    rotation: structure_block.rotation.0,
                    metadata: structure_block.metadata,
                    integrity: structure_block.integrity,
                    seed: structure_block.seed.0,
                    flags: structure_block.flags,
                })
            }
            packet::Packet::UpdateViewDistance(view_distance) => {
                mapped_packet::MappedPacket::UpdateViewDistance(UpdateViewDistance {
                    view_distance: view_distance.view_distance.0,
                })
            }
            packet::Packet::UseEntity_Hand(use_entity) => {
                mapped_packet::MappedPacket::UseEntity(UseEntity {
                    target_id: use_entity.target_id.0,
                    ty: use_entity.ty.0,
                    target_x: Some(use_entity.target_x),
                    target_y: Some(use_entity.target_y),
                    target_z: Some(use_entity.target_z),
                    hand: Some(use_entity.hand.0),
                    sneaking: None,
                })
            }
            packet::Packet::UseEntity_Handsfree(use_entity) => {
                mapped_packet::MappedPacket::UseEntity(UseEntity {
                    target_id: use_entity.target_id.0,
                    ty: use_entity.ty.0,
                    target_x: Some(use_entity.target_x),
                    target_y: Some(use_entity.target_y),
                    target_z: Some(use_entity.target_z),
                    hand: None,
                    sneaking: None,
                })
            }
            packet::Packet::UseEntity_Handsfree_i32(use_entity) => {
                mapped_packet::MappedPacket::UseEntity(UseEntity {
                    target_id: use_entity.target_id,
                    ty: use_entity.ty as i32,
                    target_x: None,
                    target_y: None,
                    target_z: None,
                    hand: None,
                    sneaking: None,
                })
            }
            packet::Packet::UseEntity_Sneakflag(use_entity) => {
                mapped_packet::MappedPacket::UseEntity(UseEntity {
                    target_id: use_entity.target_id.0,
                    ty: use_entity.ty.0,
                    target_x: Some(use_entity.target_x),
                    target_y: Some(use_entity.target_y),
                    target_z: Some(use_entity.target_z),
                    hand: Some(use_entity.hand.0),
                    sneaking: Some(use_entity.sneaking),
                })
            }
            packet::Packet::UseItem(use_item) => mapped_packet::MappedPacket::UseItem(UseItem {
                hand: use_item.hand.0,
            }),
            packet::Packet::VehicleMove(vehicle_move) => {
                mapped_packet::MappedPacket::VehicleMove(VehicleMove {
                    x: vehicle_move.x,
                    y: vehicle_move.y,
                    z: vehicle_move.z,
                    yaw: vehicle_move.yaw,
                    pitch: vehicle_move.pitch,
                })
            }
            packet::Packet::VehicleTeleport(teleport) => {
                mapped_packet::MappedPacket::VehicleTeleport(VehicleTeleport {
                    x: teleport.x,
                    y: teleport.y,
                    z: teleport.z,
                    yaw: teleport.yaw,
                    pitch: teleport.pitch,
                })
            }
            packet::Packet::WindowItems(items) => {
                mapped_packet::MappedPacket::WindowItems(WindowItems {
                    id: items.id,
                    items: items.items.data,
                })
            }
            packet::Packet::WindowClose(close) => {
                mapped_packet::MappedPacket::WindowClose(WindowClose { id: close.id })
            }
            packet::Packet::WindowOpen(open) => {
                mapped_packet::MappedPacket::WindowOpen(WindowOpen {
                    id: open.id as i32,
                    ty: None,
                    ty_name: Some(open.ty),
                    title: open.title,
                    slot_count: Some(open.slot_count),
                    use_provided_window_title: None,
                    entity_id: Some(open.entity_id),
                })
            }
            packet::Packet::WindowOpen_u8(open) => {
                mapped_packet::MappedPacket::WindowOpen(WindowOpen {
                    id: open.id as i32,
                    ty: Some(open.ty as i32),
                    ty_name: None,
                    title: open.title,
                    slot_count: Some(open.slot_count),
                    use_provided_window_title: Some(open.use_provided_window_title),
                    entity_id: Some(open.entity_id),
                })
            }
            packet::Packet::WindowOpen_VarInt(open) => {
                mapped_packet::MappedPacket::WindowOpen(WindowOpen {
                    id: open.id.0,
                    ty: Some(open.ty.0),
                    ty_name: None,
                    title: open.title,
                    slot_count: None,
                    use_provided_window_title: None,
                    entity_id: None,
                })
            }
            packet::Packet::WindowOpenHorse(open) => {
                mapped_packet::MappedPacket::WindowOpenHorse(WindowOpenHorse {
                    window_id: open.window_id,
                    number_of_slots: open.number_of_slots.0,
                    entity_id: open.entity_id,
                })
            }
            packet::Packet::WindowProperty(property) => {
                mapped_packet::MappedPacket::WindowProperty(WindowProperty {
                    id: property.id,
                    property: property.property,
                    value: property.value,
                })
            }
            packet::Packet::WindowSetSlot(set_slot) => {
                mapped_packet::MappedPacket::WindowSetSlot(WindowSetSlot {
                    id: set_slot.id,
                    slot: set_slot.slot,
                    item: set_slot.item,
                })
            }
            packet::Packet::WorldBorder(border) => {
                mapped_packet::MappedPacket::WorldBorder(WorldBorder {
                    action: border.action.0,
                    old_radius: border.old_radius,
                    new_radius: border.new_radius,
                    speed: border.speed.map(|x| x.0),
                    x: border.x,
                    z: border.z,
                    portal_boundary: border.portal_boundary.map(|x| x.0),
                    warning_time: border.warning_time.map(|x| x.0),
                    warning_blocks: border.warning_blocks.map(|x| x.0),
                })
            }
        }
    }
}


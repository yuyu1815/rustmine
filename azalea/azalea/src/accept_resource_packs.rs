use azalea_client::{
    InConfigState,
    chunks::handle_chunk_batch_finished_event,
    client_information::send_client_information,
    inventory::InventorySystems,
    packet::{
        config::SendConfigPacketEvent,
        death_event_on_0_health,
        game::{ResourcePackEvent, SendGamePacketEvent},
    },
    resources::{
        ServerResourcePackAck, ServerResourcePackAckAction, ServerResourcePackApplyState,
        ServerResourcePackRequest,
    },
    respawn::perform_respawn,
};
use azalea_protocol::packets::{
    config,
    game::s_resource_pack::{self, ServerboundResourcePack},
};
use bevy_app::Update;
use bevy_ecs::prelude::*;

use crate::app::{App, Plugin};

/// A plugin that makes it so bots automatically accept resource packs.
#[derive(Clone, Default)]
pub struct AcceptResourcePacksPlugin;
impl Plugin for AcceptResourcePacksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            accept_resource_pack
                .before(perform_respawn)
                .after(death_event_on_0_health)
                .after(handle_chunk_batch_finished_event)
                .after(InventorySystems)
                .after(send_client_information),
        );
    }
}

fn accept_resource_pack(
    mut events: MessageReader<ResourcePackEvent>,
    mut commands: Commands,
    query_in_config_state: Query<Option<&InConfigState>>,
) {
    for event in events.read() {
        let Ok(in_config_state_option) = query_in_config_state.get(event.entity) else {
            continue;
        };

        let mut pack = ServerResourcePackApplyState::new(ServerResourcePackRequest::new(
            event.id,
            event.url.to_owned(),
            event.hash.to_owned(),
            event.required,
            event.prompt.clone(),
        ));
        let acks = [pack.accept(), {
            pack.start_download();
            pack.download_succeeded();
            pack.apply_downloaded()
        }];

        for ack in acks {
            send_resource_pack_ack(
                &mut commands,
                event.entity,
                in_config_state_option.is_some(),
                ack,
            );
        }
    }
}

fn send_resource_pack_ack(
    commands: &mut Commands,
    entity: Entity,
    in_config_state: bool,
    ack: ServerResourcePackAck,
) {
    if in_config_state {
        commands.trigger(SendConfigPacketEvent::new(
            entity,
            config::ServerboundResourcePack {
                id: ack.id,
                action: config_resource_pack_ack_action(ack.action),
            },
        ));
    } else {
        commands.trigger(SendGamePacketEvent::new(
            entity,
            ServerboundResourcePack {
                id: ack.id,
                action: game_resource_pack_ack_action(ack.action),
            },
        ));
    }
}

fn config_resource_pack_ack_action(
    action: ServerResourcePackAckAction,
) -> config::s_resource_pack::Action {
    match action {
        ServerResourcePackAckAction::SuccessfullyLoaded => {
            config::s_resource_pack::Action::SuccessfullyLoaded
        }
        ServerResourcePackAckAction::Declined => config::s_resource_pack::Action::Declined,
        ServerResourcePackAckAction::FailedDownload => {
            config::s_resource_pack::Action::FailedDownload
        }
        ServerResourcePackAckAction::Accepted => config::s_resource_pack::Action::Accepted,
        ServerResourcePackAckAction::InvalidUrl => config::s_resource_pack::Action::InvalidUrl,
        ServerResourcePackAckAction::FailedReload => config::s_resource_pack::Action::FailedReload,
        ServerResourcePackAckAction::Discarded => config::s_resource_pack::Action::Discarded,
    }
}

fn game_resource_pack_ack_action(action: ServerResourcePackAckAction) -> s_resource_pack::Action {
    match action {
        ServerResourcePackAckAction::SuccessfullyLoaded => {
            s_resource_pack::Action::SuccessfullyLoaded
        }
        ServerResourcePackAckAction::Declined => s_resource_pack::Action::Declined,
        ServerResourcePackAckAction::FailedDownload => s_resource_pack::Action::FailedDownload,
        ServerResourcePackAckAction::Accepted => s_resource_pack::Action::Accepted,
        ServerResourcePackAckAction::InvalidUrl => s_resource_pack::Action::InvalidUrl,
        ServerResourcePackAckAction::FailedReload => s_resource_pack::Action::FailedReload,
        ServerResourcePackAckAction::Discarded => s_resource_pack::Action::Discarded,
    }
}

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

        let ack = initial_resource_pack_ack(&mut pack);
        send_resource_pack_ack(
            &mut commands,
            event.entity,
            in_config_state_option.is_some(),
            ack,
        );
    }
}

fn initial_resource_pack_ack(pack: &mut ServerResourcePackApplyState) -> ServerResourcePackAck {
    if pack.validate_url().is_err() {
        return pack.invalid_url();
    }

    pack.accept()
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
        ServerResourcePackAckAction::Downloaded => config::s_resource_pack::Action::Downloaded,
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
        ServerResourcePackAckAction::Downloaded => s_resource_pack::Action::Downloaded,
        ServerResourcePackAckAction::InvalidUrl => s_resource_pack::Action::InvalidUrl,
        ServerResourcePackAckAction::FailedReload => s_resource_pack::Action::FailedReload,
        ServerResourcePackAckAction::Discarded => s_resource_pack::Action::Discarded,
    }
}

#[cfg(test)]
mod tests {
    use azalea_client::resources::ServerResourcePackStatus;
    use uuid::Uuid;

    use super::*;

    #[test]
    fn valid_server_pack_push_only_sends_initial_accepted_ack() {
        let id = Uuid::from_u128(1);
        let mut pack = ServerResourcePackApplyState::new(ServerResourcePackRequest::new(
            id,
            "https://example.com/server-pack.zip",
            "0123456789abcdef0123456789abcdef01234567",
            true,
            None,
        ));

        let ack = initial_resource_pack_ack(&mut pack);

        assert_eq!(ack.id, id);
        assert_eq!(ack.action, ServerResourcePackAckAction::Accepted);
        assert_eq!(pack.status(), ServerResourcePackStatus::Accepted);
    }

    #[test]
    fn invalid_server_pack_url_still_sends_invalid_url_ack() {
        let id = Uuid::from_u128(2);
        let mut pack = ServerResourcePackApplyState::new(ServerResourcePackRequest::new(
            id,
            "ftp://example.com/server-pack.zip",
            "0123456789abcdef0123456789abcdef01234567",
            false,
            None,
        ));

        let ack = initial_resource_pack_ack(&mut pack);

        assert_eq!(ack.id, id);
        assert_eq!(ack.action, ServerResourcePackAckAction::InvalidUrl);
        assert!(matches!(pack.status(), ServerResourcePackStatus::Failed(_)));
    }
}

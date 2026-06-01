#[cfg(feature = "online-mode")]
use std::path::PathBuf;

#[cfg(any(feature = "online-mode", test))]
use azalea_client::resources::{ClientResourcePack, ClientResourceStack, ResourceReloadManager};
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
        ServerResourcePackAck, ServerResourcePackAckAction, ServerResourcePackApplyModel,
        ServerResourcePackApplyState, ServerResourcePackRequest,
    },
    respawn::perform_respawn,
};
use azalea_protocol::packets::{
    config,
    game::s_resource_pack::{self, ServerboundResourcePack},
};
use bevy_app::Update;
use bevy_ecs::prelude::*;
#[cfg(feature = "online-mode")]
use bevy_tasks::{IoTaskPool, Task, futures_lite::future};

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

        #[cfg(feature = "online-mode")]
        app.add_systems(Update, poll_accept_resource_pack_tasks);
    }
}

#[cfg(feature = "online-mode")]
#[derive(Component)]
struct AcceptResourcePackTask(Task<AcceptResourcePackTaskOutput>);

#[cfg(feature = "online-mode")]
struct AcceptResourcePackTaskOutput {
    pack: ServerResourcePackApplyState,
    acks: Vec<ServerResourcePackAck>,
}

fn accept_resource_pack(
    mut events: MessageReader<ResourcePackEvent>,
    mut commands: Commands,
    query: Query<(
        Option<&InConfigState>,
        Option<&ServerResourcePackApplyModel>,
    )>,
) {
    for event in events.read() {
        let Ok((in_config_state_option, _server_resource_packs)) = query.get(event.entity) else {
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

        #[cfg(feature = "online-mode")]
        if ack.action == ServerResourcePackAckAction::Accepted {
            let base_stack = _server_resource_packs
                .map(ServerResourcePackApplyModel::resource_stack)
                .unwrap_or_else(ClientResourceStack::vanilla);
            let task =
                IoTaskPool::get().spawn(async_compat::Compat::new(apply_accepted_resource_pack(
                    pack,
                    base_stack,
                    default_server_resource_pack_cache_dir(),
                )));
            commands
                .entity(event.entity)
                .insert(AcceptResourcePackTask(task));
        }
    }
}

fn initial_resource_pack_ack(pack: &mut ServerResourcePackApplyState) -> ServerResourcePackAck {
    if pack.validate_url().is_err() {
        return pack.invalid_url();
    }

    pack.accept()
}

#[cfg(any(feature = "online-mode", test))]
fn resource_pack_apply_ack_sequence(
    pack: &mut ServerResourcePackApplyState,
    base_stack: ClientResourceStack,
    download_result: Result<ServerResourcePackAck, ServerResourcePackAck>,
) -> Vec<ServerResourcePackAck> {
    let downloaded = match download_result {
        Ok(ack) => ack,
        Err(ack) => return vec![ack],
    };

    let mut acks = vec![downloaded];
    match pack.open_downloaded() {
        Ok(()) if reload_accepted_resource_pack(pack, base_stack).is_ok() => {
            acks.push(pack.apply_opened())
        }
        Ok(()) => acks.push(pack.reload_failed()),
        Err(ack) => acks.push(ack),
    }
    acks
}

#[cfg(any(feature = "online-mode", test))]
fn reload_accepted_resource_pack(
    pack: &ServerResourcePackApplyState,
    base_stack: ClientResourceStack,
) -> azalea_client::resources::ResourceReloadResult<azalea_client::resources::ResourceReloadReport>
{
    let stack = accepted_resource_pack_reload_stack(base_stack, pack.resource_pack());
    ResourceReloadManager::with_default_client_resources(stack).run()
}

#[cfg(any(feature = "online-mode", test))]
fn accepted_resource_pack_reload_stack(
    base_stack: ClientResourceStack,
    pack: ClientResourcePack,
) -> ClientResourceStack {
    let mut packs = base_stack.packs().to_vec();
    packs.push(pack);
    ClientResourceStack::new(packs)
}

#[cfg(feature = "online-mode")]
async fn apply_accepted_resource_pack(
    mut pack: ServerResourcePackApplyState,
    base_stack: ClientResourceStack,
    cache_dir: PathBuf,
) -> AcceptResourcePackTaskOutput {
    let client = reqwest::Client::new();
    let download_result = pack
        .download_and_cache(&client, cache_dir)
        .await
        .map(|report| report.ack)
        .map_err(|err| err.ack());

    let acks = resource_pack_apply_ack_sequence(&mut pack, base_stack, download_result);
    AcceptResourcePackTaskOutput { pack, acks }
}

#[cfg(feature = "online-mode")]
fn poll_accept_resource_pack_tasks(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut AcceptResourcePackTask,
        Option<&InConfigState>,
        Option<&mut ServerResourcePackApplyModel>,
    )>,
) {
    for (entity, mut task, in_config_state, server_resource_packs) in query.iter_mut() {
        if let Some(output) = future::block_on(future::poll_once(&mut task.0)) {
            commands.entity(entity).remove::<AcceptResourcePackTask>();
            record_finished_resource_pack(
                &mut commands,
                entity,
                server_resource_packs,
                output.pack,
            );
            for ack in output.acks {
                send_resource_pack_ack(&mut commands, entity, in_config_state.is_some(), ack);
            }
        }
    }
}

#[cfg(feature = "online-mode")]
fn record_finished_resource_pack(
    commands: &mut Commands,
    entity: Entity,
    server_resource_packs: Option<Mut<ServerResourcePackApplyModel>>,
    pack: ServerResourcePackApplyState,
) {
    if let Some(mut server_resource_packs) = server_resource_packs {
        server_resource_packs.record(pack);
    } else {
        let mut server_resource_packs = ServerResourcePackApplyModel::with_vanilla();
        server_resource_packs.record(pack);
        commands.entity(entity).insert(server_resource_packs);
    }
}

#[cfg(feature = "online-mode")]
fn default_server_resource_pack_cache_dir() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".minecraft")
        .join("server-resource-packs")
        .join("azalea")
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
    use std::{
        fs,
        path::Path,
        sync::atomic::{AtomicUsize, Ordering},
        time::{SystemTime, UNIX_EPOCH},
    };

    use azalea_client::resources::ServerResourcePackStatus;
    use uuid::Uuid;

    use super::*;

    #[test]
    fn valid_server_pack_push_sends_initial_accepted_ack() {
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
    fn successful_apply_sequence_sends_downloaded_then_successfully_loaded() {
        let id = Uuid::from_u128(2);
        let temp = TempPack::new();
        temp.write(
            "pack.mcmeta",
            r#"{"pack":{"min_format":84,"max_format":84,"description":"test"}}"#,
        );
        let mut pack = test_pack(id);

        pack.accept();
        pack.start_download();
        let downloaded = pack
            .download_path_succeeded(temp.path())
            .expect("directory pack with valid metadata should download");
        let acks = resource_pack_apply_ack_sequence(
            &mut pack,
            ClientResourceStack::vanilla(),
            Ok(downloaded),
        );

        assert_eq!(
            ack_actions(&acks),
            [
                ServerResourcePackAckAction::Downloaded,
                ServerResourcePackAckAction::SuccessfullyLoaded,
            ]
        );
        assert!(acks.iter().all(|ack| ack.id == id));
        assert_eq!(pack.status(), ServerResourcePackStatus::Applied);
    }

    #[test]
    fn open_failure_sequence_sends_downloaded_then_failed_reload() {
        let id = Uuid::from_u128(3);
        let temp = TempPack::new();
        let mut pack = test_pack(id);

        pack.accept();
        pack.start_download();
        let downloaded = pack
            .download_path_succeeded(temp.path())
            .expect("directory pack without a hash should download");
        let acks = resource_pack_apply_ack_sequence(
            &mut pack,
            ClientResourceStack::vanilla(),
            Ok(downloaded),
        );

        assert_eq!(
            ack_actions(&acks),
            [
                ServerResourcePackAckAction::Downloaded,
                ServerResourcePackAckAction::FailedReload,
            ]
        );
        assert!(acks.iter().all(|ack| ack.id == id));
        assert!(matches!(pack.status(), ServerResourcePackStatus::Failed(_)));
    }

    #[test]
    fn download_failure_sequence_sends_failed_download() {
        let id = Uuid::from_u128(4);
        let mut pack = test_pack(id);

        pack.accept();
        let failed_download = pack.download_failed();
        let acks = resource_pack_apply_ack_sequence(
            &mut pack,
            ClientResourceStack::vanilla(),
            Err(failed_download),
        );

        assert_eq!(
            ack_actions(&acks),
            [ServerResourcePackAckAction::FailedDownload]
        );
        assert_eq!(acks[0].id, id);
        assert!(matches!(pack.status(), ServerResourcePackStatus::Failed(_)));
    }

    #[test]
    fn reload_failure_sequence_never_sends_successfully_loaded() {
        let id = Uuid::from_u128(6);
        let temp = TempPack::new();
        temp.write(
            "pack.mcmeta",
            r#"{"pack":{"min_format":84,"max_format":84,"description":"test"}}"#,
        );
        let mut pack = test_pack(id);

        pack.accept();
        pack.start_download();
        let downloaded = pack
            .download_path_succeeded(temp.path())
            .expect("directory pack with valid metadata should download");
        let acks = resource_pack_apply_ack_sequence(
            &mut pack,
            ClientResourceStack::new(Vec::new()),
            Ok(downloaded),
        );

        assert_eq!(
            ack_actions(&acks),
            [
                ServerResourcePackAckAction::Downloaded,
                ServerResourcePackAckAction::FailedReload,
            ]
        );
        assert!(!ack_actions(&acks).contains(&ServerResourcePackAckAction::SuccessfullyLoaded));
        assert_eq!(
            pack.status(),
            ServerResourcePackStatus::Failed(
                azalea_client::resources::ServerResourcePackFailure::Reload
            )
        );
    }

    #[test]
    fn task_output_pack_can_be_recorded_above_existing_model_stack() {
        let id = Uuid::from_u128(7);
        let base = TempPack::new();
        let server = TempPack::new();
        base.write(
            "assets/minecraft/lang/en_us.json",
            r#"{"menu.play":"Base"}"#,
        );
        server.write(
            "pack.mcmeta",
            r#"{"pack":{"min_format":84,"max_format":84,"description":"server"}}"#,
        );
        server.write(
            "assets/minecraft/lang/en_us.json",
            r#"{"menu.play":"Server"}"#,
        );
        let mut model = ServerResourcePackApplyModel::new(ClientResourceStack::new(vec![
            ClientResourcePack::vanilla(),
            ClientResourcePack::new("base", base.path()),
        ]));
        let mut pack = test_pack(id);

        pack.accept();
        pack.start_download();
        let downloaded = pack
            .download_path_succeeded(server.path())
            .expect("directory pack with valid metadata should download");
        let acks =
            resource_pack_apply_ack_sequence(&mut pack, model.resource_stack(), Ok(downloaded));
        model.record(pack);

        assert_eq!(
            ack_actions(&acks),
            [
                ServerResourcePackAckAction::Downloaded,
                ServerResourcePackAckAction::SuccessfullyLoaded,
            ]
        );
        assert_eq!(
            model
                .resource_stack()
                .find_resource("assets/minecraft/lang/en_us.json")
                .expect("server resource should resolve")
                .pack_id,
            format!("server:{id}")
        );
    }

    #[test]
    fn ack_action_mapping_supports_config_and_game_packet_paths() {
        assert_eq!(
            config_resource_pack_ack_action(ServerResourcePackAckAction::SuccessfullyLoaded),
            config::s_resource_pack::Action::SuccessfullyLoaded
        );
        assert_eq!(
            game_resource_pack_ack_action(ServerResourcePackAckAction::SuccessfullyLoaded),
            s_resource_pack::Action::SuccessfullyLoaded
        );
        assert_eq!(
            config_resource_pack_ack_action(ServerResourcePackAckAction::FailedReload),
            config::s_resource_pack::Action::FailedReload
        );
        assert_eq!(
            game_resource_pack_ack_action(ServerResourcePackAckAction::FailedReload),
            s_resource_pack::Action::FailedReload
        );
    }

    #[test]
    fn invalid_server_pack_url_still_sends_invalid_url_ack() {
        let id = Uuid::from_u128(5);
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

    fn test_pack(id: Uuid) -> ServerResourcePackApplyState {
        ServerResourcePackApplyState::new(ServerResourcePackRequest::new(
            id,
            "https://example.com/server-pack.zip",
            "",
            true,
            None,
        ))
    }

    fn ack_actions(acks: &[ServerResourcePackAck]) -> Vec<ServerResourcePackAckAction> {
        acks.iter().map(|ack| ack.action).collect()
    }

    static TEMP_PACK_COUNTER: AtomicUsize = AtomicUsize::new(0);

    struct TempPack {
        root: std::path::PathBuf,
    }

    impl TempPack {
        fn new() -> Self {
            let nanos = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system clock should be after unix epoch")
                .as_nanos();
            let root = std::env::temp_dir().join(format!(
                "azalea-accept-resource-packs-test-{}-{nanos}-{}",
                std::process::id(),
                TEMP_PACK_COUNTER.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir_all(&root).expect("temp resource pack directory should be created");
            Self { root }
        }

        fn path(&self) -> &Path {
            &self.root
        }

        fn write(&self, resource: &str, contents: &str) {
            let path = self.root.join(resource);
            fs::create_dir_all(path.parent().expect("resource should have a parent"))
                .expect("resource parent directory should be created");
            fs::write(path, contents).expect("resource should be written");
        }
    }

    impl Drop for TempPack {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
}

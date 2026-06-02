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
        game::{ResourcePackEvent, ResourcePackPopEvent, SendGamePacketEvent},
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
        app.add_systems(Update, discard_resource_pack);

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
    mut query: Query<(
        Option<&InConfigState>,
        Option<&mut ServerResourcePackApplyModel>,
    )>,
) {
    for event in events.read() {
        let Ok((in_config_state_option, server_resource_packs)) = query.get_mut(event.entity)
        else {
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
        let base_stack = server_resource_packs
            .as_deref()
            .map(ServerResourcePackApplyModel::resource_stack)
            .unwrap_or_else(ClientResourceStack::vanilla);

        record_initial_resource_pack(
            &mut commands,
            event.entity,
            server_resource_packs,
            pack.clone(),
        );

        #[cfg(feature = "online-mode")]
        if ack.action == ServerResourcePackAckAction::Accepted {
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

fn record_initial_resource_pack(
    commands: &mut Commands,
    entity: Entity,
    server_resource_packs: Option<Mut<ServerResourcePackApplyModel>>,
    pack: ServerResourcePackApplyState,
) {
    if let Some(mut server_resource_packs) = server_resource_packs {
        record_initial_resource_pack_in_model(&mut server_resource_packs, pack);
    } else {
        commands
            .entity(entity)
            .insert(initial_resource_pack_model(pack));
    }
}

fn record_initial_resource_pack_in_model(
    model: &mut ServerResourcePackApplyModel,
    pack: ServerResourcePackApplyState,
) {
    model.record(pack);
}

fn initial_resource_pack_model(pack: ServerResourcePackApplyState) -> ServerResourcePackApplyModel {
    let mut model = ServerResourcePackApplyModel::with_vanilla();
    record_initial_resource_pack_in_model(&mut model, pack);
    model
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
            let ack = pack.apply_opened();
            push_resource_pack_apply_ack(&mut acks, pack, ack);
        }
        Ok(()) => acks.push(pack.reload_failed()),
        Err(ack) => acks.push(ack),
    }
    acks
}

#[cfg(any(feature = "online-mode", test))]
fn push_resource_pack_apply_ack(
    acks: &mut Vec<ServerResourcePackAck>,
    pack: &mut ServerResourcePackApplyState,
    ack: ServerResourcePackAck,
) {
    if ack.action == ServerResourcePackAckAction::SuccessfullyLoaded
        && !pack.apply_plan().can_send_successfully_loaded()
    {
        pack.successfully_loaded_ack_suppressed();
        return;
    }

    acks.push(ack);
}

#[cfg(any(feature = "online-mode", test))]
fn reload_accepted_resource_pack(
    pack: &mut ServerResourcePackApplyState,
    base_stack: ClientResourceStack,
) -> azalea_client::resources::ResourceReloadResult<azalea_client::resources::ResourceReloadReport>
{
    let stack = accepted_resource_pack_reload_stack(base_stack, pack.resource_pack());
    ResourceReloadManager::with_default_client_resources(stack).run_with_events(|event| {
        pack.record_resource_reload_event(event);
    })
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

fn discard_resource_pack(
    mut events: MessageReader<ResourcePackPopEvent>,
    mut commands: Commands,
    mut query: Query<(
        Option<&InConfigState>,
        Option<&mut ServerResourcePackApplyModel>,
    )>,
) {
    for event in events.read() {
        let Ok((in_config_state_option, server_resource_packs)) = query.get_mut(event.entity)
        else {
            continue;
        };
        let Some(mut server_resource_packs) = server_resource_packs else {
            continue;
        };

        for discarded_id in discard_resource_pack_from_model(&mut server_resource_packs, event.id) {
            send_resource_pack_ack(
                &mut commands,
                event.entity,
                in_config_state_option.is_some(),
                ServerResourcePackAck {
                    id: discarded_id,
                    action: ServerResourcePackAckAction::Discarded,
                },
            );
        }
    }
}

fn discard_resource_pack_from_model(
    model: &mut ServerResourcePackApplyModel,
    id: Option<uuid::Uuid>,
) -> Vec<uuid::Uuid> {
    match id {
        Some(id) if model.pop(id) => vec![id],
        Some(_) => Vec::new(),
        None => {
            let ids = model
                .packs()
                .iter()
                .map(|pack| pack.request().id())
                .collect::<Vec<_>>();
            if model.pop_all() { ids } else { Vec::new() }
        }
    }
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

    use azalea_client::resources::{ServerResourcePackReloadOutcome, ServerResourcePackStatus};
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
        assert_eq!(
            pack.reload_outcome(),
            Some(ServerResourcePackReloadOutcome::Succeeded {
                successfully_loaded_ack_sent: true,
            })
        );
    }

    #[test]
    fn opened_pack_does_not_send_successfully_loaded_until_applied() {
        let id = Uuid::from_u128(8);
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
        let mut acks = vec![downloaded];
        pack.open_downloaded()
            .expect("downloaded directory pack should open");

        assert_eq!(pack.status(), ServerResourcePackStatus::Opened);
        assert!(!pack.apply_plan().can_send_successfully_loaded());

        push_resource_pack_apply_ack(
            &mut acks,
            &mut pack,
            ServerResourcePackAck {
                id,
                action: ServerResourcePackAckAction::SuccessfullyLoaded,
            },
        );

        assert_eq!(
            ack_actions(&acks),
            [ServerResourcePackAckAction::Downloaded]
        );
        assert_eq!(
            pack.reload_outcome(),
            Some(ServerResourcePackReloadOutcome::Succeeded {
                successfully_loaded_ack_sent: false,
            })
        );

        let applied_ack = pack.apply_opened();
        assert_eq!(pack.status(), ServerResourcePackStatus::Applied);
        assert!(pack.apply_plan().can_send_successfully_loaded());
        push_resource_pack_apply_ack(&mut acks, &mut pack, applied_ack);

        assert_eq!(
            ack_actions(&acks),
            [
                ServerResourcePackAckAction::Downloaded,
                ServerResourcePackAckAction::SuccessfullyLoaded,
            ]
        );
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
        let report = pack.runtime_report();
        let report_item = report
            .items()
            .iter()
            .find(|item| item.starts_with("server_resource_pack_apply_runtime_report_pack:"))
            .expect("pack runtime report row should exist");

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
        assert_eq!(
            pack.reload_outcome(),
            Some(ServerResourcePackReloadOutcome::Failed)
        );
        assert!(!pack.reload_progress_snapshots().is_empty());
        assert!(pack.last_reload_progress_snapshot().is_some());
        assert!(report_item.contains(&format!(
            "reload_snapshot_count:{}",
            pack.reload_progress_snapshots().len()
        )));
        assert!(report_item.contains("reload_last_actual_progress:"));
        assert!(!report_item.contains("reload_last_actual_progress:none"));
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
        assert!(!pack.reload_progress_snapshots().is_empty());
        assert_eq!(
            pack.last_reload_progress_snapshot()
                .map(|snapshot| snapshot.actual_progress()),
            Some(1.0)
        );
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
    fn server_pack_pop_by_id_removes_only_matching_applied_pack_from_stack() {
        let removed_id = Uuid::from_u128(9);
        let kept_id = Uuid::from_u128(10);
        let removed = TempPack::new();
        let kept = TempPack::new();
        write_valid_pack(&removed);
        write_valid_pack(&kept);
        removed.write(
            "assets/minecraft/lang/removed.json",
            r#"{"pack":"removed"}"#,
        );
        kept.write("assets/minecraft/lang/kept.json", r#"{"pack":"kept"}"#);
        let mut model = ServerResourcePackApplyModel::with_vanilla();
        model.record(applied_test_pack(removed_id, removed.path()));
        model.record(applied_test_pack(kept_id, kept.path()));

        let discarded_ids = discard_resource_pack_from_model(&mut model, Some(removed_id));

        assert_eq!(discarded_ids, [removed_id]);
        assert!(
            model
                .resource_stack()
                .find_resource("assets/minecraft/lang/removed.json")
                .is_none()
        );
        assert_eq!(
            model
                .resource_stack()
                .find_resource("assets/minecraft/lang/kept.json")
                .expect("kept server resource should stay applied")
                .pack_id,
            format!("server:{kept_id}")
        );
    }

    #[test]
    fn server_pack_pop_all_keeps_vanilla_stack_only() {
        let first_id = Uuid::from_u128(11);
        let second_id = Uuid::from_u128(12);
        let first = TempPack::new();
        let second = TempPack::new();
        write_valid_pack(&first);
        write_valid_pack(&second);
        first.write("assets/minecraft/lang/first.json", r#"{"pack":"first"}"#);
        second.write("assets/minecraft/lang/second.json", r#"{"pack":"second"}"#);
        let mut model = ServerResourcePackApplyModel::with_vanilla();
        model.record(applied_test_pack(first_id, first.path()));
        model.record(applied_test_pack(second_id, second.path()));

        let discarded_ids = discard_resource_pack_from_model(&mut model, None);

        assert_eq!(discarded_ids, [first_id, second_id]);
        assert_eq!(model.packs(), []);
        assert_eq!(model.resource_stack(), ClientResourceStack::vanilla());
    }

    #[test]
    fn server_pack_pop_missing_id_is_no_op() {
        let kept_id = Uuid::from_u128(13);
        let missing_id = Uuid::from_u128(14);
        let kept = TempPack::new();
        write_valid_pack(&kept);
        kept.write("assets/minecraft/lang/kept.json", r#"{"pack":"kept"}"#);
        let mut model = ServerResourcePackApplyModel::with_vanilla();
        model.record(applied_test_pack(kept_id, kept.path()));

        let discarded_ids = discard_resource_pack_from_model(&mut model, Some(missing_id));

        assert!(discarded_ids.is_empty());
        assert_eq!(model.packs().len(), 1);
        assert_eq!(
            model
                .resource_stack()
                .find_resource("assets/minecraft/lang/kept.json")
                .expect("kept server resource should stay applied")
                .pack_id,
            format!("server:{kept_id}")
        );
    }

    #[test]
    fn discarded_ack_action_mapping_supports_config_and_game_packet_paths() {
        assert_eq!(
            config_resource_pack_ack_action(ServerResourcePackAckAction::Discarded),
            config::s_resource_pack::Action::Discarded
        );
        assert_eq!(
            game_resource_pack_ack_action(ServerResourcePackAckAction::Discarded),
            s_resource_pack::Action::Discarded
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

    #[test]
    fn initial_accepted_pack_is_recorded_into_existing_model_without_entering_stack() {
        let id = Uuid::from_u128(15);
        let mut model = ServerResourcePackApplyModel::with_vanilla();
        let mut pack = test_pack(id);
        initial_resource_pack_ack(&mut pack);

        record_initial_resource_pack_in_model(&mut model, pack);

        let state = model.state();
        let item = state
            .find(id)
            .expect("accepted server pack should be visible in apply model");
        assert_eq!(item.status(), ServerResourcePackStatus::Accepted);
        assert_eq!(
            item.ack_history(),
            [ServerResourcePackAck {
                id,
                action: ServerResourcePackAckAction::Accepted,
            }]
        );
        assert!(!item.enters_resource_stack());
        assert_eq!(state.server_pack_count(), 1);
        assert_eq!(state.resulting_stack_pack_ids(), ["vanilla"]);
    }

    #[test]
    fn server_resource_pack_apply_runtime_report_surfaces_initial_accept_flow() {
        let id = Uuid::from_u128(19);
        let mut model = ServerResourcePackApplyModel::with_vanilla();
        let mut pack = test_pack(id);
        initial_resource_pack_ack(&mut pack);

        record_initial_resource_pack_in_model(&mut model, pack);

        let report = model.runtime_report();
        let item = report
            .items()
            .iter()
            .find(|item| item.starts_with("server_resource_pack_apply_runtime_report_pack:"))
            .expect("initial accepted pack should be reported");

        assert!(report.summary_fragment().contains("status:pending"));
        assert!(item.contains(&format!("id:{id}")));
        assert!(item.contains("status:accepted"));
        assert!(item.contains("request_url:https://example.com/server-pack.zip"));
        assert!(item.contains("request_hash:none"));
        assert!(item.contains("required:true"));
        assert!(item.contains("selected_pack_id:none"));
        assert!(item.contains("ack_sequence:accepted"));
        assert!(item.contains("reload_outcome:none"));
        assert!(item.contains("stack_pack_count:1"));
        assert!(
            item.contains("download_boundary:server_resource_pack_download_client_cache_boundary")
        );
        assert!(
            item.contains("open_boundary:server_resource_pack_open_metadata_validation_boundary")
        );
        assert!(
            item.contains("reload_boundary:server_resource_pack_client_resources_reload_boundary")
        );
        assert!(item.contains("apply_boundary:server_resource_pack_apply_resource_stack_boundary"));
    }

    #[test]
    fn initial_pack_model_uses_vanilla_when_entity_has_no_existing_model() {
        let id = Uuid::from_u128(16);
        let mut pack = test_pack(id);
        initial_resource_pack_ack(&mut pack);

        let model = initial_resource_pack_model(pack);

        let state = model.state();
        assert_eq!(state.base_stack_pack_ids(), ["vanilla"]);
        assert_eq!(state.resulting_stack_pack_ids(), ["vanilla"]);
        assert_eq!(
            state
                .find(id)
                .expect("accepted server pack should be inserted")
                .status(),
            ServerResourcePackStatus::Accepted
        );
    }

    #[test]
    fn initial_invalid_url_state_is_retained_in_apply_model() {
        let id = Uuid::from_u128(17);
        let mut pack = ServerResourcePackApplyState::new(ServerResourcePackRequest::new(
            id,
            "ftp://example.com/server-pack.zip",
            "",
            false,
            None,
        ));
        initial_resource_pack_ack(&mut pack);

        let model = initial_resource_pack_model(pack);
        let state = model.state();
        let item = state
            .find(id)
            .expect("invalid server pack should be visible in apply model");

        assert!(matches!(item.status(), ServerResourcePackStatus::Failed(_)));
        assert_eq!(
            item.ack_history(),
            [ServerResourcePackAck {
                id,
                action: ServerResourcePackAckAction::InvalidUrl,
            }]
        );
        assert!(!item.enters_resource_stack());
    }

    #[test]
    fn later_finished_pack_replaces_initial_accepted_state_for_same_id() {
        let id = Uuid::from_u128(18);
        let server = TempPack::new();
        write_valid_pack(&server);
        let mut initial = test_pack(id);
        initial_resource_pack_ack(&mut initial);
        let mut model = initial_resource_pack_model(initial);

        model.record(applied_test_pack(id, server.path()));

        let state = model.state();
        let item = state
            .find(id)
            .expect("finished pack should replace initial server pack state");
        assert_eq!(state.server_pack_count(), 1);
        assert_eq!(item.status(), ServerResourcePackStatus::Applied);
        assert!(item.enters_resource_stack());
        assert_eq!(item.stack_pack_id(), Some(format!("server:{id}").as_str()));
        assert_eq!(
            model
                .resource_stack()
                .find_resource("pack.mcmeta")
                .expect("applied server pack should enter resource stack")
                .pack_id,
            format!("server:{id}")
        );
    }

    #[test]
    fn server_resource_pack_apply_runtime_report_surfaces_finished_apply_flow() {
        let id = Uuid::from_u128(20);
        let server = TempPack::new();
        write_valid_pack(&server);
        let mut initial = test_pack(id);
        initial_resource_pack_ack(&mut initial);
        let mut model = initial_resource_pack_model(initial);

        model.record(applied_test_pack(id, server.path()));

        let report = model.runtime_report();
        let item = report
            .items()
            .iter()
            .find(|item| item.starts_with("server_resource_pack_apply_runtime_report_pack:"))
            .expect("finished applied pack should be reported");

        assert!(report.summary_fragment().contains("status:applied"));
        assert!(report.summary_fragment().contains("stack_pack_count:2"));
        assert!(item.contains("status:applied"));
        assert!(item.contains(&format!("selected_pack_id:server:{id}")));
        assert!(item.contains(&format!(
            "selected_pack_path:{}",
            server.path().to_string_lossy()
        )));
        assert!(item.contains("selected_pack_source:directory"));
        assert!(item.contains("ack_sequence:accepted>downloaded>successfully_loaded"));
        assert!(item.contains("reload_outcome:succeeded_ack_sent"));
        assert!(item.contains("enters_resource_stack:true"));
        assert!(item.contains("failure_reason:none"));
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

    fn applied_test_pack(id: Uuid, path: &Path) -> ServerResourcePackApplyState {
        let mut pack = test_pack(id);
        pack.accept();
        pack.start_download();
        pack.download_path_succeeded(path)
            .expect("directory pack should download");
        pack.open_downloaded()
            .expect("valid directory pack should open");
        pack.apply_opened();
        pack
    }

    fn write_valid_pack(pack: &TempPack) {
        pack.write(
            "pack.mcmeta",
            r#"{"pack":{"min_format":84,"max_format":84,"description":"test"}}"#,
        );
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

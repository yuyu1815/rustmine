//! Resource loading adapter for startup UI flow state.
//!
//! A future real resource loader can report task updates here without knowing
//! how the startup screen lays out loading panels.

use super::{
    account_flow::StoredLauncherAccount,
    startup_flow::{
        ResourceLoadingEvent, ResourceLoadingUpdate, StartupFlow, StartupLoadingPhase,
        StartupScreen, WeightedReloadProgress, WeightedReloadStageProgress,
    },
};
use crate::resources::{
    ClientResourceRepository, ClientResourceStack,
    ResourceReloadEvent as ClientResourceReloadEvent, ResourceReloadManager,
    ResourceReloadProgressSnapshot, ResourceReloadReport, ResourceReloadResult,
};

#[derive(Clone, Debug)]
pub struct ResourceLoadingTracker {
    flow: StartupFlow,
    resource_reload_progress: Option<WeightedReloadProgress>,
}

impl ResourceLoadingTracker {
    pub fn new(accounts: Vec<StoredLauncherAccount>) -> Self {
        Self {
            flow: StartupFlow::new(accounts),
            resource_reload_progress: None,
        }
    }

    pub fn from_flow(flow: StartupFlow) -> Self {
        Self {
            flow,
            resource_reload_progress: None,
        }
    }

    pub fn flow(&self) -> &StartupFlow {
        &self.flow
    }

    pub fn into_flow(self) -> StartupFlow {
        self.flow
    }

    pub fn screen(&self) -> StartupScreen<'_> {
        self.flow.screen()
    }

    pub fn weighted_progress(&self) -> WeightedReloadProgress {
        if self.flow.loading_phase() == StartupLoadingPhase::Complete {
            return WeightedReloadProgress::complete_simple_reload_instance();
        }

        if let Some(progress) = &self.resource_reload_progress {
            return progress.clone();
        }

        WeightedReloadProgress::from_loading_tasks(self.flow.loading_overlay().unwrap_or_default())
    }

    pub fn apply_update(&mut self, update: ResourceLoadingUpdate) {
        match update {
            ResourceLoadingUpdate::TaskProgress(task) => {
                self.flow.upsert_loading_task(task.into_loading_task());
            }
            ResourceLoadingUpdate::TaskFinished { name, file } => {
                self.flow.finish_loading_task(name, file);
            }
            ResourceLoadingUpdate::Complete => {
                self.mark_complete();
            }
        }
    }

    pub fn apply_event(&mut self, event: ResourceLoadingEvent) {
        self.flow.apply_resource_loading_event(event);
    }

    pub fn apply_resource_reload_event(&mut self, event: &ClientResourceReloadEvent) {
        self.apply_resource_reload_snapshot(&event.progress_snapshot);
    }

    pub fn apply_resource_reload_snapshot(&mut self, snapshot: &ResourceReloadProgressSnapshot) {
        self.flow.begin_loading();
        self.resource_reload_progress = Some(weighted_progress_from_resource_reload(snapshot));
    }

    pub fn run_resource_reload(
        &mut self,
        manager: &ResourceReloadManager,
    ) -> ResourceReloadResult<ResourceReloadReport> {
        let report = manager.run_with_events(|event| self.apply_resource_reload_event(event));

        if report.is_ok() {
            self.mark_complete();
        }

        report
    }

    pub fn run_client_resource_reload(
        &mut self,
        stack: ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadReport> {
        self.run_initial_client_resource_reload(stack)
    }

    pub fn run_initial_client_resource_reload(
        &mut self,
        stack: ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadReport> {
        let manager = ResourceReloadManager::with_default_client_resources(stack);
        self.run_resource_reload(&manager)
    }

    pub fn run_initial_client_resource_reload_from_repository(
        &mut self,
        repository: &ClientResourceRepository,
    ) -> ResourceReloadResult<ResourceReloadReport> {
        self.run_initial_client_resource_reload(repository.stack())
    }

    pub fn run_vanilla_client_resource_reload(
        &mut self,
    ) -> ResourceReloadResult<ResourceReloadReport> {
        self.run_initial_client_resource_reload(ClientResourceStack::vanilla())
    }

    pub fn advance_presentation(&mut self) {
        self.flow.advance_loading_presentation();
    }

    pub fn mark_complete(&mut self) {
        self.flow.finish_loading();
    }
}

fn weighted_progress_from_resource_reload(
    snapshot: &ResourceReloadProgressSnapshot,
) -> WeightedReloadProgress {
    let prepare_weight = snapshot.started_prepare_tasks() as f32 * 2.0;
    let reload_weight = snapshot.started_reload_tasks() as f32 * 2.0;
    let listener_weight = snapshot.listener_count() as f32;

    if prepare_weight + reload_weight + listener_weight == 0.0 {
        return WeightedReloadProgress::complete_simple_reload_instance();
    }

    WeightedReloadProgress::new([
        WeightedReloadStageProgress::new(
            "prepare",
            progress_ratio(
                snapshot.finished_prepare_tasks(),
                snapshot.started_prepare_tasks(),
            ),
            prepare_weight,
        ),
        WeightedReloadStageProgress::new(
            "reload",
            progress_ratio(
                snapshot.finished_reload_tasks(),
                snapshot.started_reload_tasks(),
            ),
            reload_weight,
        ),
        WeightedReloadStageProgress::new(
            "listener",
            progress_ratio(snapshot.prepared_listeners(), snapshot.listener_count()),
            listener_weight,
        ),
    ])
}

fn progress_ratio(finished: u32, started: u32) -> f32 {
    if started == 0 {
        1.0
    } else {
        finished as f32 / started as f32
    }
}

#[cfg(test)]
mod tests {
    use std::fmt;

    use super::*;
    use crate::{
        resources::{
            ClientResourceRepository, ClientResourceStack, ResourceReloadError,
            ResourceReloadListener, ResourceReloadManager, ResourceReloadResult,
            ResourceReloadTaskReport,
        },
        ui::startup_flow::{
            LoadingTask, LoadingTaskPresentationState, StartupDestination, StartupLoadingPhase,
            loading_task_names,
        },
    };

    #[test]
    fn tracker_applies_progress_finishing_presentation_advance_and_complete() {
        let mut tracker = ResourceLoadingTracker::new(Vec::new());

        tracker.apply_update(ResourceLoadingUpdate::task_progress(
            loading_task_names::DOWNLOADING_ASSET,
            "stone.png",
            25,
            100,
        ));

        let screen = tracker.screen();
        assert_eq!(screen.loading_phase, StartupLoadingPhase::Loading);
        assert_eq!(
            screen.loading_overlay,
            Some(
                [LoadingTask::new(
                    loading_task_names::DOWNLOADING_ASSET,
                    "stone.png",
                    0.25,
                )]
                .as_slice()
            )
        );

        tracker.apply_update(ResourceLoadingUpdate::task_finished(
            loading_task_names::DOWNLOADING_ASSET,
            "stone.png",
        ));

        let screen = tracker.screen();
        let panels = screen.loading_panels();
        assert_eq!(panels.len(), 1);
        assert_eq!(
            panels[0].task.presentation_state,
            LoadingTaskPresentationState::Finishing
        );
        assert_eq!(panels[0].task.progress, 1.0);

        tracker.advance_presentation();

        let screen = tracker.screen();
        assert_eq!(screen.loading_phase, StartupLoadingPhase::Loading);
        assert_eq!(screen.loading_overlay, None);

        tracker.apply_update(ResourceLoadingUpdate::Complete);

        let screen = tracker.screen();
        assert_eq!(screen.loading_phase, StartupLoadingPhase::Complete);
        assert_eq!(screen.loading_overlay, None);
    }

    #[test]
    fn tracker_exposes_owned_flow_for_other_startup_actions() {
        let flow = StartupFlow::new(vec![StoredLauncherAccount::offline("Alex")]);
        let mut tracker = ResourceLoadingTracker::from_flow(flow);

        tracker.apply_update(ResourceLoadingUpdate::task_progress(
            loading_task_names::UNPACKING_CORE_ASSETS,
            "client.jar",
            1,
            4,
        ));

        assert_eq!(
            tracker.flow().loading_overlay(),
            Some(
                [LoadingTask::new(
                    loading_task_names::UNPACKING_CORE_ASSETS,
                    "client.jar",
                    0.25,
                )]
                .as_slice()
            )
        );
        assert!(tracker.into_flow().is_loading());
    }

    #[test]
    fn tracker_exposes_weighted_aggregate_progress_for_vanilla_overlay() {
        let mut tracker = ResourceLoadingTracker::new(Vec::new());

        tracker.apply_update(ResourceLoadingUpdate::task_progress(
            loading_task_names::DOWNLOADING_ASSET_INDEX,
            "1.21.6.json",
            1,
            4,
        ));
        tracker.apply_update(ResourceLoadingUpdate::task_progress(
            loading_task_names::DOWNLOADING_ASSET,
            "stone.png",
            3,
            4,
        ));

        assert_eq!(tracker.weighted_progress().actual_progress(), 0.5);

        tracker.apply_update(ResourceLoadingUpdate::Complete);

        assert_eq!(tracker.weighted_progress().actual_progress(), 1.0);
    }

    #[test]
    fn tracker_uses_resource_reload_snapshot_actual_progress_for_vanilla_overlay() {
        let report = ResourceReloadManager::new(ClientResourceStack::new(Vec::new()))
            .with_listener(TestReloadListener("textures"))
            .with_listener(TestReloadListener("sounds"))
            .run()
            .unwrap();
        let event = &report.events()[1];
        let mut tracker = ResourceLoadingTracker::new(Vec::new());

        tracker.apply_resource_reload_event(event);

        assert_eq!(tracker.flow().loading_phase(), StartupLoadingPhase::Loading);
        assert_eq!(
            tracker.weighted_progress().actual_progress(),
            event.progress_snapshot.actual_progress()
        );
    }

    #[test]
    fn tracker_complete_overrides_resource_reload_snapshot_progress() {
        let report = ResourceReloadManager::new(ClientResourceStack::new(Vec::new()))
            .with_listener(TestReloadListener("textures"))
            .run()
            .unwrap();
        let event = &report.events()[0];
        let mut tracker = ResourceLoadingTracker::new(Vec::new());

        tracker.apply_resource_reload_event(event);
        tracker.apply_update(ResourceLoadingUpdate::Complete);

        assert_eq!(tracker.weighted_progress().actual_progress(), 1.0);
    }

    #[test]
    fn tracker_receives_intermediate_reload_progress_as_manager_runs() {
        let manager = ResourceReloadManager::new(ClientResourceStack::new(Vec::new()))
            .with_listener(TestReloadListener("textures"))
            .with_listener(TestReloadListener("sounds"));
        let mut tracker = ResourceLoadingTracker::new(Vec::new());
        let mut observed_progress = Vec::new();

        manager
            .run_with_events(|event| {
                tracker.apply_resource_reload_event(event);
                observed_progress.push(tracker.weighted_progress().actual_progress());
            })
            .expect("reload should complete");

        assert!(observed_progress.len() > 1);
        assert!(observed_progress[0] < 1.0);
        assert_eq!(observed_progress.last().copied(), Some(1.0));
    }

    #[test]
    fn tracker_run_resource_reload_marks_complete_on_success() {
        let manager = ResourceReloadManager::new(ClientResourceStack::new(Vec::new()))
            .with_listener(TestReloadListener("textures"));
        let mut tracker = ResourceLoadingTracker::new(Vec::new());

        let report = tracker
            .run_resource_reload(&manager)
            .expect("reload should complete");

        assert_eq!(report.events().len(), 4);
        assert_eq!(
            tracker.flow().loading_phase(),
            StartupLoadingPhase::Complete
        );
        assert_eq!(tracker.weighted_progress().actual_progress(), 1.0);
    }

    #[test]
    fn tracker_run_vanilla_client_resource_reload_marks_complete() {
        let mut tracker = ResourceLoadingTracker::new(Vec::new());

        let report = tracker
            .run_vanilla_client_resource_reload()
            .expect("vanilla client resources should reload through tracker");

        assert!(!report.listener_reports().is_empty());
        assert_eq!(
            tracker.flow().loading_phase(),
            StartupLoadingPhase::Complete
        );
        assert_eq!(tracker.weighted_progress().actual_progress(), 1.0);
    }

    #[test]
    fn tracker_sees_default_client_resource_reload_progress_before_completion() {
        let manager = ResourceReloadManager::with_default_vanilla_client_resources();
        let mut tracker = ResourceLoadingTracker::new(Vec::new());
        let mut saw_intermediate_progress = false;

        manager
            .run_with_events(|event| {
                tracker.apply_resource_reload_event(event);
                let progress = tracker.weighted_progress().actual_progress();

                if progress > 0.0 && progress < 1.0 {
                    saw_intermediate_progress = true;
                    assert_eq!(tracker.flow().loading_phase(), StartupLoadingPhase::Loading);
                }
            })
            .expect("vanilla client resources should reload");

        assert!(saw_intermediate_progress);
        assert_ne!(
            tracker.flow().loading_phase(),
            StartupLoadingPhase::Complete
        );
    }

    #[test]
    fn tracker_run_initial_client_resource_reload_from_repository_completes_to_title_menu() {
        let repository = ClientResourceRepository::committed_vanilla();
        let mut tracker = ResourceLoadingTracker::new(Vec::new());

        let report = tracker
            .run_initial_client_resource_reload_from_repository(&repository)
            .expect("committed vanilla client resources should reload through startup tracker");

        assert!(!report.events().is_empty());
        assert!(!report.listener_reports().is_empty());
        assert_eq!(
            tracker.flow().loading_phase(),
            StartupLoadingPhase::Complete
        );
        assert_eq!(
            tracker.flow().startup_destination(),
            Some(StartupDestination::TitleMenu)
        );
        assert_eq!(tracker.weighted_progress().actual_progress(), 1.0);
    }

    #[test]
    fn tracker_run_resource_reload_propagates_error_and_keeps_last_progress() {
        let manager = ResourceReloadManager::new(ClientResourceStack::new(Vec::new()))
            .with_listener(TestReloadListener("textures"))
            .with_listener(FailingReloadListener("sounds"));
        let mut tracker = ResourceLoadingTracker::new(Vec::new());

        let error = tracker
            .run_resource_reload(&manager)
            .expect_err("reload failure should propagate");

        assert!(
            matches!(error, ResourceReloadError::MissingResource(resource) if resource == "test:sounds")
        );
        assert_ne!(
            tracker.flow().loading_phase(),
            StartupLoadingPhase::Complete
        );
        assert!(
            (tracker.weighted_progress().actual_progress() - (7.0 / 10.0)).abs() < f32::EPSILON
        );
    }

    #[test]
    fn tracker_bridges_lifecycle_events_into_flow_state() {
        let mut tracker = ResourceLoadingTracker::new(Vec::new());

        tracker.apply_event(ResourceLoadingEvent::started(
            loading_task_names::DOWNLOADING_CORE_ASSETS,
            "client.jar",
        ));
        tracker.apply_event(ResourceLoadingEvent::progress(
            loading_task_names::DOWNLOADING_CORE_ASSETS,
            "client.jar",
            1,
            4,
        ));
        tracker.apply_event(ResourceLoadingEvent::finished(
            loading_task_names::DOWNLOADING_CORE_ASSETS,
            "client.jar",
        ));

        assert_eq!(
            tracker.flow().loading_overlay(),
            Some(
                [LoadingTask::finishing(
                    loading_task_names::DOWNLOADING_CORE_ASSETS,
                    "client.jar",
                )]
                .as_slice()
            )
        );

        tracker.advance_presentation();
        tracker.apply_event(ResourceLoadingEvent::Complete);

        assert_eq!(tracker.flow().loading_overlay(), None);
        assert_eq!(
            tracker.flow().loading_phase(),
            StartupLoadingPhase::Complete
        );
    }

    #[derive(Clone, Copy)]
    struct TestReloadListener(&'static str);

    impl fmt::Debug for TestReloadListener {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter
                .debug_tuple("TestReloadListener")
                .field(&self.0)
                .finish()
        }
    }

    impl ResourceReloadListener for TestReloadListener {
        fn name(&self) -> &str {
            self.0
        }

        fn prepare(
            &self,
            _stack: &ClientResourceStack,
        ) -> ResourceReloadResult<ResourceReloadTaskReport> {
            Ok(ResourceReloadTaskReport::empty())
        }

        fn reload(
            &self,
            _stack: &ClientResourceStack,
        ) -> ResourceReloadResult<ResourceReloadTaskReport> {
            Ok(ResourceReloadTaskReport::empty())
        }
    }

    #[derive(Clone, Copy)]
    struct FailingReloadListener(&'static str);

    impl fmt::Debug for FailingReloadListener {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter
                .debug_tuple("FailingReloadListener")
                .field(&self.0)
                .finish()
        }
    }

    impl ResourceReloadListener for FailingReloadListener {
        fn name(&self) -> &str {
            self.0
        }

        fn prepare(
            &self,
            _stack: &ClientResourceStack,
        ) -> ResourceReloadResult<ResourceReloadTaskReport> {
            Err(ResourceReloadError::MissingResource(format!(
                "test:{}",
                self.0
            )))
        }

        fn reload(
            &self,
            _stack: &ClientResourceStack,
        ) -> ResourceReloadResult<ResourceReloadTaskReport> {
            Ok(ResourceReloadTaskReport::empty())
        }
    }
}

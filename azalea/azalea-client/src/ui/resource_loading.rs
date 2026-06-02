//! Resource loading adapter for startup UI flow state.
//!
//! A future real resource loader can report task updates here without knowing
//! how the startup screen lays out loading panels.

use std::time::Duration;

use super::{
    account_flow::StoredLauncherAccount,
    startup_flow::{
        ResourceLoadingEvent, ResourceLoadingUpdate, StartupFlow, StartupLoadingPhase,
        StartupScreen, VanillaLoadingBackground, VanillaLoadingOverlay, VanillaLoadingOverlayView,
        VanillaLoadingTextView, WeightedReloadProgress, WeightedReloadStageProgress,
    },
};
use crate::resources::{
    ClientResourceRepository, ClientResourceStack,
    ResourceReloadEvent as ClientResourceReloadEvent, ResourceReloadManager,
    ResourceReloadProgressSnapshot, ResourceReloadReport, ResourceReloadResult, ResourceReloadStep,
};

pub const MOJANG_STUDIOS_LOGO_ID: &str = "minecraft:textures/gui/title/mojangstudios.png";
pub const MOJANG_STUDIOS_LOGO_RESOURCE: &str =
    "assets/minecraft/textures/gui/title/mojangstudios.png";
pub const MOJANG_STUDIOS_RED_BACKGROUND_ARGB: u32 = 0xFFEF_323D;
pub const MOJANG_STUDIOS_BLACK_BACKGROUND_ARGB: u32 = 0xFF00_0000;
pub const FALLBACK_RELOAD_LABEL: &str = VanillaLoadingTextView::FALLBACK_LOADING_MINECRAFT;

#[derive(Clone, Debug)]
pub struct ResourceLoadingTracker {
    flow: StartupFlow,
    resource_reload_snapshot: Option<ResourceReloadProgressSnapshot>,
    resource_reload_error: Option<String>,
    pack_load_failure_toast: Option<ResourceLoadingPackLoadFailureToast>,
}

impl ResourceLoadingTracker {
    pub fn new(accounts: Vec<StoredLauncherAccount>) -> Self {
        Self {
            flow: StartupFlow::new(accounts),
            resource_reload_snapshot: None,
            resource_reload_error: None,
            pack_load_failure_toast: None,
        }
    }

    pub fn from_flow(flow: StartupFlow) -> Self {
        Self {
            flow,
            resource_reload_snapshot: None,
            resource_reload_error: None,
            pack_load_failure_toast: None,
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

        if let Some(snapshot) = &self.resource_reload_snapshot {
            return weighted_progress_from_resource_reload(snapshot);
        }

        WeightedReloadProgress::from_loading_tasks(self.flow.loading_overlay().unwrap_or_default())
    }

    pub fn resource_reload_snapshot(&self) -> Option<&ResourceReloadProgressSnapshot> {
        self.resource_reload_snapshot.as_ref()
    }

    pub fn resource_reload_error(&self) -> Option<&str> {
        self.resource_reload_error.as_deref()
    }

    pub fn pack_load_failure_toast(&self) -> Option<&ResourceLoadingPackLoadFailureToast> {
        self.pack_load_failure_toast.as_ref()
    }

    pub fn loading_overlay_actual_progress(&self) -> f32 {
        if self.flow.loading_phase() == StartupLoadingPhase::Complete {
            return 1.0;
        }

        self.resource_reload_snapshot
            .as_ref()
            .map(ResourceReloadProgressSnapshot::actual_progress)
            .unwrap_or_else(|| self.weighted_progress().actual_progress())
            .clamp(0.0, 1.0)
    }

    pub fn loading_overlay_view(
        &self,
        overlay: &VanillaLoadingOverlay,
        elapsed: Duration,
    ) -> MojangLoadingOverlayViewModel {
        let vanilla = overlay.view(self.loading_overlay_actual_progress(), elapsed);
        MojangLoadingOverlayViewModel {
            background_argb: background_argb(vanilla.background),
            logo_texture: Some(MojangLogoTexture::default()),
            reload: self.reload_view(),
            smoothing: LoadingOverlaySmoothing::vanilla(),
            fade: LoadingOverlayFadeTiming::vanilla(),
            vanilla,
        }
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
        self.flow.show_mojang_loading_overlay();
        self.resource_reload_snapshot = Some(snapshot.clone());
        self.resource_reload_error = None;
        self.pack_load_failure_toast = None;
    }

    pub fn run_resource_reload(
        &mut self,
        manager: &ResourceReloadManager,
    ) -> ResourceReloadResult<ResourceReloadReport> {
        let report = manager.run_with_events(|event| self.apply_resource_reload_event(event));

        match &report {
            Ok(_) => self.mark_complete(),
            Err(error) => {
                let error = error.to_string();
                self.resource_reload_error = Some(error.clone());
                self.pack_load_failure_toast =
                    Some(ResourceLoadingPackLoadFailureToast::from_error(error));
            }
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
        self.resource_reload_error = None;
        self.pack_load_failure_toast = None;
        self.flow.finish_loading();
    }

    fn reload_view(&self) -> ResourceLoadingReloadView {
        if let Some(error) = &self.resource_reload_error {
            return ResourceLoadingReloadView {
                label: error.clone(),
                phase: ResourceLoadingReloadPhase::Error,
            };
        }

        if self.flow.loading_phase() == StartupLoadingPhase::Complete {
            return ResourceLoadingReloadView {
                label: "Complete".to_owned(),
                phase: ResourceLoadingReloadPhase::Complete,
            };
        }

        let Some(snapshot) = &self.resource_reload_snapshot else {
            return ResourceLoadingReloadView::fallback();
        };

        ResourceLoadingReloadView {
            label: snapshot
                .current_listener()
                .filter(|label| !label.is_empty())
                .unwrap_or(FALLBACK_RELOAD_LABEL)
                .to_owned(),
            phase: snapshot
                .current_step()
                .map(ResourceLoadingReloadPhase::from)
                .unwrap_or(ResourceLoadingReloadPhase::Fallback),
        }
    }
}

fn weighted_progress_from_resource_reload(
    snapshot: &ResourceReloadProgressSnapshot,
) -> WeightedReloadProgress {
    let label = snapshot
        .current_listener()
        .filter(|label| !label.is_empty())
        .unwrap_or(FALLBACK_RELOAD_LABEL);
    WeightedReloadProgress::new([WeightedReloadStageProgress::new(
        label,
        snapshot.actual_progress(),
        1.0,
    )])
}

#[derive(Clone, Debug, PartialEq)]
pub struct MojangLoadingOverlayViewModel {
    pub vanilla: VanillaLoadingOverlayView,
    pub background_argb: u32,
    pub logo_texture: Option<MojangLogoTexture>,
    pub reload: ResourceLoadingReloadView,
    pub smoothing: LoadingOverlaySmoothing,
    pub fade: LoadingOverlayFadeTiming,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MojangLogoTexture {
    pub id: &'static str,
    pub resource: &'static str,
}

impl Default for MojangLogoTexture {
    fn default() -> Self {
        Self {
            id: MOJANG_STUDIOS_LOGO_ID,
            resource: MOJANG_STUDIOS_LOGO_RESOURCE,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceLoadingReloadView {
    pub label: String,
    pub phase: ResourceLoadingReloadPhase,
}

impl ResourceLoadingReloadView {
    pub fn fallback() -> Self {
        Self {
            label: FALLBACK_RELOAD_LABEL.to_owned(),
            phase: ResourceLoadingReloadPhase::Fallback,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourceLoadingReloadPhase {
    Fallback,
    InitialPreparation,
    Preparation,
    Reload,
    ListenerComplete,
    Complete,
    Error,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceLoadingPackLoadFailureToast {
    pub id: ResourceLoadingSystemToastId,
    pub title_key: &'static str,
    pub message: Option<String>,
}

impl ResourceLoadingPackLoadFailureToast {
    pub const TITLE_KEY: &'static str = "resourcePack.load_fail";

    pub fn from_error(error: impl Into<String>) -> Self {
        Self {
            id: ResourceLoadingSystemToastId::PackLoadFailure,
            title_key: Self::TITLE_KEY,
            message: Some(error.into()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourceLoadingSystemToastId {
    PackLoadFailure,
}

impl From<ResourceReloadStep> for ResourceLoadingReloadPhase {
    fn from(step: ResourceReloadStep) -> Self {
        match step {
            ResourceReloadStep::InitialPreparation => Self::InitialPreparation,
            ResourceReloadStep::Preparation => Self::Preparation,
            ResourceReloadStep::Reload => Self::Reload,
            ResourceReloadStep::ListenerComplete => Self::ListenerComplete,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LoadingOverlaySmoothing {
    pub previous_displayed_progress_weight: f32,
    pub actual_progress_weight: f32,
}

impl LoadingOverlaySmoothing {
    pub fn vanilla() -> Self {
        Self {
            previous_displayed_progress_weight:
                VanillaLoadingOverlay::SMOOTHED_DISPLAYED_PROGRESS_WEIGHT,
            actual_progress_weight: VanillaLoadingOverlay::NEW_ACTUAL_PROGRESS_WEIGHT,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LoadingOverlayFadeTiming {
    pub fade_in: Duration,
    pub fade_out: Duration,
}

impl LoadingOverlayFadeTiming {
    pub fn vanilla() -> Self {
        Self {
            fade_in: VanillaLoadingOverlay::FADE_IN,
            fade_out: VanillaLoadingOverlay::FADE_OUT,
        }
    }
}

fn background_argb(background: VanillaLoadingBackground) -> u32 {
    match background {
        VanillaLoadingBackground::Red => MOJANG_STUDIOS_RED_BACKGROUND_ARGB,
        VanillaLoadingBackground::Black => MOJANG_STUDIOS_BLACK_BACKGROUND_ARGB,
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
            LoadingTask, LoadingTaskPresentationState, StartupDestination,
            StartupGenericMessageView, StartupLoadingPhase, StartupLoadingScreen,
            StartupMojangLoadingOverlaySurface, loading_task_names,
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
    fn resource_loading_overlay_view_uses_snapshot_actual_progress_and_reload_phase() {
        let report = ResourceReloadManager::new(ClientResourceStack::new(Vec::new()))
            .with_listener(TestReloadListener("textures"))
            .with_listener(TestReloadListener("sounds"))
            .run()
            .unwrap();
        let preparation_event = &report.events()[1];
        let mut tracker = ResourceLoadingTracker::new(Vec::new());
        tracker.apply_update(ResourceLoadingUpdate::task_progress(
            loading_task_names::DOWNLOADING_ASSET,
            "stone.png",
            9,
            10,
        ));

        tracker.apply_resource_reload_event(preparation_event);

        assert_eq!(
            tracker.loading_overlay_actual_progress(),
            preparation_event.progress_snapshot.actual_progress()
        );
        assert_ne!(tracker.loading_overlay_actual_progress(), 0.9);

        let mut overlay = VanillaLoadingOverlay::new();
        overlay.tick(tracker.loading_overlay_actual_progress());
        let view = tracker.loading_overlay_view(&overlay, Duration::from_millis(250));

        assert_eq!(
            view.vanilla.actual_progress,
            preparation_event.progress_snapshot.actual_progress()
        );
        assert!(
            (view.vanilla.progress_bar.displayed_progress
                - preparation_event.progress_snapshot.actual_progress()
                    * VanillaLoadingOverlay::NEW_ACTUAL_PROGRESS_WEIGHT)
                .abs()
                < f32::EPSILON
        );
        assert_eq!(view.reload.label, "textures");
        assert_eq!(view.reload.phase, ResourceLoadingReloadPhase::Preparation);
    }

    #[test]
    fn resource_loading_overlay_view_reports_complete_and_error_states() {
        let report = ResourceReloadManager::new(ClientResourceStack::new(Vec::new()))
            .with_listener(TestReloadListener("textures"))
            .run()
            .unwrap();
        let mut tracker = ResourceLoadingTracker::new(Vec::new());
        tracker.apply_resource_reload_event(&report.events()[0]);
        tracker.apply_update(ResourceLoadingUpdate::Complete);

        let complete_view =
            tracker.loading_overlay_view(&VanillaLoadingOverlay::new(), Duration::ZERO);

        assert_eq!(complete_view.vanilla.actual_progress, 1.0);
        assert_eq!(
            complete_view.reload,
            ResourceLoadingReloadView {
                label: "Complete".to_owned(),
                phase: ResourceLoadingReloadPhase::Complete,
            }
        );
        assert_eq!(tracker.resource_reload_error(), None);
        assert_eq!(tracker.pack_load_failure_toast(), None);

        let failing_manager = ResourceReloadManager::new(ClientResourceStack::new(Vec::new()))
            .with_listener(TestReloadListener("textures"))
            .with_listener(FailingReloadListener("sounds"));
        let mut failing_tracker = ResourceLoadingTracker::new(Vec::new());

        let error = failing_tracker
            .run_resource_reload(&failing_manager)
            .expect_err("reload failure should be surfaced");
        let error_message = error.to_string();
        let error_view =
            failing_tracker.loading_overlay_view(&VanillaLoadingOverlay::new(), Duration::ZERO);

        assert_eq!(
            failing_tracker.resource_reload_error(),
            Some(error_message.as_str())
        );
        assert_eq!(
            failing_tracker.pack_load_failure_toast(),
            Some(&ResourceLoadingPackLoadFailureToast {
                id: ResourceLoadingSystemToastId::PackLoadFailure,
                title_key: "resourcePack.load_fail",
                message: Some(error_message),
            })
        );
        assert_eq!(error_view.reload.phase, ResourceLoadingReloadPhase::Error);
        assert!(error_view.reload.label.contains("test:sounds"));
        assert_ne!(
            failing_tracker.flow().loading_phase(),
            StartupLoadingPhase::Complete
        );
    }

    #[test]
    fn pack_load_failure_toast_is_exposed_with_reload_error_view() {
        let manager = ResourceReloadManager::new(ClientResourceStack::new(Vec::new()))
            .with_listener(TestReloadListener("textures"))
            .with_listener(FailingReloadListener("sounds"));
        let mut tracker = ResourceLoadingTracker::new(Vec::new());

        let error = tracker
            .run_resource_reload(&manager)
            .expect_err("reload failure should expose pack load failure toast");
        let error_message = error.to_string();
        let view = tracker.loading_overlay_view(&VanillaLoadingOverlay::new(), Duration::ZERO);

        assert_eq!(view.reload.phase, ResourceLoadingReloadPhase::Error);
        assert_eq!(view.reload.label, error_message);
        assert_eq!(
            tracker.pack_load_failure_toast(),
            Some(&ResourceLoadingPackLoadFailureToast {
                id: ResourceLoadingSystemToastId::PackLoadFailure,
                title_key: ResourceLoadingPackLoadFailureToast::TITLE_KEY,
                message: Some(error_message),
            })
        );
    }

    #[test]
    fn pack_load_failure_toast_is_absent_after_success_and_complete() {
        let success_manager = ResourceReloadManager::new(ClientResourceStack::new(Vec::new()))
            .with_listener(TestReloadListener("textures"));
        let mut success_tracker = ResourceLoadingTracker::new(Vec::new());

        success_tracker
            .run_resource_reload(&success_manager)
            .expect("successful reload should complete");

        assert_eq!(success_tracker.resource_reload_error(), None);
        assert_eq!(success_tracker.pack_load_failure_toast(), None);

        let failing_manager = ResourceReloadManager::new(ClientResourceStack::new(Vec::new()))
            .with_listener(FailingReloadListener("sounds"));
        let report = ResourceReloadManager::new(ClientResourceStack::new(Vec::new()))
            .with_listener(TestReloadListener("textures"))
            .run()
            .expect("reload should provide a fresh snapshot");
        let mut recovering_tracker = ResourceLoadingTracker::new(Vec::new());

        recovering_tracker
            .run_resource_reload(&failing_manager)
            .expect_err("failure should create toast state");
        assert!(recovering_tracker.pack_load_failure_toast().is_some());

        recovering_tracker.apply_resource_reload_event(&report.events()[0]);
        assert_eq!(recovering_tracker.resource_reload_error(), None);
        assert_eq!(recovering_tracker.pack_load_failure_toast(), None);

        recovering_tracker
            .run_resource_reload(&failing_manager)
            .expect_err("failure should recreate toast state");
        assert!(recovering_tracker.pack_load_failure_toast().is_some());

        recovering_tracker.mark_complete();
        assert_eq!(recovering_tracker.resource_reload_error(), None);
        assert_eq!(recovering_tracker.pack_load_failure_toast(), None);
    }

    #[test]
    fn resource_loading_overlay_view_keeps_loading_minecraft_fallback_label() {
        let tracker = ResourceLoadingTracker::new(Vec::new());

        let view = tracker.loading_overlay_view(&VanillaLoadingOverlay::new(), Duration::ZERO);

        assert_eq!(
            view.reload,
            ResourceLoadingReloadView {
                label: "Loading Minecraft".to_owned(),
                phase: ResourceLoadingReloadPhase::Fallback,
            }
        );
        assert_eq!(view.vanilla.text.text, "Loading Minecraft");
        assert_eq!(view.vanilla.actual_progress, 0.0);
    }

    #[test]
    fn fallback_loading_screen_progress_snapshot_and_completion_are_distinct_startup_states() {
        let report = ResourceReloadManager::new(ClientResourceStack::new(Vec::new()))
            .with_listener(TestReloadListener("textures"))
            .run()
            .unwrap();
        let preparation_event = &report.events()[1];
        let mut tracker = ResourceLoadingTracker::new(Vec::new());

        assert_eq!(
            tracker.screen().loading_screen,
            StartupLoadingScreen::GenericMessage(StartupGenericMessageView::loading_minecraft())
        );
        assert_eq!(tracker.screen().loading_overlay, None);

        tracker.apply_resource_reload_event(preparation_event);

        let reloading_screen = tracker.screen();
        assert_eq!(
            reloading_screen.loading_screen,
            StartupLoadingScreen::MojangLoadingOverlay(StartupMojangLoadingOverlaySurface {
                tasks: None,
            })
        );
        assert_eq!(reloading_screen.loading_overlay, None);
        assert_eq!(
            tracker
                .loading_overlay_view(&VanillaLoadingOverlay::new(), Duration::ZERO)
                .reload,
            ResourceLoadingReloadView {
                label: "textures".to_owned(),
                phase: ResourceLoadingReloadPhase::Preparation,
            }
        );

        tracker.mark_complete();

        let complete_screen = tracker.screen();
        assert_eq!(
            complete_screen.loading_screen,
            StartupLoadingScreen::CompleteDestination(Some(StartupDestination::TitleMenu))
        );
        assert_eq!(
            complete_screen.startup_destination,
            Some(StartupDestination::TitleMenu)
        );
    }

    #[test]
    fn resource_loading_overlay_view_exposes_stable_vanilla_layout_constants() {
        let red_view = ResourceLoadingTracker::new(Vec::new())
            .loading_overlay_view(&VanillaLoadingOverlay::new(), Duration::ZERO);
        let black_view = ResourceLoadingTracker::new(Vec::new()).loading_overlay_view(
            &VanillaLoadingOverlay::new().with_background(VanillaLoadingBackground::Black),
            Duration::ZERO,
        );

        assert_eq!(red_view.background_argb, MOJANG_STUDIOS_RED_BACKGROUND_ARGB);
        assert_eq!(
            black_view.background_argb,
            MOJANG_STUDIOS_BLACK_BACKGROUND_ARGB
        );
        assert_eq!(
            red_view.logo_texture,
            Some(MojangLogoTexture {
                id: "minecraft:textures/gui/title/mojangstudios.png",
                resource: "assets/minecraft/textures/gui/title/mojangstudios.png",
            })
        );
        assert_eq!(red_view.vanilla.progress_bar.center_x_fraction, 0.5);
        assert_eq!(red_view.vanilla.progress_bar.center_y_fraction, 0.8325);
        assert_eq!(red_view.vanilla.progress_bar.outer_height, 10.0);
        assert_eq!(red_view.vanilla.progress_bar.border_width, 1.0);
        assert_eq!(red_view.vanilla.progress_bar.fill_inset, 2.0);
        assert_eq!(
            red_view.smoothing,
            LoadingOverlaySmoothing {
                previous_displayed_progress_weight: 0.95,
                actual_progress_weight: 0.05,
            }
        );
        assert_eq!(red_view.fade.fade_in, Duration::from_millis(500));
        assert_eq!(red_view.fade.fade_out, Duration::from_millis(1_000));
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

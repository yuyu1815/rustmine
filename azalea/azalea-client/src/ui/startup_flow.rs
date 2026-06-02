//! Startup screen flow state.
//!
//! This keeps Stevenarella's startup storyboard visible to presentation code:
//! background and launcher are available immediately, while resource loading
//! appears as an overlay instead of a blocking screen.

use std::time::Duration;

use bevy_ecs::prelude::Message;

use super::account_flow::{AccountFlow, AccountFlowScreen, StoredLauncherAccount};

#[derive(Clone, Debug)]
pub struct StartupFlow {
    account_flow: AccountFlow,
    loading_phase: StartupLoadingPhase,
    loading_surface: StartupLoadingSurface,
    startup_destination: Option<StartupDestination>,
    loading_tasks: Vec<LoadingTask>,
}

impl StartupFlow {
    pub fn new(accounts: Vec<StoredLauncherAccount>) -> Self {
        Self {
            account_flow: AccountFlow::new(accounts),
            loading_phase: StartupLoadingPhase::WaitingForTasks,
            loading_surface: StartupLoadingSurface::GenericMessage,
            startup_destination: None,
            loading_tasks: Vec::new(),
        }
    }

    pub fn screen(&self) -> StartupScreen<'_> {
        StartupScreen {
            background_visible: true,
            account_screen: self.account_flow.screen(),
            loading_phase: self.loading_phase,
            loading_screen: self.loading_screen(),
            startup_destination: self.startup_destination,
            loading_overlay: self.loading_overlay(),
        }
    }

    pub fn account_flow(&self) -> &AccountFlow {
        &self.account_flow
    }

    pub fn account_flow_mut(&mut self) -> &mut AccountFlow {
        &mut self.account_flow
    }

    pub fn loading_overlay(&self) -> Option<&[LoadingTask]> {
        (!self.loading_tasks.is_empty()).then_some(&self.loading_tasks)
    }

    pub fn loading_phase(&self) -> StartupLoadingPhase {
        self.loading_phase
    }

    pub fn loading_screen(&self) -> StartupLoadingScreen<'_> {
        match self.loading_surface {
            StartupLoadingSurface::GenericMessage => {
                StartupLoadingScreen::GenericMessage(StartupGenericMessageView::loading_minecraft())
            }
            StartupLoadingSurface::MojangLoadingOverlay => {
                StartupLoadingScreen::MojangLoadingOverlay(StartupMojangLoadingOverlaySurface {
                    tasks: self.loading_overlay(),
                })
            }
            StartupLoadingSurface::CompleteDestination => {
                StartupLoadingScreen::CompleteDestination(self.startup_destination)
            }
        }
    }

    pub fn startup_destination(&self) -> Option<StartupDestination> {
        self.startup_destination
    }

    pub fn is_loading(&self) -> bool {
        self.loading_phase == StartupLoadingPhase::Loading
    }

    pub fn loading_is_complete(&self) -> bool {
        self.loading_phase == StartupLoadingPhase::Complete
    }

    pub fn begin_loading(&mut self) {
        if !self.loading_is_complete() {
            self.loading_phase = StartupLoadingPhase::Loading;
            self.loading_surface = StartupLoadingSurface::GenericMessage;
            self.startup_destination = None;
        }
    }

    pub fn show_mojang_loading_overlay(&mut self) {
        if !self.loading_is_complete() {
            self.loading_phase = StartupLoadingPhase::Loading;
            self.loading_surface = StartupLoadingSurface::MojangLoadingOverlay;
            self.startup_destination = None;
        }
    }

    pub fn replace_loading_tasks(&mut self, tasks: impl IntoIterator<Item = LoadingTask>) {
        self.loading_tasks = tasks.into_iter().collect();
        if !self.loading_tasks.is_empty() {
            self.loading_phase = StartupLoadingPhase::Loading;
            self.loading_surface = StartupLoadingSurface::MojangLoadingOverlay;
            self.startup_destination = None;
        }
    }

    pub fn upsert_loading_task(&mut self, task: LoadingTask) {
        if let Some(existing_task) = self
            .loading_tasks
            .iter_mut()
            .find(|existing_task| existing_task.has_same_identity_as(&task))
        {
            existing_task.merge_progress_from(task);
        } else {
            self.loading_tasks.push(task);
        }
        self.loading_phase = StartupLoadingPhase::Loading;
        self.loading_surface = StartupLoadingSurface::MojangLoadingOverlay;
        self.startup_destination = None;
    }

    pub fn remove_loading_task(
        &mut self,
        name: impl AsRef<str>,
        file: impl AsRef<str>,
    ) -> Option<LoadingTask> {
        let index = self
            .loading_tasks
            .iter()
            .position(|task| task.matches(name.as_ref(), file.as_ref()))?;
        let removed_task = self.loading_tasks.remove(index);
        Some(removed_task)
    }

    pub fn finish_loading_task(
        &mut self,
        name: impl Into<String>,
        file: impl Into<String>,
    ) -> LoadingTask {
        let name = name.into();
        let file = file.into();

        if let Some(existing_task) = self
            .loading_tasks
            .iter_mut()
            .find(|task| task.matches(&name, &file))
        {
            existing_task.mark_finishing();
            return existing_task.clone();
        }

        let task = LoadingTask::finishing(name, file);
        self.loading_tasks.push(task.clone());
        self.loading_phase = StartupLoadingPhase::Loading;
        self.loading_surface = StartupLoadingSurface::MojangLoadingOverlay;
        self.startup_destination = None;
        task
    }

    pub fn advance_loading_presentation(&mut self) {
        self.loading_tasks.retain(|task| !task.is_finishing());
    }

    pub fn finish_loading(&mut self) {
        self.finish_loading_to(StartupDestination::TitleMenu);
    }

    pub fn finish_loading_to(&mut self, destination: StartupDestination) {
        self.loading_tasks.clear();
        self.loading_phase = StartupLoadingPhase::Complete;
        self.loading_surface = StartupLoadingSurface::CompleteDestination;
        self.startup_destination = Some(destination);
    }

    pub fn clear_loading_tasks(&mut self) {
        self.loading_tasks.clear();
    }

    pub fn apply_resource_loading_update(&mut self, update: ResourceLoadingUpdate) {
        match update {
            ResourceLoadingUpdate::TaskProgress(task) => {
                self.upsert_loading_task(task.into_loading_task());
            }
            ResourceLoadingUpdate::TaskFinished { name, file } => {
                self.finish_loading_task(name, file);
            }
            ResourceLoadingUpdate::Complete => {
                self.finish_loading();
            }
        }
    }

    pub fn apply_resource_loading_event(&mut self, event: ResourceLoadingEvent) {
        match event {
            ResourceLoadingEvent::Started { name, file } => {
                self.begin_loading();
                self.upsert_loading_task(LoadingTask::new(name, file, 0.0));
            }
            ResourceLoadingEvent::Progress {
                name,
                file,
                progress,
                total,
            } => {
                self.apply_resource_loading_update(ResourceLoadingUpdate::task_progress(
                    name, file, progress, total,
                ));
            }
            ResourceLoadingEvent::Finished { name, file } => {
                self.apply_resource_loading_update(ResourceLoadingUpdate::task_finished(
                    name, file,
                ));
            }
            ResourceLoadingEvent::Complete => {
                self.apply_resource_loading_update(ResourceLoadingUpdate::Complete);
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StartupScreen<'a> {
    pub background_visible: bool,
    pub account_screen: &'a AccountFlowScreen,
    pub loading_phase: StartupLoadingPhase,
    pub loading_screen: StartupLoadingScreen<'a>,
    pub startup_destination: Option<StartupDestination>,
    pub loading_overlay: Option<&'a [LoadingTask]>,
}

impl StartupScreen<'_> {
    pub fn loading_panels(&self) -> Vec<LoadingOverlayPanel<'_>> {
        self.loading_overlay
            .unwrap_or_default()
            .iter()
            .enumerate()
            .map(|(stack_index, task)| LoadingOverlayPanel::new(stack_index, task))
            .collect()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StartupLoadingPhase {
    WaitingForTasks,
    Loading,
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum StartupLoadingSurface {
    GenericMessage,
    MojangLoadingOverlay,
    CompleteDestination,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StartupLoadingScreen<'a> {
    GenericMessage(StartupGenericMessageView),
    MojangLoadingOverlay(StartupMojangLoadingOverlaySurface<'a>),
    CompleteDestination(Option<StartupDestination>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StartupGenericMessageView {
    pub message: VanillaLoadingTextView,
}

impl StartupGenericMessageView {
    pub fn loading_minecraft() -> Self {
        Self {
            message: VanillaLoadingTextView::fallback_loading_minecraft(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StartupMojangLoadingOverlaySurface<'a> {
    pub tasks: Option<&'a [LoadingTask]>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StartupDestination {
    TitleMenu,
    QuickPlay,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VanillaLoadingOverlay {
    displayed_progress: f32,
    fade_out_start: Option<Duration>,
    background: VanillaLoadingBackground,
}

impl VanillaLoadingOverlay {
    pub const SMOOTHED_DISPLAYED_PROGRESS_WEIGHT: f32 = 0.95;
    pub const NEW_ACTUAL_PROGRESS_WEIGHT: f32 = 0.05;
    pub const FADE_IN: Duration = Duration::from_millis(500);
    pub const FADE_OUT: Duration = Duration::from_millis(1_000);

    pub fn new() -> Self {
        Self {
            displayed_progress: 0.0,
            fade_out_start: None,
            background: VanillaLoadingBackground::Red,
        }
    }

    pub fn with_background(mut self, background: VanillaLoadingBackground) -> Self {
        self.background = background;
        self
    }

    pub fn displayed_progress(&self) -> f32 {
        self.displayed_progress
    }

    pub fn fade_out_start(&self) -> Option<Duration> {
        self.fade_out_start
    }

    pub fn tick(&mut self, actual_progress: f32) {
        self.displayed_progress = (self.displayed_progress
            * Self::SMOOTHED_DISPLAYED_PROGRESS_WEIGHT
            + actual_progress.clamp(0.0, 1.0) * Self::NEW_ACTUAL_PROGRESS_WEIGHT)
            .clamp(0.0, 1.0);
    }

    pub fn update_fade_out(&mut self, loading_phase: StartupLoadingPhase, elapsed: Duration) {
        self.update_fade_out_when_ready(loading_phase, elapsed, true);
    }

    pub fn update_fade_out_when_ready(
        &mut self,
        loading_phase: StartupLoadingPhase,
        elapsed: Duration,
        fade_in_ready: bool,
    ) {
        if loading_phase == StartupLoadingPhase::Complete
            && fade_in_ready
            && self.fade_out_start.is_none()
        {
            self.fade_out_start = Some(elapsed);
        }
    }

    pub fn view(&self, actual_progress: f32, elapsed: Duration) -> VanillaLoadingOverlayView {
        let actual_progress = actual_progress.clamp(0.0, 1.0);
        VanillaLoadingOverlayView {
            background: self.background,
            logo: VanillaLoadingLogoView::centered_mojang_studios(),
            text: VanillaLoadingTextView::fallback_loading_minecraft(),
            progress_bar: VanillaLoadingProgressBarView::from_displayed_progress(
                self.displayed_progress,
            ),
            displayed_progress: self.displayed_progress,
            actual_progress,
            fade_alpha: self.fade_alpha(elapsed),
            should_render: self.should_render(elapsed),
        }
    }

    pub fn fade_alpha(&self, elapsed: Duration) -> f32 {
        if let Some(start) = self.fade_out_start {
            let fade_out =
                elapsed.saturating_sub(start).as_secs_f32() / Self::FADE_OUT.as_secs_f32();
            return (1.0 - fade_out).clamp(0.0, 1.0);
        }

        (elapsed.as_secs_f32() / Self::FADE_IN.as_secs_f32()).clamp(0.0, 1.0)
    }

    pub fn should_render(&self, elapsed: Duration) -> bool {
        self.fade_out_start
            .is_none_or(|start| elapsed.saturating_sub(start) < Self::FADE_OUT * 2)
    }
}

impl Default for VanillaLoadingOverlay {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VanillaLoadingBackground {
    Red,
    Black,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VanillaLoadingLogoKind {
    MojangStudios,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VanillaLoadingLogoPlacement {
    Centered,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VanillaLoadingLogoView {
    pub kind: VanillaLoadingLogoKind,
    pub placement: VanillaLoadingLogoPlacement,
    pub split_texture_halves: bool,
}

impl VanillaLoadingLogoView {
    pub fn centered_mojang_studios() -> Self {
        Self {
            kind: VanillaLoadingLogoKind::MojangStudios,
            placement: VanillaLoadingLogoPlacement::Centered,
            split_texture_halves: true,
        }
    }

    pub fn height_for_viewport(viewport_width: f32, viewport_height: f32) -> f32 {
        (viewport_width * 0.75).min(viewport_height) * 0.25
    }

    pub fn width_for_viewport(viewport_width: f32, viewport_height: f32) -> f32 {
        Self::height_for_viewport(viewport_width, viewport_height) * 4.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VanillaLoadingTextView {
    pub text: &'static str,
}

impl VanillaLoadingTextView {
    pub const FALLBACK_LOADING_MINECRAFT: &'static str = "Loading Minecraft";

    pub fn fallback_loading_minecraft() -> Self {
        Self {
            text: Self::FALLBACK_LOADING_MINECRAFT,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VanillaLoadingOverlayView {
    pub background: VanillaLoadingBackground,
    pub logo: VanillaLoadingLogoView,
    pub text: VanillaLoadingTextView,
    pub progress_bar: VanillaLoadingProgressBarView,
    pub displayed_progress: f32,
    pub actual_progress: f32,
    pub fade_alpha: f32,
    pub should_render: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VanillaLoadingProgressBarPlacement {
    CenteredBottom,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VanillaLoadingProgressBarColor {
    White,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VanillaLoadingProgressBarView {
    pub placement: VanillaLoadingProgressBarPlacement,
    pub color: VanillaLoadingProgressBarColor,
    pub center_x_fraction: f32,
    pub center_y_fraction: f32,
    pub outer_height: f32,
    pub border_width: f32,
    pub fill_inset: f32,
    pub displayed_progress: f32,
}

impl VanillaLoadingProgressBarView {
    pub const CENTER_X_FRACTION: f32 = 0.5;
    pub const CENTER_Y_FRACTION: f32 = 0.8325;
    pub const OUTER_HEIGHT: f32 = 10.0;
    pub const BORDER_WIDTH: f32 = 1.0;
    pub const FILL_INSET: f32 = 2.0;

    pub fn from_displayed_progress(displayed_progress: f32) -> Self {
        let displayed_progress = displayed_progress.clamp(0.0, 1.0);
        Self {
            placement: VanillaLoadingProgressBarPlacement::CenteredBottom,
            color: VanillaLoadingProgressBarColor::White,
            center_x_fraction: Self::CENTER_X_FRACTION,
            center_y_fraction: Self::CENTER_Y_FRACTION,
            outer_height: Self::OUTER_HEIGHT,
            border_width: Self::BORDER_WIDTH,
            fill_inset: Self::FILL_INSET,
            displayed_progress,
        }
    }

    pub fn outer_rect(&self, viewport_width: f32, viewport_height: f32) -> VanillaLoadingRect {
        let logo_width =
            VanillaLoadingLogoView::width_for_viewport(viewport_width, viewport_height);
        let x = viewport_width * self.center_x_fraction - logo_width / 2.0;
        let y = viewport_height * self.center_y_fraction - self.outer_height / 2.0;
        VanillaLoadingRect {
            x,
            y,
            width: logo_width,
            height: self.outer_height,
        }
    }

    pub fn fill_rect(&self, viewport_width: f32, viewport_height: f32) -> VanillaLoadingRect {
        let outer = self.outer_rect(viewport_width, viewport_height);
        let width = ((outer.width - self.fill_inset * 2.0) * self.displayed_progress).ceil();
        VanillaLoadingRect {
            x: outer.x + self.fill_inset,
            y: outer.y + self.fill_inset,
            width,
            height: (outer.height - self.fill_inset * 2.0).max(0.0),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VanillaLoadingRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WeightedReloadProgress {
    stages: Vec<WeightedReloadStageProgress>,
}

impl WeightedReloadProgress {
    pub fn new(stages: impl IntoIterator<Item = WeightedReloadStageProgress>) -> Self {
        Self {
            stages: stages.into_iter().collect(),
        }
    }

    pub fn simple_reload_instance(
        prepare_progress: f32,
        reload_progress: f32,
        listener_progress: f32,
    ) -> Self {
        Self::new([
            WeightedReloadStageProgress::new("prepare", prepare_progress, 2.0),
            WeightedReloadStageProgress::new("reload", reload_progress, 2.0),
            WeightedReloadStageProgress::new("listener", listener_progress, 1.0),
        ])
    }

    pub fn from_loading_tasks(tasks: &[LoadingTask]) -> Self {
        if tasks.is_empty() {
            return Self::new([]);
        }

        Self::new(
            tasks
                .iter()
                .map(|task| WeightedReloadStageProgress::new(&task.name, task.progress, 1.0)),
        )
    }

    pub fn complete_simple_reload_instance() -> Self {
        Self::simple_reload_instance(1.0, 1.0, 1.0)
    }

    pub fn actual_progress(&self) -> f32 {
        let total_weight = self
            .stages
            .iter()
            .map(|stage| stage.weight.max(0.0))
            .sum::<f32>();
        if total_weight == 0.0 {
            return 0.0;
        }

        self.stages
            .iter()
            .map(|stage| stage.progress.clamp(0.0, 1.0) * stage.weight.max(0.0))
            .sum::<f32>()
            / total_weight
    }
}

impl Default for WeightedReloadProgress {
    fn default() -> Self {
        Self::simple_reload_instance(0.0, 0.0, 0.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WeightedReloadStageProgress {
    pub name: String,
    pub progress: f32,
    pub weight: f32,
}

impl WeightedReloadStageProgress {
    pub fn new(name: impl Into<String>, progress: f32, weight: f32) -> Self {
        Self {
            name: name.into(),
            progress,
            weight,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoadingOverlayAnchor {
    BottomLeft,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LoadingOverlayPanel<'a> {
    pub anchor: LoadingOverlayAnchor,
    pub stack_index: usize,
    pub offset_y: f32,
    pub width: f32,
    pub height: f32,
    pub progress_bar_height: f32,
    pub progress_bar_width: f32,
    pub task: &'a LoadingTask,
}

impl<'a> LoadingOverlayPanel<'a> {
    pub const WIDTH: f32 = 350.0;
    pub const HEIGHT: f32 = 32.0;
    pub const PROGRESS_BAR_HEIGHT: f32 = 10.0;

    fn new(stack_index: usize, task: &'a LoadingTask) -> Self {
        Self {
            anchor: LoadingOverlayAnchor::BottomLeft,
            stack_index,
            offset_y: stack_index as f32 * Self::HEIGHT,
            width: Self::WIDTH,
            height: Self::HEIGHT,
            progress_bar_height: Self::PROGRESS_BAR_HEIGHT,
            progress_bar_width: Self::WIDTH * task.progress,
            task,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LoadingTask {
    pub name: String,
    pub file: String,
    pub progress: f32,
    pub presentation_state: LoadingTaskPresentationState,
}

impl LoadingTask {
    pub fn new(name: impl Into<String>, file: impl Into<String>, progress: f32) -> Self {
        let progress = progress.clamp(0.0, 1.0);
        Self {
            name: name.into(),
            file: file.into(),
            progress,
            presentation_state: if progress >= 1.0 {
                LoadingTaskPresentationState::Finishing
            } else {
                LoadingTaskPresentationState::Active
            },
        }
    }

    pub fn finishing(name: impl Into<String>, file: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            file: file.into(),
            progress: 1.0,
            presentation_state: LoadingTaskPresentationState::Finishing,
        }
    }

    pub fn matches(&self, name: &str, file: &str) -> bool {
        self.name == name && self.file == file
    }

    fn has_same_identity_as(&self, other: &LoadingTask) -> bool {
        self.matches(&other.name, &other.file)
    }

    fn merge_progress_from(&mut self, task: LoadingTask) {
        self.progress = self.progress.max(task.progress);
        if self.progress >= 1.0 || task.is_finishing() {
            self.mark_finishing();
        }
    }

    fn mark_finishing(&mut self) {
        self.progress = 1.0;
        self.presentation_state = LoadingTaskPresentationState::Finishing;
    }

    pub fn is_finishing(&self) -> bool {
        self.presentation_state == LoadingTaskPresentationState::Finishing
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoadingTaskPresentationState {
    Active,
    Finishing,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceLoadingTask {
    pub name: String,
    pub file: String,
    pub progress: u64,
    pub total: u64,
}

impl ResourceLoadingTask {
    pub fn new(
        name: impl Into<String>,
        file: impl Into<String>,
        progress: u64,
        total: u64,
    ) -> Self {
        Self {
            name: name.into(),
            file: file.into(),
            progress,
            total,
        }
    }

    pub(super) fn into_loading_task(self) -> LoadingTask {
        let progress = if self.total == 0 {
            0.0
        } else {
            self.progress as f32 / self.total as f32
        };
        LoadingTask::new(self.name, self.file, progress)
    }
}

#[derive(Clone, Debug, Message, PartialEq, Eq)]
pub enum ResourceLoadingUpdate {
    TaskProgress(ResourceLoadingTask),
    TaskFinished { name: String, file: String },
    Complete,
}

impl ResourceLoadingUpdate {
    pub fn task_progress(
        name: impl Into<String>,
        file: impl Into<String>,
        progress: u64,
        total: u64,
    ) -> Self {
        Self::TaskProgress(ResourceLoadingTask::new(name, file, progress, total))
    }

    pub fn task_finished(name: impl Into<String>, file: impl Into<String>) -> Self {
        Self::TaskFinished {
            name: name.into(),
            file: file.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResourceLoadingEvent {
    Started {
        name: String,
        file: String,
    },
    Progress {
        name: String,
        file: String,
        progress: u64,
        total: u64,
    },
    Finished {
        name: String,
        file: String,
    },
    Complete,
}

impl ResourceLoadingEvent {
    pub fn started(name: impl Into<String>, file: impl Into<String>) -> Self {
        Self::Started {
            name: name.into(),
            file: file.into(),
        }
    }

    pub fn progress(
        name: impl Into<String>,
        file: impl Into<String>,
        progress: u64,
        total: u64,
    ) -> Self {
        Self::Progress {
            name: name.into(),
            file: file.into(),
            progress,
            total,
        }
    }

    pub fn finished(name: impl Into<String>, file: impl Into<String>) -> Self {
        Self::Finished {
            name: name.into(),
            file: file.into(),
        }
    }
}

pub mod loading_task_names {
    pub const DOWNLOADING_ASSET_INDEX: &str = "Downloading Asset Index";
    pub const DOWNLOADING_ASSETS: &str = "Downloading Assets";
    pub const DOWNLOADING_ASSET: &str = "Downloading Asset";
    pub const DOWNLOADING_CORE_ASSETS: &str = "Downloading Core Assets";
    pub const UNPACKING_CORE_ASSETS: &str = "Unpacking Core Assets";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn startup_shows_launcher_background_without_loading_overlay() {
        let flow = StartupFlow::new(vec![StoredLauncherAccount::offline("Alex")]);

        assert_eq!(
            flow.screen(),
            StartupScreen {
                background_visible: true,
                account_screen: &AccountFlowScreen::AccountSelection,
                loading_phase: StartupLoadingPhase::WaitingForTasks,
                loading_screen: StartupLoadingScreen::GenericMessage(
                    StartupGenericMessageView::loading_minecraft()
                ),
                startup_destination: None,
                loading_overlay: None,
            }
        );
    }

    #[test]
    fn startup_initially_exposes_vanilla_loading_minecraft_fallback_message() {
        let flow = StartupFlow::new(Vec::new());

        assert_eq!(
            flow.screen().loading_screen,
            StartupLoadingScreen::GenericMessage(StartupGenericMessageView {
                message: VanillaLoadingTextView::fallback_loading_minecraft(),
            })
        );
    }

    #[test]
    fn loading_tasks_overlay_launcher_instead_of_replacing_it() {
        let mut flow = StartupFlow::new(Vec::new());

        flow.replace_loading_tasks([LoadingTask::new(
            loading_task_names::DOWNLOADING_CORE_ASSETS,
            "client.jar",
            0.25,
        )]);

        let screen = flow.screen();
        assert!(screen.background_visible);
        assert_eq!(screen.account_screen, &AccountFlowScreen::AccountSelection);
        assert_eq!(screen.loading_phase, StartupLoadingPhase::Loading);
        assert_eq!(
            screen.loading_screen,
            StartupLoadingScreen::MojangLoadingOverlay(StartupMojangLoadingOverlaySurface {
                tasks: screen.loading_overlay,
            })
        );
        assert_eq!(
            screen.loading_overlay,
            Some(
                [LoadingTask::new(
                    loading_task_names::DOWNLOADING_CORE_ASSETS,
                    "client.jar",
                    0.25,
                )]
                .as_slice()
            )
        );
    }

    #[test]
    fn completing_loading_removes_only_the_overlay() {
        let mut flow = StartupFlow::new(Vec::new());
        flow.replace_loading_tasks([LoadingTask::new(
            loading_task_names::DOWNLOADING_ASSETS,
            "1.21.6.json",
            1.0,
        )]);

        flow.finish_loading();

        assert_eq!(flow.screen().loading_overlay, None);
        assert_eq!(flow.screen().loading_phase, StartupLoadingPhase::Complete);
        assert_eq!(
            flow.screen().loading_screen,
            StartupLoadingScreen::CompleteDestination(Some(StartupDestination::TitleMenu))
        );
        assert_eq!(
            flow.screen().startup_destination,
            Some(StartupDestination::TitleMenu)
        );
        assert_eq!(
            flow.screen().account_screen,
            &AccountFlowScreen::AccountSelection
        );
    }

    #[test]
    fn startup_destination_is_absent_until_loading_finishes() {
        let mut flow = StartupFlow::new(Vec::new());
        assert_eq!(flow.startup_destination(), None);

        flow.begin_loading();
        assert_eq!(flow.startup_destination(), None);

        flow.upsert_loading_task(LoadingTask::new(
            loading_task_names::DOWNLOADING_ASSET,
            "stone.png",
            0.5,
        ));
        assert_eq!(flow.screen().startup_destination, None);
    }

    #[test]
    fn resource_loading_complete_targets_main_menu_pending() {
        let mut flow = StartupFlow::new(Vec::new());
        flow.apply_resource_loading_update(ResourceLoadingUpdate::task_progress(
            loading_task_names::DOWNLOADING_CORE_ASSETS,
            "client.jar",
            1,
            4,
        ));

        flow.apply_resource_loading_update(ResourceLoadingUpdate::Complete);

        let screen = flow.screen();
        assert_eq!(screen.loading_overlay, None);
        assert_eq!(screen.loading_phase, StartupLoadingPhase::Complete);
        assert_eq!(
            screen.loading_screen,
            StartupLoadingScreen::CompleteDestination(Some(StartupDestination::TitleMenu))
        );
        assert_eq!(
            screen.startup_destination,
            Some(StartupDestination::TitleMenu)
        );
        assert_eq!(screen.account_screen, &AccountFlowScreen::AccountSelection);
    }

    #[test]
    fn finish_loading_can_target_quick_play_without_changing_account_flow() {
        let mut flow = StartupFlow::new(vec![StoredLauncherAccount::offline("Alex")]);
        flow.begin_loading();

        flow.finish_loading_to(StartupDestination::QuickPlay);

        let screen = flow.screen();
        assert_eq!(screen.loading_overlay, None);
        assert_eq!(screen.loading_phase, StartupLoadingPhase::Complete);
        assert_eq!(
            screen.loading_screen,
            StartupLoadingScreen::CompleteDestination(Some(StartupDestination::QuickPlay))
        );
        assert_eq!(
            screen.startup_destination,
            Some(StartupDestination::QuickPlay)
        );
        assert_eq!(screen.account_screen, &AccountFlowScreen::AccountSelection);
    }

    #[test]
    fn loading_progress_is_clamped_for_presentation() {
        assert_eq!(LoadingTask::new("task", "file", -0.5).progress, 0.0);
        assert_eq!(LoadingTask::new("task", "file", 1.5).progress, 1.0);
    }

    #[test]
    fn resource_loading_progress_is_normalized_for_presentation() {
        let mut flow = StartupFlow::new(Vec::new());

        flow.apply_resource_loading_update(ResourceLoadingUpdate::task_progress(
            loading_task_names::DOWNLOADING_ASSET_INDEX,
            "1.21.6.json",
            25,
            100,
        ));
        flow.apply_resource_loading_update(ResourceLoadingUpdate::task_progress(
            loading_task_names::DOWNLOADING_ASSET,
            "stone.png",
            1,
            0,
        ));
        flow.apply_resource_loading_update(ResourceLoadingUpdate::task_progress(
            loading_task_names::DOWNLOADING_CORE_ASSETS,
            "client.jar",
            5,
            4,
        ));

        assert_eq!(
            flow.loading_overlay(),
            Some(
                [
                    LoadingTask::new(
                        loading_task_names::DOWNLOADING_ASSET_INDEX,
                        "1.21.6.json",
                        0.25,
                    ),
                    LoadingTask::new(loading_task_names::DOWNLOADING_ASSET, "stone.png", 0.0),
                    LoadingTask::finishing(
                        loading_task_names::DOWNLOADING_CORE_ASSETS,
                        "client.jar",
                    ),
                ]
                .as_slice()
            )
        );
    }

    #[test]
    fn updating_the_same_loading_task_does_not_duplicate_the_panel() {
        let mut flow = StartupFlow::new(Vec::new());

        flow.upsert_loading_task(LoadingTask::new(
            loading_task_names::DOWNLOADING_ASSET,
            "stone.png",
            0.25,
        ));
        flow.upsert_loading_task(LoadingTask::new(
            loading_task_names::DOWNLOADING_ASSET,
            "stone.png",
            0.75,
        ));

        assert_eq!(
            flow.loading_overlay(),
            Some(
                [LoadingTask::new(
                    loading_task_names::DOWNLOADING_ASSET,
                    "stone.png",
                    0.75,
                )]
                .as_slice()
            )
        );
    }

    #[test]
    fn updating_the_same_loading_task_only_grows_progress() {
        let mut flow = StartupFlow::new(Vec::new());

        flow.upsert_loading_task(LoadingTask::new(
            loading_task_names::DOWNLOADING_ASSET,
            "stone.png",
            0.75,
        ));
        flow.upsert_loading_task(LoadingTask::new(
            loading_task_names::DOWNLOADING_ASSET,
            "stone.png",
            0.25,
        ));

        assert_eq!(
            flow.loading_overlay(),
            Some(
                [LoadingTask::new(
                    loading_task_names::DOWNLOADING_ASSET,
                    "stone.png",
                    0.75,
                )]
                .as_slice()
            )
        );
    }

    #[test]
    fn reaching_full_progress_keeps_a_finishing_panel_until_presentation_advances() {
        let mut flow = StartupFlow::new(Vec::new());

        flow.upsert_loading_task(LoadingTask::new(
            loading_task_names::DOWNLOADING_ASSET,
            "stone.png",
            0.5,
        ));
        flow.upsert_loading_task(LoadingTask::new(
            loading_task_names::DOWNLOADING_ASSET,
            "stone.png",
            1.0,
        ));

        assert_eq!(
            flow.loading_overlay(),
            Some(
                [LoadingTask::finishing(
                    loading_task_names::DOWNLOADING_ASSET,
                    "stone.png",
                )]
                .as_slice()
            )
        );
        assert!(flow.is_loading());

        flow.advance_loading_presentation();

        assert_eq!(flow.loading_overlay(), None);
        assert!(flow.is_loading());
    }

    #[test]
    fn finishing_unknown_loading_task_creates_finishing_panel() {
        let mut flow = StartupFlow::new(Vec::new());

        let finished_task =
            flow.finish_loading_task(loading_task_names::DOWNLOADING_ASSET, "unknown.png");

        assert_eq!(
            finished_task,
            LoadingTask::finishing(loading_task_names::DOWNLOADING_ASSET, "unknown.png")
        );
        assert_eq!(
            flow.loading_overlay(),
            Some(
                [LoadingTask::finishing(
                    loading_task_names::DOWNLOADING_ASSET,
                    "unknown.png",
                )]
                .as_slice()
            )
        );
        assert!(flow.is_loading());
    }

    #[test]
    fn removing_the_last_loading_task_leaves_loading_active_until_completion_signal() {
        let mut flow = StartupFlow::new(Vec::new());
        flow.upsert_loading_task(LoadingTask::new(
            loading_task_names::UNPACKING_CORE_ASSETS,
            "client.jar",
            0.5,
        ));

        let removed_task =
            flow.remove_loading_task(loading_task_names::UNPACKING_CORE_ASSETS, "client.jar");

        assert_eq!(
            removed_task,
            Some(LoadingTask::new(
                loading_task_names::UNPACKING_CORE_ASSETS,
                "client.jar",
                0.5,
            ))
        );
        assert!(flow.is_loading());
        assert_eq!(flow.loading_overlay(), None);
    }

    #[test]
    fn removing_one_of_multiple_loading_tasks_keeps_loading_active() {
        let mut flow = StartupFlow::new(Vec::new());
        flow.replace_loading_tasks([
            LoadingTask::new(
                loading_task_names::DOWNLOADING_ASSET_INDEX,
                "1.21.6.json",
                0.5,
            ),
            LoadingTask::new(loading_task_names::DOWNLOADING_ASSET, "grass.png", 0.25),
        ]);

        flow.remove_loading_task(loading_task_names::DOWNLOADING_ASSET_INDEX, "1.21.6.json");

        assert!(flow.is_loading());
        assert_eq!(
            flow.loading_overlay(),
            Some(
                [LoadingTask::new(
                    loading_task_names::DOWNLOADING_ASSET,
                    "grass.png",
                    0.25,
                )]
                .as_slice()
            )
        );
    }

    #[test]
    fn empty_task_snapshot_does_not_complete_loading_without_completion_signal() {
        let mut flow = StartupFlow::new(Vec::new());

        flow.begin_loading();
        flow.replace_loading_tasks([]);

        assert!(flow.is_loading());
        assert_eq!(flow.loading_overlay(), None);
    }

    #[test]
    fn clearing_overlay_keeps_current_loading_phase() {
        let mut flow = StartupFlow::new(Vec::new());
        flow.upsert_loading_task(LoadingTask::new(
            loading_task_names::DOWNLOADING_CORE_ASSETS,
            "client.jar",
            0.5,
        ));

        flow.clear_loading_tasks();

        assert!(flow.is_loading());
        assert_eq!(flow.loading_overlay(), None);
    }

    #[test]
    fn loading_panels_expose_stevenarella_bottom_left_stack_geometry() {
        let mut flow = StartupFlow::new(Vec::new());
        flow.replace_loading_tasks([
            LoadingTask::new(
                loading_task_names::DOWNLOADING_ASSET_INDEX,
                "1.21.6.json",
                0.25,
            ),
            LoadingTask::new(
                loading_task_names::DOWNLOADING_ASSET,
                "minecraft/textures/block/stone.png",
                0.5,
            ),
        ]);

        let screen = flow.screen();
        let panels = screen.loading_panels();

        assert_eq!(panels.len(), 2);
        assert_eq!(panels[0].anchor, LoadingOverlayAnchor::BottomLeft);
        assert_eq!(panels[0].stack_index, 0);
        assert_eq!(panels[0].offset_y, 0.0);
        assert_eq!(panels[0].width, 350.0);
        assert_eq!(panels[0].height, 32.0);
        assert_eq!(panels[0].progress_bar_height, 10.0);
        assert_eq!(panels[0].progress_bar_width, 87.5);
        assert_eq!(panels[1].stack_index, 1);
        assert_eq!(panels[1].offset_y, 32.0);
        assert_eq!(panels[1].progress_bar_width, 175.0);
    }

    #[test]
    fn loading_panels_expose_task_presentation_state() {
        let mut flow = StartupFlow::new(Vec::new());
        flow.replace_loading_tasks([
            LoadingTask::new(loading_task_names::DOWNLOADING_ASSET, "stone.png", 0.25),
            LoadingTask::finishing(loading_task_names::UNPACKING_CORE_ASSETS, "client.jar"),
        ]);

        let screen = flow.screen();
        let panels = screen.loading_panels();

        assert_eq!(
            panels[0].task.presentation_state,
            LoadingTaskPresentationState::Active
        );
        assert_eq!(
            panels[1].task.presentation_state,
            LoadingTaskPresentationState::Finishing
        );
    }

    #[test]
    fn vanilla_overlay_view_exposes_centered_mojang_loading_minecraft_concepts() {
        let overlay = VanillaLoadingOverlay::new();

        assert_eq!(VanillaLoadingOverlay::FADE_IN, Duration::from_millis(500));
        assert_eq!(
            VanillaLoadingOverlay::FADE_OUT,
            Duration::from_millis(1_000)
        );

        let view = overlay.view(0.25, Duration::from_millis(250));
        assert_eq!(view.background, VanillaLoadingBackground::Red);
        assert_eq!(
            view.logo,
            VanillaLoadingLogoView {
                kind: VanillaLoadingLogoKind::MojangStudios,
                placement: VanillaLoadingLogoPlacement::Centered,
                split_texture_halves: true,
            }
        );
        assert_eq!(view.text.text, "Loading Minecraft");
        assert_eq!(view.actual_progress, 0.25);
        assert_eq!(view.displayed_progress, 0.0);
        assert_eq!(view.fade_alpha, 0.5);
        assert!(view.should_render);
    }

    #[test]
    fn vanilla_overlay_supports_black_background_option() {
        let overlay = VanillaLoadingOverlay::new().with_background(VanillaLoadingBackground::Black);

        assert_eq!(
            overlay.view(0.0, Duration::ZERO).background,
            VanillaLoadingBackground::Black
        );
    }

    #[test]
    fn vanilla_overlay_progress_bar_uses_displayed_progress_for_stable_geometry() {
        let mut overlay = VanillaLoadingOverlay::new();

        let zero = overlay.view(1.0, Duration::ZERO).progress_bar;
        assert_eq!(
            zero,
            VanillaLoadingProgressBarView {
                placement: VanillaLoadingProgressBarPlacement::CenteredBottom,
                color: VanillaLoadingProgressBarColor::White,
                center_x_fraction: 0.5,
                center_y_fraction: 0.8325,
                outer_height: 10.0,
                border_width: 1.0,
                fill_inset: 2.0,
                displayed_progress: 0.0,
            }
        );
        let zero_outer = zero.outer_rect(1280.0, 960.0);
        assert_eq!(zero_outer.x, 160.0);
        assert!((zero_outer.y - 794.2).abs() < 0.001);
        assert_eq!(zero_outer.width, 960.0);
        assert_eq!(zero_outer.height, 10.0);

        let zero_fill = zero.fill_rect(1280.0, 960.0);
        assert_eq!(zero_fill.x, 162.0);
        assert!((zero_fill.y - 796.2).abs() < 0.001);
        assert_eq!(zero_fill.width, 0.0);
        assert_eq!(zero_fill.height, 6.0);

        overlay.displayed_progress = 0.5;
        let half = overlay.view(1.0, Duration::ZERO).progress_bar;
        assert_eq!(half.outer_height, zero.outer_height);
        assert_eq!(half.center_x_fraction, zero.center_x_fraction);
        assert_eq!(half.center_y_fraction, zero.center_y_fraction);
        assert_eq!(half.fill_rect(1280.0, 960.0).width, 478.0);

        overlay.displayed_progress = 1.0;
        let full = overlay.view(1.0, Duration::ZERO).progress_bar;
        assert_eq!(full.outer_height, zero.outer_height);
        assert_eq!(full.fill_rect(1280.0, 960.0).width, 956.0);
        let wide_outer = full.outer_rect(1920.0, 1080.0);
        assert_eq!(wide_outer.x, 420.0);
        assert!((wide_outer.y - 894.1).abs() < 0.001);
        assert_eq!(wide_outer.width, 1080.0);
        assert_eq!(wide_outer.height, 10.0);
    }

    #[test]
    fn vanilla_overlay_progress_bar_does_not_change_background_text_or_logo() {
        let mut overlay =
            VanillaLoadingOverlay::new().with_background(VanillaLoadingBackground::Black);
        overlay.displayed_progress = 1.0;

        let view = overlay.view(0.0, Duration::ZERO);

        assert_eq!(view.background, VanillaLoadingBackground::Black);
        assert_eq!(
            view.logo,
            VanillaLoadingLogoView {
                kind: VanillaLoadingLogoKind::MojangStudios,
                placement: VanillaLoadingLogoPlacement::Centered,
                split_texture_halves: true,
            }
        );
        assert_eq!(
            view.text,
            VanillaLoadingTextView::fallback_loading_minecraft()
        );
    }

    #[test]
    fn vanilla_overlay_smooths_displayed_progress_like_loading_overlay() {
        let mut overlay = VanillaLoadingOverlay::new();

        overlay.tick(1.0);
        overlay.tick(1.0);

        assert!((overlay.displayed_progress() - 0.0975).abs() < f32::EPSILON);
    }

    #[test]
    fn vanilla_overlay_fades_in_and_fades_out_after_completion() {
        let mut overlay = VanillaLoadingOverlay::new();

        assert_eq!(overlay.fade_alpha(Duration::ZERO), 0.0);
        assert_eq!(overlay.fade_alpha(VanillaLoadingOverlay::FADE_IN / 2), 0.5);
        assert_eq!(overlay.fade_alpha(VanillaLoadingOverlay::FADE_IN), 1.0);

        overlay.update_fade_out(StartupLoadingPhase::Loading, Duration::from_millis(900));
        assert_eq!(overlay.fade_out_start(), None);

        overlay.update_fade_out_when_ready(
            StartupLoadingPhase::Complete,
            Duration::from_millis(1_000),
            false,
        );
        assert_eq!(overlay.fade_out_start(), None);

        overlay.update_fade_out_when_ready(
            StartupLoadingPhase::Complete,
            Duration::from_millis(1_200),
            true,
        );
        assert_eq!(overlay.fade_out_start(), Some(Duration::from_millis(1_200)));
        assert_eq!(
            overlay.fade_alpha(Duration::from_millis(1_200) + VanillaLoadingOverlay::FADE_OUT / 2),
            0.5
        );
        assert!(overlay.should_render(
            Duration::from_millis(1_200) + VanillaLoadingOverlay::FADE_OUT * 2
                - Duration::from_millis(1)
        ));
        assert!(
            overlay.should_render(Duration::from_millis(1_200) + VanillaLoadingOverlay::FADE_OUT)
        );
        assert!(
            !overlay
                .should_render(Duration::from_millis(1_200) + VanillaLoadingOverlay::FADE_OUT * 2)
        );
    }

    #[test]
    fn weighted_reload_progress_uses_simple_reload_instance_weights() {
        let progress = WeightedReloadProgress::simple_reload_instance(1.0, 0.5, 0.25);

        assert!((progress.actual_progress() - 0.65).abs() < f32::EPSILON);
    }

    #[test]
    fn weighted_reload_progress_clamps_progress_and_ignores_negative_weights() {
        let progress = WeightedReloadProgress::new([
            WeightedReloadStageProgress::new("over", 2.0, 2.0),
            WeightedReloadStageProgress::new("negative", 1.0, -10.0),
            WeightedReloadStageProgress::new("under", -1.0, 2.0),
        ]);

        assert_eq!(progress.actual_progress(), 0.5);
    }

    #[test]
    fn resource_loading_update_upserts_the_same_task() {
        let mut flow = StartupFlow::new(Vec::new());

        flow.apply_resource_loading_update(ResourceLoadingUpdate::task_progress(
            loading_task_names::DOWNLOADING_ASSET,
            "stone.png",
            25,
            100,
        ));
        flow.apply_resource_loading_update(ResourceLoadingUpdate::task_progress(
            loading_task_names::DOWNLOADING_ASSET,
            "stone.png",
            75,
            100,
        ));

        assert_eq!(
            flow.loading_overlay(),
            Some(
                [LoadingTask::new(
                    loading_task_names::DOWNLOADING_ASSET,
                    "stone.png",
                    0.75,
                )]
                .as_slice()
            )
        );
    }

    #[test]
    fn resource_loading_event_starts_progress_finishes_and_completes_without_demo_timeline() {
        let mut flow = StartupFlow::new(Vec::new());

        flow.apply_resource_loading_event(ResourceLoadingEvent::started(
            loading_task_names::DOWNLOADING_ASSET_INDEX,
            "1.21.6.json",
        ));
        flow.apply_resource_loading_event(ResourceLoadingEvent::progress(
            loading_task_names::DOWNLOADING_ASSET_INDEX,
            "1.21.6.json",
            25,
            100,
        ));
        flow.apply_resource_loading_event(ResourceLoadingEvent::finished(
            loading_task_names::DOWNLOADING_ASSET_INDEX,
            "1.21.6.json",
        ));

        assert_eq!(
            flow.loading_overlay(),
            Some(
                [LoadingTask::finishing(
                    loading_task_names::DOWNLOADING_ASSET_INDEX,
                    "1.21.6.json",
                )]
                .as_slice()
            )
        );
        assert!(flow.is_loading());

        flow.advance_loading_presentation();
        flow.apply_resource_loading_event(ResourceLoadingEvent::Complete);

        assert_eq!(flow.loading_overlay(), None);
        assert!(flow.loading_is_complete());
    }

    #[test]
    fn resource_loading_update_keeps_finished_task_until_presentation_advances() {
        let mut flow = StartupFlow::new(Vec::new());
        flow.apply_resource_loading_update(ResourceLoadingUpdate::task_progress(
            loading_task_names::DOWNLOADING_ASSET_INDEX,
            "1.21.6.json",
            50,
            100,
        ));
        flow.apply_resource_loading_update(ResourceLoadingUpdate::task_progress(
            loading_task_names::DOWNLOADING_ASSET,
            "grass.png",
            25,
            100,
        ));

        flow.apply_resource_loading_update(ResourceLoadingUpdate::task_finished(
            loading_task_names::DOWNLOADING_ASSET_INDEX,
            "1.21.6.json",
        ));

        assert!(flow.is_loading());
        assert_eq!(
            flow.loading_overlay(),
            Some(
                [
                    LoadingTask::finishing(
                        loading_task_names::DOWNLOADING_ASSET_INDEX,
                        "1.21.6.json",
                    ),
                    LoadingTask::new(loading_task_names::DOWNLOADING_ASSET, "grass.png", 0.25),
                ]
                .as_slice()
            )
        );

        flow.advance_loading_presentation();

        assert_eq!(
            flow.loading_overlay(),
            Some(
                [LoadingTask::new(
                    loading_task_names::DOWNLOADING_ASSET,
                    "grass.png",
                    0.25,
                )]
                .as_slice()
            )
        );
    }

    #[test]
    fn resource_loading_complete_finishes_the_overlay() {
        let mut flow = StartupFlow::new(Vec::new());
        flow.apply_resource_loading_update(ResourceLoadingUpdate::task_progress(
            loading_task_names::UNPACKING_CORE_ASSETS,
            "client.jar",
            1,
            4,
        ));
        flow.apply_resource_loading_update(ResourceLoadingUpdate::task_finished(
            loading_task_names::DOWNLOADING_ASSET,
            "grass.png",
        ));

        flow.apply_resource_loading_update(ResourceLoadingUpdate::Complete);

        assert_eq!(flow.loading_overlay(), None);
        assert_eq!(flow.screen().loading_panels(), Vec::new());
        assert!(flow.loading_is_complete());
    }
}

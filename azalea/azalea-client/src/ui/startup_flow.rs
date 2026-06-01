//! Startup screen flow state.
//!
//! This keeps Stevenarella's startup storyboard visible to presentation code:
//! background and launcher are available immediately, while resource loading
//! appears as an overlay instead of a blocking screen.

use super::account_flow::{AccountFlow, AccountFlowScreen, StoredLauncherAccount};

#[derive(Clone, Debug)]
pub struct StartupFlow {
    account_flow: AccountFlow,
    loading_phase: StartupLoadingPhase,
    loading_tasks: Vec<LoadingTask>,
}

impl StartupFlow {
    pub fn new(accounts: Vec<StoredLauncherAccount>) -> Self {
        Self {
            account_flow: AccountFlow::new(accounts),
            loading_phase: StartupLoadingPhase::WaitingForTasks,
            loading_tasks: Vec::new(),
        }
    }

    pub fn screen(&self) -> StartupScreen<'_> {
        StartupScreen {
            background_visible: true,
            account_screen: self.account_flow.screen(),
            loading_phase: self.loading_phase,
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

    pub fn is_loading(&self) -> bool {
        self.loading_phase == StartupLoadingPhase::Loading
    }

    pub fn loading_is_complete(&self) -> bool {
        self.loading_phase == StartupLoadingPhase::Complete
    }

    pub fn begin_loading(&mut self) {
        if !self.loading_is_complete() {
            self.loading_phase = StartupLoadingPhase::Loading;
        }
    }

    pub fn replace_loading_tasks(&mut self, tasks: impl IntoIterator<Item = LoadingTask>) {
        self.loading_tasks = tasks.into_iter().collect();
        if !self.loading_tasks.is_empty() {
            self.loading_phase = StartupLoadingPhase::Loading;
        }
    }

    pub fn upsert_loading_task(&mut self, task: LoadingTask) {
        if let Some(existing_task) = self
            .loading_tasks
            .iter_mut()
            .find(|existing_task| existing_task.has_same_identity_as(&task))
        {
            *existing_task = task;
        } else {
            self.loading_tasks.push(task);
        }
        self.loading_phase = StartupLoadingPhase::Loading;
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

    pub fn finish_loading(&mut self) {
        self.loading_tasks.clear();
        self.loading_phase = StartupLoadingPhase::Complete;
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
                self.remove_loading_task(name, file);
            }
            ResourceLoadingUpdate::Complete => {
                self.finish_loading();
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StartupScreen<'a> {
    pub background_visible: bool,
    pub account_screen: &'a AccountFlowScreen,
    pub loading_phase: StartupLoadingPhase,
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
}

impl LoadingTask {
    pub fn new(name: impl Into<String>, file: impl Into<String>, progress: f32) -> Self {
        Self {
            name: name.into(),
            file: file.into(),
            progress: progress.clamp(0.0, 1.0),
        }
    }

    pub fn matches(&self, name: &str, file: &str) -> bool {
        self.name == name && self.file == file
    }

    fn has_same_identity_as(&self, other: &LoadingTask) -> bool {
        self.matches(&other.name, &other.file)
    }
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

    fn into_loading_task(self) -> LoadingTask {
        let progress = if self.total == 0 {
            0.0
        } else {
            self.progress as f32 / self.total as f32
        };
        LoadingTask::new(self.name, self.file, progress)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
                loading_overlay: None,
            }
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
            flow.screen().account_screen,
            &AccountFlowScreen::AccountSelection
        );
    }

    #[test]
    fn loading_progress_is_clamped_for_presentation() {
        assert_eq!(LoadingTask::new("task", "file", -0.5).progress, 0.0);
        assert_eq!(LoadingTask::new("task", "file", 1.5).progress, 1.0);
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
    fn resource_loading_update_removes_finished_task() {
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

        flow.apply_resource_loading_update(ResourceLoadingUpdate::Complete);

        assert_eq!(flow.loading_overlay(), None);
        assert!(flow.loading_is_complete());
    }
}

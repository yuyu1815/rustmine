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

    pub fn replace_loading_tasks(&mut self, tasks: impl IntoIterator<Item = LoadingTask>) {
        self.loading_tasks = tasks.into_iter().collect();
        self.loading_phase = if self.loading_tasks.is_empty() {
            StartupLoadingPhase::Complete
        } else {
            StartupLoadingPhase::Loading
        };
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
        if self.loading_tasks.is_empty() {
            self.loading_phase = StartupLoadingPhase::Complete;
        }
        Some(removed_task)
    }

    pub fn clear_loading_tasks(&mut self) {
        self.loading_tasks.clear();
        self.loading_phase = StartupLoadingPhase::Complete;
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StartupScreen<'a> {
    pub background_visible: bool,
    pub account_screen: &'a AccountFlowScreen,
    pub loading_phase: StartupLoadingPhase,
    pub loading_overlay: Option<&'a [LoadingTask]>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StartupLoadingPhase {
    WaitingForTasks,
    Loading,
    Complete,
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

        flow.clear_loading_tasks();

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
    fn removing_the_last_loading_task_completes_loading() {
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
        assert!(flow.loading_is_complete());
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
}

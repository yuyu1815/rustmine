//! Resource loading adapter for startup UI flow state.
//!
//! A future real resource loader can report task updates here without knowing
//! how the startup screen lays out loading panels.

use super::{
    account_flow::StoredLauncherAccount,
    startup_flow::{
        ResourceLoadingEvent, ResourceLoadingUpdate, StartupFlow, StartupLoadingPhase,
        StartupScreen, WeightedReloadProgress,
    },
};

#[derive(Clone, Debug)]
pub struct ResourceLoadingTracker {
    flow: StartupFlow,
}

impl ResourceLoadingTracker {
    pub fn new(accounts: Vec<StoredLauncherAccount>) -> Self {
        Self {
            flow: StartupFlow::new(accounts),
        }
    }

    pub fn from_flow(flow: StartupFlow) -> Self {
        Self { flow }
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

    pub fn advance_presentation(&mut self) {
        self.flow.advance_loading_presentation();
    }

    pub fn mark_complete(&mut self) {
        self.flow.finish_loading();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::startup_flow::{
        LoadingTask, LoadingTaskPresentationState, StartupLoadingPhase, loading_task_names,
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
}

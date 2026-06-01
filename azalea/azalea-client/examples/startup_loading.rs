use azalea_client::ui::{
    resource_loading::ResourceLoadingTracker,
    startup_flow::{
        LoadingTaskPresentationState, ResourceLoadingUpdate, StartupLoadingPhase,
        loading_task_names,
    },
};

fn main() {
    let mut tracker = ResourceLoadingTracker::new(Vec::new());

    show("startup", &tracker);

    tracker.apply_update(ResourceLoadingUpdate::task_progress(
        loading_task_names::DOWNLOADING_ASSET_INDEX,
        "1.21.6.json",
        1,
        4,
    ));
    show("progress", &tracker);

    tracker.apply_update(ResourceLoadingUpdate::task_progress(
        loading_task_names::DOWNLOADING_ASSET_INDEX,
        "1.21.6.json",
        4,
        4,
    ));
    show("finished by progress", &tracker);

    tracker.advance_presentation();
    show("presentation advance", &tracker);

    tracker.apply_update(ResourceLoadingUpdate::task_progress(
        loading_task_names::UNPACKING_CORE_ASSETS,
        "client.jar",
        2,
        3,
    ));
    show("next progress", &tracker);

    tracker.apply_update(ResourceLoadingUpdate::task_finished(
        loading_task_names::UNPACKING_CORE_ASSETS,
        "client.jar",
    ));
    show("explicit finishing", &tracker);

    tracker.advance_presentation();
    show("clear finishing panel", &tracker);

    tracker.apply_update(ResourceLoadingUpdate::Complete);
    show("complete", &tracker);

    assert_eq!(
        tracker.screen().loading_phase,
        StartupLoadingPhase::Complete
    );
}

fn show(label: &str, tracker: &ResourceLoadingTracker) {
    let screen = tracker.screen();
    println!();
    println!("== {label} ==");
    println!(
        "background: {} | phase: {:?} | overlay panels: {}",
        if screen.background_visible {
            "visible"
        } else {
            "hidden"
        },
        screen.loading_phase,
        screen.loading_panels().len()
    );

    for panel in screen.loading_panels() {
        println!(
            "  [{:>3}%] {:<24} {:<12} {}",
            (panel.task.progress * 100.0).round() as u8,
            panel.task.name,
            presentation_state(panel.task.presentation_state),
            panel.task.file
        );
    }
}

fn presentation_state(state: LoadingTaskPresentationState) -> &'static str {
    match state {
        LoadingTaskPresentationState::Active => "active",
        LoadingTaskPresentationState::Finishing => "finishing",
    }
}

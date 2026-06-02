//! Resource loading adapter for startup UI flow state.
//!
//! A future real resource loader can report task updates here without knowing
//! how the startup screen lays out loading panels.

use std::{path::PathBuf, time::Duration};

use super::{
    account_flow::StoredLauncherAccount,
    startup_flow::{
        ResourceLoadingEvent, ResourceLoadingUpdate, StartupDestination, StartupFlow,
        StartupLoadingPhase, StartupScreen, VanillaLoadingBackground, VanillaLoadingLogoView,
        VanillaLoadingOverlay, VanillaLoadingOverlayView, VanillaLoadingProgressBarView,
        VanillaLoadingTextView, WeightedReloadProgress, WeightedReloadStageProgress,
    },
};
use crate::resources::{
    ClientResourceReloadManagerRuntimeReport, ClientResourceReloadManagerRuntimeStatus,
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
    startup_destination: StartupDestination,
    resource_reload_snapshot: Option<ResourceReloadProgressSnapshot>,
    resource_reload_manager_report: Option<ClientResourceReloadManagerRuntimeReport>,
    resource_reload_error: Option<String>,
    pack_load_failure_toast: Option<ResourceLoadingPackLoadFailureToast>,
}

impl ResourceLoadingTracker {
    pub fn new(accounts: Vec<StoredLauncherAccount>) -> Self {
        Self {
            flow: StartupFlow::new(accounts),
            startup_destination: StartupDestination::TitleMenu,
            resource_reload_snapshot: None,
            resource_reload_manager_report: None,
            resource_reload_error: None,
            pack_load_failure_toast: None,
        }
    }

    pub fn from_flow(flow: StartupFlow) -> Self {
        Self {
            flow,
            startup_destination: StartupDestination::TitleMenu,
            resource_reload_snapshot: None,
            resource_reload_manager_report: None,
            resource_reload_error: None,
            pack_load_failure_toast: None,
        }
    }

    pub fn with_startup_destination(mut self, destination: StartupDestination) -> Self {
        self.startup_destination = destination;
        self
    }

    pub fn startup_destination(&self) -> StartupDestination {
        self.startup_destination
    }

    pub fn set_startup_destination(&mut self, destination: StartupDestination) {
        self.startup_destination = destination;
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

    pub fn resource_reload_manager_report(
        &self,
    ) -> Option<&ClientResourceReloadManagerRuntimeReport> {
        self.resource_reload_manager_report.as_ref()
    }

    pub fn resource_reload_manager_view(&self) -> Option<ResourceLoadingManagerRuntimeView> {
        self.resource_reload_manager_report
            .as_ref()
            .map(ResourceLoadingManagerRuntimeView::from)
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
            logo_texture_state: MojangLogoTextureState::default(),
            reload: self.reload_view(),
            manager_runtime: self.resource_reload_manager_view(),
            smoothing: LoadingOverlaySmoothing::vanilla(),
            fade: LoadingOverlayFadeTiming::vanilla(),
            vanilla,
        }
    }

    pub fn loading_overlay_view_with_logo_state(
        &self,
        overlay: &VanillaLoadingOverlay,
        elapsed: Duration,
        logo_texture_state: MojangLogoTextureState,
    ) -> MojangLoadingOverlayViewModel {
        let mut view = self.loading_overlay_view(overlay, elapsed);
        view.apply_logo_texture_state(logo_texture_state);
        view
    }

    pub fn loading_overlay_view_with_resource_stack(
        &self,
        overlay: &VanillaLoadingOverlay,
        elapsed: Duration,
        stack: &ClientResourceStack,
    ) -> MojangLoadingOverlayViewModel {
        self.loading_overlay_view_with_logo_state(
            overlay,
            elapsed,
            MojangLogoTextureState::resolve(stack),
        )
    }

    pub fn loading_overlay_view_with_resource_repository(
        &self,
        overlay: &VanillaLoadingOverlay,
        elapsed: Duration,
        repository: &ClientResourceRepository,
    ) -> MojangLoadingOverlayViewModel {
        self.loading_overlay_view_with_resource_stack(overlay, elapsed, &repository.stack())
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
        match event {
            ResourceLoadingEvent::Complete => self.mark_complete(),
            event => self.flow.apply_resource_loading_event(event),
        }
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

    pub fn track_pending_resource_reload_manager(&mut self, manager: &ResourceReloadManager) {
        self.resource_reload_manager_report = Some(manager.runtime_report());
    }

    pub fn run_resource_reload(
        &mut self,
        manager: &ResourceReloadManager,
    ) -> ResourceReloadResult<ResourceReloadReport> {
        self.track_pending_resource_reload_manager(manager);
        let report = manager.run_with_events(|event| self.apply_resource_reload_event(event));

        match &report {
            Ok(completed_report) => {
                self.resource_reload_manager_report =
                    Some(ClientResourceReloadManagerRuntimeReport::from_completed(
                        manager,
                        completed_report,
                    ));
                self.mark_complete();
            }
            Err(reload_error) => {
                let error = reload_error.to_string();
                self.resource_reload_manager_report = Some(
                    ClientResourceReloadManagerRuntimeReport::from_failure(manager, reload_error),
                );
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
        self.mark_complete_to(self.startup_destination);
    }

    pub fn mark_complete_to(&mut self, destination: StartupDestination) {
        self.resource_reload_error = None;
        self.pack_load_failure_toast = None;
        self.startup_destination = destination;
        self.flow.finish_loading_to(destination);
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
    pub logo_texture_state: MojangLogoTextureState,
    pub reload: ResourceLoadingReloadView,
    pub manager_runtime: Option<ResourceLoadingManagerRuntimeView>,
    pub smoothing: LoadingOverlaySmoothing,
    pub fade: LoadingOverlayFadeTiming,
}

impl MojangLoadingOverlayViewModel {
    pub fn frame(&self, gui_width: f32, gui_height: f32) -> MojangLoadingOverlayFrame {
        MojangLoadingOverlayFrame::from_view_model(self, gui_width, gui_height)
    }

    pub fn draw_list(&self, gui_width: f32, gui_height: f32) -> MojangLoadingOverlayDrawList {
        MojangLoadingOverlayDrawList::from_view_model(self, gui_width, gui_height)
    }

    pub fn apply_logo_texture_state(&mut self, logo_texture_state: MojangLogoTextureState) {
        self.logo_texture = logo_texture_state.texture();
        self.logo_texture_state = logo_texture_state;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MojangLoadingOverlayFrame {
    pub background_argb: u32,
    pub fade_alpha: f32,
    pub should_render: bool,
    pub progress: MojangLoadingProgress,
    pub logo: MojangLogoPlacement,
    pub progress_bar: MojangProgressBarFrame,
}

impl MojangLoadingOverlayFrame {
    pub fn from_view_model(
        view: &MojangLoadingOverlayViewModel,
        gui_width: f32,
        gui_height: f32,
    ) -> Self {
        let logo = MojangLogoPlacement::from_view_model(view, gui_width, gui_height);
        let progress = MojangLoadingProgress::new(
            view.vanilla.actual_progress,
            view.vanilla.displayed_progress,
        );
        Self {
            background_argb: view.background_argb,
            fade_alpha: view.vanilla.fade_alpha.clamp(0.0, 1.0),
            should_render: view.vanilla.should_render,
            progress,
            logo,
            progress_bar: MojangProgressBarFrame::from_logo(
                &logo,
                gui_height,
                progress.clamped_displayed_progress,
                view.vanilla.progress_bar.border_width,
            ),
        }
    }

    pub fn draw_list(&self, gui_width: f32, gui_height: f32) -> MojangLoadingOverlayDrawList {
        MojangLoadingOverlayDrawList::from_frame(self, gui_width, gui_height)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MojangLoadingOverlayDrawList {
    pub commands: Vec<MojangLoadingOverlayDrawCommand>,
}

impl MojangLoadingOverlayDrawList {
    pub fn from_view_model(
        view: &MojangLoadingOverlayViewModel,
        gui_width: f32,
        gui_height: f32,
    ) -> Self {
        let frame = view.frame(gui_width, gui_height);
        Self::from_frame_with_text(
            &frame,
            gui_width,
            gui_height,
            Some(MojangLoadingOverlayTextCommand {
                text: view.reload.label.clone(),
                fallback_text: view.vanilla.text.text,
                phase: view.reload.phase,
                color_argb: MOJANG_LOADING_OVERLAY_WHITE_ARGB,
            }),
        )
    }

    pub fn from_frame(frame: &MojangLoadingOverlayFrame, gui_width: f32, gui_height: f32) -> Self {
        Self::from_frame_with_text(frame, gui_width, gui_height, None)
    }

    fn from_frame_with_text(
        frame: &MojangLoadingOverlayFrame,
        gui_width: f32,
        gui_height: f32,
        text: Option<MojangLoadingOverlayTextCommand>,
    ) -> Self {
        let mut commands = Vec::with_capacity(if text.is_some() { 6 } else { 5 });
        commands.push(MojangLoadingOverlayDrawCommand::BackgroundFill {
            rect: MojangLoadingRect {
                x: 0.0,
                y: 0.0,
                width: gui_width,
                height: gui_height,
            },
            color_argb: frame.background_argb,
            fade_alpha: frame.fade_alpha,
            should_render: frame.should_render,
        });

        match frame.logo.texture {
            Some(texture) => {
                commands.push(MojangLoadingOverlayDrawCommand::LogoTextureHalf {
                    texture,
                    half: MojangLogoHalf::Left,
                    source_rect: frame.logo.left_half.source_rect,
                    target_rect: frame.logo.left_half.target_rect,
                    fade_alpha: frame.fade_alpha,
                });
                commands.push(MojangLoadingOverlayDrawCommand::LogoTextureHalf {
                    texture,
                    half: MojangLogoHalf::Right,
                    source_rect: frame.logo.right_half.source_rect,
                    target_rect: frame.logo.right_half.target_rect,
                    fade_alpha: frame.fade_alpha,
                });
            }
            None => commands.push(MojangLoadingOverlayDrawCommand::FallbackLogo {
                metadata: frame.logo.fallback,
                target_rect: frame.logo.target_rect,
                color_argb: MOJANG_LOADING_OVERLAY_WHITE_ARGB,
                fade_alpha: frame.fade_alpha,
            }),
        }

        commands.push(MojangLoadingOverlayDrawCommand::ProgressBarOuter {
            rect: frame.progress_bar.outer_rect,
            color_argb: MOJANG_LOADING_OVERLAY_WHITE_ARGB,
            border_width: frame.progress_bar.border_width,
            clamped_progress: frame.progress_bar.clamped_progress,
            actual_progress: frame.progress.actual_progress,
            displayed_progress: frame.progress.displayed_progress,
            fade_alpha: frame.fade_alpha,
        });
        commands.push(MojangLoadingOverlayDrawCommand::ProgressBarInnerFill {
            rect: frame.progress_bar.inner_fill_rect,
            color_argb: MOJANG_LOADING_OVERLAY_WHITE_ARGB,
            clamped_progress: frame.progress_bar.clamped_progress,
            inner_fill_width: frame.progress_bar.inner_fill_width,
            fade_alpha: frame.fade_alpha,
        });

        if let Some(text) = text {
            commands.push(MojangLoadingOverlayDrawCommand::LoadingText(text));
        }

        Self { commands }
    }
}

pub const MOJANG_LOADING_OVERLAY_WHITE_ARGB: u32 = 0xFFFF_FFFF;

#[derive(Clone, Debug, PartialEq)]
pub enum MojangLoadingOverlayDrawCommand {
    BackgroundFill {
        rect: MojangLoadingRect,
        color_argb: u32,
        fade_alpha: f32,
        should_render: bool,
    },
    LogoTextureHalf {
        texture: MojangLogoTexture,
        half: MojangLogoHalf,
        source_rect: MojangLogoSourceRect,
        target_rect: MojangLoadingRect,
        fade_alpha: f32,
    },
    FallbackLogo {
        metadata: MojangFallbackLogoMetadata,
        target_rect: MojangLoadingRect,
        color_argb: u32,
        fade_alpha: f32,
    },
    ProgressBarOuter {
        rect: MojangLoadingRect,
        color_argb: u32,
        border_width: f32,
        clamped_progress: f32,
        actual_progress: f32,
        displayed_progress: f32,
        fade_alpha: f32,
    },
    ProgressBarInnerFill {
        rect: MojangLoadingRect,
        color_argb: u32,
        clamped_progress: f32,
        inner_fill_width: f32,
        fade_alpha: f32,
    },
    LoadingText(MojangLoadingOverlayTextCommand),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MojangLogoHalf {
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MojangLoadingOverlayTextCommand {
    pub text: String,
    pub fallback_text: &'static str,
    pub phase: ResourceLoadingReloadPhase,
    pub color_argb: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MojangLoadingProgress {
    pub actual_progress: f32,
    pub displayed_progress: f32,
    pub clamped_displayed_progress: f32,
}

impl MojangLoadingProgress {
    pub fn new(actual_progress: f32, displayed_progress: f32) -> Self {
        Self {
            actual_progress: actual_progress.clamp(0.0, 1.0),
            displayed_progress,
            clamped_displayed_progress: displayed_progress.clamp(0.0, 1.0),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MojangLogoPlacement {
    pub texture: Option<MojangLogoTexture>,
    pub target_rect: MojangLoadingRect,
    pub left_half: MojangLogoHalfPlacement,
    pub right_half: MojangLogoHalfPlacement,
    pub fallback: MojangFallbackLogoMetadata,
}

impl MojangLogoPlacement {
    pub const CENTER_X_FRACTION: f32 = 0.5;
    pub const CENTER_Y_FRACTION: f32 = 0.5;
    pub const WIDTH_PER_HEIGHT: f32 = 4.0;

    fn from_view_model(
        view: &MojangLoadingOverlayViewModel,
        gui_width: f32,
        gui_height: f32,
    ) -> Self {
        let logo_height = VanillaLoadingLogoView::height_for_viewport(gui_width, gui_height);
        let logo_width = logo_height * Self::WIDTH_PER_HEIGHT;
        let center_x = gui_width * Self::CENTER_X_FRACTION;
        let center_y = gui_height * Self::CENTER_Y_FRACTION;
        let half_width = logo_width * 0.5;
        let top = center_y - logo_height * 0.5;
        let target_rect = MojangLoadingRect {
            x: center_x - half_width,
            y: top,
            width: logo_width,
            height: logo_height,
        };
        Self {
            texture: view.logo_texture,
            target_rect,
            left_half: MojangLogoHalfPlacement {
                target_rect: MojangLoadingRect {
                    x: target_rect.x,
                    y: top,
                    width: half_width,
                    height: logo_height,
                },
                source_rect: MojangLogoSourceRect::LEFT_HALF,
            },
            right_half: MojangLogoHalfPlacement {
                target_rect: MojangLoadingRect {
                    x: center_x,
                    y: top,
                    width: half_width,
                    height: logo_height,
                },
                source_rect: MojangLogoSourceRect::RIGHT_HALF,
            },
            fallback: MojangFallbackLogoMetadata::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MojangLogoHalfPlacement {
    pub target_rect: MojangLoadingRect,
    pub source_rect: MojangLogoSourceRect,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MojangLogoSourceRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl MojangLogoSourceRect {
    pub const TEXTURE_SIZE: f32 = 120.0;
    pub const HALF_SOURCE_HEIGHT: f32 = 60.0;
    pub const LEFT_HALF: Self = Self {
        x: -0.0625,
        y: 0.0,
        width: Self::TEXTURE_SIZE,
        height: Self::HALF_SOURCE_HEIGHT,
    };
    pub const RIGHT_HALF: Self = Self {
        x: 0.0625,
        y: Self::HALF_SOURCE_HEIGHT,
        width: Self::TEXTURE_SIZE,
        height: Self::HALF_SOURCE_HEIGHT,
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MojangFallbackLogoMetadata {
    pub primary_word: &'static str,
    pub secondary_word: &'static str,
    pub glyph_columns: u8,
    pub glyph_rows: u8,
}

impl Default for MojangFallbackLogoMetadata {
    fn default() -> Self {
        Self {
            primary_word: "MOJANG",
            secondary_word: "STUDIOS",
            glyph_columns: 5,
            glyph_rows: 7,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MojangProgressBarFrame {
    pub outer_rect: MojangLoadingRect,
    pub inner_fill_rect: MojangLoadingRect,
    pub inner_fill_width: f32,
    pub clamped_progress: f32,
    pub border_width: f32,
}

impl MojangProgressBarFrame {
    fn from_logo(
        logo: &MojangLogoPlacement,
        gui_height: f32,
        displayed_progress: f32,
        border_width: f32,
    ) -> Self {
        let center_y = gui_height * VanillaLoadingProgressBarView::CENTER_Y_FRACTION;
        let outer_rect = MojangLoadingRect {
            x: logo.target_rect.x,
            y: center_y - VanillaLoadingProgressBarView::OUTER_HEIGHT * 0.5,
            width: logo.target_rect.width,
            height: VanillaLoadingProgressBarView::OUTER_HEIGHT,
        };
        let clamped_progress = displayed_progress.clamp(0.0, 1.0);
        let inner_width = ((outer_rect.width - border_width * 2.0) * clamped_progress)
            .ceil()
            .max(0.0);
        let inner_fill_rect = MojangLoadingRect {
            x: outer_rect.x + border_width,
            y: outer_rect.y + border_width,
            width: inner_width,
            height: (outer_rect.height - border_width * 2.0).max(0.0),
        };
        Self {
            outer_rect,
            inner_fill_rect,
            inner_fill_width: inner_width,
            clamped_progress,
            border_width,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MojangLoadingRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
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
pub struct MojangLogoTextureState {
    pub id: &'static str,
    pub resource: &'static str,
    pub status: MojangLogoTextureResolutionStatus,
    pub resolved_pack_id: Option<String>,
    pub resolved_path: Option<PathBuf>,
}

impl MojangLogoTextureState {
    pub fn default_texture() -> Self {
        Self {
            id: MOJANG_STUDIOS_LOGO_ID,
            resource: MOJANG_STUDIOS_LOGO_RESOURCE,
            status: MojangLogoTextureResolutionStatus::DefaultTexture,
            resolved_pack_id: None,
            resolved_path: None,
        }
    }

    pub fn resolve(stack: &ClientResourceStack) -> Self {
        match stack.find_resource(MOJANG_STUDIOS_LOGO_RESOURCE) {
            Some(location) => Self {
                id: MOJANG_STUDIOS_LOGO_ID,
                resource: MOJANG_STUDIOS_LOGO_RESOURCE,
                status: MojangLogoTextureResolutionStatus::Resolved,
                resolved_pack_id: Some(location.pack_id),
                resolved_path: Some(location.path),
            },
            None => Self {
                id: MOJANG_STUDIOS_LOGO_ID,
                resource: MOJANG_STUDIOS_LOGO_RESOURCE,
                status: MojangLogoTextureResolutionStatus::MissingFallback,
                resolved_pack_id: None,
                resolved_path: None,
            },
        }
    }

    pub fn texture(&self) -> Option<MojangLogoTexture> {
        match self.status {
            MojangLogoTextureResolutionStatus::DefaultTexture
            | MojangLogoTextureResolutionStatus::Resolved => Some(MojangLogoTexture {
                id: self.id,
                resource: self.resource,
            }),
            MojangLogoTextureResolutionStatus::MissingFallback => None,
        }
    }

    pub fn is_fallback(&self) -> bool {
        self.status == MojangLogoTextureResolutionStatus::MissingFallback
    }
}

impl Default for MojangLogoTextureState {
    fn default() -> Self {
        Self::default_texture()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MojangLogoTextureResolutionStatus {
    DefaultTexture,
    Resolved,
    MissingFallback,
}

pub fn resolve_mojang_logo_texture_state(stack: &ClientResourceStack) -> MojangLogoTextureState {
    MojangLogoTextureState::resolve(stack)
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceLoadingManagerRuntimeView {
    pub status: ClientResourceReloadManagerRuntimeStatus,
    pub status_label: &'static str,
    pub listener_count: usize,
    pub listener_names: Vec<String>,
    pub pack_ids: Vec<String>,
    pub boundary: &'static str,
    pub error_text: Option<String>,
}

impl From<&ClientResourceReloadManagerRuntimeReport> for ResourceLoadingManagerRuntimeView {
    fn from(report: &ClientResourceReloadManagerRuntimeReport) -> Self {
        let status = report.status();
        Self {
            status,
            status_label: status.as_str(),
            listener_count: report.listener_count(),
            listener_names: report.listener_names().to_vec(),
            pack_ids: report.pack_ids().to_vec(),
            boundary: report.boundary(),
            error_text: report.error_text().map(ToOwned::to_owned),
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
    use std::{
        fmt, fs,
        path::{Path, PathBuf},
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::*;
    use crate::{
        resources::{
            ClientResourcePack, ClientResourceRepository, ClientResourceStack, ResourceReloadError,
            ResourceReloadListener, ResourceReloadManager, ResourceReloadResult,
            ResourceReloadTaskReport,
        },
        ui::startup_flow::{
            LoadingTask, LoadingTaskPresentationState, StartupDestination,
            StartupDestinationActionKind, StartupDestinationHandoffView, StartupGenericMessageView,
            StartupLoadingPhase, StartupLoadingScreen, StartupMojangLoadingOverlaySurface,
            StartupQuickPlayHandoffView, StartupTitleMenuView, loading_task_names,
        },
    };

    fn unique_test_dir(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("{name}-{}-{nanos}", std::process::id()));
        fs::create_dir_all(&path).expect("test temp directory should be created");
        path
    }

    fn write_logo_resource(pack_root: &Path) {
        let resource_path = pack_root.join(MOJANG_STUDIOS_LOGO_RESOURCE);
        fs::create_dir_all(
            resource_path
                .parent()
                .expect("logo resource should have a parent directory"),
        )
        .expect("logo resource parent should be created");
        fs::write(resource_path, b"fake png bytes").expect("logo resource should be written");
    }

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
    fn tracker_retains_reload_manager_runtime_report_pending_before_events() {
        let manager = ResourceReloadManager::new(ClientResourceStack::new(vec![
            ClientResourcePack::new("base-pack", "."),
            ClientResourcePack::new("overlay-pack", "."),
        ]))
        .with_listener(TestReloadListener("textures"))
        .with_listener(TestReloadListener("sounds"));
        let mut tracker = ResourceLoadingTracker::new(Vec::new());

        tracker.track_pending_resource_reload_manager(&manager);

        let report = tracker
            .resource_reload_manager_report()
            .expect("pending manager report should be retained");
        assert_eq!(
            report.status(),
            ClientResourceReloadManagerRuntimeStatus::Pending
        );
        assert_eq!(report.listener_count(), 2);
        assert_eq!(report.listener_names(), ["textures", "sounds"]);
        assert_eq!(report.pack_ids(), ["base-pack", "overlay-pack"]);
        assert_eq!(
            report.boundary(),
            "client_resources_reloaded_runtime_application_pending"
        );

        let view = tracker
            .resource_reload_manager_view()
            .expect("pending manager report should have a UI view");
        assert_eq!(view.status_label, "pending");
        assert_eq!(view.listener_count, 2);
        assert_eq!(view.pack_ids, ["base-pack", "overlay-pack"]);
        assert_eq!(view.listener_names, ["textures", "sounds"]);
        assert_eq!(
            view.boundary,
            "client_resources_reloaded_runtime_application_pending"
        );
    }

    #[test]
    fn tracker_retains_reload_manager_runtime_report_completed() {
        let manager =
            ResourceReloadManager::new(ClientResourceStack::new(vec![ClientResourcePack::new(
                "base-pack",
                ".",
            )]))
            .with_listener(TestReloadListener("textures"))
            .with_listener(TestReloadListener("sounds"));
        let mut tracker = ResourceLoadingTracker::new(Vec::new());

        let report = tracker
            .run_resource_reload(&manager)
            .expect("reload should complete");

        let manager_report = tracker
            .resource_reload_manager_report()
            .expect("completed manager report should be retained");
        assert_eq!(
            manager_report.status(),
            ClientResourceReloadManagerRuntimeStatus::Completed
        );
        assert_eq!(manager_report.listener_count(), 2);
        assert_eq!(manager_report.pack_ids(), ["base-pack"]);
        assert_eq!(
            manager_report.completed_listener_report_count(),
            report.listener_reports().len()
        );
        assert_eq!(manager_report.actual_progress(), 1.0);

        let overlay_view =
            tracker.loading_overlay_view(&VanillaLoadingOverlay::new(), Duration::ZERO);
        let manager_view = overlay_view
            .manager_runtime
            .expect("overlay should expose completed manager runtime view");
        assert_eq!(
            manager_view.status,
            ClientResourceReloadManagerRuntimeStatus::Completed
        );
        assert_eq!(manager_view.status_label, "completed");
        assert_eq!(manager_view.listener_count, 2);
        assert_eq!(manager_view.pack_ids, ["base-pack"]);
        assert_eq!(manager_view.error_text, None);
    }

    #[test]
    fn tracker_retains_reload_manager_runtime_report_failed() {
        let manager = ResourceReloadManager::new(ClientResourceStack::new(vec![
            ClientResourcePack::new("base-pack", "."),
            ClientResourcePack::new("broken-pack", "."),
        ]))
        .with_listener(TestReloadListener("textures"))
        .with_listener(FailingReloadListener("sounds"));
        let mut tracker = ResourceLoadingTracker::new(Vec::new());

        let error = tracker
            .run_resource_reload(&manager)
            .expect_err("reload should fail");
        let error_text = error.to_string();

        let manager_report = tracker
            .resource_reload_manager_report()
            .expect("failed manager report should be retained");
        assert_eq!(
            manager_report.status(),
            ClientResourceReloadManagerRuntimeStatus::Failed
        );
        assert_eq!(manager_report.listener_count(), 2);
        assert_eq!(manager_report.listener_names(), ["textures", "sounds"]);
        assert_eq!(manager_report.pack_ids(), ["base-pack", "broken-pack"]);
        assert_eq!(manager_report.error_text(), Some(error_text.as_str()));
        assert_eq!(
            manager_report.boundary(),
            "client_resources_reloaded_runtime_application_pending"
        );

        let overlay_view =
            tracker.loading_overlay_view(&VanillaLoadingOverlay::new(), Duration::ZERO);
        let manager_view = overlay_view
            .manager_runtime
            .expect("overlay should expose failed manager runtime view");
        assert_eq!(
            manager_view.status,
            ClientResourceReloadManagerRuntimeStatus::Failed
        );
        assert_eq!(manager_view.status_label, "failed");
        assert_eq!(manager_view.listener_names, ["textures", "sounds"]);
        assert_eq!(manager_view.pack_ids, ["base-pack", "broken-pack"]);
        assert_eq!(manager_view.error_text, Some(error_text));
    }

    #[test]
    fn tracker_retains_reload_manager_runtime_report_without_overriding_overlay_snapshot() {
        let manager =
            ResourceReloadManager::new(ClientResourceStack::new(vec![ClientResourcePack::new(
                "base-pack",
                ".",
            )]))
            .with_listener(TestReloadListener("textures"))
            .with_listener(TestReloadListener("sounds"));
        let report = manager.run().unwrap();
        let preparation_event = &report.events()[1];
        let mut tracker = ResourceLoadingTracker::new(Vec::new());

        tracker.track_pending_resource_reload_manager(&manager);
        tracker.apply_update(ResourceLoadingUpdate::task_progress(
            loading_task_names::DOWNLOADING_ASSET,
            "stone.png",
            9,
            10,
        ));
        tracker.apply_resource_reload_event(preparation_event);

        let overlay_view =
            tracker.loading_overlay_view(&VanillaLoadingOverlay::new(), Duration::ZERO);

        assert_eq!(
            overlay_view.vanilla.actual_progress,
            preparation_event.progress_snapshot.actual_progress()
        );
        assert_ne!(overlay_view.vanilla.actual_progress, 0.9);
        assert_eq!(
            overlay_view
                .manager_runtime
                .expect("overlay should expose pending manager runtime view")
                .status,
            ClientResourceReloadManagerRuntimeStatus::Pending
        );
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
        assert_eq!(
            complete_screen.completed_destination_handoff,
            Some(StartupDestinationHandoffView::TitleMenu(
                StartupTitleMenuView::vanilla_initial()
            ))
        );
        assert_eq!(
            complete_screen.title_menu,
            Some(StartupTitleMenuView::vanilla_initial())
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
        assert_eq!(
            red_view.logo_texture_state,
            MojangLogoTextureState::default()
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
    fn mojang_logo_texture_state_resolves_vanilla_logo_from_client_resource_stack() {
        let state = resolve_mojang_logo_texture_state(&ClientResourceStack::vanilla());

        assert_eq!(state.id, MOJANG_STUDIOS_LOGO_ID);
        assert_eq!(state.resource, MOJANG_STUDIOS_LOGO_RESOURCE);
        assert_eq!(state.status, MojangLogoTextureResolutionStatus::Resolved);
        assert_eq!(state.resolved_pack_id.as_deref(), Some("vanilla"));
        assert!(
            state
                .resolved_path
                .as_ref()
                .is_some_and(|path| path.ends_with(MOJANG_STUDIOS_LOGO_RESOURCE))
        );
        assert_eq!(state.texture(), Some(MojangLogoTexture::default()));
    }

    #[test]
    fn mojang_logo_texture_state_reports_missing_logo_as_fallback() {
        let state = resolve_mojang_logo_texture_state(&ClientResourceStack::new(Vec::new()));

        assert_eq!(state.id, MOJANG_STUDIOS_LOGO_ID);
        assert_eq!(state.resource, MOJANG_STUDIOS_LOGO_RESOURCE);
        assert_eq!(
            state.status,
            MojangLogoTextureResolutionStatus::MissingFallback
        );
        assert_eq!(state.resolved_pack_id, None);
        assert_eq!(state.resolved_path, None);
        assert_eq!(state.texture(), None);
        assert!(state.is_fallback());
    }

    #[test]
    fn mojang_logo_texture_state_reports_selected_overlay_pack_priority() {
        let root = unique_test_dir("mojang-logo-overlay");
        let base = root.join("base");
        let overlay = root.join("overlay");
        write_logo_resource(&base);
        write_logo_resource(&overlay);

        let state = resolve_mojang_logo_texture_state(&ClientResourceStack::new(vec![
            ClientResourcePack::new("base-pack", &base),
            ClientResourcePack::new("overlay-pack", &overlay),
        ]));

        assert_eq!(state.status, MojangLogoTextureResolutionStatus::Resolved);
        assert_eq!(state.resolved_pack_id.as_deref(), Some("overlay-pack"));
        assert_eq!(
            state.resolved_path,
            Some(overlay.join(MOJANG_STUDIOS_LOGO_RESOURCE))
        );

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn resource_loading_overlay_view_with_missing_logo_state_emits_fallback_logo() {
        let view = ResourceLoadingTracker::new(Vec::new())
            .loading_overlay_view_with_resource_stack(
                &VanillaLoadingOverlay::new(),
                Duration::ZERO,
                &ClientResourceStack::new(Vec::new()),
            );

        assert_eq!(view.logo_texture, None);
        assert_eq!(
            view.logo_texture_state.status,
            MojangLogoTextureResolutionStatus::MissingFallback
        );

        let draw_list = view.draw_list(900.0, 520.0);

        assert!(matches!(
            draw_list.commands[1],
            MojangLoadingOverlayDrawCommand::FallbackLogo { .. }
        ));
    }

    #[test]
    fn resource_loading_overlay_frame_exposes_desktop_mojang_geometry() {
        let mut overlay = VanillaLoadingOverlay::new();
        overlay.tick(1.0);
        let view = ResourceLoadingTracker::new(Vec::new())
            .loading_overlay_view(&overlay, Duration::from_millis(250));

        let frame = view.frame(900.0, 520.0);

        assert_eq!(frame.background_argb, MOJANG_STUDIOS_RED_BACKGROUND_ARGB);
        assert_eq!(frame.fade_alpha, 0.5);
        assert_eq!(frame.progress.actual_progress, 0.0);
        assert_eq!(frame.progress.displayed_progress, 0.05);
        assert_eq!(frame.progress.clamped_displayed_progress, 0.05);
        assert_eq!(
            frame.logo.target_rect,
            MojangLoadingRect {
                x: 190.0,
                y: 195.0,
                width: 520.0,
                height: 130.0,
            }
        );
        assert_eq!(
            frame.logo.left_half.target_rect,
            MojangLoadingRect {
                x: 190.0,
                y: 195.0,
                width: 260.0,
                height: 130.0,
            }
        );
        assert_eq!(
            frame.logo.right_half.target_rect,
            MojangLoadingRect {
                x: 450.0,
                y: 195.0,
                width: 260.0,
                height: 130.0,
            }
        );
        assert_eq!(
            frame.logo.left_half.source_rect,
            MojangLogoSourceRect::LEFT_HALF
        );
        assert_eq!(
            frame.logo.right_half.source_rect,
            MojangLogoSourceRect::RIGHT_HALF
        );
        assert_eq!(
            frame.progress_bar.outer_rect,
            MojangLoadingRect {
                x: 190.0,
                y: 427.9,
                width: 520.0,
                height: 10.0,
            }
        );
        assert_eq!(
            frame.progress_bar.inner_fill_rect,
            MojangLoadingRect {
                x: 191.0,
                y: 428.9,
                width: 26.0,
                height: 8.0,
            }
        );
    }

    #[test]
    fn resource_loading_overlay_frame_exposes_narrow_mojang_geometry() {
        let view = ResourceLoadingTracker::new(Vec::new())
            .loading_overlay_view(&VanillaLoadingOverlay::new(), Duration::ZERO);

        let frame = view.frame(320.0, 640.0);

        assert_eq!(
            frame.logo.target_rect,
            MojangLoadingRect {
                x: 40.0,
                y: 290.0,
                width: 240.0,
                height: 60.0,
            }
        );
        assert_eq!(
            frame.progress_bar.outer_rect,
            MojangLoadingRect {
                x: 40.0,
                y: 527.8,
                width: 240.0,
                height: 10.0,
            }
        );
        assert_eq!(frame.progress_bar.inner_fill_width, 0.0);
    }

    #[test]
    fn resource_loading_overlay_frame_clamps_progress_for_fill_geometry() {
        let mut view = ResourceLoadingTracker::new(Vec::new())
            .loading_overlay_view(&VanillaLoadingOverlay::new(), Duration::ZERO);
        view.vanilla.actual_progress = 2.0;
        view.vanilla.displayed_progress = 1.5;

        let full_frame = view.frame(900.0, 520.0);

        assert_eq!(full_frame.progress.actual_progress, 1.0);
        assert_eq!(full_frame.progress.displayed_progress, 1.5);
        assert_eq!(full_frame.progress.clamped_displayed_progress, 1.0);
        assert_eq!(full_frame.progress_bar.clamped_progress, 1.0);
        assert_eq!(full_frame.progress_bar.inner_fill_width, 518.0);

        view.vanilla.displayed_progress = -0.25;
        let empty_frame = view.frame(900.0, 520.0);

        assert_eq!(empty_frame.progress.clamped_displayed_progress, 0.0);
        assert_eq!(empty_frame.progress_bar.clamped_progress, 0.0);
        assert_eq!(empty_frame.progress_bar.inner_fill_width, 0.0);
    }

    #[test]
    fn resource_loading_overlay_frame_preserves_black_background_and_logo_metadata() {
        let mut view = ResourceLoadingTracker::new(Vec::new()).loading_overlay_view(
            &VanillaLoadingOverlay::new().with_background(VanillaLoadingBackground::Black),
            Duration::ZERO,
        );

        let textured_frame = view.frame(900.0, 520.0);

        assert_eq!(
            textured_frame.background_argb,
            MOJANG_STUDIOS_BLACK_BACKGROUND_ARGB
        );
        assert_eq!(
            textured_frame.logo.texture,
            Some(MojangLogoTexture::default())
        );
        assert_eq!(
            textured_frame.logo.fallback,
            MojangFallbackLogoMetadata {
                primary_word: "MOJANG",
                secondary_word: "STUDIOS",
                glyph_columns: 5,
                glyph_rows: 7,
            }
        );

        view.logo_texture = None;
        let fallback_frame = view.frame(900.0, 520.0);

        assert_eq!(fallback_frame.logo.texture, None);
        assert_eq!(
            fallback_frame.logo.fallback,
            MojangFallbackLogoMetadata::default()
        );
    }

    #[test]
    fn resource_loading_overlay_draw_exposes_desktop_command_order() {
        let mut overlay = VanillaLoadingOverlay::new();
        overlay.tick(1.0);
        let view = ResourceLoadingTracker::new(Vec::new())
            .loading_overlay_view(&overlay, Duration::from_millis(250));

        let draw_list = view.draw_list(900.0, 520.0);

        assert_eq!(draw_list.commands.len(), 6);
        assert!(matches!(
            draw_list.commands[0],
            MojangLoadingOverlayDrawCommand::BackgroundFill { .. }
        ));
        assert!(matches!(
            draw_list.commands[1],
            MojangLoadingOverlayDrawCommand::LogoTextureHalf {
                half: MojangLogoHalf::Left,
                ..
            }
        ));
        assert!(matches!(
            draw_list.commands[2],
            MojangLoadingOverlayDrawCommand::LogoTextureHalf {
                half: MojangLogoHalf::Right,
                ..
            }
        ));
        assert!(matches!(
            draw_list.commands[3],
            MojangLoadingOverlayDrawCommand::ProgressBarOuter { .. }
        ));
        assert!(matches!(
            draw_list.commands[4],
            MojangLoadingOverlayDrawCommand::ProgressBarInnerFill { .. }
        ));
        assert_eq!(
            draw_list.commands[5],
            MojangLoadingOverlayDrawCommand::LoadingText(MojangLoadingOverlayTextCommand {
                text: "Loading Minecraft".to_owned(),
                fallback_text: "Loading Minecraft",
                phase: ResourceLoadingReloadPhase::Fallback,
                color_argb: MOJANG_LOADING_OVERLAY_WHITE_ARGB,
            })
        );
    }

    #[test]
    fn resource_loading_overlay_draw_uses_fallback_logo_when_texture_is_absent() {
        let mut view = ResourceLoadingTracker::new(Vec::new())
            .loading_overlay_view(&VanillaLoadingOverlay::new(), Duration::ZERO);
        view.logo_texture = None;

        let draw_list = view.draw_list(900.0, 520.0);

        assert_eq!(
            draw_list.commands[1],
            MojangLoadingOverlayDrawCommand::FallbackLogo {
                metadata: MojangFallbackLogoMetadata::default(),
                target_rect: MojangLoadingRect {
                    x: 190.0,
                    y: 195.0,
                    width: 520.0,
                    height: 130.0,
                },
                color_argb: MOJANG_LOADING_OVERLAY_WHITE_ARGB,
                fade_alpha: 0.0,
            }
        );
        assert!(matches!(
            draw_list.commands[2],
            MojangLoadingOverlayDrawCommand::ProgressBarOuter { .. }
        ));
        assert!(!draw_list.commands.iter().any(|command| matches!(
            command,
            MojangLoadingOverlayDrawCommand::LogoTextureHalf { .. }
        )));
    }

    #[test]
    fn resource_loading_overlay_draw_exposes_progress_fill_width() {
        let mut view = ResourceLoadingTracker::new(Vec::new())
            .loading_overlay_view(&VanillaLoadingOverlay::new(), Duration::ZERO);
        view.vanilla.actual_progress = 0.5;
        view.vanilla.displayed_progress = 0.5;

        let draw_list = view.draw_list(900.0, 520.0);

        assert_eq!(
            draw_list.commands[4],
            MojangLoadingOverlayDrawCommand::ProgressBarInnerFill {
                rect: MojangLoadingRect {
                    x: 191.0,
                    y: 428.9,
                    width: 259.0,
                    height: 8.0,
                },
                color_argb: MOJANG_LOADING_OVERLAY_WHITE_ARGB,
                clamped_progress: 0.5,
                inner_fill_width: 259.0,
                fade_alpha: 0.0,
            }
        );
    }

    #[test]
    fn resource_loading_overlay_draw_preserves_black_and_red_backgrounds() {
        let red_view = ResourceLoadingTracker::new(Vec::new())
            .loading_overlay_view(&VanillaLoadingOverlay::new(), Duration::ZERO);
        let black_view = ResourceLoadingTracker::new(Vec::new()).loading_overlay_view(
            &VanillaLoadingOverlay::new().with_background(VanillaLoadingBackground::Black),
            Duration::ZERO,
        );

        assert_eq!(
            red_view.draw_list(900.0, 520.0).commands[0],
            MojangLoadingOverlayDrawCommand::BackgroundFill {
                rect: MojangLoadingRect {
                    x: 0.0,
                    y: 0.0,
                    width: 900.0,
                    height: 520.0,
                },
                color_argb: MOJANG_STUDIOS_RED_BACKGROUND_ARGB,
                fade_alpha: 0.0,
                should_render: true,
            }
        );
        assert_eq!(
            black_view.draw_list(900.0, 520.0).commands[0],
            MojangLoadingOverlayDrawCommand::BackgroundFill {
                rect: MojangLoadingRect {
                    x: 0.0,
                    y: 0.0,
                    width: 900.0,
                    height: 520.0,
                },
                color_argb: MOJANG_STUDIOS_BLACK_BACKGROUND_ARGB,
                fade_alpha: 0.0,
                should_render: true,
            }
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
        assert_eq!(
            tracker.screen().title_menu,
            Some(StartupTitleMenuView::vanilla_initial())
        );
        assert_eq!(
            tracker.screen().completed_destination_handoff,
            Some(StartupDestinationHandoffView::TitleMenu(
                StartupTitleMenuView::vanilla_initial()
            ))
        );
        assert_eq!(tracker.weighted_progress().actual_progress(), 1.0);
    }

    #[test]
    fn resource_loading_startup_destination_defaults_to_title_menu_after_resource_reload() {
        let manager = ResourceReloadManager::new(ClientResourceStack::new(Vec::new()))
            .with_listener(TestReloadListener("textures"));
        let mut tracker = ResourceLoadingTracker::new(Vec::new());

        tracker
            .run_resource_reload(&manager)
            .expect("reload should complete");

        assert_eq!(tracker.startup_destination(), StartupDestination::TitleMenu);
        assert_eq!(
            tracker.flow().startup_destination(),
            Some(StartupDestination::TitleMenu)
        );
        assert_eq!(
            tracker.screen().loading_screen,
            StartupLoadingScreen::CompleteDestination(Some(StartupDestination::TitleMenu))
        );
        assert_eq!(
            tracker.screen().title_menu,
            Some(StartupTitleMenuView::vanilla_initial())
        );
        let handoff = tracker
            .screen()
            .completed_destination_handoff
            .expect("completed default reload should expose a title handoff");
        assert_eq!(handoff.destination(), StartupDestination::TitleMenu);
        assert!(handoff.requires_title_menu());
        assert!(!handoff.requires_external_action());
        assert_eq!(
            handoff.action_kind(),
            StartupDestinationActionKind::ShowTitleMenu
        );
    }

    #[test]
    fn resource_loading_startup_destination_can_target_quick_play_after_resource_reload() {
        let manager = ResourceReloadManager::new(ClientResourceStack::new(Vec::new()))
            .with_listener(TestReloadListener("textures"));
        let mut tracker = ResourceLoadingTracker::new(Vec::new())
            .with_startup_destination(StartupDestination::QuickPlay);

        tracker
            .run_resource_reload(&manager)
            .expect("reload should complete");

        assert_eq!(tracker.startup_destination(), StartupDestination::QuickPlay);
        assert_eq!(
            tracker.flow().startup_destination(),
            Some(StartupDestination::QuickPlay)
        );
        assert_eq!(
            tracker.screen().loading_screen,
            StartupLoadingScreen::CompleteDestination(Some(StartupDestination::QuickPlay))
        );
        assert_eq!(
            tracker.screen().completed_destination_handoff,
            Some(StartupDestinationHandoffView::QuickPlay(
                StartupQuickPlayHandoffView::external_launcher_action()
            ))
        );
        assert_eq!(tracker.screen().title_menu, None);
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

use std::{
    fs,
    io::{self, Cursor},
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use azalea_client::ui::{
    resource_loading::ResourceLoadingTracker,
    startup_flow::{
        ResourceLoadingUpdate, StartupLoadingPhase, VanillaLoadingOverlay, WeightedReloadProgress,
        loading_task_names,
    },
};
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 900;
const HEIGHT: usize = 520;
const MOJANG_RED: u32 = 0xef323d;
const BLACK: u32 = 0x000000;
const WHITE: u32 = 0xffffff;
const MIN_FADE_OUT_DELAY_MS: u128 = 500;
const DEMO_END_MS: u128 = 1_200;
const MOJANG_STUDIOS_ASSET_PATH: &str = "assets/minecraft/textures/gui/title/mojangstudios.png";
const VANILLA_PACK_SEARCH_PARENT_LIMIT: usize = 6;
const LOGO_TEXTURE_SIZE: f32 = 120.0;
const LOGO_HALF_SOURCE_HEIGHT: f32 = 60.0;
const LOGO_LEFT_SOURCE_X: f32 = -0.0625;
const LOGO_RIGHT_SOURCE_X: f32 = 0.0625;
const LOGO_LEFT_SOURCE_Y: f32 = 0.0;
const LOGO_RIGHT_SOURCE_Y: f32 = 60.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vanilla_pack = vanilla_pack_path()?;
    let mojang_studios_logo = match load_mojang_studios_logo(&vanilla_pack) {
        Ok(logo) => Some(logo),
        Err(error) => {
            eprintln!(
                "Could not load Vanilla Mojang Studios logo from {} ({error}). Falling back to the built-in approximation.",
                vanilla_pack.join(MOJANG_STUDIOS_ASSET_PATH).display()
            );
            None
        }
    };

    let mut window = Window::new(
        "Azalea startup loading UI - Vanilla LoadingOverlay",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )?;
    window.set_target_fps(60);

    let mut buffer = vec![0; WIDTH * HEIGHT];
    let mut tracker = ResourceLoadingTracker::new(Vec::new());
    let mut reload_progress = DemoReloadProgress::default();
    let mut overlay = VanillaLoadingOverlay::new();
    let start = Instant::now();
    let mut next_step = 0;
    let demo_timing = demo_timing();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let elapsed = start.elapsed();
        apply_demo_step(&mut tracker, &mut reload_progress, elapsed, &mut next_step);
        let actual_progress = reload_progress.actual_progress();
        overlay.tick(actual_progress);
        overlay.update_fade_out_when_ready(
            tracker.screen().loading_phase,
            elapsed,
            elapsed >= demo_timing.min_fade_out_delay,
        );
        render(
            &mut buffer,
            &overlay,
            actual_progress,
            elapsed,
            mojang_studios_logo.as_ref(),
        );

        window.update_with_buffer(&buffer, WIDTH, HEIGHT)?;

        if elapsed.as_millis() >= demo_timing.demo_end.as_millis()
            || !overlay.should_render(elapsed)
        {
            break;
        }
    }

    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DemoTiming {
    min_fade_out_delay: Duration,
    demo_end: Duration,
}

impl DemoTiming {
    fn default() -> Self {
        Self {
            min_fade_out_delay: Duration::from_millis(MIN_FADE_OUT_DELAY_MS as u64),
            demo_end: Duration::from_millis(DEMO_END_MS as u64),
        }
    }
}

fn demo_timing() -> DemoTiming {
    demo_timing_from(std::env::args())
}

fn demo_timing_from(args: impl IntoIterator<Item = String>) -> DemoTiming {
    let mut timing = DemoTiming::default();

    for arg in args {
        if let Some(value) = arg
            .strip_prefix("--min-fade-out-delay-ms=")
            .and_then(|value| value.parse::<u64>().ok())
        {
            timing.min_fade_out_delay = Duration::from_millis(value);
        } else if let Some(value) = arg
            .strip_prefix("--demo-end-ms=")
            .and_then(|value| value.parse::<u64>().ok())
        {
            timing.demo_end = Duration::from_millis(value);
        } else if let Some(value) = arg
            .strip_prefix("--auto-close-ms=")
            .and_then(|value| value.parse::<u64>().ok())
        {
            timing.demo_end = Duration::from_millis(value);
        }
    }

    timing
}

fn vanilla_pack_path() -> io::Result<PathBuf> {
    vanilla_pack_path_from(std::env::args(), default_vanilla_pack_path)
}

fn vanilla_pack_path_from(
    args: impl IntoIterator<Item = String>,
    default_vanilla_pack: impl FnOnce() -> io::Result<PathBuf>,
) -> io::Result<PathBuf> {
    if let Some(path) = args.into_iter().find_map(|arg| {
        arg.strip_prefix("--vanilla-pack=")
            .map(|value| PathBuf::from(value.to_owned()))
    }) {
        return Ok(path);
    }

    default_vanilla_pack()
}

fn default_vanilla_pack_path() -> io::Result<PathBuf> {
    default_vanilla_pack_path_from(std::env::current_dir().ok(), std::env::current_exe().ok())
}

fn default_vanilla_pack_path_from(
    cwd: Option<PathBuf>,
    current_exe: Option<PathBuf>,
) -> io::Result<PathBuf> {
    let candidates = vanilla_pack_path_candidates(cwd, current_exe);
    candidates
        .iter()
        .find_map(|candidate| {
            candidate
                .join(MOJANG_STUDIOS_ASSET_PATH)
                .is_file()
                .then(|| canonical_path(candidate))
        })
        .ok_or_else(|| vanilla_pack_not_found_error(&candidates))
}

fn canonical_path(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

fn vanilla_pack_path_candidates(
    cwd: Option<PathBuf>,
    current_exe: Option<PathBuf>,
) -> Vec<PathBuf> {
    let mut candidates = Vec::new();

    if let Some(cwd) = cwd {
        push_vanilla_pack_parent_candidates(&mut candidates, &cwd);
    }

    if let Some(exe_dir) = current_exe.and_then(|path| path.parent().map(Path::to_path_buf)) {
        push_vanilla_pack_parent_candidates(&mut candidates, &exe_dir);
    }

    candidates
}

fn push_vanilla_pack_parent_candidates(candidates: &mut Vec<PathBuf>, start: &Path) {
    for ancestor in start.ancestors().take(VANILLA_PACK_SEARCH_PARENT_LIMIT) {
        push_unique_candidate(candidates, ancestor.join("assets").join("vanilla-pack"));
    }
}

fn push_unique_candidate(candidates: &mut Vec<PathBuf>, candidate: PathBuf) {
    if !candidates.contains(&candidate) {
        candidates.push(candidate);
    }
}

fn vanilla_pack_not_found_error(candidates: &[PathBuf]) -> io::Error {
    let tried = candidates
        .iter()
        .map(|candidate| candidate.display().to_string())
        .collect::<Vec<_>>()
        .join(", ");
    io::Error::new(
        io::ErrorKind::NotFound,
        format!(
            "Could not find the vanilla pack by walking upward from the current directory and executable directory for assets/vanilla-pack/{MOJANG_STUDIOS_ASSET_PATH}. Run from the azalea repo, or provide --vanilla-pack=PATH. Tried: {tried}"
        ),
    )
}

#[derive(Debug)]
struct MojangStudiosLogo {
    width: usize,
    height: usize,
    argb: Vec<u32>,
}

fn load_mojang_studios_logo(
    vanilla_pack: &Path,
) -> Result<MojangStudiosLogo, Box<dyn std::error::Error>> {
    let png_bytes = fs::read(vanilla_pack.join(MOJANG_STUDIOS_ASSET_PATH))?;
    decode_png_to_argb(&png_bytes)
}

fn decode_png_to_argb(png_bytes: &[u8]) -> Result<MojangStudiosLogo, Box<dyn std::error::Error>> {
    let mut decoder = png::Decoder::new(Cursor::new(png_bytes));
    decoder.set_transformations(png::Transformations::EXPAND | png::Transformations::STRIP_16);
    let mut reader = decoder.read_info()?;
    let output_buffer_size = reader
        .output_buffer_size()
        .ok_or_else(|| invalid_data("png output buffer size is unknown"))?;
    let mut output = vec![0; output_buffer_size];
    let info = reader.next_frame(&mut output)?;
    let bytes = &output[..info.buffer_size()];
    let argb = match info.color_type {
        png::ColorType::Rgba => bytes
            .chunks_exact(4)
            .map(|pixel| {
                ((pixel[3] as u32) << 24)
                    | ((pixel[0] as u32) << 16)
                    | ((pixel[1] as u32) << 8)
                    | pixel[2] as u32
            })
            .collect(),
        png::ColorType::Rgb => bytes
            .chunks_exact(3)
            .map(|pixel| {
                0xff00_0000 | ((pixel[0] as u32) << 16) | ((pixel[1] as u32) << 8) | pixel[2] as u32
            })
            .collect(),
        png::ColorType::GrayscaleAlpha => bytes
            .chunks_exact(2)
            .map(|pixel| {
                ((pixel[1] as u32) << 24)
                    | ((pixel[0] as u32) << 16)
                    | ((pixel[0] as u32) << 8)
                    | pixel[0] as u32
            })
            .collect(),
        png::ColorType::Grayscale => bytes
            .iter()
            .map(|value| {
                0xff00_0000 | ((*value as u32) << 16) | ((*value as u32) << 8) | *value as u32
            })
            .collect(),
        png::ColorType::Indexed => {
            return Err(invalid_data("indexed png was not expanded by decoder").into());
        }
    };

    Ok(MojangStudiosLogo {
        width: info.width as usize,
        height: info.height as usize,
        argb,
    })
}

fn invalid_data(message: impl Into<String>) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, message.into())
}

fn apply_demo_step(
    tracker: &mut ResourceLoadingTracker,
    reload_progress: &mut DemoReloadProgress,
    elapsed: Duration,
    next_step: &mut usize,
) {
    let steps = [
        (
            50,
            DemoEvent::Progress {
                update: ResourceLoadingUpdate::task_progress(
                    loading_task_names::DOWNLOADING_ASSET_INDEX,
                    "1.21.6.json",
                    1,
                    5,
                ),
                stage: WeightedReloadStage::Prepare,
                stage_progress: 0.2,
            },
        ),
        (
            125,
            DemoEvent::Progress {
                update: ResourceLoadingUpdate::task_progress(
                    loading_task_names::DOWNLOADING_ASSET_INDEX,
                    "1.21.6.json",
                    3,
                    5,
                ),
                stage: WeightedReloadStage::Prepare,
                stage_progress: 0.6,
            },
        ),
        (
            200,
            DemoEvent::Progress {
                update: ResourceLoadingUpdate::task_progress(
                    loading_task_names::DOWNLOADING_ASSET_INDEX,
                    "1.21.6.json",
                    5,
                    5,
                ),
                stage: WeightedReloadStage::Prepare,
                stage_progress: 1.0,
            },
        ),
        (275, DemoEvent::AdvancePresentation),
        (
            350,
            DemoEvent::Progress {
                update: ResourceLoadingUpdate::task_progress(
                    loading_task_names::DOWNLOADING_ASSETS,
                    "minecraft/textures/block/stone.png",
                    44,
                    128,
                ),
                stage: WeightedReloadStage::Reload,
                stage_progress: 44.0 / 128.0,
            },
        ),
        (
            425,
            DemoEvent::Progress {
                update: ResourceLoadingUpdate::task_progress(
                    loading_task_names::DOWNLOADING_ASSETS,
                    "minecraft/textures/block/stone.png",
                    92,
                    128,
                ),
                stage: WeightedReloadStage::Reload,
                stage_progress: 92.0 / 128.0,
            },
        ),
        (
            500,
            DemoEvent::Progress {
                update: ResourceLoadingUpdate::task_finished(
                    loading_task_names::DOWNLOADING_ASSETS,
                    "minecraft/textures/block/stone.png",
                ),
                stage: WeightedReloadStage::Reload,
                stage_progress: 1.0,
            },
        ),
        (575, DemoEvent::AdvancePresentation),
        (
            650,
            DemoEvent::Progress {
                update: ResourceLoadingUpdate::task_progress(
                    loading_task_names::UNPACKING_CORE_ASSETS,
                    "client.jar",
                    2,
                    6,
                ),
                stage: WeightedReloadStage::Listener,
                stage_progress: 2.0 / 6.0,
            },
        ),
        (
            725,
            DemoEvent::Progress {
                update: ResourceLoadingUpdate::task_progress(
                    loading_task_names::UNPACKING_CORE_ASSETS,
                    "client.jar",
                    6,
                    6,
                ),
                stage: WeightedReloadStage::Listener,
                stage_progress: 1.0,
            },
        ),
        (800, DemoEvent::Complete),
    ];

    while *next_step < steps.len() && elapsed.as_millis() >= steps[*next_step].0 {
        match steps[*next_step].1.clone() {
            DemoEvent::Progress {
                update,
                stage,
                stage_progress,
            } => {
                reload_progress.set_stage(stage, stage_progress);
                tracker.apply_update(update);
            }
            DemoEvent::AdvancePresentation => tracker.advance_presentation(),
            DemoEvent::Complete => {
                reload_progress.complete();
                tracker.apply_update(ResourceLoadingUpdate::Complete);
            }
        }
        *next_step += 1;
    }
}

#[derive(Clone)]
enum DemoEvent {
    Progress {
        update: ResourceLoadingUpdate,
        stage: WeightedReloadStage,
        stage_progress: f32,
    },
    AdvancePresentation,
    Complete,
}

#[derive(Clone, Copy)]
enum WeightedReloadStage {
    Prepare,
    Reload,
    Listener,
}

#[derive(Default)]
struct DemoReloadProgress {
    prepare: f32,
    reload: f32,
    listener: f32,
}

impl DemoReloadProgress {
    fn set_stage(&mut self, stage: WeightedReloadStage, progress: f32) {
        let progress = progress.clamp(0.0, 1.0);
        match stage {
            WeightedReloadStage::Prepare => self.prepare = progress,
            WeightedReloadStage::Reload => self.reload = progress,
            WeightedReloadStage::Listener => self.listener = progress,
        }
    }

    fn complete(&mut self) {
        self.prepare = 1.0;
        self.reload = 1.0;
        self.listener = 1.0;
    }

    fn actual_progress(&self) -> f32 {
        WeightedReloadProgress::simple_reload_instance(self.prepare, self.reload, self.listener)
            .actual_progress()
    }
}

#[derive(Debug, PartialEq)]
struct LoadingOverlayGeometry {
    center_x: f32,
    center_y: f32,
    logo_height: f32,
    content_width: f32,
}

impl LoadingOverlayGeometry {
    fn extract(gui_width: usize, gui_height: usize) -> Self {
        let gui_width = gui_width as f32;
        let gui_height = gui_height as f32;
        let logo_height = (gui_width * 0.75).min(gui_height) * 0.25;
        Self {
            center_x: gui_width * 0.5,
            center_y: gui_height * 0.5,
            logo_height,
            content_width: logo_height * 4.0,
        }
    }

    fn logo_width_half(&self) -> f32 {
        self.content_width * 0.5
    }
}

#[derive(Debug, PartialEq)]
struct ProgressBarGeometry {
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    inner_fill_width: i32,
}

impl ProgressBarGeometry {
    fn extract(gui_width: usize, gui_height: usize, progress: f32) -> Self {
        let overlay = LoadingOverlayGeometry::extract(gui_width, gui_height);
        let logo_width_half = overlay.logo_width_half();
        let bar_y = gui_height as f32 * 0.8325;
        let x0 = overlay.center_x - logo_width_half;
        let x1 = overlay.center_x + logo_width_half;
        Self {
            x0,
            y0: bar_y - 5.0,
            x1,
            y1: bar_y + 5.0,
            inner_fill_width: ((x1 - x0 - 2.0) * progress.clamp(0.0, 1.0)).ceil() as i32,
        }
    }
}

fn render(
    buffer: &mut [u32],
    overlay: &VanillaLoadingOverlay,
    actual_progress: f32,
    elapsed: Duration,
    mojang_studios_logo: Option<&MojangStudiosLogo>,
) {
    buffer.fill(BLACK);
    let view = overlay.view(actual_progress, elapsed);
    let fade = view.fade_alpha;
    draw_rect_alpha(buffer, 0, 0, WIDTH as i32, HEIGHT as i32, MOJANG_RED, 255);
    let geometry = LoadingOverlayGeometry::extract(WIDTH, HEIGHT);
    if let Some(logo) = mojang_studios_logo {
        draw_mojang_studios_logo(buffer, &geometry, fade, logo);
    } else {
        draw_mojang_studios_logo_fallback(buffer, &geometry, fade);
    }
    draw_progress_bar(
        buffer,
        &ProgressBarGeometry::extract(WIDTH, HEIGHT, view.displayed_progress),
        fade,
    );
}

fn draw_mojang_studios_logo(
    buffer: &mut [u32],
    geometry: &LoadingOverlayGeometry,
    fade: f32,
    logo: &MojangStudiosLogo,
) {
    let top = (geometry.center_y - geometry.logo_height * 0.5).round() as i32;
    let half_width = geometry.logo_width_half().round() as i32;
    let height = geometry.logo_height.round() as i32;
    let left = geometry.center_x.round() as i32 - half_width;
    let center = geometry.center_x.round() as i32;

    draw_mojang_studios_logo_half(
        buffer,
        logo,
        left,
        top,
        half_width,
        height,
        LOGO_LEFT_SOURCE_X,
        LOGO_LEFT_SOURCE_Y,
        fade,
    );
    draw_mojang_studios_logo_half(
        buffer,
        logo,
        center,
        top,
        half_width,
        height,
        LOGO_RIGHT_SOURCE_X,
        LOGO_RIGHT_SOURCE_Y,
        fade,
    );
}

fn draw_mojang_studios_logo_half(
    buffer: &mut [u32],
    logo: &MojangStudiosLogo,
    left: i32,
    top: i32,
    width: i32,
    height: i32,
    source_x: f32,
    source_y: f32,
    fade: f32,
) {
    let fade_alpha = fade_to_alpha(fade) as u32;

    for y in 0..height {
        let target_y = top + y;
        if !(0..HEIGHT as i32).contains(&target_y) {
            continue;
        }

        let v = (source_y + ((y as f32 + 0.5) / height as f32) * LOGO_HALF_SOURCE_HEIGHT)
            / LOGO_TEXTURE_SIZE;
        let source_row = ((v * logo.height as f32).floor() as usize).min(logo.height - 1);

        for x in 0..width {
            let target_x = left + x;
            if !(0..WIDTH as i32).contains(&target_x) {
                continue;
            }

            let u = (source_x + ((x as f32 + 0.5) / width as f32) * LOGO_TEXTURE_SIZE)
                / LOGO_TEXTURE_SIZE;
            let source_column = ((u * logo.width as f32).floor() as usize).min(logo.width - 1);
            let source = logo.argb[source_row * logo.width + source_column];
            let source_alpha = (source >> 24) & 0xff;
            if source_alpha == 0 {
                continue;
            }

            let alpha = ((source_alpha * fade_alpha) / 255) as u8;
            let color = source & 0x00ff_ffff;
            let index = target_y as usize * WIDTH + target_x as usize;
            buffer[index] = blend(buffer[index], color, alpha);
        }
    }
}

fn draw_mojang_studios_logo_fallback(
    buffer: &mut [u32],
    geometry: &LoadingOverlayGeometry,
    fade: f32,
) {
    let alpha = fade_to_alpha(fade);
    let logo_top = (geometry.center_y - geometry.logo_height * 0.5).round() as i32;
    let primary_scale = (geometry.logo_height / 13.0).round().max(4.0) as i32;
    let secondary_scale = (primary_scale / 2).max(3);
    let primary = ["M", "O", "J", "A", "N", "G"];
    let secondary = ["S", "T", "U", "D", "I", "O", "S"];

    draw_logo_word(
        buffer,
        &primary,
        geometry.center_x.round() as i32,
        logo_top,
        primary_scale,
        alpha,
    );
    draw_logo_word(
        buffer,
        &secondary,
        geometry.center_x.round() as i32,
        logo_top + primary_scale * 9,
        secondary_scale,
        alpha,
    );
}

fn draw_logo_word(
    buffer: &mut [u32],
    letters: &[&str],
    center_x: i32,
    top: i32,
    scale: i32,
    alpha: u8,
) {
    let letter_width = 5 * scale;
    let gap = scale;
    let width = letters.len() as i32 * letter_width + (letters.len() as i32 - 1) * gap;
    let mut x = center_x - width / 2;

    for letter in letters {
        draw_logo_letter(buffer, letter, x, top, scale, alpha);
        x += letter_width + gap;
    }
}

fn draw_logo_letter(buffer: &mut [u32], letter: &str, left: i32, top: i32, scale: i32, alpha: u8) {
    for (row, pattern) in logo_letter_rows(letter).iter().enumerate() {
        for (column, cell) in pattern.as_bytes().iter().enumerate() {
            if *cell == b'1' {
                draw_rect_alpha(
                    buffer,
                    left + column as i32 * scale,
                    top + row as i32 * scale,
                    scale,
                    scale,
                    WHITE,
                    alpha,
                );
            }
        }
    }
}

fn logo_letter_rows(letter: &str) -> [&'static str; 7] {
    match letter {
        "A" => [
            "01110", "10001", "10001", "11111", "10001", "10001", "10001",
        ],
        "D" => [
            "11110", "10001", "10001", "10001", "10001", "10001", "11110",
        ],
        "G" => [
            "01111", "10000", "10000", "10111", "10001", "10001", "01111",
        ],
        "I" => [
            "11111", "00100", "00100", "00100", "00100", "00100", "11111",
        ],
        "J" => [
            "00111", "00010", "00010", "00010", "10010", "10010", "01100",
        ],
        "M" => [
            "10001", "11011", "10101", "10101", "10001", "10001", "10001",
        ],
        "N" => [
            "10001", "11001", "10101", "10011", "10001", "10001", "10001",
        ],
        "O" => [
            "01110", "10001", "10001", "10001", "10001", "10001", "01110",
        ],
        "S" => [
            "01111", "10000", "10000", "01110", "00001", "00001", "11110",
        ],
        "T" => [
            "11111", "00100", "00100", "00100", "00100", "00100", "00100",
        ],
        "U" => [
            "10001", "10001", "10001", "10001", "10001", "10001", "01110",
        ],
        _ => [
            "00000", "00000", "00000", "00000", "00000", "00000", "00000",
        ],
    }
}

fn draw_progress_bar(buffer: &mut [u32], geometry: &ProgressBarGeometry, fade: f32) {
    let alpha = fade_to_alpha(fade);
    let x0 = geometry.x0.round() as i32;
    let y0 = geometry.y0.round() as i32;
    let width = (geometry.x1 - geometry.x0).round() as i32;
    let height = (geometry.y1 - geometry.y0).round() as i32;

    draw_rect_alpha(buffer, x0, y0, width, 1, WHITE, alpha);
    draw_rect_alpha(buffer, x0, y0 + height - 1, width, 1, WHITE, alpha);
    draw_rect_alpha(buffer, x0, y0, 1, height, WHITE, alpha);
    draw_rect_alpha(buffer, x0 + width - 1, y0, 1, height, WHITE, alpha);
    draw_rect_alpha(
        buffer,
        x0 + 1,
        y0 + 1,
        geometry.inner_fill_width,
        height - 2,
        WHITE,
        alpha,
    );
}

fn draw_rect_alpha(
    buffer: &mut [u32],
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    color: u32,
    alpha: u8,
) {
    let left = x.max(0) as usize;
    let top = y.max(0) as usize;
    let right = (x + width).clamp(0, WIDTH as i32) as usize;
    let bottom = (y + height).clamp(0, HEIGHT as i32) as usize;

    for row in top..bottom {
        for pixel in &mut buffer[row * WIDTH + left..row * WIDTH + right] {
            *pixel = blend(*pixel, color, alpha);
        }
    }
}

fn fade_to_alpha(fade: f32) -> u8 {
    (fade.clamp(0.0, 1.0) * 255.0).round() as u8
}

fn blend(base: u32, color: u32, alpha: u8) -> u32 {
    if alpha == 255 {
        return color;
    }

    let alpha = alpha as u32;
    let inverse = 255 - alpha;
    let br = (base >> 16) & 0xff;
    let bg = (base >> 8) & 0xff;
    let bb = base & 0xff;
    let cr = (color >> 16) & 0xff;
    let cg = (color >> 8) & 0xff;
    let cb = color & 0xff;

    (((cr * alpha + br * inverse) / 255) << 16)
        | (((cg * alpha + bg * inverse) / 255) << 8)
        | ((cb * alpha + bb * inverse) / 255)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weighted_reload_progress_uses_prepare_reload_listener_weights() {
        let mut progress = DemoReloadProgress::default();

        progress.set_stage(WeightedReloadStage::Prepare, 1.0);
        progress.set_stage(WeightedReloadStage::Reload, 0.5);
        progress.set_stage(WeightedReloadStage::Listener, 0.25);

        assert!((progress.actual_progress() - 0.65).abs() < f32::EPSILON);
    }

    #[test]
    fn vanilla_overlay_smooths_actual_progress() {
        let mut overlay = VanillaLoadingOverlay::new();

        overlay.tick(1.0);
        overlay.tick(1.0);

        assert!((overlay.displayed_progress() - 0.0975).abs() < f32::EPSILON);
    }

    #[test]
    fn fade_out_waits_for_demo_delay_and_uses_vanilla_duration() {
        let mut overlay = VanillaLoadingOverlay::new();
        let before_delay = Duration::from_millis(MIN_FADE_OUT_DELAY_MS as u64 - 1);
        overlay.update_fade_out_when_ready(StartupLoadingPhase::Complete, before_delay, false);
        assert_eq!(overlay.fade_out_start(), None);

        overlay.update_fade_out_when_ready(
            StartupLoadingPhase::Complete,
            Duration::from_millis(MIN_FADE_OUT_DELAY_MS as u64),
            true,
        );
        assert_eq!(
            overlay.fade_out_start(),
            Some(Duration::from_millis(MIN_FADE_OUT_DELAY_MS as u64))
        );
        assert!(
            (overlay.fade_alpha(
                Duration::from_millis(MIN_FADE_OUT_DELAY_MS as u64)
                    + VanillaLoadingOverlay::FADE_OUT / 2
            ) - 0.5)
                .abs()
                < f32::EPSILON
        );
        assert!(overlay.should_render(
            Duration::from_millis(MIN_FADE_OUT_DELAY_MS as u64)
                + VanillaLoadingOverlay::FADE_OUT * 2
                - Duration::from_millis(1)
        ));
        assert!(overlay.should_render(
            Duration::from_millis(MIN_FADE_OUT_DELAY_MS as u64) + VanillaLoadingOverlay::FADE_OUT
        ));
        assert!(!overlay.should_render(
            Duration::from_millis(MIN_FADE_OUT_DELAY_MS as u64)
                + VanillaLoadingOverlay::FADE_OUT * 2
        ));
    }

    #[test]
    fn vanilla_geometry_matches_loading_overlay_formulas() {
        let logo = LoadingOverlayGeometry::extract(900, 520);
        assert_eq!(logo.center_x, 450.0);
        assert_eq!(logo.center_y, 260.0);
        assert_eq!(logo.logo_height, 130.0);
        assert_eq!(logo.content_width, 520.0);

        let bar = ProgressBarGeometry::extract(900, 520, 0.5);
        assert_eq!(bar.x0, 190.0);
        assert_eq!(bar.x1, 710.0);
        assert!((bar.y0 - 427.9).abs() < 0.001);
        assert!((bar.y1 - 437.9).abs() < 0.001);
        assert_eq!(bar.inner_fill_width, 259);
    }

    #[test]
    fn vanilla_logo_blit_uses_two_vertical_texture_halves() {
        assert_eq!(LOGO_TEXTURE_SIZE, 120.0);
        assert_eq!(LOGO_HALF_SOURCE_HEIGHT, 60.0);
        assert_eq!(LOGO_LEFT_SOURCE_X, -0.0625);
        assert_eq!(LOGO_RIGHT_SOURCE_X, 0.0625);
        assert_eq!(LOGO_LEFT_SOURCE_Y, 0.0);
        assert_eq!(LOGO_RIGHT_SOURCE_Y, 60.0);
    }

    #[test]
    fn vanilla_pack_path_defaults_to_cwd_relative_assets() {
        let root = unique_test_dir("cwd_assets");
        let vanilla_pack = root.join("assets").join("vanilla-pack");
        create_vanilla_pack_marker(&vanilla_pack);

        let path = vanilla_pack_path_from(["startup_loading_ui".to_owned()], || {
            default_vanilla_pack_path_from(Some(root.clone()), None)
        })
        .expect("cwd-relative vanilla pack should resolve");

        assert_eq!(path, canonical_path(&vanilla_pack));
    }

    #[test]
    fn vanilla_pack_path_cli_overrides_default_discovery() {
        let cli_pack = unique_test_dir("cli_pack");
        let path = vanilla_pack_path_from(
            [
                "startup_loading_ui".to_owned(),
                format!("--vanilla-pack={}", cli_pack.display()),
            ],
            default_vanilla_pack_path,
        )
        .expect("cli vanilla pack should resolve");

        assert_eq!(path, cli_pack);
    }

    #[test]
    fn demo_timing_uses_shorter_defaults_and_accepts_cli_overrides() {
        let defaults = demo_timing_from(["startup_loading_ui".to_owned()]);
        assert_eq!(
            defaults,
            DemoTiming {
                min_fade_out_delay: Duration::from_millis(MIN_FADE_OUT_DELAY_MS as u64),
                demo_end: Duration::from_millis(DEMO_END_MS as u64),
            }
        );

        let overridden = demo_timing_from([
            "startup_loading_ui".to_owned(),
            "--min-fade-out-delay-ms=250".to_owned(),
            "--demo-end-ms=700".to_owned(),
        ]);

        assert_eq!(
            overridden,
            DemoTiming {
                min_fade_out_delay: Duration::from_millis(250),
                demo_end: Duration::from_millis(700),
            }
        );
    }

    #[test]
    fn vanilla_pack_path_default_checks_parent_assets() {
        let root = unique_test_dir("parent_assets");
        let cwd = root.join("azalea-client");
        let vanilla_pack = root.join("assets").join("vanilla-pack");
        fs::create_dir_all(&cwd).expect("test cwd should be created");
        create_vanilla_pack_marker(&vanilla_pack);

        let path = default_vanilla_pack_path_from(Some(cwd), None)
            .expect("parent assets vanilla pack should resolve");

        assert_eq!(path, canonical_path(&vanilla_pack));
    }

    #[test]
    fn vanilla_pack_path_default_walks_up_from_nested_cwd() {
        let root = unique_test_dir("nested_cwd_assets");
        let cwd = root.join("azalea-client").join("examples").join("nested");
        let vanilla_pack = root.join("assets").join("vanilla-pack");
        fs::create_dir_all(&cwd).expect("test cwd should be created");
        create_vanilla_pack_marker(&vanilla_pack);

        let path = default_vanilla_pack_path_from(Some(cwd), None)
            .expect("nested cwd vanilla pack should resolve by walking parents");

        assert_eq!(path, canonical_path(&vanilla_pack));
    }

    #[test]
    fn vanilla_pack_path_default_walks_up_from_executable_dir() {
        let root = unique_test_dir("exe_assets");
        let exe = root
            .join("target")
            .join("debug")
            .join("examples")
            .join("demo");
        let vanilla_pack = root.join("assets").join("vanilla-pack");
        fs::create_dir_all(exe.parent().expect("test exe should have a parent"))
            .expect("test exe directory should be created");
        create_vanilla_pack_marker(&vanilla_pack);

        let path = default_vanilla_pack_path_from(None, Some(exe))
            .expect("executable-dir vanilla pack should resolve by walking parents");

        assert_eq!(path, canonical_path(&vanilla_pack));
    }

    #[test]
    fn vanilla_pack_path_error_explains_overrides() {
        let root = unique_test_dir("missing");
        fs::create_dir_all(&root).expect("test root should be created");

        let error = default_vanilla_pack_path_from(Some(root), None)
            .expect_err("missing vanilla pack should return a not-found error");
        let message = error.to_string();

        assert!(message.contains("--vanilla-pack=PATH"));
        assert!(message.contains("walking upward"));
        assert!(message.contains("Tried:"));
    }

    #[test]
    fn extracted_vanilla_pack_logo_can_be_decoded_when_available() {
        let Ok(vanilla_pack_path) = default_vanilla_pack_path() else {
            return;
        };

        let png_path = vanilla_pack_path.join(MOJANG_STUDIOS_ASSET_PATH);
        let png_bytes = fs::read(png_path)
            .expect("extracted vanilla pack should contain the Mojang Studios PNG");
        assert!(png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));

        let logo = decode_png_to_argb(&png_bytes).expect("Mojang Studios PNG should decode");
        assert_eq!(logo.width, 512);
        assert_eq!(logo.height, 512);
        assert_eq!(logo.argb.len(), logo.width * logo.height);
        assert!(logo.argb.iter().any(|pixel| pixel >> 24 != 0));
    }

    #[test]
    fn fade_alpha_uses_rounded_white_alpha() {
        assert_eq!(fade_to_alpha(0.0), 0);
        assert_eq!(fade_to_alpha(0.5), 128);
        assert_eq!(fade_to_alpha(1.0), 255);
    }

    fn unique_test_dir(name: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("startup_loading_ui_{name}_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        dir
    }

    fn create_vanilla_pack_marker(vanilla_pack: &Path) {
        let marker = vanilla_pack.join(MOJANG_STUDIOS_ASSET_PATH);
        fs::create_dir_all(marker.parent().expect("marker path should have parent"))
            .expect("test vanilla pack marker directory should be created");
        fs::write(marker, []).expect("test vanilla pack marker should be created");
    }
}

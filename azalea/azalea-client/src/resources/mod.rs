//! Headless client resource reload foundation.
//!
//! This keeps Minecraft client-resource reload shape visible without pulling in
//! rendering, audio, or packet handling.

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt, fs,
    io::{self, Cursor, Read},
    path::{Path, PathBuf},
};

use azalea_chat::FormattedText;
use serde::Deserialize;
use thiserror::Error;
use uuid::Uuid;

pub const VANILLA_PACK_ID: &str = "vanilla";
pub const VANILLA_KNOWN_PACK_NAMESPACE: &str = "minecraft";
pub const VANILLA_KNOWN_PACK_ID: &str = "core";
pub const VANILLA_KNOWN_PACK_VERSION: &str = "26.1.2";
pub const VANILLA_PACK_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../assets/vanilla-pack");
pub const INITIAL_RELOAD_TASK_NAME: &str = "initial";
pub const DEFAULT_LANGUAGE_CODE: &str = "en_us";
pub const SPLASHES_RESOURCE: &str = "assets/minecraft/texts/splashes.txt";
pub const GRASS_COLORMAP_RESOURCE: &str = "assets/minecraft/textures/colormap/grass.png";
pub const FOLIAGE_COLORMAP_RESOURCE: &str = "assets/minecraft/textures/colormap/foliage.png";
pub const DRY_FOLIAGE_COLORMAP_RESOURCE: &str =
    "assets/minecraft/textures/colormap/dry_foliage.png";
pub const CLOUDS_TEXTURE_RESOURCE: &str = "assets/minecraft/textures/environment/clouds.png";
pub const GPU_WARNLIST_RESOURCE: &str = "assets/minecraft/gpu_warnlist.json";
pub const REGIONAL_COMPLIANCIES_RESOURCE: &str = "assets/minecraft/regional_compliancies.json";
pub const SOUND_EVENTS_RESOURCE: &str = "assets/minecraft/sounds.json";
pub const EQUIPMENT_MANIFEST_DIR: &str = "assets/minecraft/equipment";
pub const PARTICLE_MANIFEST_DIR: &str = "assets/minecraft/particles";
pub const WAYPOINT_STYLE_MANIFEST_DIR: &str = "assets/minecraft/waypoint_style";
pub const FONT_DEFINITION_DIR: &str = "assets/minecraft/font";
pub const SHADER_SOURCE_DIR: &str = "assets/minecraft/shaders";
pub const BLOCKSTATE_DEFINITION_DIR: &str = "assets/minecraft/blockstates";
pub const SERVER_RESOURCE_PACK_MAX_DOWNLOAD_BYTES: usize = 262_144_000;

const DEFAULT_REQUIRED_VANILLA_ASSETS: &[&str] = &[
    "assets/minecraft/lang/en_us.json",
    "assets/minecraft/textures/gui/title/mojangstudios.png",
    SPLASHES_RESOURCE,
    "assets/minecraft/atlases/blocks.json",
];
const DEFAULT_ATLAS_MANIFESTS: &[&str] = &[
    "assets/minecraft/atlases/blocks.json",
    "assets/minecraft/atlases/items.json",
    "assets/minecraft/atlases/particles.json",
];
const DEFAULT_COLORMAPS: &[&str] = &[
    GRASS_COLORMAP_RESOURCE,
    FOLIAGE_COLORMAP_RESOURCE,
    DRY_FOLIAGE_COLORMAP_RESOURCE,
];
const DEFAULT_REPRESENTATIVE_TEXTURES: &[&str] = &[
    "assets/minecraft/textures/misc/unknown_pack.png",
    "assets/minecraft/textures/misc/pumpkinblur.png",
    "assets/minecraft/textures/gui/title/mojangstudios.png",
];
const DEFAULT_MODEL_SMOKE_RESOURCES: &[&str] = &[
    "assets/minecraft/models/block/stone.json",
    "assets/minecraft/models/item/stick.json",
    "assets/minecraft/blockstates/stone.json",
    "assets/minecraft/items/stone.json",
];
const DEFAULT_MODEL_DEPENDENCY_BLOCKSTATES: &[&str] = &["assets/minecraft/blockstates/stone.json"];
const DEFAULT_MODEL_DEPENDENCY_BLOCK_MODELS: &[&str] =
    &["assets/minecraft/models/block/stone.json"];
const DEFAULT_MODEL_DEPENDENCY_ITEM_MODELS: &[&str] = &["assets/minecraft/models/item/stick.json"];
const DEFAULT_MODEL_DEPENDENCY_ITEM_ROOTS: &[&str] = &["assets/minecraft/items/stone.json"];
const DEFAULT_REPRESENTATIVE_SHADER_SOURCES: &[&str] = &[
    "assets/minecraft/shaders/core/position_tex.vsh",
    "assets/minecraft/shaders/core/gui.fsh",
    "assets/minecraft/shaders/post/blit.fsh",
];
const DEFAULT_SHADER_INCLUDE_SOURCES: &[&str] = &[
    "assets/minecraft/shaders/include/animation_sprite.glsl",
    "assets/minecraft/shaders/include/chunksection.glsl",
    "assets/minecraft/shaders/include/dynamictransforms.glsl",
    "assets/minecraft/shaders/include/fog.glsl",
    "assets/minecraft/shaders/include/globals.glsl",
    "assets/minecraft/shaders/include/light.glsl",
    "assets/minecraft/shaders/include/matrix.glsl",
    "assets/minecraft/shaders/include/projection.glsl",
    "assets/minecraft/shaders/include/sample_lightmap.glsl",
];
const FONT_DEFINITION_ROOT_DIR: &str = "assets/minecraft/font";
const DEFAULT_WAYPOINT_STYLE_MANIFEST_IDS: &[&str] = &["default", "bowtie"];
const DEFAULT_WAYPOINT_STYLE_NEAR_DISTANCE: u32 = 128;
const DEFAULT_WAYPOINT_STYLE_FAR_DISTANCE: u32 = 332;
const WAYPOINT_STYLE_SPRITE_LOCATION_PREFIX: &str = "hud/locator_bar_dot";
const VANILLA_EXCLUDED_SPLASH_JAVA_HASH: i32 = 125_780_783;
const PNG_SIGNATURE: &[u8; 8] = b"\x89PNG\r\n\x1a\n";
const PACK_MCMETA_RESOURCE: &str = "pack.mcmeta";
const CLIENT_RESOURCE_PACK_FORMAT: u32 = 84;

pub type ResourceReloadResult<T> = Result<T, ResourceReloadError>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClientResourcePack {
    id: String,
    root: PathBuf,
    source: ClientResourcePackSource,
}

impl ClientResourcePack {
    pub fn new(id: impl Into<String>, root: impl Into<PathBuf>) -> Self {
        let root = root.into();
        let source = if root.is_file() {
            ClientResourcePackSource::RootZip(root.clone())
        } else {
            ClientResourcePackSource::Directory(root.clone())
        };
        Self {
            id: id.into(),
            root,
            source,
        }
    }

    pub fn vanilla() -> Self {
        Self::new(VANILLA_PACK_ID, VANILLA_PACK_PATH)
    }

    pub fn server_placeholder(id: Uuid) -> Self {
        let root = PathBuf::from(format!("<server-resource-pack:{id}>"));
        Self {
            id: format!("server:{id}"),
            root: root.clone(),
            source: ClientResourcePackSource::Unavailable(root),
        }
    }

    pub fn server(id: Uuid, root: impl Into<PathBuf>) -> Self {
        Self::new(format!("server:{id}"), root)
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn resource_path(&self, resource: impl AsRef<Path>) -> PathBuf {
        self.root.join(resource)
    }

    pub fn contains(&self, resource: impl AsRef<Path>) -> bool {
        self.resource_location(resource.as_ref()).is_some()
    }

    fn resource_location(&self, resource: &Path) -> Option<ResourceLocation> {
        self.source.location(&self.id, resource)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ClientResourcePackSource {
    Directory(PathBuf),
    RootZip(PathBuf),
    Unavailable(PathBuf),
}

impl ClientResourcePackSource {
    fn location(&self, pack_id: &str, resource: &Path) -> Option<ResourceLocation> {
        match self {
            Self::Directory(root) => {
                let path = root.join(resource);
                path.is_file().then(|| ResourceLocation {
                    pack_id: pack_id.to_owned(),
                    path: path.clone(),
                    source: ResourceLocationSource::Directory(path),
                })
            }
            Self::RootZip(root) => {
                root_zip_contains_resource(root, resource).then(|| ResourceLocation {
                    pack_id: pack_id.to_owned(),
                    path: root.join(resource),
                    source: ResourceLocationSource::RootZip {
                        root: root.clone(),
                        resource: zip_resource_name(resource),
                    },
                })
            }
            Self::Unavailable(_) => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClientResourceStack {
    packs: Vec<ClientResourcePack>,
}

impl ClientResourceStack {
    pub fn new(packs: Vec<ClientResourcePack>) -> Self {
        Self { packs }
    }

    pub fn vanilla() -> Self {
        Self::new(vec![ClientResourcePack::vanilla()])
    }

    pub fn packs(&self) -> &[ClientResourcePack] {
        &self.packs
    }

    pub fn find_resource(&self, resource: impl AsRef<Path>) -> Option<ResourceLocation> {
        let resource = resource.as_ref();
        self.packs
            .iter()
            .rev()
            .find_map(|pack| pack.resource_location(resource))
    }

    pub fn resource_stack(&self, resource: impl AsRef<Path>) -> Vec<ResourceLocation> {
        let resource = resource.as_ref();
        self.packs
            .iter()
            .filter_map(|pack| pack.resource_location(resource))
            .collect()
    }

    pub fn require_resource(
        &self,
        resource: impl AsRef<Path>,
    ) -> ResourceReloadResult<ResourceLocation> {
        let resource = resource.as_ref();
        self.find_resource(resource).ok_or_else(|| {
            ResourceReloadError::MissingResource(resource.to_string_lossy().into_owned())
        })
    }
}

impl Default for ClientResourceStack {
    fn default() -> Self {
        Self::vanilla()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ServerResourcePackRequest {
    id: Uuid,
    url: String,
    hash: String,
    required: bool,
    prompt: Option<FormattedText>,
}

impl ServerResourcePackRequest {
    pub fn new(
        id: Uuid,
        url: impl Into<String>,
        hash: impl Into<String>,
        required: bool,
        prompt: Option<FormattedText>,
    ) -> Self {
        Self {
            id,
            url: url.into(),
            hash: hash.into(),
            required,
            prompt,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn hash(&self) -> &str {
        &self.hash
    }

    pub fn required(&self) -> bool {
        self.required
    }

    pub fn prompt(&self) -> Option<&FormattedText> {
        self.prompt.as_ref()
    }

    pub fn validate_url(&self) -> Result<(), ServerResourcePackValidationError> {
        validate_server_resource_pack_url(&self.url)
    }

    pub fn parsed_sha1_hash(&self) -> Option<String> {
        parse_server_resource_pack_sha1_hash(&self.hash)
    }

    pub fn validation(&self) -> ServerResourcePackValidation {
        ServerResourcePackValidation {
            url: self.validate_url(),
            sha1_hash: self.parsed_sha1_hash(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ServerResourcePackValidation {
    pub url: Result<(), ServerResourcePackValidationError>,
    pub sha1_hash: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServerResourcePackValidationError {
    InvalidUrl,
}

fn validate_server_resource_pack_url(url: &str) -> Result<(), ServerResourcePackValidationError> {
    if is_valid_server_resource_pack_url(url) {
        Ok(())
    } else {
        Err(ServerResourcePackValidationError::InvalidUrl)
    }
}

fn is_valid_server_resource_pack_url(url: &str) -> bool {
    let Some((scheme, rest)) = url.split_once("://") else {
        return false;
    };
    if !scheme.eq_ignore_ascii_case("http") && !scheme.eq_ignore_ascii_case("https") {
        return false;
    }
    if rest.is_empty()
        || rest
            .bytes()
            .any(|byte| byte.is_ascii_whitespace() || byte < 0x20)
    {
        return false;
    }

    let authority_end = rest.find(['/', '?', '#']).unwrap_or(rest.len());
    let authority = &rest[..authority_end];
    is_valid_server_resource_pack_authority(authority)
}

fn is_valid_server_resource_pack_authority(authority: &str) -> bool {
    if authority.is_empty() {
        return false;
    }

    if let Some(authority) = authority.strip_prefix('[') {
        let Some((host, rest)) = authority.split_once(']') else {
            return false;
        };
        let port_is_valid = if rest.is_empty() {
            true
        } else if let Some(port) = rest.strip_prefix(':') {
            is_valid_server_resource_pack_port(port)
        } else {
            false
        };
        return !host.is_empty()
            && !host
                .bytes()
                .any(|byte| byte.is_ascii_whitespace() || byte < 0x20)
            && port_is_valid;
    }

    let (host, port) = match authority.rsplit_once(':') {
        Some((host, port)) => (host, Some(port)),
        None => (authority, None),
    };
    !host.is_empty()
        && !host.starts_with('.')
        && !host.ends_with('.')
        && !host.bytes().any(|byte| {
            byte.is_ascii_whitespace()
                || byte < 0x20
                || matches!(byte, b'/' | b'?' | b'#' | b'[' | b']')
        })
        && port.map(is_valid_server_resource_pack_port).unwrap_or(true)
}

fn is_valid_server_resource_pack_port(port: &str) -> bool {
    !port.is_empty()
        && port.bytes().all(|byte| byte.is_ascii_digit())
        && port.parse::<u16>().is_ok()
}

fn parse_server_resource_pack_sha1_hash(hash: &str) -> Option<String> {
    if hash.len() == 40 && hash.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        Some(hash.to_ascii_lowercase())
    } else {
        None
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServerResourcePackStatus {
    Pending,
    Accepted,
    Downloading,
    Downloaded,
    Opened,
    Applied,
    Failed(ServerResourcePackFailure),
    Declined,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServerResourcePackFailure {
    Download,
    HashMismatch,
    Open,
    InvalidUrl,
    Reload,
    Discarded,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServerResourcePackAckAction {
    SuccessfullyLoaded,
    Declined,
    FailedDownload,
    Accepted,
    Downloaded,
    InvalidUrl,
    FailedReload,
    Discarded,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ServerResourcePackAck {
    pub id: Uuid,
    pub action: ServerResourcePackAckAction,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ServerResourcePackApplyState {
    request: ServerResourcePackRequest,
    status: ServerResourcePackStatus,
    downloaded: Option<ServerResourcePackDownload>,
}

impl ServerResourcePackApplyState {
    pub fn new(request: ServerResourcePackRequest) -> Self {
        Self {
            request,
            status: ServerResourcePackStatus::Pending,
            downloaded: None,
        }
    }

    pub fn request(&self) -> &ServerResourcePackRequest {
        &self.request
    }

    pub fn status(&self) -> ServerResourcePackStatus {
        self.status
    }

    pub fn validate_url(&self) -> Result<(), ServerResourcePackValidationError> {
        self.request.validate_url()
    }

    pub fn parsed_sha1_hash(&self) -> Option<String> {
        self.request.parsed_sha1_hash()
    }

    pub fn validation(&self) -> ServerResourcePackValidation {
        self.request.validation()
    }

    pub fn accept(&mut self) -> ServerResourcePackAck {
        self.status = ServerResourcePackStatus::Accepted;
        self.ack(ServerResourcePackAckAction::Accepted)
    }

    pub fn decline(&mut self) -> Result<ServerResourcePackAck, ServerResourcePackApplyError> {
        if self.request.required {
            return Err(ServerResourcePackApplyError::RequiredPackCannotBeDeclined {
                id: self.request.id,
            });
        }

        self.status = ServerResourcePackStatus::Declined;
        Ok(self.ack(ServerResourcePackAckAction::Declined))
    }

    pub fn start_download(&mut self) {
        self.status = ServerResourcePackStatus::Downloading;
        self.downloaded = None;
    }

    pub fn download_succeeded(&mut self) -> ServerResourcePackAck {
        self.status = ServerResourcePackStatus::Downloaded;
        self.ack(ServerResourcePackAckAction::Downloaded)
    }

    pub fn download_bytes_succeeded(
        &mut self,
        bytes: &[u8],
    ) -> Result<ServerResourcePackAck, ServerResourcePackAck> {
        let actual_sha1 = server_resource_pack_sha1_hex(bytes);
        if !self.hash_matches(&actual_sha1) {
            return Err(self.hash_mismatch());
        }

        self.downloaded = Some(ServerResourcePackDownload::Bytes {
            len: bytes.len(),
            sha1: actual_sha1,
        });
        Ok(self.download_succeeded())
    }

    pub fn deterministic_cache_path(
        &self,
        cache_dir: impl AsRef<Path>,
        actual_sha1: &str,
    ) -> PathBuf {
        cache_dir
            .as_ref()
            .join(self.request.id.to_string())
            .join(actual_sha1)
    }

    pub fn cache_downloaded_bytes(
        &mut self,
        cache_dir: impl AsRef<Path>,
        bytes: &[u8],
    ) -> Result<ServerResourcePackAck, ServerResourcePackCacheError> {
        self.cache_downloaded_bytes_with_report(cache_dir, bytes)
            .map(|report| report.ack)
    }

    pub fn cache_downloaded_bytes_with_report(
        &mut self,
        cache_dir: impl AsRef<Path>,
        bytes: &[u8],
    ) -> Result<ServerResourcePackCacheReport, ServerResourcePackCacheError> {
        self.validate_download_size(bytes.len())?;

        let actual_sha1 = server_resource_pack_sha1_hex(bytes);
        if !self.hash_matches(&actual_sha1) {
            return Err(ServerResourcePackCacheError::Rejected(self.hash_mismatch()));
        }

        let path = self.deterministic_cache_path(cache_dir, &actual_sha1);
        let reused_existing_file = match cached_server_resource_pack_matches(&path, &actual_sha1) {
            Ok(reused_existing_file) => reused_existing_file,
            Err(source) => {
                let ack = self.download_failed();
                return Err(ServerResourcePackCacheError::Io { ack, source });
            }
        };

        if !reused_existing_file {
            if let Err(source) = write_server_resource_pack_cache_file(&path, bytes) {
                let ack = self.download_failed();
                return Err(ServerResourcePackCacheError::Io { ack, source });
            }
        }

        self.downloaded = Some(ServerResourcePackDownload::Path {
            path: path.clone(),
            len: bytes.len(),
            sha1: actual_sha1.clone(),
        });
        let ack = self.download_succeeded();
        Ok(ServerResourcePackCacheReport {
            ack,
            path,
            len: bytes.len(),
            sha1: actual_sha1,
            reused_existing_file,
        })
    }

    pub fn download_path_succeeded(
        &mut self,
        path: impl Into<PathBuf>,
    ) -> Result<ServerResourcePackAck, ServerResourcePackAck> {
        let path = path.into();
        if path.is_dir() {
            if self.parsed_sha1_hash().is_some() {
                return Err(self.hash_mismatch());
            }
            self.downloaded = Some(ServerResourcePackDownload::Path {
                path,
                len: 0,
                sha1: String::new(),
            });
            return Ok(self.download_succeeded());
        }

        let bytes = match fs::read(&path) {
            Ok(bytes) => bytes,
            Err(_) => return Err(self.download_failed()),
        };
        let actual_sha1 = server_resource_pack_sha1_hex(&bytes);
        if !self.hash_matches(&actual_sha1) {
            return Err(self.hash_mismatch());
        }

        self.downloaded = Some(ServerResourcePackDownload::Path {
            path,
            len: bytes.len(),
            sha1: actual_sha1,
        });
        Ok(self.download_succeeded())
    }

    pub fn downloaded(&self) -> Option<&ServerResourcePackDownload> {
        self.downloaded.as_ref()
    }

    pub fn open_downloaded(&mut self) -> Result<(), ServerResourcePackAck> {
        if self.status != ServerResourcePackStatus::Downloaded {
            return Err(self.open_failed());
        }
        let Some(downloaded) = &self.downloaded else {
            return Err(self.open_failed());
        };
        if !downloaded.is_openable() {
            return Err(self.open_failed());
        }

        self.status = ServerResourcePackStatus::Opened;
        Ok(())
    }

    pub fn apply_downloaded(&mut self) -> ServerResourcePackAck {
        self.apply_opened()
    }

    pub fn apply_opened(&mut self) -> ServerResourcePackAck {
        if self.status != ServerResourcePackStatus::Opened {
            return self.reload_failed();
        }

        self.status = ServerResourcePackStatus::Applied;
        self.ack(ServerResourcePackAckAction::SuccessfullyLoaded)
    }

    pub fn download_failed(&mut self) -> ServerResourcePackAck {
        self.status = ServerResourcePackStatus::Failed(ServerResourcePackFailure::Download);
        self.downloaded = None;
        self.ack(ServerResourcePackAckAction::FailedDownload)
    }

    pub fn hash_mismatch(&mut self) -> ServerResourcePackAck {
        self.status = ServerResourcePackStatus::Failed(ServerResourcePackFailure::HashMismatch);
        self.downloaded = None;
        self.ack(ServerResourcePackAckAction::FailedDownload)
    }

    pub fn open_failed(&mut self) -> ServerResourcePackAck {
        self.status = ServerResourcePackStatus::Failed(ServerResourcePackFailure::Open);
        self.ack(ServerResourcePackAckAction::FailedReload)
    }

    pub fn invalid_url(&mut self) -> ServerResourcePackAck {
        self.status = ServerResourcePackStatus::Failed(ServerResourcePackFailure::InvalidUrl);
        self.ack(ServerResourcePackAckAction::InvalidUrl)
    }

    pub fn reload_failed(&mut self) -> ServerResourcePackAck {
        self.status = ServerResourcePackStatus::Failed(ServerResourcePackFailure::Reload);
        self.ack(ServerResourcePackAckAction::FailedReload)
    }

    pub fn discarded(&mut self) -> ServerResourcePackAck {
        self.status = ServerResourcePackStatus::Failed(ServerResourcePackFailure::Discarded);
        self.downloaded = None;
        self.ack(ServerResourcePackAckAction::Discarded)
    }

    pub fn placeholder_pack(&self) -> ClientResourcePack {
        ClientResourcePack::server_placeholder(self.request.id)
    }

    pub fn resource_pack(&self) -> ClientResourcePack {
        self.downloaded
            .as_ref()
            .and_then(ServerResourcePackDownload::path)
            .map(|path| ClientResourcePack::server(self.request.id, path))
            .unwrap_or_else(|| self.placeholder_pack())
    }

    fn ack(&self, action: ServerResourcePackAckAction) -> ServerResourcePackAck {
        ServerResourcePackAck {
            id: self.request.id,
            action,
        }
    }

    fn hash_matches(&self, actual_sha1: &str) -> bool {
        self.parsed_sha1_hash()
            .map(|expected| expected == actual_sha1)
            .unwrap_or(true)
    }

    fn validate_download_size(&mut self, len: usize) -> Result<(), ServerResourcePackCacheError> {
        if server_resource_pack_download_size_is_allowed(len) {
            return Ok(());
        }

        let ack = self.download_failed();
        Err(ServerResourcePackCacheError::TooLarge {
            ack,
            len,
            max: SERVER_RESOURCE_PACK_MAX_DOWNLOAD_BYTES,
        })
    }

    #[cfg(feature = "online-mode")]
    pub async fn download_and_cache(
        &mut self,
        client: &reqwest::Client,
        cache_dir: impl AsRef<Path>,
    ) -> Result<ServerResourcePackCacheReport, ServerResourcePackCacheError> {
        if self.validate_url().is_err() {
            return Err(ServerResourcePackCacheError::Rejected(self.invalid_url()));
        }

        let url = self.request.url().to_owned();
        self.start_download();

        let mut response = client
            .get(url)
            .send()
            .await
            .map_err(|source| self.download_error(source))?
            .error_for_status()
            .map_err(|source| self.download_error(source))?;

        if !server_resource_pack_content_length_is_allowed(response.content_length()) {
            let len = response
                .content_length()
                .and_then(|len| usize::try_from(len).ok())
                .unwrap_or(usize::MAX);
            return Err(self.download_too_large(len));
        }

        let capacity = response
            .content_length()
            .and_then(|len| usize::try_from(len).ok())
            .unwrap_or(0);
        let mut bytes = Vec::with_capacity(capacity);

        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|source| self.download_error(source))?
        {
            let Some(next_len) =
                server_resource_pack_download_len_after_chunk(bytes.len(), chunk.len())
            else {
                return Err(self.download_too_large(bytes.len().saturating_add(chunk.len())));
            };
            bytes.reserve(next_len - bytes.len());
            bytes.extend_from_slice(&chunk);
        }

        self.cache_downloaded_bytes_with_report(cache_dir, &bytes)
    }

    #[cfg(feature = "online-mode")]
    fn download_too_large(&mut self, len: usize) -> ServerResourcePackCacheError {
        let ack = self.download_failed();
        ServerResourcePackCacheError::TooLarge {
            ack,
            len,
            max: SERVER_RESOURCE_PACK_MAX_DOWNLOAD_BYTES,
        }
    }

    #[cfg(feature = "online-mode")]
    fn download_error(&mut self, source: reqwest::Error) -> ServerResourcePackCacheError {
        let ack = self.download_failed();
        ServerResourcePackCacheError::Download { ack, source }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ServerResourcePackApplyError {
    RequiredPackCannotBeDeclined { id: Uuid },
}

#[derive(Debug)]
pub enum ServerResourcePackCacheError {
    Rejected(ServerResourcePackAck),
    TooLarge {
        ack: ServerResourcePackAck,
        len: usize,
        max: usize,
    },
    #[cfg(feature = "online-mode")]
    Download {
        ack: ServerResourcePackAck,
        source: reqwest::Error,
    },
    Io {
        ack: ServerResourcePackAck,
        source: io::Error,
    },
}

impl ServerResourcePackCacheError {
    pub fn ack(&self) -> ServerResourcePackAck {
        match self {
            Self::Rejected(ack) | Self::TooLarge { ack, .. } | Self::Io { ack, .. } => *ack,
            #[cfg(feature = "online-mode")]
            Self::Download { ack, .. } => *ack,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ServerResourcePackCacheReport {
    pub ack: ServerResourcePackAck,
    pub path: PathBuf,
    pub len: usize,
    pub sha1: String,
    pub reused_existing_file: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ServerResourcePackDownload {
    Bytes {
        len: usize,
        sha1: String,
    },
    Path {
        path: PathBuf,
        len: usize,
        sha1: String,
    },
}

impl ServerResourcePackDownload {
    pub fn len(&self) -> usize {
        match self {
            Self::Bytes { len, .. } | Self::Path { len, .. } => *len,
        }
    }

    pub fn sha1(&self) -> &str {
        match self {
            Self::Bytes { sha1, .. } | Self::Path { sha1, .. } => sha1,
        }
    }

    pub fn path(&self) -> Option<&Path> {
        match self {
            Self::Bytes { .. } => None,
            Self::Path { path, .. } => Some(path),
        }
    }

    fn is_openable(&self) -> bool {
        match self {
            Self::Bytes { .. } => true,
            Self::Path { path, .. } if path.is_dir() => {
                server_resource_pack_directory_metadata_is_valid(path)
            }
            Self::Path { path, .. } => server_resource_pack_zip_metadata_is_valid(path),
        }
    }
}

#[derive(Debug, Deserialize)]
struct ClientResourcePackMetadata {
    pack: ClientResourcePackMetadataSection,
}

#[derive(Debug, Deserialize)]
struct ClientResourcePackMetadataSection {
    description: serde_json::Value,
    min_format: u32,
    max_format: u32,
}

fn server_resource_pack_directory_metadata_is_valid(path: &Path) -> bool {
    fs::read_to_string(path.join(PACK_MCMETA_RESOURCE))
        .ok()
        .and_then(|metadata| validate_client_resource_pack_metadata(&metadata).ok())
        .is_some()
}

fn server_resource_pack_zip_metadata_is_valid(path: &Path) -> bool {
    let Ok(file) = fs::File::open(path) else {
        return false;
    };
    let Ok(mut archive) = zip::ZipArchive::new(file) else {
        return false;
    };
    let Ok(mut metadata_file) = archive.by_name(PACK_MCMETA_RESOURCE) else {
        return false;
    };
    let mut metadata = String::new();
    if metadata_file.read_to_string(&mut metadata).is_err() {
        return false;
    }

    validate_client_resource_pack_metadata(&metadata).is_ok()
}

fn validate_client_resource_pack_metadata(
    metadata: &str,
) -> Result<ClientResourcePackMetadata, ClientResourcePackMetadataError> {
    let metadata = serde_json::from_str::<ClientResourcePackMetadata>(metadata)?;
    let _ = &metadata.pack.description;
    if metadata.pack.min_format > CLIENT_RESOURCE_PACK_FORMAT
        || metadata.pack.max_format < CLIENT_RESOURCE_PACK_FORMAT
        || metadata.pack.min_format > metadata.pack.max_format
    {
        return Err(ClientResourcePackMetadataError::UnsupportedFormat);
    }
    Ok(metadata)
}

#[derive(Debug)]
enum ClientResourcePackMetadataError {
    Json,
    UnsupportedFormat,
}

impl From<serde_json::Error> for ClientResourcePackMetadataError {
    fn from(_error: serde_json::Error) -> Self {
        Self::Json
    }
}

fn server_resource_pack_sha1_hex(bytes: &[u8]) -> String {
    let digest = azalea_crypto::digest_data(bytes, &[], &[]);
    digest
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>()
}

fn server_resource_pack_download_size_is_allowed(len: usize) -> bool {
    len <= SERVER_RESOURCE_PACK_MAX_DOWNLOAD_BYTES
}

#[cfg(feature = "online-mode")]
fn server_resource_pack_content_length_is_allowed(content_length: Option<u64>) -> bool {
    content_length
        .map(|len| len <= SERVER_RESOURCE_PACK_MAX_DOWNLOAD_BYTES as u64)
        .unwrap_or(true)
}

#[cfg(feature = "online-mode")]
fn server_resource_pack_download_len_after_chunk(
    current_len: usize,
    chunk_len: usize,
) -> Option<usize> {
    current_len
        .checked_add(chunk_len)
        .filter(|len| server_resource_pack_download_size_is_allowed(*len))
}

fn cached_server_resource_pack_matches(path: &Path, expected_sha1: &str) -> io::Result<bool> {
    if !path.exists() {
        return Ok(false);
    }

    let bytes = fs::read(path)?;
    Ok(server_resource_pack_download_size_is_allowed(bytes.len())
        && server_resource_pack_sha1_hex(&bytes) == expected_sha1)
}

fn write_server_resource_pack_cache_file(path: &Path, bytes: &[u8]) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, bytes)
}

impl fmt::Display for ServerResourcePackApplyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequiredPackCannotBeDeclined { id } => {
                write!(f, "required server resource pack `{id}` cannot be declined")
            }
        }
    }
}

impl std::error::Error for ServerResourcePackApplyError {}

impl fmt::Display for ServerResourcePackCacheError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rejected(ack) => write!(
                f,
                "server resource pack `{}` rejected cache write with {:?}",
                ack.id, ack.action
            ),
            Self::TooLarge { ack, len, max } => write!(
                f,
                "server resource pack `{}` cache write failed with {:?}: {len} bytes exceeds {max} bytes",
                ack.id, ack.action
            ),
            Self::Io { ack, source } => write!(
                f,
                "server resource pack `{}` cache write failed with {:?}: {source}",
                ack.id, ack.action
            ),
            #[cfg(feature = "online-mode")]
            Self::Download { ack, source } => write!(
                f,
                "server resource pack `{}` download failed with {:?}: {source}",
                ack.id, ack.action
            ),
        }
    }
}

impl std::error::Error for ServerResourcePackCacheError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Rejected(_) | Self::TooLarge { .. } => None,
            Self::Io { source, .. } => Some(source),
            #[cfg(feature = "online-mode")]
            Self::Download { source, .. } => Some(source),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ServerResourcePackApplyModel {
    base_stack: ClientResourceStack,
    packs: Vec<ServerResourcePackApplyState>,
}

impl ServerResourcePackApplyModel {
    pub fn new(base_stack: ClientResourceStack) -> Self {
        Self {
            base_stack,
            packs: Vec::new(),
        }
    }

    pub fn with_vanilla() -> Self {
        Self::new(ClientResourceStack::vanilla())
    }

    pub fn receive(
        &mut self,
        request: ServerResourcePackRequest,
    ) -> &mut ServerResourcePackApplyState {
        self.packs.push(ServerResourcePackApplyState::new(request));
        self.packs
            .last_mut()
            .expect("just-pushed server resource pack should exist")
    }

    pub fn packs(&self) -> &[ServerResourcePackApplyState] {
        &self.packs
    }

    pub fn resource_stack(&self) -> ClientResourceStack {
        let mut packs = self.base_stack.packs().to_vec();
        packs.extend(
            self.packs
                .iter()
                .filter(|pack| pack.status() == ServerResourcePackStatus::Applied)
                .map(ServerResourcePackApplyState::resource_pack),
        );
        ClientResourceStack::new(packs)
    }

    pub fn pop(&mut self, id: Uuid) -> bool {
        let original_len = self.packs.len();
        self.packs.retain(|pack| pack.request().id() != id);
        self.packs.len() != original_len
    }

    pub fn pop_all(&mut self) -> bool {
        let had_packs = !self.packs.is_empty();
        self.packs.clear();
        had_packs
    }
}

impl Default for ServerResourcePackApplyModel {
    fn default() -> Self {
        Self::with_vanilla()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClientResourceRepository {
    vanilla_pack: ClientResourcePack,
    available_packs: BTreeMap<String, AvailableClientResourcePack>,
    selected_pack_ids: Vec<String>,
}

impl ClientResourceRepository {
    pub fn new(vanilla_pack: ClientResourcePack) -> Self {
        let vanilla_id = vanilla_pack.id().to_owned();
        let mut available_packs = BTreeMap::new();
        available_packs.insert(
            vanilla_id,
            AvailableClientResourcePack::new(vanilla_pack.clone())
                .with_known_pack_id(KnownPackId::vanilla()),
        );

        Self {
            vanilla_pack,
            available_packs,
            selected_pack_ids: Vec::new(),
        }
    }

    pub fn committed_vanilla() -> Self {
        Self::new(ClientResourcePack::vanilla())
    }

    pub fn vanilla_pack(&self) -> &ClientResourcePack {
        &self.vanilla_pack
    }

    pub fn available_packs(&self) -> impl Iterator<Item = &AvailableClientResourcePack> {
        self.available_packs.values()
    }

    pub fn available_pack(&self, id: &str) -> Option<&AvailableClientResourcePack> {
        self.available_packs.get(id)
    }

    pub fn selected_pack_ids(&self) -> &[String] {
        &self.selected_pack_ids
    }

    pub fn with_available_pack(mut self, pack: AvailableClientResourcePack) -> Self {
        self.add_available_pack(pack);
        self
    }

    pub fn add_available_pack(&mut self, pack: AvailableClientResourcePack) {
        if pack.id() == self.vanilla_pack.id() {
            return;
        }

        self.available_packs.insert(pack.id().to_owned(), pack);
    }

    pub fn with_selected_pack_ids(
        mut self,
        selected_pack_ids: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.set_selected_pack_ids(selected_pack_ids);
        self
    }

    pub fn set_selected_pack_ids(
        &mut self,
        selected_pack_ids: impl IntoIterator<Item = impl Into<String>>,
    ) {
        self.selected_pack_ids = selected_pack_ids.into_iter().map(Into::into).collect();
    }

    pub fn stack(&self) -> ClientResourceStack {
        self.rebuild_stack().stack
    }

    pub fn rebuild_stack(&self) -> ClientResourcePackSelectionReport {
        let mut packs = vec![self.vanilla_pack.clone()];
        let mut selected_pack_ids = Vec::new();
        let mut missing_selected_pack_ids = Vec::new();
        let mut seen = BTreeSet::new();

        for id in &self.selected_pack_ids {
            if id == self.vanilla_pack.id() {
                continue;
            }

            if !seen.insert(id.clone()) {
                continue;
            }

            match self.available_packs.get(id) {
                Some(available) => {
                    selected_pack_ids.push(id.clone());
                    packs.push(available.pack().clone());
                }
                None => missing_selected_pack_ids.push(id.clone()),
            }
        }

        ClientResourcePackSelectionReport {
            stack: ClientResourceStack::new(packs),
            selected_pack_ids,
            missing_selected_pack_ids,
        }
    }

    pub fn known_pack_ids(&self) -> Vec<KnownPackId> {
        let mut known_pack_ids = Vec::new();

        if let Some(vanilla) = self
            .available_packs
            .get(self.vanilla_pack.id())
            .and_then(AvailableClientResourcePack::known_pack_id)
        {
            known_pack_ids.push(vanilla.clone());
        }

        for id in &self.rebuild_stack().selected_pack_ids {
            if let Some(known_pack_id) = self
                .available_packs
                .get(id)
                .and_then(AvailableClientResourcePack::known_pack_id)
            {
                known_pack_ids.push(known_pack_id.clone());
            }
        }

        known_pack_ids
    }

    pub fn recognized_known_pack_ids<'a>(
        &self,
        offered: impl IntoIterator<Item = &'a KnownPackId>,
    ) -> Vec<KnownPackId> {
        let known_pack_ids = self.known_pack_ids();
        offered
            .into_iter()
            .filter(|known_pack_id| known_pack_ids.contains(known_pack_id))
            .cloned()
            .collect()
    }
}

impl Default for ClientResourceRepository {
    fn default() -> Self {
        Self::committed_vanilla()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AvailableClientResourcePack {
    pack: ClientResourcePack,
    known_pack_id: Option<KnownPackId>,
}

impl AvailableClientResourcePack {
    pub fn new(pack: ClientResourcePack) -> Self {
        Self {
            pack,
            known_pack_id: None,
        }
    }

    pub fn with_known_pack_id(mut self, known_pack_id: KnownPackId) -> Self {
        self.known_pack_id = Some(known_pack_id);
        self
    }

    pub fn id(&self) -> &str {
        self.pack.id()
    }

    pub fn pack(&self) -> &ClientResourcePack {
        &self.pack
    }

    pub fn known_pack_id(&self) -> Option<&KnownPackId> {
        self.known_pack_id.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KnownPackId {
    pub namespace: String,
    pub id: String,
    pub version: String,
}

impl KnownPackId {
    pub fn new(
        namespace: impl Into<String>,
        id: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        Self {
            namespace: namespace.into(),
            id: id.into(),
            version: version.into(),
        }
    }

    pub fn vanilla() -> Self {
        Self::new(
            VANILLA_KNOWN_PACK_NAMESPACE,
            VANILLA_KNOWN_PACK_ID,
            VANILLA_KNOWN_PACK_VERSION,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClientResourcePackSelectionReport {
    stack: ClientResourceStack,
    selected_pack_ids: Vec<String>,
    missing_selected_pack_ids: Vec<String>,
}

impl ClientResourcePackSelectionReport {
    pub fn stack(&self) -> &ClientResourceStack {
        &self.stack
    }

    pub fn into_stack(self) -> ClientResourceStack {
        self.stack
    }

    pub fn selected_pack_ids(&self) -> &[String] {
        &self.selected_pack_ids
    }

    pub fn missing_selected_pack_ids(&self) -> &[String] {
        &self.missing_selected_pack_ids
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceLocation {
    pub pack_id: String,
    pub path: PathBuf,
    source: ResourceLocationSource,
}

impl ResourceLocation {
    pub fn read_bytes(&self) -> io::Result<Vec<u8>> {
        self.source.read_bytes()
    }

    pub fn read_to_string(&self) -> io::Result<String> {
        self.source.read_to_string()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ResourceLocationSource {
    Directory(PathBuf),
    RootZip { root: PathBuf, resource: String },
}

impl ResourceLocationSource {
    fn read_bytes(&self) -> io::Result<Vec<u8>> {
        match self {
            Self::Directory(path) => fs::read(path),
            Self::RootZip { root, resource } => read_root_zip_resource(root, resource),
        }
    }

    fn read_to_string(&self) -> io::Result<String> {
        let bytes = self.read_bytes()?;
        String::from_utf8(bytes).map_err(|error| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("resource is not valid utf-8: {error}"),
            )
        })
    }
}

fn root_zip_contains_resource(root: &Path, resource: &Path) -> bool {
    let Ok(file) = fs::File::open(root) else {
        return false;
    };
    let Ok(mut archive) = zip::ZipArchive::new(file) else {
        return false;
    };
    archive.by_name(&zip_resource_name(resource)).is_ok()
}

fn read_root_zip_resource(root: &Path, resource: &str) -> io::Result<Vec<u8>> {
    let file = fs::File::open(root)?;
    let mut archive = zip::ZipArchive::new(file).map_err(zip_error_to_io)?;
    let mut entry = archive.by_name(resource).map_err(zip_error_to_io)?;
    let mut bytes = Vec::new();
    entry.read_to_end(&mut bytes)?;
    Ok(bytes)
}

fn zip_resource_name(resource: &Path) -> String {
    resource.to_string_lossy().replace('\\', "/")
}

fn zip_error_to_io(error: zip::result::ZipError) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, error)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceReloadPlan {
    listeners: Vec<String>,
    total_weight: u32,
}

impl ResourceReloadPlan {
    pub fn new(listener_names: impl IntoIterator<Item = impl Into<String>>) -> Self {
        let listeners = listener_names
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>();
        let total_weight = ResourceReloadStep::InitialPreparation.weight()
            + listeners.len() as u32 * ResourceReloadStep::per_listener_weight();

        Self {
            listeners,
            total_weight,
        }
    }

    pub fn from_listeners(listeners: &[Box<dyn ResourceReloadListener>]) -> Self {
        Self::new(listeners.iter().map(|listener| listener.name()))
    }

    pub fn listeners(&self) -> &[String] {
        &self.listeners
    }

    pub fn total_weight(&self) -> u32 {
        self.total_weight
    }

    pub fn initial_task_weight(&self) -> u32 {
        ResourceReloadStep::InitialPreparation.weight()
    }

    pub fn progress_snapshot(&self, completed_weight: u32) -> ResourceReloadProgressSnapshot {
        ResourceReloadProgressSnapshot::new(self, completed_weight)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceReloadState {
    plan: ResourceReloadPlan,
    completed_weight: u32,
    started_prepare_tasks: u32,
    finished_prepare_tasks: u32,
    started_reload_tasks: u32,
    finished_reload_tasks: u32,
    completed_listeners: u32,
    current_listener: Option<String>,
    current_step: Option<ResourceReloadStep>,
}

impl ResourceReloadState {
    pub fn new(plan: ResourceReloadPlan) -> Self {
        Self {
            plan,
            completed_weight: 0,
            started_prepare_tasks: 0,
            finished_prepare_tasks: 0,
            started_reload_tasks: 0,
            finished_reload_tasks: 0,
            completed_listeners: 0,
            current_listener: None,
            current_step: None,
        }
    }

    pub fn plan(&self) -> &ResourceReloadPlan {
        &self.plan
    }

    pub fn completed_weight(&self) -> u32 {
        self.completed_weight
    }

    pub fn current_listener(&self) -> Option<&str> {
        self.current_listener.as_deref()
    }

    pub fn current_step(&self) -> Option<ResourceReloadStep> {
        self.current_step
    }

    pub fn progress(&self) -> f32 {
        if self.plan.total_weight == 0 {
            1.0
        } else {
            self.completed_weight as f32 / self.plan.total_weight as f32
        }
    }

    pub fn progress_snapshot(&self) -> ResourceReloadProgressSnapshot {
        ResourceReloadProgressSnapshot::from_state(self)
    }

    fn finish_step(&mut self, listener: &str, step: ResourceReloadStep) {
        self.current_listener = Some(listener.to_owned());
        self.current_step = Some(step);
        self.completed_weight += step.weight();

        match step {
            ResourceReloadStep::InitialPreparation => {
                self.started_prepare_tasks = self.plan.listeners().len() as u32;
            }
            ResourceReloadStep::Preparation => {
                self.finished_prepare_tasks += 1;
            }
            ResourceReloadStep::Reload => {
                self.started_reload_tasks += 1;
                self.finished_reload_tasks += 1;
            }
            ResourceReloadStep::ListenerComplete => {
                self.completed_listeners += 1;
            }
        }
    }

    fn finish_initial_task(&mut self) {
        self.finish_step(
            INITIAL_RELOAD_TASK_NAME,
            ResourceReloadStep::InitialPreparation,
        );
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResourceReloadProgressSnapshot {
    entries: Vec<ResourceReloadProgressEntry>,
    completed_weight: u32,
    total_weight: u32,
    started_prepare_tasks: u32,
    finished_prepare_tasks: u32,
    started_reload_tasks: u32,
    finished_reload_tasks: u32,
    completed_listeners: u32,
    listener_count: u32,
}

impl ResourceReloadProgressSnapshot {
    pub fn new(plan: &ResourceReloadPlan, completed_weight: u32) -> Self {
        Self::from_completed_weight(plan, completed_weight)
    }

    fn from_completed_weight(plan: &ResourceReloadPlan, completed_weight: u32) -> Self {
        let completed_weight = completed_weight.min(plan.total_weight());
        let listener_count = plan.listeners().len() as u32;
        let started_prepare_tasks =
            if completed_weight >= ResourceReloadStep::InitialPreparation.weight() {
                listener_count
            } else {
                0
            };
        let listener_progress_weight =
            completed_weight.saturating_sub(ResourceReloadStep::InitialPreparation.weight());
        let finished_prepare_tasks = (listener_progress_weight
            / ResourceReloadStep::per_listener_weight())
        .min(listener_count);
        let started_reload_tasks = finished_prepare_tasks;
        let finished_reload_tasks = finished_prepare_tasks;
        let completed_listeners = (listener_progress_weight
            / ResourceReloadStep::per_listener_weight())
        .min(listener_count);

        Self::from_parts(
            plan,
            completed_weight,
            started_prepare_tasks,
            finished_prepare_tasks,
            started_reload_tasks,
            finished_reload_tasks,
            completed_listeners,
        )
    }

    fn from_state(state: &ResourceReloadState) -> Self {
        Self::from_parts(
            state.plan(),
            state.completed_weight(),
            state.started_prepare_tasks,
            state.finished_prepare_tasks,
            state.started_reload_tasks,
            state.finished_reload_tasks,
            state.completed_listeners,
        )
    }

    fn from_parts(
        plan: &ResourceReloadPlan,
        completed_weight: u32,
        started_prepare_tasks: u32,
        finished_prepare_tasks: u32,
        started_reload_tasks: u32,
        finished_reload_tasks: u32,
        completed_listeners: u32,
    ) -> Self {
        let completed_weight = completed_weight.min(plan.total_weight());
        let mut offset = 0;
        let mut entries = Vec::with_capacity(1 + plan.listeners().len() * 3);

        push_reload_progress_entry(
            &mut entries,
            INITIAL_RELOAD_TASK_NAME,
            ResourceReloadStep::InitialPreparation,
            completed_weight,
            &mut offset,
        );

        for listener in plan.listeners() {
            push_reload_progress_entry(
                &mut entries,
                listener,
                ResourceReloadStep::Preparation,
                completed_weight,
                &mut offset,
            );
            push_reload_progress_entry(
                &mut entries,
                listener,
                ResourceReloadStep::Reload,
                completed_weight,
                &mut offset,
            );
            push_reload_progress_entry(
                &mut entries,
                listener,
                ResourceReloadStep::ListenerComplete,
                completed_weight,
                &mut offset,
            );
        }

        Self {
            entries,
            completed_weight,
            total_weight: plan.total_weight(),
            started_prepare_tasks,
            finished_prepare_tasks,
            started_reload_tasks,
            finished_reload_tasks,
            completed_listeners,
            listener_count: plan.listeners().len() as u32,
        }
    }

    pub fn entries(&self) -> &[ResourceReloadProgressEntry] {
        &self.entries
    }

    pub fn completed_weight(&self) -> u32 {
        self.completed_weight
    }

    pub fn total_weight(&self) -> u32 {
        self.total_weight
    }

    pub fn started_prepare_tasks(&self) -> u32 {
        self.started_prepare_tasks
    }

    pub fn finished_prepare_tasks(&self) -> u32 {
        self.finished_prepare_tasks
    }

    pub fn started_reload_tasks(&self) -> u32 {
        self.started_reload_tasks
    }

    pub fn finished_reload_tasks(&self) -> u32 {
        self.finished_reload_tasks
    }

    pub fn completed_listeners(&self) -> u32 {
        self.completed_listeners
    }

    pub fn listener_count(&self) -> u32 {
        self.listener_count
    }

    pub fn actual_progress(&self) -> f32 {
        let completed = self.finished_prepare_tasks * ResourceReloadStep::Preparation.weight()
            + self.finished_reload_tasks * ResourceReloadStep::Reload.weight()
            + self.completed_listeners * ResourceReloadStep::ListenerComplete.weight();
        let started = self.started_prepare_tasks * ResourceReloadStep::Preparation.weight()
            + self.started_reload_tasks * ResourceReloadStep::Reload.weight()
            + self.listener_count * ResourceReloadStep::ListenerComplete.weight();

        if started == 0 {
            1.0
        } else {
            completed as f32 / started as f32
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResourceReloadProgressEntry {
    listener: String,
    step: ResourceReloadStep,
    progress: f32,
    weight: u32,
}

impl ResourceReloadProgressEntry {
    fn new(
        listener: impl Into<String>,
        step: ResourceReloadStep,
        progress: f32,
        weight: u32,
    ) -> Self {
        Self {
            listener: listener.into(),
            step,
            progress,
            weight,
        }
    }

    pub fn listener(&self) -> &str {
        &self.listener
    }

    pub fn step(&self) -> ResourceReloadStep {
        self.step
    }

    pub fn progress(&self) -> f32 {
        self.progress
    }

    pub fn weight(&self) -> u32 {
        self.weight
    }
}

fn push_reload_progress_entry(
    entries: &mut Vec<ResourceReloadProgressEntry>,
    listener: &str,
    step: ResourceReloadStep,
    completed_weight: u32,
    offset: &mut u32,
) {
    let weight = step.weight();
    let completed_in_step = completed_weight.saturating_sub(*offset).min(weight);
    let progress = if weight == 0 {
        1.0
    } else {
        completed_in_step as f32 / weight as f32
    };

    entries.push(ResourceReloadProgressEntry::new(
        listener, step, progress, weight,
    ));
    *offset += weight;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResourceReloadStep {
    InitialPreparation,
    Preparation,
    Reload,
    ListenerComplete,
}

impl ResourceReloadStep {
    pub const fn weight(self) -> u32 {
        match self {
            Self::InitialPreparation => 2,
            Self::Preparation => 2,
            Self::Reload => 2,
            Self::ListenerComplete => 1,
        }
    }

    pub const fn per_listener_weight() -> u32 {
        Self::Preparation.weight() + Self::Reload.weight() + Self::ListenerComplete.weight()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceReloadTaskReport {
    items: Vec<String>,
}

impl ResourceReloadTaskReport {
    pub fn new(items: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            items: items.into_iter().map(Into::into).collect(),
        }
    }

    pub fn empty() -> Self {
        Self { items: Vec::new() }
    }

    pub fn items(&self) -> &[String] {
        &self.items
    }
}

impl Default for ResourceReloadTaskReport {
    fn default() -> Self {
        Self::empty()
    }
}

pub trait ResourceReloadListener: fmt::Debug {
    fn name(&self) -> &str;

    fn prepare(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport>;

    fn reload(&self, stack: &ClientResourceStack)
    -> ResourceReloadResult<ResourceReloadTaskReport>;
}

#[derive(Debug)]
pub struct ResourceReloadManager {
    stack: ClientResourceStack,
    listeners: Vec<Box<dyn ResourceReloadListener>>,
}

impl ResourceReloadManager {
    pub fn new(stack: ClientResourceStack) -> Self {
        Self {
            stack,
            listeners: Vec::new(),
        }
    }

    pub fn with_default_vanilla_assets() -> Self {
        Self::new(ClientResourceStack::vanilla())
            .with_listener(RequiredVanillaAssetsListener::default())
    }

    pub fn with_default_client_resources(stack: ClientResourceStack) -> Self {
        let has_sound_events = stack.find_resource(SOUND_EVENTS_RESOURCE).is_some();
        let mut manager = Self::new(stack)
            .with_listener(ClientLanguageReloadListener::new(DEFAULT_LANGUAGE_CODE))
            .with_listener(TextureMetadataReloadListener::default())
            .with_listener(HeadlessShaderSourceReloadListener::default());

        if has_sound_events {
            manager = manager.with_listener(SoundEventsReloadListener::default());
        }

        manager
            .with_listener(SplashesReloadListener::default())
            .with_listener(AtlasSourceReloadListener::default())
            .with_listener(FontDefinitionsReloadListener::default())
            .with_listener(ColormapReloadListener::default())
            .with_listener(ModelDependencyReloadListener::default())
            .with_listener(EquipmentAssetsReloadListener::default())
            .with_listener(ParticleManifestReloadListener::default())
            .with_listener(WaypointStyleManifestReloadListener::default())
            .with_listener(CloudTextureReloadListener::default())
            .with_listener(GpuWarnlistReloadListener::default())
            .with_listener(RegionalComplianciesReloadListener::default())
    }

    pub fn with_default_vanilla_client_resources() -> Self {
        Self::with_default_client_resources(ClientResourceStack::vanilla())
    }

    pub fn with_listener(mut self, listener: impl ResourceReloadListener + 'static) -> Self {
        self.listeners.push(Box::new(listener));
        self
    }

    pub fn plan(&self) -> ResourceReloadPlan {
        ResourceReloadPlan::from_listeners(&self.listeners)
    }

    pub fn run(&self) -> ResourceReloadResult<ResourceReloadReport> {
        self.run_with_events(|_| {})
    }

    pub fn run_with_events(
        &self,
        mut on_event: impl FnMut(&ResourceReloadEvent),
    ) -> ResourceReloadResult<ResourceReloadReport> {
        let mut state = ResourceReloadState::new(self.plan());
        let mut events = Vec::new();
        let mut listener_reports = Vec::new();

        state.finish_initial_task();
        push_resource_reload_event(&state, &mut events, &mut on_event);

        for listener in &self.listeners {
            let name = listener.name();

            let preparation = listener.prepare(&self.stack)?;
            state.finish_step(name, ResourceReloadStep::Preparation);
            push_resource_reload_event(&state, &mut events, &mut on_event);

            let reload = listener.reload(&self.stack)?;
            state.finish_step(name, ResourceReloadStep::Reload);
            push_resource_reload_event(&state, &mut events, &mut on_event);

            state.finish_step(name, ResourceReloadStep::ListenerComplete);
            push_resource_reload_event(&state, &mut events, &mut on_event);

            listener_reports.push(CompletedResourceReloadListener {
                name: name.to_owned(),
                preparation,
                reload,
            });
        }

        Ok(ResourceReloadReport {
            state,
            events,
            listener_reports,
        })
    }
}

fn push_resource_reload_event(
    state: &ResourceReloadState,
    events: &mut Vec<ResourceReloadEvent>,
    on_event: &mut impl FnMut(&ResourceReloadEvent),
) {
    let event = ResourceReloadEvent::from_state(state);
    on_event(&event);
    events.push(event);
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResourceReloadEvent {
    pub listener: String,
    pub step: ResourceReloadStep,
    pub completed_weight: u32,
    pub progress: f32,
    pub progress_snapshot: ResourceReloadProgressSnapshot,
}

impl ResourceReloadEvent {
    fn from_state(state: &ResourceReloadState) -> Self {
        let progress_snapshot = state.progress_snapshot();
        Self {
            listener: state.current_listener.clone().unwrap_or_default(),
            step: state
                .current_step
                .unwrap_or(ResourceReloadStep::ListenerComplete),
            completed_weight: state.completed_weight,
            progress: progress_snapshot.actual_progress(),
            progress_snapshot,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResourceReloadReport {
    state: ResourceReloadState,
    events: Vec<ResourceReloadEvent>,
    listener_reports: Vec<CompletedResourceReloadListener>,
}

impl ResourceReloadReport {
    pub fn state(&self) -> &ResourceReloadState {
        &self.state
    }

    pub fn events(&self) -> &[ResourceReloadEvent] {
        &self.events
    }

    pub fn listener_reports(&self) -> &[CompletedResourceReloadListener] {
        &self.listener_reports
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompletedResourceReloadListener {
    pub name: String,
    pub preparation: ResourceReloadTaskReport,
    pub reload: ResourceReloadTaskReport,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClientJsonResource {
    value: serde_json::Value,
    report: ClientJsonResourceReloadReport,
}

impl ClientJsonResource {
    pub fn value(&self) -> &serde_json::Value {
        &self.value
    }

    pub fn report(&self) -> &ClientJsonResourceReloadReport {
        &self.report
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClientJsonResourceReloadReport {
    resource: String,
    pack_id: String,
    top_level_shape: ClientJsonTopLevelShape,
}

impl ClientJsonResourceReloadReport {
    fn new(
        resource: impl Into<String>,
        pack_id: impl Into<String>,
        top_level_shape: ClientJsonTopLevelShape,
    ) -> Self {
        Self {
            resource: resource.into(),
            pack_id: pack_id.into(),
            top_level_shape,
        }
    }

    pub fn resource(&self) -> &str {
        &self.resource
    }

    pub fn pack_id(&self) -> &str {
        &self.pack_id
    }

    pub fn loaded_resource_pack(&self) -> String {
        format!("{}@{}", self.resource, self.pack_id)
    }

    pub fn top_level_shape(&self) -> &ClientJsonTopLevelShape {
        &self.top_level_shape
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClientJsonTopLevelShape {
    Object { keys: Vec<String> },
    Array { len: usize },
    String,
    Number,
    Boolean,
    Null,
}

impl ClientJsonTopLevelShape {
    fn from_value(value: &serde_json::Value) -> Self {
        match value {
            serde_json::Value::Object(object) => Self::Object {
                keys: object.keys().cloned().collect(),
            },
            serde_json::Value::Array(array) => Self::Array { len: array.len() },
            serde_json::Value::String(_) => Self::String,
            serde_json::Value::Number(_) => Self::Number,
            serde_json::Value::Bool(_) => Self::Boolean,
            serde_json::Value::Null => Self::Null,
        }
    }

    fn report_fragment(&self) -> String {
        match self {
            Self::Object { keys } => format!("object keys:{}", keys.join(",")),
            Self::Array { len } => format!("array len:{len}"),
            Self::String => "string".to_owned(),
            Self::Number => "number".to_owned(),
            Self::Boolean => "boolean".to_owned(),
            Self::Null => "null".to_owned(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GpuWarnlistReloadListener {
    resource: String,
}

impl GpuWarnlistReloadListener {
    pub fn new(resource: impl Into<String>) -> Self {
        Self {
            resource: resource.into(),
        }
    }

    pub fn load(&self, stack: &ClientResourceStack) -> ResourceReloadResult<ClientJsonResource> {
        load_client_json_resource(stack, &self.resource)
    }
}

impl Default for GpuWarnlistReloadListener {
    fn default() -> Self {
        Self::new(GPU_WARNLIST_RESOURCE)
    }
}

impl ResourceReloadListener for GpuWarnlistReloadListener {
    fn name(&self) -> &str {
        "gpu_warnlist"
    }

    fn prepare(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        stack.require_resource(&self.resource)?;
        Ok(ResourceReloadTaskReport::new([self.resource.clone()]))
    }

    fn reload(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        let resource = self.load(stack)?;
        Ok(ResourceReloadTaskReport::new([format!(
            "{}:{}",
            resource.report().loaded_resource_pack(),
            resource.report().top_level_shape().report_fragment()
        )]))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegionalComplianciesReloadListener {
    resource: String,
}

impl RegionalComplianciesReloadListener {
    pub fn new(resource: impl Into<String>) -> Self {
        Self {
            resource: resource.into(),
        }
    }

    pub fn load(&self, stack: &ClientResourceStack) -> ResourceReloadResult<ClientJsonResource> {
        load_client_json_resource(stack, &self.resource)
    }
}

impl Default for RegionalComplianciesReloadListener {
    fn default() -> Self {
        Self::new(REGIONAL_COMPLIANCIES_RESOURCE)
    }
}

impl ResourceReloadListener for RegionalComplianciesReloadListener {
    fn name(&self) -> &str {
        "regional_compliancies"
    }

    fn prepare(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        stack.require_resource(&self.resource)?;
        Ok(ResourceReloadTaskReport::new([self.resource.clone()]))
    }

    fn reload(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        let resource = self.load(stack)?;
        Ok(ResourceReloadTaskReport::new([format!(
            "{}:{}",
            resource.report().loaded_resource_pack(),
            resource.report().top_level_shape().report_fragment()
        )]))
    }
}

pub fn load_client_json_resource(
    stack: &ClientResourceStack,
    resource: impl AsRef<Path>,
) -> ResourceReloadResult<ClientJsonResource> {
    let resource = resource.as_ref();
    let location = stack.require_resource(resource)?;
    read_client_json_resource(resource, &location)
}

fn read_client_json_resource(
    resource: &Path,
    location: &ResourceLocation,
) -> ResourceReloadResult<ClientJsonResource> {
    let contents =
        location
            .read_to_string()
            .map_err(|source| ResourceReloadError::ReadResource {
                resource: resource.to_string_lossy().into_owned(),
                path: location.path.clone(),
                source,
            })?;

    let value: serde_json::Value = serde_json::from_str(&contents).map_err(|source| {
        ResourceReloadError::ParseResourceJson {
            resource: resource.to_string_lossy().into_owned(),
            path: location.path.clone(),
            source,
        }
    })?;
    let report = ClientJsonResourceReloadReport::new(
        resource.to_string_lossy(),
        location.pack_id.clone(),
        ClientJsonTopLevelShape::from_value(&value),
    );

    Ok(ClientJsonResource { value, report })
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClientJsonManifestSet {
    manifests: Vec<ClientJsonManifest>,
}

impl ClientJsonManifestSet {
    pub fn manifests(&self) -> &[ClientJsonManifest] {
        &self.manifests
    }

    pub fn reports(&self) -> impl Iterator<Item = &ClientJsonResourceReloadReport> {
        self.manifests
            .iter()
            .map(|manifest| manifest.resource.report())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClientJsonManifest {
    id: String,
    resource: ClientJsonResource,
}

impl ClientJsonManifest {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn resource(&self) -> &ClientJsonResource {
        &self.resource
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClientEquipmentAssetSet {
    assets: Vec<ClientEquipmentAsset>,
}

impl ClientEquipmentAssetSet {
    pub fn assets(&self) -> &[ClientEquipmentAsset] {
        &self.assets
    }

    pub fn reports(&self) -> impl Iterator<Item = &ClientEquipmentAssetReloadReport> {
        self.assets.iter().map(ClientEquipmentAsset::report)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClientEquipmentAsset {
    id: String,
    layers: Vec<ClientEquipmentLayer>,
    report: ClientEquipmentAssetReloadReport,
}

impl ClientEquipmentAsset {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn layers(&self) -> &[ClientEquipmentLayer] {
        &self.layers
    }

    pub fn report(&self) -> &ClientEquipmentAssetReloadReport {
        &self.report
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClientEquipmentLayer {
    layer_type: String,
    entries: Vec<ClientEquipmentLayerEntry>,
}

impl ClientEquipmentLayer {
    pub fn layer_type(&self) -> &str {
        &self.layer_type
    }

    pub fn entries(&self) -> &[ClientEquipmentLayerEntry] {
        &self.entries
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClientEquipmentLayerEntry {
    texture: String,
    dyeable: Option<ClientEquipmentDyeable>,
    use_player_texture: Option<bool>,
}

impl ClientEquipmentLayerEntry {
    pub fn texture(&self) -> &str {
        &self.texture
    }

    pub fn dyeable(&self) -> Option<&ClientEquipmentDyeable> {
        self.dyeable.as_ref()
    }

    pub fn use_player_texture(&self) -> Option<bool> {
        self.use_player_texture
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClientEquipmentDyeable {
    color_when_undyed: Option<i32>,
}

impl ClientEquipmentDyeable {
    pub fn color_when_undyed(&self) -> Option<i32> {
        self.color_when_undyed
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClientEquipmentAssetReloadReport {
    resource: String,
    pack_id: String,
    layer_types: Vec<String>,
    layer_count: usize,
    entry_count: usize,
    textures: Vec<String>,
    texture_locations: Vec<String>,
    dyeable_entry_count: usize,
    player_texture_entry_count: usize,
}

impl ClientEquipmentAssetReloadReport {
    fn new(
        resource: impl Into<String>,
        pack_id: impl Into<String>,
        layers: &[ClientEquipmentLayer],
    ) -> Self {
        let layer_types = layers
            .iter()
            .map(|layer| layer.layer_type.clone())
            .collect::<Vec<_>>();
        let entries = layers
            .iter()
            .flat_map(|layer| layer.entries.iter())
            .collect::<Vec<_>>();
        let textures = entries
            .iter()
            .map(|entry| entry.texture.clone())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let texture_locations = layers
            .iter()
            .flat_map(|layer| {
                layer.entries.iter().map(|entry| {
                    equipment_layer_texture_location(
                        layer.layer_type.as_str(),
                        entry.texture.as_str(),
                    )
                })
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let dyeable_entry_count = entries
            .iter()
            .filter(|entry| entry.dyeable.is_some())
            .count();
        let player_texture_entry_count = entries
            .iter()
            .filter(|entry| entry.use_player_texture == Some(true))
            .count();

        Self {
            resource: resource.into(),
            pack_id: pack_id.into(),
            layer_types,
            layer_count: layers.len(),
            entry_count: entries.len(),
            textures,
            texture_locations,
            dyeable_entry_count,
            player_texture_entry_count,
        }
    }

    pub fn resource(&self) -> &str {
        &self.resource
    }

    pub fn pack_id(&self) -> &str {
        &self.pack_id
    }

    pub fn layer_types(&self) -> &[String] {
        &self.layer_types
    }

    pub fn layer_count(&self) -> usize {
        self.layer_count
    }

    pub fn entry_count(&self) -> usize {
        self.entry_count
    }

    pub fn textures(&self) -> &[String] {
        &self.textures
    }

    pub fn texture_locations(&self) -> &[String] {
        &self.texture_locations
    }

    pub fn dyeable_entry_count(&self) -> usize {
        self.dyeable_entry_count
    }

    pub fn player_texture_entry_count(&self) -> usize {
        self.player_texture_entry_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClientParticleDescriptionSet {
    descriptions: Vec<ClientParticleDescription>,
}

impl ClientParticleDescriptionSet {
    pub fn descriptions(&self) -> &[ClientParticleDescription] {
        &self.descriptions
    }

    pub fn reports(&self) -> impl Iterator<Item = &ClientParticleDescriptionReloadReport> {
        self.descriptions
            .iter()
            .map(ClientParticleDescription::report)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClientParticleDescription {
    id: String,
    sprites: Vec<String>,
    report: ClientParticleDescriptionReloadReport,
}

impl ClientParticleDescription {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn sprites(&self) -> &[String] {
        &self.sprites
    }

    pub fn report(&self) -> &ClientParticleDescriptionReloadReport {
        &self.report
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClientParticleDescriptionReloadReport {
    resource: String,
    pack_id: String,
    sprite_count: usize,
    sprites: Vec<String>,
}

impl ClientParticleDescriptionReloadReport {
    fn new(
        resource: impl Into<String>,
        pack_id: impl Into<String>,
        sprites: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        let sprites: Vec<String> = sprites.into_iter().map(Into::into).collect();
        Self {
            resource: resource.into(),
            pack_id: pack_id.into(),
            sprite_count: sprites.len(),
            sprites,
        }
    }

    pub fn resource(&self) -> &str {
        &self.resource
    }

    pub fn pack_id(&self) -> &str {
        &self.pack_id
    }

    pub fn sprite_count(&self) -> usize {
        self.sprite_count
    }

    pub fn sprites(&self) -> &[String] {
        &self.sprites
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClientWaypointStyleSet {
    styles: Vec<ClientWaypointStyle>,
}

impl ClientWaypointStyleSet {
    pub fn styles(&self) -> &[ClientWaypointStyle] {
        &self.styles
    }

    pub fn reports(&self) -> impl Iterator<Item = &ClientWaypointStyleReloadReport> {
        self.styles.iter().map(ClientWaypointStyle::report)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClientWaypointStyle {
    id: String,
    sprites: Vec<String>,
    sprite_locations: Vec<String>,
    near_distance: u32,
    far_distance: u32,
    report: ClientWaypointStyleReloadReport,
}

impl ClientWaypointStyle {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn sprites(&self) -> &[String] {
        &self.sprites
    }

    pub fn sprite_locations(&self) -> &[String] {
        &self.sprite_locations
    }

    pub fn near_distance(&self) -> u32 {
        self.near_distance
    }

    pub fn far_distance(&self) -> u32 {
        self.far_distance
    }

    pub fn report(&self) -> &ClientWaypointStyleReloadReport {
        &self.report
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClientWaypointStyleReloadReport {
    resource: String,
    pack_id: String,
    sprite_count: usize,
    sprites: Vec<String>,
    sprite_locations: Vec<String>,
    near_distance: u32,
    far_distance: u32,
}

impl ClientWaypointStyleReloadReport {
    fn new(
        resource: impl Into<String>,
        pack_id: impl Into<String>,
        sprites: impl IntoIterator<Item = impl Into<String>>,
        near_distance: u32,
        far_distance: u32,
    ) -> Self {
        let sprites: Vec<String> = sprites.into_iter().map(Into::into).collect();
        let sprite_locations = sprites
            .iter()
            .map(|sprite| waypoint_style_sprite_location(sprite))
            .collect::<Vec<_>>();
        Self {
            resource: resource.into(),
            pack_id: pack_id.into(),
            sprite_count: sprites.len(),
            sprites,
            sprite_locations,
            near_distance,
            far_distance,
        }
    }

    pub fn resource(&self) -> &str {
        &self.resource
    }

    pub fn pack_id(&self) -> &str {
        &self.pack_id
    }

    pub fn sprite_count(&self) -> usize {
        self.sprite_count
    }

    pub fn sprites(&self) -> &[String] {
        &self.sprites
    }

    pub fn sprite_locations(&self) -> &[String] {
        &self.sprite_locations
    }

    pub fn near_distance(&self) -> u32 {
        self.near_distance
    }

    pub fn far_distance(&self) -> u32 {
        self.far_distance
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClientSoundEvents {
    report: ClientSoundEventsReloadReport,
}

impl ClientSoundEvents {
    pub fn report(&self) -> &ClientSoundEventsReloadReport {
        &self.report
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClientSoundEventsReloadReport {
    resource: String,
    pack_id: String,
    event_count: usize,
}

impl ClientSoundEventsReloadReport {
    fn new(resource: impl Into<String>, pack_id: impl Into<String>, event_count: usize) -> Self {
        Self {
            resource: resource.into(),
            pack_id: pack_id.into(),
            event_count,
        }
    }

    pub fn resource(&self) -> &str {
        &self.resource
    }

    pub fn pack_id(&self) -> &str {
        &self.pack_id
    }

    pub fn event_count(&self) -> usize {
        self.event_count
    }

    pub fn loaded_resource_pack(&self) -> String {
        format!("{}@{}", self.resource, self.pack_id)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SoundEventsReloadListener {
    resource: String,
}

impl SoundEventsReloadListener {
    pub fn new(resource: impl Into<String>) -> Self {
        Self {
            resource: resource.into(),
        }
    }

    pub fn load(&self, stack: &ClientResourceStack) -> ResourceReloadResult<ClientSoundEvents> {
        load_client_sound_events(stack, &self.resource)
    }
}

impl Default for SoundEventsReloadListener {
    fn default() -> Self {
        Self::new(SOUND_EVENTS_RESOURCE)
    }
}

impl ResourceReloadListener for SoundEventsReloadListener {
    fn name(&self) -> &str {
        "sound_events"
    }

    fn prepare(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        stack.require_resource(&self.resource)?;
        Ok(ResourceReloadTaskReport::new([self.resource.clone()]))
    }

    fn reload(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        let sound_events = self.load(stack)?;
        Ok(ResourceReloadTaskReport::new([format!(
            "{}:{} events",
            sound_events.report().loaded_resource_pack(),
            sound_events.report().event_count()
        )]))
    }
}

pub fn load_client_sound_events(
    stack: &ClientResourceStack,
    resource: impl AsRef<Path>,
) -> ResourceReloadResult<ClientSoundEvents> {
    let resource = resource.as_ref();
    let json = load_client_json_resource(stack, resource)?;
    let events =
        json.value()
            .as_object()
            .ok_or_else(|| ResourceReloadError::InvalidSoundEvents {
                resource: resource.to_string_lossy().into_owned(),
                path: stack
                    .require_resource(resource)
                    .map(|location| location.path)
                    .unwrap_or_default(),
                reason: "top-level value must be an object".to_owned(),
            })?;

    if let Some((event, _)) = events.iter().find(|(_, value)| !value.is_object()) {
        let location = stack.require_resource(resource)?;
        return Err(ResourceReloadError::InvalidSoundEvents {
            resource: resource.to_string_lossy().into_owned(),
            path: location.path,
            reason: format!("sound event `{event}` must be an object"),
        });
    }

    for (event, value) in events {
        let Some(sounds) = value.get("sounds") else {
            let location = stack.require_resource(resource)?;
            return Err(ResourceReloadError::InvalidSoundEvents {
                resource: resource.to_string_lossy().into_owned(),
                path: location.path,
                reason: format!("sound event `{event}` must define a sounds array"),
            });
        };
        if !sounds.is_array() {
            let location = stack.require_resource(resource)?;
            return Err(ResourceReloadError::InvalidSoundEvents {
                resource: resource.to_string_lossy().into_owned(),
                path: location.path,
                reason: format!("sound event `{event}` sounds must be an array"),
            });
        }
    }

    Ok(ClientSoundEvents {
        report: ClientSoundEventsReloadReport::new(
            resource.to_string_lossy(),
            json.report().pack_id(),
            events.len(),
        ),
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParticleManifestReloadListener {
    ids: Vec<String>,
}

impl ParticleManifestReloadListener {
    pub fn new(ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            ids: ids.into_iter().map(Into::into).collect(),
        }
    }

    pub fn ids(&self) -> &[String] {
        &self.ids
    }

    pub fn load(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ClientParticleDescriptionSet> {
        load_client_particle_descriptions(stack, &self.ids)
    }
}

impl Default for ParticleManifestReloadListener {
    fn default() -> Self {
        Self { ids: Vec::new() }
    }
}

impl ResourceReloadListener for ParticleManifestReloadListener {
    fn name(&self) -> &str {
        "particle_manifests"
    }

    fn prepare(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        let resources = if self.ids.is_empty() {
            manifest_ids_in_directory(stack, PARTICLE_MANIFEST_DIR)?
                .into_iter()
                .map(|id| {
                    manifest_resource_path(PARTICLE_MANIFEST_DIR, &id)
                        .to_string_lossy()
                        .into_owned()
                })
                .collect::<Vec<_>>()
        } else {
            available_manifest_paths(stack, PARTICLE_MANIFEST_DIR, &self.ids)
        };
        Ok(ResourceReloadTaskReport::new(resources))
    }

    fn reload(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        let descriptions = self.load(stack)?;
        Ok(ResourceReloadTaskReport::new(
            particle_description_report_items(&descriptions),
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WaypointStyleManifestReloadListener {
    ids: Vec<String>,
}

impl WaypointStyleManifestReloadListener {
    pub fn new(ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            ids: ids.into_iter().map(Into::into).collect(),
        }
    }

    pub fn ids(&self) -> &[String] {
        &self.ids
    }

    pub fn load(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ClientWaypointStyleSet> {
        load_client_waypoint_styles(stack, &self.ids)
    }
}

impl Default for WaypointStyleManifestReloadListener {
    fn default() -> Self {
        Self::new(DEFAULT_WAYPOINT_STYLE_MANIFEST_IDS.iter().copied())
    }
}

impl ResourceReloadListener for WaypointStyleManifestReloadListener {
    fn name(&self) -> &str {
        "waypoint_style_manifests"
    }

    fn prepare(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        Ok(ResourceReloadTaskReport::new(available_manifest_paths(
            stack,
            WAYPOINT_STYLE_MANIFEST_DIR,
            &self.ids,
        )))
    }

    fn reload(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        let styles = self.load(stack)?;
        Ok(ResourceReloadTaskReport::new(waypoint_style_report_items(
            &styles,
        )))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EquipmentAssetsReloadListener {
    directory: String,
}

impl EquipmentAssetsReloadListener {
    pub fn new(directory: impl Into<String>) -> Self {
        Self {
            directory: directory.into(),
        }
    }

    pub fn directory(&self) -> &str {
        &self.directory
    }

    pub fn load(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ClientEquipmentAssetSet> {
        load_client_equipment_assets(stack, &self.directory)
    }
}

impl Default for EquipmentAssetsReloadListener {
    fn default() -> Self {
        Self::new(EQUIPMENT_MANIFEST_DIR)
    }
}

impl ResourceReloadListener for EquipmentAssetsReloadListener {
    fn name(&self) -> &str {
        "equipment_assets"
    }

    fn prepare(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        Ok(ResourceReloadTaskReport::new(
            manifest_ids_in_directory(stack, &self.directory)?
                .into_iter()
                .map(|id| {
                    manifest_resource_path(&self.directory, &id)
                        .to_string_lossy()
                        .into_owned()
                }),
        ))
    }

    fn reload(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        let assets = self.load(stack)?;
        Ok(ResourceReloadTaskReport::new(equipment_asset_report_items(
            &assets,
        )))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelEntrySmokeReloadListener {
    resources: Vec<String>,
}

impl ModelEntrySmokeReloadListener {
    pub fn new(resources: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            resources: resources.into_iter().map(Into::into).collect(),
        }
    }

    pub fn resources(&self) -> &[String] {
        &self.resources
    }
}

impl Default for ModelEntrySmokeReloadListener {
    fn default() -> Self {
        Self::new(DEFAULT_MODEL_SMOKE_RESOURCES.iter().copied())
    }
}

impl ResourceReloadListener for ModelEntrySmokeReloadListener {
    fn name(&self) -> &str {
        "model_entry_smoke"
    }

    fn prepare(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        for resource in &self.resources {
            stack.require_resource(resource)?;
        }

        Ok(ResourceReloadTaskReport::new(self.resources.clone()))
    }

    fn reload(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        let mut loaded = Vec::with_capacity(self.resources.len());
        for resource in &self.resources {
            let resource = load_client_json_resource(stack, resource)?;
            loaded.push(json_resource_report_item(resource.report()));
        }

        Ok(ResourceReloadTaskReport::new(loaded))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HeadlessBlockstateModelReferenceReloadListener {
    blockstate_ids: Vec<String>,
}

impl HeadlessBlockstateModelReferenceReloadListener {
    pub fn new(blockstate_ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            blockstate_ids: blockstate_ids.into_iter().map(Into::into).collect(),
        }
    }

    pub fn blockstate_ids(&self) -> &[String] {
        &self.blockstate_ids
    }

    pub fn load(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<Vec<HeadlessBlockstateModelReferenceReport>> {
        let manifests =
            load_client_json_manifest_set(stack, BLOCKSTATE_DEFINITION_DIR, &self.blockstate_ids)?;
        manifests
            .manifests()
            .iter()
            .map(|manifest| {
                let model_references =
                    extract_blockstate_model_references(manifest.resource().value()).map_err(
                        |reason| ResourceReloadError::InvalidBlockstateModelReferences {
                            resource: manifest.resource().report().resource().to_owned(),
                            pack_id: manifest.resource().report().pack_id().to_owned(),
                            reason,
                        },
                    )?;

                Ok(HeadlessBlockstateModelReferenceReport {
                    resource_report: manifest.resource().report().clone(),
                    model_references,
                })
            })
            .collect()
    }
}

impl Default for HeadlessBlockstateModelReferenceReloadListener {
    fn default() -> Self {
        Self::new(["stone"])
    }
}

impl ResourceReloadListener for HeadlessBlockstateModelReferenceReloadListener {
    fn name(&self) -> &str {
        "blockstate_model_references"
    }

    fn prepare(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        Ok(ResourceReloadTaskReport::new(available_manifest_paths(
            stack,
            BLOCKSTATE_DEFINITION_DIR,
            &self.blockstate_ids,
        )))
    }

    fn reload(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        Ok(ResourceReloadTaskReport::new(
            self.load(stack)?
                .iter()
                .map(blockstate_model_reference_report_item),
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HeadlessBlockstateModelReferenceReport {
    resource_report: ClientJsonResourceReloadReport,
    model_references: BTreeSet<String>,
}

impl HeadlessBlockstateModelReferenceReport {
    pub fn resource_report(&self) -> &ClientJsonResourceReloadReport {
        &self.resource_report
    }

    pub fn model_references(&self) -> &BTreeSet<String> {
        &self.model_references
    }
}

pub fn extract_blockstate_model_references(
    blockstate: &serde_json::Value,
) -> Result<BTreeSet<String>, String> {
    let object = blockstate
        .as_object()
        .ok_or_else(|| "blockstate root must be an object".to_owned())?;
    let mut model_references = BTreeSet::new();

    if let Some(variants) = object.get("variants") {
        let variants = variants
            .as_object()
            .ok_or_else(|| "blockstate variants must be an object".to_owned())?;

        for variant in variants.values() {
            collect_blockstate_variant_model_references(variant, &mut model_references)?;
        }
    }

    if let Some(multipart) = object.get("multipart") {
        let multipart = multipart
            .as_array()
            .ok_or_else(|| "blockstate multipart must be an array".to_owned())?;

        for entry in multipart {
            let entry = entry
                .as_object()
                .ok_or_else(|| "blockstate multipart entries must be objects".to_owned())?;
            let Some(apply) = entry.get("apply") else {
                continue;
            };
            collect_blockstate_variant_model_references(apply, &mut model_references)?;
        }
    }

    if !object.contains_key("variants") && !object.contains_key("multipart") {
        return Err("blockstate must contain variants or multipart".to_owned());
    }

    Ok(model_references)
}

fn collect_blockstate_variant_model_references(
    variant: &serde_json::Value,
    model_references: &mut BTreeSet<String>,
) -> Result<(), String> {
    match variant {
        serde_json::Value::String(model) => {
            model_references.insert(model.clone());
            Ok(())
        }
        serde_json::Value::Object(object) => {
            collect_blockstate_model_object_reference(object, model_references)
        }
        serde_json::Value::Array(array) => {
            for entry in array {
                let object = entry
                    .as_object()
                    .ok_or_else(|| "blockstate model arrays must contain objects".to_owned())?;
                collect_blockstate_model_object_reference(object, model_references)?;
            }
            Ok(())
        }
        _ => Err("blockstate model entry must be an object, array, or string".to_owned()),
    }
}

fn collect_blockstate_model_object_reference(
    object: &serde_json::Map<String, serde_json::Value>,
    model_references: &mut BTreeSet<String>,
) -> Result<(), String> {
    let Some(model) = object.get("model") else {
        return Ok(());
    };
    let model = model
        .as_str()
        .ok_or_else(|| "blockstate model field must be a string".to_owned())?;
    model_references.insert(model.to_owned());
    Ok(())
}

fn blockstate_model_reference_report_item(
    report: &HeadlessBlockstateModelReferenceReport,
) -> String {
    format!(
        "{}:{}",
        report.resource_report().loaded_resource_pack(),
        report
            .model_references()
            .iter()
            .cloned()
            .collect::<Vec<_>>()
            .join(",")
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelDependencyReloadListener {
    blockstates: Vec<String>,
    block_models: Vec<String>,
    item_models: Vec<String>,
    item_roots: Vec<String>,
}

impl ModelDependencyReloadListener {
    pub fn new(
        blockstates: impl IntoIterator<Item = impl Into<String>>,
        block_models: impl IntoIterator<Item = impl Into<String>>,
        item_models: impl IntoIterator<Item = impl Into<String>>,
        item_roots: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            blockstates: blockstates.into_iter().map(Into::into).collect(),
            block_models: block_models.into_iter().map(Into::into).collect(),
            item_models: item_models.into_iter().map(Into::into).collect(),
            item_roots: item_roots.into_iter().map(Into::into).collect(),
        }
    }

    pub fn load(&self, stack: &ClientResourceStack) -> ResourceReloadResult<ModelDependencyReport> {
        load_model_dependencies(
            stack,
            &self.blockstates,
            &self.block_models,
            &self.item_models,
            &self.item_roots,
        )
    }

    fn resources(&self) -> impl Iterator<Item = &String> {
        self.blockstates
            .iter()
            .chain(self.block_models.iter())
            .chain(self.item_models.iter())
            .chain(self.item_roots.iter())
    }
}

impl Default for ModelDependencyReloadListener {
    fn default() -> Self {
        Self::new(
            DEFAULT_MODEL_DEPENDENCY_BLOCKSTATES.iter().copied(),
            DEFAULT_MODEL_DEPENDENCY_BLOCK_MODELS.iter().copied(),
            DEFAULT_MODEL_DEPENDENCY_ITEM_MODELS.iter().copied(),
            DEFAULT_MODEL_DEPENDENCY_ITEM_ROOTS.iter().copied(),
        )
    }
}

impl ResourceReloadListener for ModelDependencyReloadListener {
    fn name(&self) -> &str {
        "model_dependencies"
    }

    fn prepare(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        let mut resources = Vec::with_capacity(
            self.blockstates.len()
                + self.block_models.len()
                + self.item_models.len()
                + self.item_roots.len(),
        );
        for resource in self.resources() {
            stack.require_resource(resource)?;
            resources.push(resource.clone());
        }

        Ok(ResourceReloadTaskReport::new(resources))
    }

    fn reload(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        let report = self.load(stack)?;
        Ok(ResourceReloadTaskReport::new(
            model_dependency_report_items(&report),
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelDependencyReport {
    resources: Vec<ModelDependencyResourceReport>,
    counts_by_top_priority_pack: BTreeMap<String, ModelDependencyPackCounts>,
    blockstate_models: BTreeSet<String>,
    item_root_models: BTreeSet<String>,
    parents: BTreeSet<String>,
    textures: BTreeSet<String>,
}

impl ModelDependencyReport {
    pub fn resources(&self) -> &[ModelDependencyResourceReport] {
        &self.resources
    }

    pub fn counts_by_top_priority_pack(&self) -> &BTreeMap<String, ModelDependencyPackCounts> {
        &self.counts_by_top_priority_pack
    }

    pub fn blockstate_models(&self) -> &BTreeSet<String> {
        &self.blockstate_models
    }

    pub fn item_root_models(&self) -> &BTreeSet<String> {
        &self.item_root_models
    }

    pub fn parents(&self) -> &BTreeSet<String> {
        &self.parents
    }

    pub fn textures(&self) -> &BTreeSet<String> {
        &self.textures
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelDependencyResourceReport {
    resource: String,
    pack_id: String,
    kind: ModelDependencyResourceKind,
}

impl ModelDependencyResourceReport {
    pub fn resource(&self) -> &str {
        &self.resource
    }

    pub fn pack_id(&self) -> &str {
        &self.pack_id
    }

    pub fn kind(&self) -> ModelDependencyResourceKind {
        self.kind
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModelDependencyResourceKind {
    Blockstate,
    BlockModel,
    ItemModel,
    ItemRoot,
}

impl ModelDependencyResourceKind {
    fn label(self) -> &'static str {
        match self {
            Self::Blockstate => "blockstates",
            Self::BlockModel => "block_models",
            Self::ItemModel => "item_models",
            Self::ItemRoot => "item_roots",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ModelDependencyPackCounts {
    pub blockstates: usize,
    pub block_models: usize,
    pub item_models: usize,
    pub item_roots: usize,
}

impl ModelDependencyPackCounts {
    fn increment(&mut self, kind: ModelDependencyResourceKind) {
        match kind {
            ModelDependencyResourceKind::Blockstate => self.blockstates += 1,
            ModelDependencyResourceKind::BlockModel => self.block_models += 1,
            ModelDependencyResourceKind::ItemModel => self.item_models += 1,
            ModelDependencyResourceKind::ItemRoot => self.item_roots += 1,
        }
    }
}

pub fn load_model_dependencies(
    stack: &ClientResourceStack,
    blockstates: &[String],
    block_models: &[String],
    item_models: &[String],
    item_roots: &[String],
) -> ResourceReloadResult<ModelDependencyReport> {
    let mut resources = Vec::with_capacity(
        blockstates.len() + block_models.len() + item_models.len() + item_roots.len(),
    );
    let mut counts_by_top_priority_pack = BTreeMap::new();
    let mut blockstate_models = BTreeSet::new();
    let mut item_root_models = BTreeSet::new();
    let mut parents = BTreeSet::new();
    let mut textures = BTreeSet::new();

    for blockstate in blockstates {
        let resource = load_client_json_resource(stack, blockstate)?;
        let model_references = extract_blockstate_model_references(resource.value())
            .map_err(|reason| invalid_model_dependency(&resource, reason))?;
        blockstate_models.extend(model_references);
        push_model_dependency_resource(
            &mut resources,
            &mut counts_by_top_priority_pack,
            resource.report(),
            ModelDependencyResourceKind::Blockstate,
        );
    }

    for block_model in block_models {
        let resource = load_client_json_resource(stack, block_model)?;
        collect_model_json_dependencies(&resource, &mut parents, &mut textures)?;
        push_model_dependency_resource(
            &mut resources,
            &mut counts_by_top_priority_pack,
            resource.report(),
            ModelDependencyResourceKind::BlockModel,
        );
    }

    for item_model in item_models {
        let resource = load_client_json_resource(stack, item_model)?;
        collect_model_json_dependencies(&resource, &mut parents, &mut textures)?;
        push_model_dependency_resource(
            &mut resources,
            &mut counts_by_top_priority_pack,
            resource.report(),
            ModelDependencyResourceKind::ItemModel,
        );
    }

    for item_root in item_roots {
        let resource = load_client_json_resource(stack, item_root)?;
        let model_references = extract_item_root_model_dependencies(resource.value())
            .map_err(|reason| invalid_model_dependency(&resource, reason))?;
        item_root_models.extend(model_references);
        push_model_dependency_resource(
            &mut resources,
            &mut counts_by_top_priority_pack,
            resource.report(),
            ModelDependencyResourceKind::ItemRoot,
        );
    }

    Ok(ModelDependencyReport {
        resources,
        counts_by_top_priority_pack,
        blockstate_models,
        item_root_models,
        parents,
        textures,
    })
}

fn push_model_dependency_resource(
    resources: &mut Vec<ModelDependencyResourceReport>,
    counts_by_top_priority_pack: &mut BTreeMap<String, ModelDependencyPackCounts>,
    report: &ClientJsonResourceReloadReport,
    kind: ModelDependencyResourceKind,
) {
    counts_by_top_priority_pack
        .entry(report.pack_id().to_owned())
        .or_default()
        .increment(kind);
    resources.push(ModelDependencyResourceReport {
        resource: report.resource().to_owned(),
        pack_id: report.pack_id().to_owned(),
        kind,
    });
}

fn collect_model_json_dependencies(
    resource: &ClientJsonResource,
    parents: &mut BTreeSet<String>,
    textures: &mut BTreeSet<String>,
) -> ResourceReloadResult<()> {
    let Some(object) = resource.value().as_object() else {
        return Err(invalid_model_dependency(
            resource,
            "model top-level value must be an object",
        ));
    };

    if let Some(parent) = object.get("parent") {
        let Some(parent) = parent.as_str() else {
            return Err(invalid_model_dependency(
                resource,
                "model parent must be a string",
            ));
        };
        parents.insert(parent.to_owned());
    }

    if let Some(texture_values) = object.get("textures") {
        let Some(texture_values) = texture_values.as_object() else {
            return Err(invalid_model_dependency(
                resource,
                "model textures must be an object",
            ));
        };
        for (texture_key, texture) in texture_values {
            let Some(texture) = texture.as_str() else {
                return Err(invalid_model_dependency(
                    resource,
                    format!("model texture `{texture_key}` must be a string"),
                ));
            };
            textures.insert(texture.to_owned());
        }
    }

    Ok(())
}

pub fn extract_item_root_model_dependencies(
    item_root: &serde_json::Value,
) -> Result<BTreeSet<String>, String> {
    let Some(object) = item_root.as_object() else {
        return Err("item root top-level value must be an object".to_owned());
    };
    let Some(model) = object.get("model") else {
        return Err("item root must contain model".to_owned());
    };

    let mut model_references = BTreeSet::new();
    collect_item_model_dependencies(model, &mut model_references)?;
    Ok(model_references)
}

fn collect_item_model_dependencies(
    item_model: &serde_json::Value,
    model_references: &mut BTreeSet<String>,
) -> Result<(), String> {
    let Some(object) = item_model.as_object() else {
        return Err("item model must be an object".to_owned());
    };
    let Some(model_type) = object.get("type") else {
        return Err("item model must contain type".to_owned());
    };
    let Some(model_type) = model_type.as_str() else {
        return Err("item model type must be a string".to_owned());
    };

    match strip_minecraft_namespace(model_type) {
        "model" => {
            let Some(model) = object.get("model") else {
                return Err("minecraft:model item model must contain model".to_owned());
            };
            let Some(model) = model.as_str() else {
                return Err("minecraft:model item model model must be a string".to_owned());
            };
            model_references.insert(model.to_owned());
        }
        "special" => {
            let Some(base) = object.get("base") else {
                return Err("minecraft:special item model must contain base".to_owned());
            };
            let Some(base) = base.as_str() else {
                return Err("minecraft:special item model base must be a string".to_owned());
            };
            model_references.insert(base.to_owned());
        }
        "range_dispatch" => {
            if let Some(entries) = object.get("entries") {
                let Some(entries) = entries.as_array() else {
                    return Err("minecraft:range_dispatch entries must be an array".to_owned());
                };
                for entry in entries {
                    let Some(entry) = entry.as_object() else {
                        return Err("minecraft:range_dispatch entry must be an object".to_owned());
                    };
                    collect_required_child_item_model(
                        entry,
                        "model",
                        "minecraft:range_dispatch entry model",
                        model_references,
                    )?;
                }
            }
            collect_optional_child_item_model(
                object,
                "fallback",
                "minecraft:range_dispatch fallback",
                model_references,
            )?;
        }
        "select" => {
            if let Some(cases) = object.get("cases") {
                let Some(cases) = cases.as_array() else {
                    return Err("minecraft:select cases must be an array".to_owned());
                };
                for case in cases {
                    let Some(case) = case.as_object() else {
                        return Err("minecraft:select case must be an object".to_owned());
                    };
                    collect_required_child_item_model(
                        case,
                        "model",
                        "minecraft:select case model",
                        model_references,
                    )?;
                }
            }
            collect_optional_child_item_model(
                object,
                "fallback",
                "minecraft:select fallback",
                model_references,
            )?;
        }
        "condition" => {
            collect_required_child_item_model(
                object,
                "on_true",
                "minecraft:condition on_true",
                model_references,
            )?;
            collect_required_child_item_model(
                object,
                "on_false",
                "minecraft:condition on_false",
                model_references,
            )?;
        }
        "composite" => {
            let Some(models) = object.get("models") else {
                return Err("minecraft:composite item model must contain models".to_owned());
            };
            let Some(models) = models.as_array() else {
                return Err("minecraft:composite models must be an array".to_owned());
            };
            for model in models {
                collect_item_model_dependencies(model, model_references)?;
            }
        }
        "empty" | "bundle/selected_item" => {}
        _ => {
            return Err(format!("unsupported item model type `{model_type}`"));
        }
    }

    Ok(())
}

fn collect_required_child_item_model(
    object: &serde_json::Map<String, serde_json::Value>,
    field: &str,
    description: &str,
    model_references: &mut BTreeSet<String>,
) -> Result<(), String> {
    let Some(model) = object.get(field) else {
        return Err(format!("{description} must be present"));
    };
    collect_item_model_dependencies(model, model_references)
}

fn collect_optional_child_item_model(
    object: &serde_json::Map<String, serde_json::Value>,
    field: &str,
    description: &str,
    model_references: &mut BTreeSet<String>,
) -> Result<(), String> {
    if let Some(model) = object.get(field) {
        if !model.is_object() {
            return Err(format!("{description} must be an object"));
        }
        collect_item_model_dependencies(model, model_references)?;
    }
    Ok(())
}

fn strip_minecraft_namespace(model_type: &str) -> &str {
    model_type.strip_prefix("minecraft:").unwrap_or(model_type)
}

fn invalid_model_dependency(
    resource: &ClientJsonResource,
    reason: impl Into<String>,
) -> ResourceReloadError {
    ResourceReloadError::InvalidModelDependency {
        resource: resource.report().resource().to_owned(),
        pack_id: resource.report().pack_id().to_owned(),
        reason: reason.into(),
    }
}

fn model_dependency_report_items(report: &ModelDependencyReport) -> Vec<String> {
    let mut items = Vec::new();

    for resource in report.resources() {
        items.push(format!(
            "{}@{}:{}",
            resource.resource(),
            resource.pack_id(),
            resource.kind().label()
        ));
    }

    for (pack_id, counts) in report.counts_by_top_priority_pack() {
        items.push(format!(
            "pack:{pack_id}:blockstates:{} block_models:{} item_models:{} item_roots:{}",
            counts.blockstates, counts.block_models, counts.item_models, counts.item_roots
        ));
    }

    items.push(format!(
        "blockstate_models:{}",
        report
            .blockstate_models()
            .iter()
            .cloned()
            .collect::<Vec<_>>()
            .join(",")
    ));
    items.push(format!(
        "item_root_models:{}",
        report
            .item_root_models()
            .iter()
            .cloned()
            .collect::<Vec<_>>()
            .join(",")
    ));
    items.push(format!(
        "parents:{}",
        report
            .parents()
            .iter()
            .cloned()
            .collect::<Vec<_>>()
            .join(",")
    ));
    items.push(format!(
        "textures:{}",
        report
            .textures()
            .iter()
            .cloned()
            .collect::<Vec<_>>()
            .join(",")
    ));

    items
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HeadlessShaderSourceReloadListener {
    sources: Vec<String>,
    includes: Vec<String>,
}

impl HeadlessShaderSourceReloadListener {
    pub fn new(
        sources: impl IntoIterator<Item = impl Into<String>>,
        includes: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            sources: sources.into_iter().map(Into::into).collect(),
            includes: includes.into_iter().map(Into::into).collect(),
        }
    }

    pub fn sources(&self) -> &[String] {
        &self.sources
    }

    pub fn includes(&self) -> &[String] {
        &self.includes
    }

    fn resources(&self) -> impl Iterator<Item = &String> {
        self.sources.iter().chain(self.includes.iter())
    }
}

impl Default for HeadlessShaderSourceReloadListener {
    fn default() -> Self {
        Self::new(
            DEFAULT_REPRESENTATIVE_SHADER_SOURCES.iter().copied(),
            DEFAULT_SHADER_INCLUDE_SOURCES.iter().copied(),
        )
    }
}

impl ResourceReloadListener for HeadlessShaderSourceReloadListener {
    fn name(&self) -> &str {
        "headless_shader_sources"
    }

    fn prepare(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        let mut resources = Vec::with_capacity(self.sources.len() + self.includes.len());
        for resource in self.resources() {
            stack.require_resource(resource)?;
            resources.push(resource.clone());
        }

        Ok(ResourceReloadTaskReport::new(resources))
    }

    fn reload(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        let mut loaded = Vec::with_capacity(self.sources.len() + self.includes.len());
        for resource in self.resources() {
            let report = load_headless_shader_source(stack, resource)?;
            loaded.push(shader_source_report_item(&report));
        }

        Ok(ResourceReloadTaskReport::new(loaded))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HeadlessShaderSourceReloadReport {
    resource: String,
    pack_id: String,
    byte_count: usize,
}

impl HeadlessShaderSourceReloadReport {
    fn new(resource: impl Into<String>, pack_id: impl Into<String>, byte_count: usize) -> Self {
        Self {
            resource: resource.into(),
            pack_id: pack_id.into(),
            byte_count,
        }
    }

    pub fn resource(&self) -> &str {
        &self.resource
    }

    pub fn pack_id(&self) -> &str {
        &self.pack_id
    }

    pub fn byte_count(&self) -> usize {
        self.byte_count
    }

    pub fn loaded_resource_pack(&self) -> String {
        format!("{}@{}", self.resource, self.pack_id)
    }
}

pub fn load_headless_shader_source(
    stack: &ClientResourceStack,
    resource: impl AsRef<Path>,
) -> ResourceReloadResult<HeadlessShaderSourceReloadReport> {
    let resource = resource.as_ref();
    let location = stack.require_resource(resource)?;
    let bytes = location
        .read_bytes()
        .map_err(|source| ResourceReloadError::ReadResource {
            resource: resource.to_string_lossy().into_owned(),
            path: location.path.clone(),
            source,
        })?;

    let resource_name = resource.to_string_lossy().into_owned();
    validate_headless_shader_source(&resource_name, &location, &bytes)?;

    Ok(HeadlessShaderSourceReloadReport::new(
        resource_name,
        location.pack_id,
        bytes.len(),
    ))
}

fn validate_headless_shader_source(
    resource: &str,
    location: &ResourceLocation,
    bytes: &[u8],
) -> ResourceReloadResult<()> {
    if bytes.is_empty() {
        return Err(ResourceReloadError::InvalidShaderSource {
            resource: resource.to_owned(),
            path: location.path.clone(),
            reason: "empty shader source".to_owned(),
        });
    }

    let source =
        std::str::from_utf8(bytes).map_err(|error| ResourceReloadError::InvalidShaderSource {
            resource: resource.to_owned(),
            path: location.path.clone(),
            reason: format!("shader source is not utf-8: {error}"),
        })?;

    if source.trim().is_empty() {
        return Err(ResourceReloadError::InvalidShaderSource {
            resource: resource.to_owned(),
            path: location.path.clone(),
            reason: "blank shader source".to_owned(),
        });
    }

    Ok(())
}

fn shader_source_report_item(report: &HeadlessShaderSourceReloadReport) -> String {
    format!(
        "{}@{}:{} bytes",
        report.resource(),
        report.pack_id(),
        report.byte_count()
    )
}

pub fn load_client_json_manifest_set(
    stack: &ClientResourceStack,
    directory: &str,
    ids: &[String],
) -> ResourceReloadResult<ClientJsonManifestSet> {
    let mut manifests = Vec::new();

    for id in ids {
        let resource = manifest_resource_path(directory, id);
        let Some(location) = stack.find_resource(&resource) else {
            continue;
        };
        let resource = read_client_json_resource(&resource, &location)?;
        manifests.push(ClientJsonManifest {
            id: id.clone(),
            resource,
        });
    }

    Ok(ClientJsonManifestSet { manifests })
}

pub fn load_client_json_manifest_directory(
    stack: &ClientResourceStack,
    directory: &str,
) -> ResourceReloadResult<ClientJsonManifestSet> {
    let ids = manifest_ids_in_directory(stack, directory)?;
    load_client_json_manifest_set(stack, directory, &ids)
}

pub fn load_client_particle_descriptions(
    stack: &ClientResourceStack,
    ids: &[String],
) -> ResourceReloadResult<ClientParticleDescriptionSet> {
    let manifests = if ids.is_empty() {
        load_client_json_manifest_directory(stack, PARTICLE_MANIFEST_DIR)?
    } else {
        load_client_json_manifest_set(stack, PARTICLE_MANIFEST_DIR, ids)?
    };
    let descriptions = manifests
        .manifests()
        .iter()
        .map(parse_client_particle_description)
        .collect::<ResourceReloadResult<Vec<_>>>()?;

    Ok(ClientParticleDescriptionSet { descriptions })
}

pub fn load_client_waypoint_styles(
    stack: &ClientResourceStack,
    ids: &[String],
) -> ResourceReloadResult<ClientWaypointStyleSet> {
    let manifests = load_client_json_manifest_set(stack, WAYPOINT_STYLE_MANIFEST_DIR, ids)?;
    let styles = manifests
        .manifests()
        .iter()
        .map(parse_client_waypoint_style)
        .collect::<ResourceReloadResult<Vec<_>>>()?;

    Ok(ClientWaypointStyleSet { styles })
}

pub fn load_client_equipment_assets(
    stack: &ClientResourceStack,
    directory: &str,
) -> ResourceReloadResult<ClientEquipmentAssetSet> {
    let manifests = load_client_json_manifest_directory(stack, directory)?;
    let assets = manifests
        .manifests()
        .iter()
        .map(parse_client_equipment_asset)
        .collect::<ResourceReloadResult<Vec<_>>>()?;

    Ok(ClientEquipmentAssetSet { assets })
}

fn parse_client_equipment_asset(
    manifest: &ClientJsonManifest,
) -> ResourceReloadResult<ClientEquipmentAsset> {
    let report = manifest.resource().report();
    let value = manifest.resource().value();
    let object = value.as_object().ok_or_else(|| {
        invalid_equipment_asset_error(report, "top-level value must be an object")
    })?;
    let layers = object
        .get("layers")
        .ok_or_else(|| invalid_equipment_asset_error(report, "layers must be an object"))?
        .as_object()
        .ok_or_else(|| invalid_equipment_asset_error(report, "layers must be an object"))?;
    if layers.is_empty() {
        return Err(invalid_equipment_asset_error(
            report,
            "layers must not be empty",
        ));
    }
    let layers = layers
        .iter()
        .map(|(layer_type, entries)| parse_client_equipment_layer(report, layer_type, entries))
        .collect::<ResourceReloadResult<Vec<_>>>()?;

    Ok(ClientEquipmentAsset {
        id: manifest.id().to_owned(),
        report: ClientEquipmentAssetReloadReport::new(report.resource(), report.pack_id(), &layers),
        layers,
    })
}

fn parse_client_equipment_layer(
    report: &ClientJsonResourceReloadReport,
    layer_type: &str,
    entries: &serde_json::Value,
) -> ResourceReloadResult<ClientEquipmentLayer> {
    if !is_known_equipment_layer_type(layer_type) {
        return Err(invalid_equipment_asset_error(
            report,
            format!("layers.{layer_type} is not a known equipment layer type"),
        ));
    }
    let entries = entries.as_array().ok_or_else(|| {
        invalid_equipment_asset_error(
            report,
            format!("layers.{layer_type} must be an array of layer entries"),
        )
    })?;
    if entries.is_empty() {
        return Err(invalid_equipment_asset_error(
            report,
            format!("layers.{layer_type} must not be empty"),
        ));
    }
    let entries = entries
        .iter()
        .enumerate()
        .map(|(index, entry)| parse_client_equipment_layer_entry(report, layer_type, index, entry))
        .collect::<ResourceReloadResult<Vec<_>>>()?;

    Ok(ClientEquipmentLayer {
        layer_type: layer_type.to_owned(),
        entries,
    })
}

fn parse_client_equipment_layer_entry(
    report: &ClientJsonResourceReloadReport,
    layer_type: &str,
    index: usize,
    entry: &serde_json::Value,
) -> ResourceReloadResult<ClientEquipmentLayerEntry> {
    let entry = entry.as_object().ok_or_else(|| {
        invalid_equipment_asset_error(
            report,
            format!("layers.{layer_type}[{index}] must be an object"),
        )
    })?;
    let texture = entry
        .get("texture")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| {
            invalid_equipment_asset_error(
                report,
                format!("layers.{layer_type}[{index}].texture must be a resource id string"),
            )
        })?;
    if !is_valid_vanilla_resource_identifier(texture) {
        return Err(invalid_equipment_asset_error(
            report,
            format!("layers.{layer_type}[{index}].texture is not a valid resource id"),
        ));
    }
    let dyeable = entry
        .get("dyeable")
        .map(|dyeable| parse_client_equipment_dyeable(report, layer_type, index, dyeable))
        .transpose()?;
    let use_player_texture = entry
        .get("use_player_texture")
        .map(|use_player_texture| {
            use_player_texture.as_bool().ok_or_else(|| {
                invalid_equipment_asset_error(
                    report,
                    format!("layers.{layer_type}[{index}].use_player_texture must be a boolean"),
                )
            })
        })
        .transpose()?;

    Ok(ClientEquipmentLayerEntry {
        texture: texture.to_owned(),
        dyeable,
        use_player_texture,
    })
}

fn parse_client_equipment_dyeable(
    report: &ClientJsonResourceReloadReport,
    layer_type: &str,
    index: usize,
    dyeable: &serde_json::Value,
) -> ResourceReloadResult<ClientEquipmentDyeable> {
    let dyeable = dyeable.as_object().ok_or_else(|| {
        invalid_equipment_asset_error(
            report,
            format!("layers.{layer_type}[{index}].dyeable must be an object"),
        )
    })?;
    let color_when_undyed = dyeable
        .get("color_when_undyed")
        .map(|color| {
            color.as_i64().and_then(|color| i32::try_from(color).ok()).ok_or_else(|| {
                invalid_equipment_asset_error(
                    report,
                    format!(
                        "layers.{layer_type}[{index}].dyeable.color_when_undyed must be an integer"
                    ),
                )
            })
        })
        .transpose()?;

    Ok(ClientEquipmentDyeable { color_when_undyed })
}

fn is_known_equipment_layer_type(layer_type: &str) -> bool {
    matches!(
        layer_type,
        "humanoid"
            | "humanoid_leggings"
            | "humanoid_baby"
            | "wings"
            | "wolf_body"
            | "horse_body"
            | "llama_body"
            | "pig_saddle"
            | "strider_saddle"
            | "camel_saddle"
            | "camel_husk_saddle"
            | "horse_saddle"
            | "donkey_saddle"
            | "mule_saddle"
            | "zombie_horse_saddle"
            | "skeleton_horse_saddle"
            | "happy_ghast_body"
            | "nautilus_saddle"
            | "nautilus_body"
    )
}

fn equipment_layer_texture_location(layer_type: &str, texture: &str) -> String {
    let (namespace, path) = texture
        .split_once(':')
        .map_or(("minecraft", texture), |(namespace, path)| {
            (namespace, path)
        });
    format!("{namespace}:textures/entity/equipment/{layer_type}/{path}.png")
}

fn parse_client_particle_description(
    manifest: &ClientJsonManifest,
) -> ResourceReloadResult<ClientParticleDescription> {
    let report = manifest.resource().report();
    let value = manifest.resource().value();
    let object = value.as_object().ok_or_else(|| {
        invalid_particle_manifest_error(report, "top-level value must be an object")
    })?;
    let sprites = match object.get("textures") {
        Some(textures) => {
            let textures = textures.as_array().ok_or_else(|| {
                invalid_particle_manifest_error(report, "textures must be an array of resource ids")
            })?;
            textures
                .iter()
                .enumerate()
                .map(|(index, texture)| {
                    let Some(texture) = texture.as_str() else {
                        return Err(invalid_particle_manifest_error(
                            report,
                            format!("textures[{index}] must be a resource id string"),
                        ));
                    };
                    if !is_valid_vanilla_resource_identifier(texture) {
                        return Err(invalid_particle_manifest_error(
                            report,
                            format!("textures[{index}] is not a valid resource id"),
                        ));
                    }
                    Ok(texture.to_owned())
                })
                .collect::<ResourceReloadResult<Vec<_>>>()?
        }
        None => Vec::new(),
    };

    Ok(ClientParticleDescription {
        id: manifest.id().to_owned(),
        report: ClientParticleDescriptionReloadReport::new(
            report.resource(),
            report.pack_id(),
            sprites.iter().cloned(),
        ),
        sprites,
    })
}

fn parse_client_waypoint_style(
    manifest: &ClientJsonManifest,
) -> ResourceReloadResult<ClientWaypointStyle> {
    let report = manifest.resource().report();
    let value = manifest.resource().value();
    let object = value.as_object().ok_or_else(|| {
        invalid_waypoint_style_manifest_error(report, "top-level value must be an object")
    })?;
    let sprites = object
        .get("sprites")
        .ok_or_else(|| {
            invalid_waypoint_style_manifest_error(
                report,
                "sprites must be an array of resource ids",
            )
        })?
        .as_array()
        .ok_or_else(|| {
            invalid_waypoint_style_manifest_error(
                report,
                "sprites must be an array of resource ids",
            )
        })?;
    let sprites = sprites
        .iter()
        .enumerate()
        .map(|(index, sprite)| {
            let Some(sprite) = sprite.as_str() else {
                return Err(invalid_waypoint_style_manifest_error(
                    report,
                    format!("sprites[{index}] must be a resource id string"),
                ));
            };
            if !is_valid_vanilla_resource_identifier(sprite) {
                return Err(invalid_waypoint_style_manifest_error(
                    report,
                    format!("sprites[{index}] is not a valid resource id"),
                ));
            }
            Ok(sprite.to_owned())
        })
        .collect::<ResourceReloadResult<Vec<_>>>()?;
    if sprites.is_empty() {
        return Err(invalid_waypoint_style_manifest_error(
            report,
            "sprites must not be empty",
        ));
    }
    let (near_distance, far_distance) = parse_waypoint_style_distances(report, object)?;

    Ok(ClientWaypointStyle {
        id: manifest.id().to_owned(),
        report: ClientWaypointStyleReloadReport::new(
            report.resource(),
            report.pack_id(),
            sprites.iter().cloned(),
            near_distance,
            far_distance,
        ),
        sprite_locations: sprites
            .iter()
            .map(|sprite| waypoint_style_sprite_location(sprite))
            .collect(),
        sprites,
        near_distance,
        far_distance,
    })
}

fn parse_waypoint_style_distances(
    report: &ClientJsonResourceReloadReport,
    object: &serde_json::Map<String, serde_json::Value>,
) -> ResourceReloadResult<(u32, u32)> {
    let near_distance = parse_optional_waypoint_distance(report, object, "near_distance")?
        .unwrap_or(DEFAULT_WAYPOINT_STYLE_NEAR_DISTANCE);
    let far_distance = parse_optional_waypoint_distance(report, object, "far_distance")?
        .unwrap_or(DEFAULT_WAYPOINT_STYLE_FAR_DISTANCE);

    if near_distance == 0 {
        return Err(invalid_waypoint_style_manifest_error(
            report,
            "near_distance must be greater than 0",
        ));
    }
    if far_distance <= near_distance {
        return Err(invalid_waypoint_style_manifest_error(
            report,
            "far_distance must be greater than near_distance",
        ));
    }

    Ok((near_distance, far_distance))
}

fn parse_optional_waypoint_distance(
    report: &ClientJsonResourceReloadReport,
    object: &serde_json::Map<String, serde_json::Value>,
    key: &str,
) -> ResourceReloadResult<Option<u32>> {
    let Some(value) = object.get(key) else {
        return Ok(None);
    };
    let Some(distance) = value
        .as_u64()
        .and_then(|distance| u32::try_from(distance).ok())
    else {
        return Err(invalid_waypoint_style_manifest_error(
            report,
            format!("{key} must be an integer"),
        ));
    };
    if distance > 60_000_000 {
        return Err(invalid_waypoint_style_manifest_error(
            report,
            format!("{key} must be at most 60000000"),
        ));
    }
    Ok(Some(distance))
}

fn waypoint_style_sprite_location(sprite: &str) -> String {
    let (namespace, path) = sprite.split_once(':').unwrap_or(("minecraft", sprite));
    format!("{namespace}:{WAYPOINT_STYLE_SPRITE_LOCATION_PREFIX}/{path}")
}

fn is_valid_vanilla_resource_identifier(value: &str) -> bool {
    let (namespace, path) = value
        .split_once(':')
        .map(|(namespace, path)| (Some(namespace), path))
        .unwrap_or((None, value));
    if path.is_empty() || value.matches(':').count() > 1 {
        return false;
    }
    namespace
        .map(is_valid_vanilla_resource_namespace)
        .unwrap_or(true)
        && is_valid_vanilla_resource_path(path)
}

fn is_valid_vanilla_resource_namespace(namespace: &str) -> bool {
    !namespace.is_empty()
        && namespace.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'_' | b'-' | b'.')
        })
}

fn is_valid_vanilla_resource_path(path: &str) -> bool {
    path.bytes().all(|byte| {
        byte.is_ascii_lowercase()
            || byte.is_ascii_digit()
            || matches!(byte, b'_' | b'-' | b'.' | b'/')
    })
}

fn invalid_waypoint_style_manifest_error(
    report: &ClientJsonResourceReloadReport,
    reason: impl Into<String>,
) -> ResourceReloadError {
    ResourceReloadError::InvalidWaypointStyleManifest {
        resource: report.resource().to_owned(),
        pack_id: report.pack_id().to_owned(),
        reason: reason.into(),
    }
}

fn invalid_particle_manifest_error(
    report: &ClientJsonResourceReloadReport,
    reason: impl Into<String>,
) -> ResourceReloadError {
    ResourceReloadError::InvalidParticleManifest {
        resource: report.resource().to_owned(),
        pack_id: report.pack_id().to_owned(),
        reason: reason.into(),
    }
}

fn invalid_equipment_asset_error(
    report: &ClientJsonResourceReloadReport,
    reason: impl Into<String>,
) -> ResourceReloadError {
    ResourceReloadError::InvalidEquipmentAsset {
        resource: report.resource().to_owned(),
        pack_id: report.pack_id().to_owned(),
        reason: reason.into(),
    }
}

fn available_manifest_paths(
    stack: &ClientResourceStack,
    directory: &str,
    ids: &[String],
) -> Vec<String> {
    ids.iter()
        .map(|id| manifest_resource_path(directory, id))
        .filter(|resource| stack.find_resource(resource).is_some())
        .map(|resource| resource.to_string_lossy().into_owned())
        .collect()
}

fn manifest_ids_in_directory(
    stack: &ClientResourceStack,
    directory: &str,
) -> ResourceReloadResult<Vec<String>> {
    let mut ids = BTreeSet::new();

    for pack in stack.packs() {
        let pack_directory = pack.resource_path(directory);
        if !pack_directory.exists() {
            continue;
        }

        let entries =
            fs::read_dir(&pack_directory).map_err(|source| ResourceReloadError::ReadResource {
                resource: directory.to_owned(),
                path: pack_directory.clone(),
                source,
            })?;

        for entry in entries {
            let entry = entry.map_err(|source| ResourceReloadError::ReadResource {
                resource: directory.to_owned(),
                path: pack_directory.clone(),
                source,
            })?;
            let path = entry.path();
            if !path.is_file()
                || path.extension().and_then(|extension| extension.to_str()) != Some("json")
            {
                continue;
            }

            let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) else {
                continue;
            };
            ids.insert(stem.to_owned());
        }
    }

    if ids.is_empty() {
        return Err(ResourceReloadError::MissingResource(format!(
            "{directory}/*.json"
        )));
    }

    Ok(ids.into_iter().collect())
}

fn particle_description_report_items(descriptions: &ClientParticleDescriptionSet) -> Vec<String> {
    descriptions
        .reports()
        .map(particle_description_report_item)
        .collect()
}

fn waypoint_style_report_items(styles: &ClientWaypointStyleSet) -> Vec<String> {
    styles.reports().map(waypoint_style_report_item).collect()
}

fn equipment_asset_report_items(assets: &ClientEquipmentAssetSet) -> Vec<String> {
    assets.reports().map(equipment_asset_report_item).collect()
}

fn equipment_asset_report_item(report: &ClientEquipmentAssetReloadReport) -> String {
    format!(
        "{}@{}:layers:{} entries:{} layer_types:{} textures:{} texture_locations:{} dyeable:{} player_textures:{}",
        report.resource(),
        report.pack_id(),
        report.layer_count(),
        report.entry_count(),
        report.layer_types().join(","),
        report.textures().join(","),
        report.texture_locations().join(","),
        report.dyeable_entry_count(),
        report.player_texture_entry_count(),
    )
}

fn particle_description_report_item(report: &ClientParticleDescriptionReloadReport) -> String {
    format!(
        "{}@{}:{} sprites:{}",
        report.resource(),
        report.pack_id(),
        report.sprite_count(),
        report.sprites().join(",")
    )
}

fn waypoint_style_report_item(report: &ClientWaypointStyleReloadReport) -> String {
    format!(
        "{}@{}:near:{} far:{} sprites:{}:{} locations:{}",
        report.resource(),
        report.pack_id(),
        report.near_distance(),
        report.far_distance(),
        report.sprite_count(),
        report.sprites().join(","),
        report.sprite_locations().join(",")
    )
}

fn json_resource_report_item(report: &ClientJsonResourceReloadReport) -> String {
    format!(
        "{}:{}",
        report.loaded_resource_pack(),
        report.top_level_shape().report_fragment()
    )
}

fn manifest_resource_path(directory: &str, id: &str) -> PathBuf {
    PathBuf::from(directory).join(format!("{id}.json"))
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClientFontDefinitionSet {
    definitions: Vec<ClientFontDefinition>,
}

impl ClientFontDefinitionSet {
    pub fn definitions(&self) -> &[ClientFontDefinition] {
        &self.definitions
    }

    pub fn reports(&self) -> impl Iterator<Item = ClientFontDefinitionReloadReport<'_>> {
        self.definitions.iter().map(ClientFontDefinition::report)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClientFontDefinition {
    resource: ClientJsonResource,
    providers: Vec<ClientFontProviderDefinition>,
}

impl ClientFontDefinition {
    pub fn resource(&self) -> &ClientJsonResource {
        &self.resource
    }

    pub fn providers(&self) -> &[ClientFontProviderDefinition] {
        &self.providers
    }

    pub fn provider_count(&self) -> usize {
        self.providers.len()
    }

    pub fn provider_types(&self) -> impl Iterator<Item = &str> {
        self.providers
            .iter()
            .map(ClientFontProviderDefinition::provider_type)
    }

    fn report(&self) -> ClientFontDefinitionReloadReport<'_> {
        ClientFontDefinitionReloadReport {
            resource: self.resource.report(),
            providers: &self.providers,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClientFontProviderDefinition {
    provider_type: String,
}

impl ClientFontProviderDefinition {
    pub fn provider_type(&self) -> &str {
        &self.provider_type
    }
}

#[derive(Deserialize)]
struct RawClientFontDefinition {
    providers: Vec<RawClientFontProviderDefinition>,
}

#[derive(Deserialize)]
struct RawClientFontProviderDefinition {
    #[serde(rename = "type")]
    provider_type: String,
    #[serde(flatten)]
    fields: BTreeMap<String, serde_json::Value>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ClientFontDefinitionReloadReport<'a> {
    resource: &'a ClientJsonResourceReloadReport,
    providers: &'a [ClientFontProviderDefinition],
}

impl ClientFontDefinitionReloadReport<'_> {
    pub fn resource(&self) -> &ClientJsonResourceReloadReport {
        self.resource
    }

    pub fn provider_count(&self) -> usize {
        self.providers.len()
    }

    pub fn provider_types(&self) -> impl Iterator<Item = &str> {
        self.providers
            .iter()
            .map(ClientFontProviderDefinition::provider_type)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FontDefinitionsReloadListener {
    definitions: Vec<String>,
}

impl FontDefinitionsReloadListener {
    pub fn new(definitions: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            definitions: definitions.into_iter().map(Into::into).collect(),
        }
    }

    pub fn definitions(&self) -> &[String] {
        &self.definitions
    }

    pub fn load(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ClientFontDefinitionSet> {
        load_client_font_definitions(stack, &self.definitions)
    }
}

impl Default for FontDefinitionsReloadListener {
    fn default() -> Self {
        Self {
            definitions: Vec::new(),
        }
    }
}

impl ResourceReloadListener for FontDefinitionsReloadListener {
    fn name(&self) -> &str {
        "font_definitions"
    }

    fn prepare(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        Ok(ResourceReloadTaskReport::new(
            self.font_definition_resources(stack)?,
        ))
    }

    fn reload(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        let definitions = self.load(stack)?;
        Ok(ResourceReloadTaskReport::new(
            definitions.reports().map(font_definition_report_item),
        ))
    }
}

impl FontDefinitionsReloadListener {
    fn font_definition_resources(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<Vec<String>> {
        if self.definitions.is_empty() {
            discover_font_definition_resources(stack)
        } else {
            for definition in &self.definitions {
                stack.require_resource(definition)?;
            }
            Ok(self.definitions.clone())
        }
    }
}

pub fn load_client_font_definitions(
    stack: &ClientResourceStack,
    definitions: &[String],
) -> ResourceReloadResult<ClientFontDefinitionSet> {
    let definitions = if definitions.is_empty() {
        discover_font_definition_resources(stack)?
    } else {
        definitions.to_vec()
    };
    let mut loaded = Vec::new();

    for definition in &definitions {
        let locations = stack.resource_stack(definition);
        if locations.is_empty() {
            return Err(ResourceReloadError::MissingResource(definition.clone()));
        }

        for location in locations {
            let resource = read_client_json_resource(Path::new(definition), &location)?;
            let providers = parse_font_definition_providers(&resource)?;
            loaded.push(ClientFontDefinition {
                resource,
                providers,
            });
        }
    }

    Ok(ClientFontDefinitionSet {
        definitions: loaded,
    })
}

fn discover_font_definition_resources(
    stack: &ClientResourceStack,
) -> ResourceReloadResult<Vec<String>> {
    let mut resources = BTreeSet::new();

    for pack in stack.packs() {
        let font_directory = pack.resource_path(FONT_DEFINITION_ROOT_DIR);
        if !font_directory.exists() {
            continue;
        }
        collect_font_definition_resources(
            FONT_DEFINITION_ROOT_DIR,
            &font_directory,
            &font_directory,
            &mut resources,
        )?;
    }

    if resources.is_empty() {
        return Err(ResourceReloadError::MissingResource(format!(
            "{FONT_DEFINITION_ROOT_DIR}/**/*.json"
        )));
    }

    Ok(resources.into_iter().collect())
}

fn collect_font_definition_resources(
    resource_root: &str,
    root: &Path,
    directory: &Path,
    resources: &mut BTreeSet<String>,
) -> ResourceReloadResult<()> {
    let entries = fs::read_dir(directory).map_err(|source| ResourceReloadError::ReadResource {
        resource: resource_root.to_owned(),
        path: directory.to_owned(),
        source,
    })?;

    for entry in entries {
        let entry = entry.map_err(|source| ResourceReloadError::ReadResource {
            resource: resource_root.to_owned(),
            path: directory.to_owned(),
            source,
        })?;
        let path = entry.path();
        if path.is_dir() {
            collect_font_definition_resources(resource_root, root, &path, resources)?;
            continue;
        }
        if !path.is_file()
            || path.extension().and_then(|extension| extension.to_str()) != Some("json")
        {
            continue;
        }

        let Ok(relative) = path.strip_prefix(root) else {
            continue;
        };
        let resource = Path::new(resource_root)
            .join(relative)
            .to_string_lossy()
            .replace('\\', "/");
        resources.insert(resource);
    }

    Ok(())
}

fn parse_font_definition_providers(
    resource: &ClientJsonResource,
) -> ResourceReloadResult<Vec<ClientFontProviderDefinition>> {
    let providers = resource
        .value()
        .get("providers")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| invalid_font_definition(resource, "missing providers array"))?;

    if let Some(index) = providers.iter().position(|provider| !provider.is_object()) {
        return Err(invalid_font_definition(
            resource,
            format!("provider {index} is not an object"),
        ));
    }

    let definition: RawClientFontDefinition = serde_json::from_value(resource.value().clone())
        .map_err(|source| {
            invalid_font_definition(resource, format!("invalid provider definition: {source}"))
        })?;

    definition
        .providers
        .into_iter()
        .enumerate()
        .map(|(index, provider)| {
            validate_font_provider_definition(resource, index, &provider)?;
            Ok(ClientFontProviderDefinition {
                provider_type: provider.provider_type,
            })
        })
        .collect()
}

fn validate_font_provider_definition(
    resource: &ClientJsonResource,
    index: usize,
    provider: &RawClientFontProviderDefinition,
) -> ResourceReloadResult<()> {
    validate_font_provider_filter(resource, index, provider)?;

    match provider.provider_type.as_str() {
        "reference" => require_font_provider_string_field(resource, index, provider, "id"),
        "bitmap" => {
            require_font_provider_string_field(resource, index, provider, "file")?;
            require_font_provider_i64_field(resource, index, provider, "ascent")?;
            validate_bitmap_font_provider_chars(resource, index, provider)
        }
        "space" => require_font_provider_object_field(resource, index, provider, "advances")
            .and_then(|advances| {
                if advances.values().all(serde_json::Value::is_number) {
                    Ok(())
                } else {
                    Err(invalid_font_definition(
                        resource,
                        format!("provider {index} space advances must be numbers"),
                    ))
                }
            }),
        "ttf" => {
            require_font_provider_string_field(resource, index, provider, "file")?;
            if let Some(shift) = provider.fields.get("shift") {
                let Some(shift) = shift.as_array() else {
                    return Err(invalid_font_definition(
                        resource,
                        format!("provider {index} ttf shift must be an array"),
                    ));
                };
                if shift.len() != 2 || !shift.iter().all(serde_json::Value::is_number) {
                    return Err(invalid_font_definition(
                        resource,
                        format!("provider {index} ttf shift must contain 2 numbers"),
                    ));
                }
            }
            Ok(())
        }
        "unihex" => require_font_provider_string_field(resource, index, provider, "hex_file"),
        other => Err(invalid_font_definition(
            resource,
            format!("provider {index} has unsupported type `{other}`"),
        )),
    }
}

fn validate_font_provider_filter(
    resource: &ClientJsonResource,
    index: usize,
    provider: &RawClientFontProviderDefinition,
) -> ResourceReloadResult<()> {
    let Some(filter) = provider.fields.get("filter") else {
        return Ok(());
    };
    let Some(filter) = filter.as_object() else {
        return Err(invalid_font_definition(
            resource,
            format!("provider {index} filter must be an object"),
        ));
    };
    for (key, value) in filter {
        if !matches!(key.as_str(), "uniform" | "jp") {
            return Err(invalid_font_definition(
                resource,
                format!("provider {index} filter has unsupported option `{key}`"),
            ));
        }
        if !value.is_boolean() {
            return Err(invalid_font_definition(
                resource,
                format!("provider {index} filter `{key}` must be a boolean"),
            ));
        }
    }
    Ok(())
}

fn validate_bitmap_font_provider_chars(
    resource: &ClientJsonResource,
    index: usize,
    provider: &RawClientFontProviderDefinition,
) -> ResourceReloadResult<()> {
    let Some(chars) = provider
        .fields
        .get("chars")
        .and_then(serde_json::Value::as_array)
    else {
        return Err(invalid_font_definition(
            resource,
            format!("provider {index} bitmap chars must be an array"),
        ));
    };
    if chars.is_empty() {
        return Err(invalid_font_definition(
            resource,
            format!("provider {index} bitmap chars must not be empty"),
        ));
    }

    let mut row_width = None;
    for row in chars {
        let Some(row) = row.as_str() else {
            return Err(invalid_font_definition(
                resource,
                format!("provider {index} bitmap chars rows must be strings"),
            ));
        };
        let width = row.chars().count();
        if width == 0 {
            return Err(invalid_font_definition(
                resource,
                format!("provider {index} bitmap chars rows must not be empty"),
            ));
        }
        match row_width {
            Some(expected) if expected != width => {
                return Err(invalid_font_definition(
                    resource,
                    format!("provider {index} bitmap chars rows must have equal width"),
                ));
            }
            Some(_) => {}
            None => row_width = Some(width),
        }
    }

    Ok(())
}

fn require_font_provider_string_field(
    resource: &ClientJsonResource,
    index: usize,
    provider: &RawClientFontProviderDefinition,
    field: &str,
) -> ResourceReloadResult<()> {
    provider
        .fields
        .get(field)
        .and_then(serde_json::Value::as_str)
        .map(|_| ())
        .ok_or_else(|| {
            invalid_font_definition(
                resource,
                format!("provider {index} `{field}` must be a string"),
            )
        })
}

fn require_font_provider_i64_field(
    resource: &ClientJsonResource,
    index: usize,
    provider: &RawClientFontProviderDefinition,
    field: &str,
) -> ResourceReloadResult<()> {
    provider
        .fields
        .get(field)
        .and_then(serde_json::Value::as_i64)
        .map(|_| ())
        .ok_or_else(|| {
            invalid_font_definition(
                resource,
                format!("provider {index} `{field}` must be an integer"),
            )
        })
}

fn require_font_provider_object_field<'a>(
    resource: &ClientJsonResource,
    index: usize,
    provider: &'a RawClientFontProviderDefinition,
    field: &str,
) -> ResourceReloadResult<&'a serde_json::Map<String, serde_json::Value>> {
    provider
        .fields
        .get(field)
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| {
            invalid_font_definition(
                resource,
                format!("provider {index} `{field}` must be an object"),
            )
        })
}

fn invalid_font_definition(
    resource: &ClientJsonResource,
    reason: impl Into<String>,
) -> ResourceReloadError {
    ResourceReloadError::InvalidFontDefinition {
        resource: resource.report().resource().to_owned(),
        pack_id: resource.report().pack_id().to_owned(),
        reason: reason.into(),
    }
}

fn font_definition_report_item(report: ClientFontDefinitionReloadReport<'_>) -> String {
    let provider_types = report.provider_types().collect::<Vec<_>>().join(",");
    format!(
        "{}:providers:{}:types:{}",
        report.resource().loaded_resource_pack(),
        report.provider_count(),
        provider_types
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClientLanguageResources {
    translations: BTreeMap<String, String>,
    report: ClientLanguageReloadReport,
}

impl ClientLanguageResources {
    pub fn translations(&self) -> &BTreeMap<String, String> {
        &self.translations
    }

    pub fn report(&self) -> &ClientLanguageReloadReport {
        &self.report
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClientLanguageReloadReport {
    language_code: String,
    loaded_files: Vec<String>,
    translation_count: usize,
}

impl ClientLanguageReloadReport {
    fn new(
        language_code: impl Into<String>,
        loaded_files: Vec<String>,
        translation_count: usize,
    ) -> Self {
        Self {
            language_code: language_code.into(),
            loaded_files,
            translation_count,
        }
    }

    pub fn language_code(&self) -> &str {
        &self.language_code
    }

    pub fn loaded_files(&self) -> &[String] {
        &self.loaded_files
    }

    pub fn translation_count(&self) -> usize {
        self.translation_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClientLanguageReloadListener {
    requested_language_code: String,
}

impl ClientLanguageReloadListener {
    pub fn new(requested_language_code: impl Into<String>) -> Self {
        Self {
            requested_language_code: requested_language_code.into(),
        }
    }

    pub fn requested_language_code(&self) -> &str {
        &self.requested_language_code
    }

    pub fn load(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ClientLanguageResources> {
        load_client_language_resources(stack, &self.requested_language_code)
    }
}

impl ResourceReloadListener for ClientLanguageReloadListener {
    fn name(&self) -> &str {
        "client_languages"
    }

    fn prepare(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        stack.require_resource(language_resource_path(DEFAULT_LANGUAGE_CODE))?;
        Ok(ResourceReloadTaskReport::new(
            language_resource_paths(&self.requested_language_code)
                .into_iter()
                .map(|path| path.to_string_lossy().into_owned()),
        ))
    }

    fn reload(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        let resources = self.load(stack)?;
        Ok(ResourceReloadTaskReport::new([
            format!("language:{}", resources.report.language_code()),
            format!("files:{}", resources.report.loaded_files().len()),
            format!("translations:{}", resources.report.translation_count()),
        ]))
    }
}

pub fn load_client_language_resources(
    stack: &ClientResourceStack,
    requested_language_code: &str,
) -> ResourceReloadResult<ClientLanguageResources> {
    stack.require_resource(language_resource_path(DEFAULT_LANGUAGE_CODE))?;

    let requested_language_code = requested_language_code.to_ascii_lowercase();
    let mut translations = BTreeMap::new();
    let mut loaded_files = Vec::new();

    for resource in language_resource_paths(&requested_language_code) {
        for location in stack.resource_stack(&resource) {
            let language_entries = read_language_resource(&resource, &location)?;
            translations.extend(language_entries);
            loaded_files.push(format!("{}@{}", resource.display(), location.pack_id));
        }
    }

    let report =
        ClientLanguageReloadReport::new(requested_language_code, loaded_files, translations.len());

    Ok(ClientLanguageResources {
        translations,
        report,
    })
}

fn language_resource_paths(language_code: &str) -> Vec<PathBuf> {
    let mut paths = vec![language_resource_path(DEFAULT_LANGUAGE_CODE)];
    if language_code != DEFAULT_LANGUAGE_CODE {
        paths.push(language_resource_path(language_code));
    }
    paths
}

fn language_resource_path(language_code: &str) -> PathBuf {
    PathBuf::from(format!("assets/minecraft/lang/{language_code}.json"))
}

fn read_language_resource(
    resource: &Path,
    location: &ResourceLocation,
) -> ResourceReloadResult<BTreeMap<String, String>> {
    let contents =
        location
            .read_to_string()
            .map_err(|source| ResourceReloadError::ReadResource {
                resource: resource.to_string_lossy().into_owned(),
                path: location.path.clone(),
                source,
            })?;

    serde_json::from_str(&contents).map_err(|source| ResourceReloadError::ParseResourceJson {
        resource: resource.to_string_lossy().into_owned(),
        path: location.path.clone(),
        source,
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequiredVanillaAssetsListener {
    name: String,
    required_assets: Vec<String>,
}

impl RequiredVanillaAssetsListener {
    pub fn new(required_assets: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            name: "vanilla_required_assets".to_owned(),
            required_assets: required_assets.into_iter().map(Into::into).collect(),
        }
    }

    pub fn required_assets(&self) -> &[String] {
        &self.required_assets
    }
}

impl Default for RequiredVanillaAssetsListener {
    fn default() -> Self {
        Self::new(DEFAULT_REQUIRED_VANILLA_ASSETS.iter().copied())
    }
}

impl ResourceReloadListener for RequiredVanillaAssetsListener {
    fn name(&self) -> &str {
        &self.name
    }

    fn prepare(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        for resource in &self.required_assets {
            stack.require_resource(resource)?;
        }

        Ok(ResourceReloadTaskReport::new(self.required_assets.clone()))
    }

    fn reload(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        let mut loaded = Vec::with_capacity(self.required_assets.len());
        for resource in &self.required_assets {
            let location = stack.require_resource(resource)?;
            let bytes =
                location
                    .read_bytes()
                    .map_err(|source| ResourceReloadError::ReadResource {
                        resource: resource.clone(),
                        path: location.path.clone(),
                        source,
                    })?;
            loaded.push(format!("{}:{} bytes", resource, bytes.len()));
        }

        Ok(ResourceReloadTaskReport::new(loaded))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListingResourceReloadListener {
    name: String,
    resources: Vec<String>,
}

impl ListingResourceReloadListener {
    pub fn new(
        name: impl Into<String>,
        resources: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            name: name.into(),
            resources: resources.into_iter().map(Into::into).collect(),
        }
    }
}

impl ResourceReloadListener for ListingResourceReloadListener {
    fn name(&self) -> &str {
        &self.name
    }

    fn prepare(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        for resource in &self.resources {
            stack.require_resource(resource)?;
        }

        Ok(ResourceReloadTaskReport::new(self.resources.clone()))
    }

    fn reload(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        let mut found = Vec::with_capacity(self.resources.len());
        for resource in &self.resources {
            let location = stack.require_resource(resource)?;
            found.push(format!("{resource}@{}", location.pack_id));
        }

        Ok(ResourceReloadTaskReport::new(found))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClientSplashes {
    lines: Vec<String>,
    report: ClientSplashesReloadReport,
}

impl ClientSplashes {
    pub fn lines(&self) -> &[String] {
        &self.lines
    }

    pub fn count(&self) -> usize {
        self.lines.len()
    }

    pub fn report(&self) -> &ClientSplashesReloadReport {
        &self.report
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClientSplashesReloadReport {
    resource: String,
    pack_id: String,
    splash_count: usize,
}

impl ClientSplashesReloadReport {
    fn new(resource: impl Into<String>, pack_id: impl Into<String>, splash_count: usize) -> Self {
        Self {
            resource: resource.into(),
            pack_id: pack_id.into(),
            splash_count,
        }
    }

    pub fn resource(&self) -> &str {
        &self.resource
    }

    pub fn pack_id(&self) -> &str {
        &self.pack_id
    }

    pub fn splash_count(&self) -> usize {
        self.splash_count
    }

    pub fn loaded_resource_pack(&self) -> String {
        format!("{}@{}", self.resource, self.pack_id)
    }
}

pub fn load_client_splashes_resource(
    stack: &ClientResourceStack,
    resource: impl AsRef<Path>,
) -> ResourceReloadResult<ClientSplashes> {
    let resource = resource.as_ref();
    let location = stack.require_resource(resource)?;
    let contents =
        location
            .read_to_string()
            .map_err(|source| ResourceReloadError::ReadResource {
                resource: resource.to_string_lossy().into_owned(),
                path: location.path.clone(),
                source,
            })?;
    let lines = parse_splash_lines(&contents);
    let report = ClientSplashesReloadReport::new(
        resource.to_string_lossy().into_owned(),
        location.pack_id,
        lines.len(),
    );

    Ok(ClientSplashes { lines, report })
}

fn parse_splash_lines(contents: &str) -> Vec<String> {
    contents
        .lines()
        .map(str::trim)
        .filter(|line| java_string_hash_code(line) != VANILLA_EXCLUDED_SPLASH_JAVA_HASH)
        .map(str::to_owned)
        .collect()
}

fn java_string_hash_code(text: &str) -> i32 {
    text.encode_utf16().fold(0_i32, |hash, unit| {
        hash.wrapping_mul(31).wrapping_add(i32::from(unit))
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SplashesReloadListener {
    name: String,
    resource: String,
}

impl SplashesReloadListener {
    pub fn new(resource: impl Into<String>) -> Self {
        Self {
            name: "splashes".to_owned(),
            resource: resource.into(),
        }
    }

    pub fn load(&self, stack: &ClientResourceStack) -> ResourceReloadResult<ClientSplashes> {
        load_client_splashes_resource(stack, &self.resource)
    }
}

impl Default for SplashesReloadListener {
    fn default() -> Self {
        Self::new(SPLASHES_RESOURCE)
    }
}

impl ResourceReloadListener for SplashesReloadListener {
    fn name(&self) -> &str {
        &self.name
    }

    fn prepare(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        stack.require_resource(&self.resource)?;

        Ok(ResourceReloadTaskReport::new([self.resource.clone()]))
    }

    fn reload(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        let splashes = self.load(stack)?;
        let report = splashes.report();

        Ok(ResourceReloadTaskReport::new([format!(
            "{}:{} splashes",
            report.loaded_resource_pack(),
            report.splash_count()
        )]))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AtlasManifestReloadListener {
    name: String,
    manifests: Vec<String>,
}

impl AtlasManifestReloadListener {
    pub fn new(manifests: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            name: "atlas_manifests".to_owned(),
            manifests: manifests.into_iter().map(Into::into).collect(),
        }
    }

    pub fn manifests(&self) -> &[String] {
        &self.manifests
    }
}

impl Default for AtlasManifestReloadListener {
    fn default() -> Self {
        Self::new(DEFAULT_ATLAS_MANIFESTS.iter().copied())
    }
}

impl ResourceReloadListener for AtlasManifestReloadListener {
    fn name(&self) -> &str {
        &self.name
    }

    fn prepare(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        for manifest in &self.manifests {
            stack.require_resource(manifest)?;
        }

        Ok(ResourceReloadTaskReport::new(self.manifests.clone()))
    }

    fn reload(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        let mut loaded = Vec::with_capacity(self.manifests.len());
        for manifest in &self.manifests {
            let location = stack.require_resource(manifest)?;
            let contents =
                location
                    .read_to_string()
                    .map_err(|source| ResourceReloadError::ReadResource {
                        resource: manifest.clone(),
                        path: location.path.clone(),
                        source,
                    })?;
            serde_json::from_str::<serde_json::Value>(&contents).map_err(|source| {
                ResourceReloadError::ParseResourceJson {
                    resource: manifest.clone(),
                    path: location.path,
                    source,
                }
            })?;
            loaded.push(format!("{manifest}@{}", location.pack_id));
        }

        Ok(ResourceReloadTaskReport::new(loaded))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AtlasSourceCollection {
    atlases: Vec<AtlasSourceManifest>,
}

impl AtlasSourceCollection {
    pub fn atlases(&self) -> &[AtlasSourceManifest] {
        &self.atlases
    }

    pub fn reports(&self) -> impl Iterator<Item = AtlasSourceManifestReport> + '_ {
        self.atlases.iter().map(AtlasSourceManifest::report)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AtlasSourceManifest {
    resource: ClientJsonResource,
    source_count: usize,
}

impl AtlasSourceManifest {
    pub fn resource(&self) -> &ClientJsonResource {
        &self.resource
    }

    pub fn source_count(&self) -> usize {
        self.source_count
    }

    pub fn report(&self) -> AtlasSourceManifestReport {
        AtlasSourceManifestReport {
            resource: self.resource.report().resource().to_owned(),
            pack_id: self.resource.report().pack_id().to_owned(),
            source_count: self.source_count,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AtlasSourceManifestReport {
    resource: String,
    pack_id: String,
    source_count: usize,
}

impl AtlasSourceManifestReport {
    pub fn resource(&self) -> &str {
        &self.resource
    }

    pub fn pack_id(&self) -> &str {
        &self.pack_id
    }

    pub fn source_count(&self) -> usize {
        self.source_count
    }

    pub fn loaded_resource_pack(&self) -> String {
        format!("{}@{}", self.resource, self.pack_id)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AtlasSourceReloadListener {
    name: String,
    manifests: Vec<String>,
}

impl AtlasSourceReloadListener {
    pub fn new(manifests: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            name: "atlas_sources".to_owned(),
            manifests: manifests.into_iter().map(Into::into).collect(),
        }
    }

    pub fn manifests(&self) -> &[String] {
        &self.manifests
    }

    pub fn load(&self, stack: &ClientResourceStack) -> ResourceReloadResult<AtlasSourceCollection> {
        load_client_atlas_sources(stack, &self.manifests)
    }
}

impl Default for AtlasSourceReloadListener {
    fn default() -> Self {
        Self::new(DEFAULT_ATLAS_MANIFESTS.iter().copied())
    }
}

impl ResourceReloadListener for AtlasSourceReloadListener {
    fn name(&self) -> &str {
        &self.name
    }

    fn prepare(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        for manifest in &self.manifests {
            stack.require_resource(manifest)?;
        }

        Ok(ResourceReloadTaskReport::new(self.manifests.clone()))
    }

    fn reload(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        let sources = self.load(stack)?;
        Ok(ResourceReloadTaskReport::new(
            sources
                .reports()
                .map(|report| atlas_source_report_item(&report)),
        ))
    }
}

pub fn load_client_atlas_sources(
    stack: &ClientResourceStack,
    manifests: &[String],
) -> ResourceReloadResult<AtlasSourceCollection> {
    let mut loaded = Vec::new();

    for manifest in manifests {
        let locations = stack.resource_stack(manifest);
        if locations.is_empty() {
            return Err(ResourceReloadError::MissingResource(manifest.clone()));
        }

        for location in locations {
            let resource = load_client_json_resource_at_location(manifest, location)?;
            let source_count = validate_atlas_sources(&resource)?;
            loaded.push(AtlasSourceManifest {
                resource,
                source_count,
            });
        }
    }

    Ok(AtlasSourceCollection { atlases: loaded })
}

fn load_client_json_resource_at_location(
    resource: &str,
    location: ResourceLocation,
) -> ResourceReloadResult<ClientJsonResource> {
    let contents =
        location
            .read_to_string()
            .map_err(|source| ResourceReloadError::ReadResource {
                resource: resource.to_owned(),
                path: location.path.clone(),
                source,
            })?;
    let value = serde_json::from_str::<serde_json::Value>(&contents).map_err(|source| {
        ResourceReloadError::ParseResourceJson {
            resource: resource.to_owned(),
            path: location.path.clone(),
            source,
        }
    })?;
    let report = ClientJsonResourceReloadReport::new(
        resource,
        location.pack_id,
        ClientJsonTopLevelShape::from_value(&value),
    );

    Ok(ClientJsonResource { value, report })
}

fn validate_atlas_sources(resource: &ClientJsonResource) -> ResourceReloadResult<usize> {
    let sources = resource
        .value()
        .get("sources")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| invalid_atlas_sources(resource, "missing sources array"))?;

    for (index, source) in sources.iter().enumerate() {
        let Some(source) = source.as_object() else {
            return Err(invalid_atlas_sources(
                resource,
                format!("source {index} is not an object"),
            ));
        };
        let Some(source_type) = source.get("type") else {
            return Err(invalid_atlas_sources(
                resource,
                format!("source {index} is missing a type field"),
            ));
        };
        if !source_type.is_string() {
            return Err(invalid_atlas_sources(
                resource,
                format!("source {index} type field is not a string"),
            ));
        }
    }

    Ok(sources.len())
}

fn invalid_atlas_sources(
    resource: &ClientJsonResource,
    reason: impl Into<String>,
) -> ResourceReloadError {
    ResourceReloadError::InvalidAtlasSources {
        resource: resource.report().resource().to_owned(),
        pack_id: resource.report().pack_id().to_owned(),
        reason: reason.into(),
    }
}

fn atlas_source_report_item(report: &AtlasSourceManifestReport) -> String {
    format!(
        "{}:{} sources",
        report.loaded_resource_pack(),
        report.source_count()
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColormapReloadListener {
    name: String,
    colormaps: Vec<String>,
}

impl ColormapReloadListener {
    pub fn new(colormaps: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            name: "colormaps".to_owned(),
            colormaps: colormaps.into_iter().map(Into::into).collect(),
        }
    }

    pub fn colormaps(&self) -> &[String] {
        &self.colormaps
    }

    pub fn load(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<Vec<ClientColormapResource>> {
        load_client_colormap_resources(stack, &self.colormaps)
    }
}

impl Default for ColormapReloadListener {
    fn default() -> Self {
        Self::new(DEFAULT_COLORMAPS.iter().copied())
    }
}

impl ResourceReloadListener for ColormapReloadListener {
    fn name(&self) -> &str {
        &self.name
    }

    fn prepare(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        for colormap in &self.colormaps {
            stack.require_resource(colormap)?;
        }

        Ok(ResourceReloadTaskReport::new(self.colormaps.clone()))
    }

    fn reload(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        Ok(ResourceReloadTaskReport::new(
            self.load(stack)?
                .iter()
                .map(colormap_report_item)
                .collect::<Vec<_>>(),
        ))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClientColormapResource {
    report: ClientColormapReloadReport,
    pixels: Vec<u32>,
}

impl ClientColormapResource {
    pub fn report(&self) -> &ClientColormapReloadReport {
        &self.report
    }

    pub fn pixels(&self) -> &[u32] {
        &self.pixels
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClientColormapReloadReport {
    resource: String,
    pack_id: String,
    byte_count: usize,
    width: u32,
    height: u32,
    pixel_count: u64,
}

impl ClientColormapReloadReport {
    fn new(
        resource: impl Into<String>,
        pack_id: impl Into<String>,
        byte_count: usize,
        image: &DecodedPngImage,
    ) -> Self {
        Self {
            resource: resource.into(),
            pack_id: pack_id.into(),
            byte_count,
            width: image.width,
            height: image.height,
            pixel_count: image.pixel_count(),
        }
    }

    pub fn resource(&self) -> &str {
        &self.resource
    }

    pub fn pack_id(&self) -> &str {
        &self.pack_id
    }

    pub fn byte_count(&self) -> usize {
        self.byte_count
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixel_count(&self) -> u64 {
        self.pixel_count
    }

    pub fn loaded_resource_pack(&self) -> String {
        format!("{}@{}", self.resource, self.pack_id)
    }
}

pub fn load_client_colormap_resources(
    stack: &ClientResourceStack,
    colormaps: &[String],
) -> ResourceReloadResult<Vec<ClientColormapResource>> {
    let mut loaded = Vec::with_capacity(colormaps.len());

    for colormap in colormaps {
        let location = stack.require_resource(colormap)?;
        let bytes = location
            .read_bytes()
            .map_err(|source| ResourceReloadError::ReadResource {
                resource: colormap.clone(),
                path: location.path.clone(),
                source,
            })?;
        let image = decode_png_rgba_image(colormap, &location, &bytes)?;

        loaded.push(ClientColormapResource {
            report: ClientColormapReloadReport::new(
                colormap.clone(),
                location.pack_id,
                bytes.len(),
                &image,
            ),
            pixels: image.pixels,
        });
    }

    Ok(loaded)
}

fn colormap_report_item(colormap: &ClientColormapResource) -> String {
    let report = colormap.report();
    format!(
        "{}:{} bytes:rgba8:{}x{}:{} pixels",
        report.loaded_resource_pack(),
        report.byte_count(),
        report.width(),
        report.height(),
        report.pixel_count()
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CloudTextureReloadListener {
    resource: String,
}

impl CloudTextureReloadListener {
    pub fn new(resource: impl Into<String>) -> Self {
        Self {
            resource: resource.into(),
        }
    }

    pub fn resource(&self) -> &str {
        &self.resource
    }
}

impl Default for CloudTextureReloadListener {
    fn default() -> Self {
        Self::new(CLOUDS_TEXTURE_RESOURCE)
    }
}

impl ResourceReloadListener for CloudTextureReloadListener {
    fn name(&self) -> &str {
        "cloud_texture"
    }

    fn prepare(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        stack.require_resource(&self.resource)?;

        Ok(ResourceReloadTaskReport::new([self.resource.clone()]))
    }

    fn reload(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        let location = stack.require_resource(&self.resource)?;
        let bytes = location
            .read_bytes()
            .map_err(|source| ResourceReloadError::ReadResource {
                resource: self.resource.clone(),
                path: location.path.clone(),
                source,
            })?;

        validate_png_signature(&self.resource, &location, &bytes)?;

        Ok(ResourceReloadTaskReport::new([format!(
            "{}@{}:{} bytes:png-signature-ok",
            self.resource,
            location.pack_id,
            bytes.len()
        )]))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextureMetadataReloadListener {
    name: String,
    textures: Vec<String>,
}

impl TextureMetadataReloadListener {
    pub fn new(textures: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            name: "texture_metadata".to_owned(),
            textures: textures.into_iter().map(Into::into).collect(),
        }
    }

    pub fn textures(&self) -> &[String] {
        &self.textures
    }

    pub fn load(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<Vec<ClientTextureMetadataResource>> {
        load_client_texture_metadata_resources(stack, &self.textures)
    }
}

impl Default for TextureMetadataReloadListener {
    fn default() -> Self {
        Self::new(DEFAULT_REPRESENTATIVE_TEXTURES.iter().copied())
    }
}

impl ResourceReloadListener for TextureMetadataReloadListener {
    fn name(&self) -> &str {
        &self.name
    }

    fn prepare(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        for texture in &self.textures {
            stack.require_resource(texture)?;
        }

        Ok(ResourceReloadTaskReport::new(self.textures.clone()))
    }

    fn reload(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        Ok(ResourceReloadTaskReport::new(
            self.load(stack)?
                .iter()
                .map(texture_metadata_report_item)
                .collect::<Vec<_>>(),
        ))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClientTextureMetadataResource {
    report: ClientTextureMetadataReloadReport,
}

impl ClientTextureMetadataResource {
    pub fn report(&self) -> &ClientTextureMetadataReloadReport {
        &self.report
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClientTextureMetadataReloadReport {
    resource: String,
    pack_id: String,
    byte_count: usize,
    mcmeta: Option<ClientJsonResourceReloadReport>,
}

impl ClientTextureMetadataReloadReport {
    fn new(
        resource: impl Into<String>,
        pack_id: impl Into<String>,
        byte_count: usize,
        mcmeta: Option<ClientJsonResourceReloadReport>,
    ) -> Self {
        Self {
            resource: resource.into(),
            pack_id: pack_id.into(),
            byte_count,
            mcmeta,
        }
    }

    pub fn resource(&self) -> &str {
        &self.resource
    }

    pub fn pack_id(&self) -> &str {
        &self.pack_id
    }

    pub fn byte_count(&self) -> usize {
        self.byte_count
    }

    pub fn mcmeta(&self) -> Option<&ClientJsonResourceReloadReport> {
        self.mcmeta.as_ref()
    }

    pub fn loaded_resource_pack(&self) -> String {
        format!("{}@{}", self.resource, self.pack_id)
    }
}

pub fn load_client_texture_metadata_resources(
    stack: &ClientResourceStack,
    textures: &[String],
) -> ResourceReloadResult<Vec<ClientTextureMetadataResource>> {
    let mut loaded = Vec::with_capacity(textures.len());

    for texture in textures {
        let location = stack.require_resource(texture)?;
        let bytes = location
            .read_bytes()
            .map_err(|source| ResourceReloadError::ReadResource {
                resource: texture.clone(),
                path: location.path.clone(),
                source,
            })?;
        validate_png_signature(texture, &location, &bytes)?;

        let mcmeta_resource = format!("{texture}.mcmeta");
        let mcmeta = if stack.find_resource(&mcmeta_resource).is_some() {
            Some(
                load_client_json_resource(stack, &mcmeta_resource)?
                    .report()
                    .clone(),
            )
        } else {
            None
        };

        loaded.push(ClientTextureMetadataResource {
            report: ClientTextureMetadataReloadReport::new(
                texture.clone(),
                location.pack_id,
                bytes.len(),
                mcmeta,
            ),
        });
    }

    Ok(loaded)
}

fn texture_metadata_report_item(texture: &ClientTextureMetadataResource) -> String {
    let report = texture.report();
    let mcmeta = report
        .mcmeta()
        .map(|mcmeta| {
            format!(
                "mcmeta@{}:{}",
                mcmeta.pack_id(),
                mcmeta.top_level_shape().report_fragment()
            )
        })
        .unwrap_or_else(|| "mcmeta:none".to_owned());

    format!(
        "{}:{} bytes:png-signature-ok:{mcmeta}",
        report.loaded_resource_pack(),
        report.byte_count()
    )
}

fn validate_png_signature(
    resource: &str,
    location: &ResourceLocation,
    bytes: &[u8],
) -> ResourceReloadResult<()> {
    if bytes.starts_with(PNG_SIGNATURE) {
        Ok(())
    } else {
        Err(ResourceReloadError::InvalidPngSignature {
            resource: resource.to_owned(),
            path: location.path.clone(),
            byte_count: bytes.len(),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DecodedPngImage {
    width: u32,
    height: u32,
    pixels: Vec<u32>,
}

impl DecodedPngImage {
    fn pixel_count(&self) -> u64 {
        u64::from(self.width) * u64::from(self.height)
    }
}

fn decode_png_rgba_image(
    resource: &str,
    location: &ResourceLocation,
    bytes: &[u8],
) -> ResourceReloadResult<DecodedPngImage> {
    validate_png_signature(resource, location, bytes)?;

    let mut decoder = png::Decoder::new(Cursor::new(bytes));
    decoder.set_transformations(
        png::Transformations::normalize_to_color8() | png::Transformations::ALPHA,
    );
    let mut reader = decoder
        .read_info()
        .map_err(|source| invalid_png_metadata(resource, location, source.to_string()))?;
    let output_buffer_size = reader.output_buffer_size().ok_or_else(|| {
        invalid_png_metadata(resource, location, "decoded png output buffer is too large")
    })?;
    let mut rgba = vec![0; output_buffer_size];
    let output = reader
        .next_frame(&mut rgba)
        .map_err(|source| invalid_png_metadata(resource, location, source.to_string()))?;

    if output.color_type != png::ColorType::Rgba || output.bit_depth != png::BitDepth::Eight {
        return Err(invalid_png_metadata(
            resource,
            location,
            format!(
                "expected RGBA8 output, got {:?} {:?}",
                output.color_type, output.bit_depth
            ),
        ));
    }

    let rgba = &rgba[..output.buffer_size()];
    let pixels = rgba
        .chunks_exact(4)
        .map(|pixel| {
            let [red, green, blue, alpha] = [pixel[0], pixel[1], pixel[2], pixel[3]];
            u32::from(alpha) << 24 | u32::from(red) << 16 | u32::from(green) << 8 | u32::from(blue)
        })
        .collect::<Vec<_>>();

    Ok(DecodedPngImage {
        width: output.width,
        height: output.height,
        pixels,
    })
}

fn invalid_png_metadata(
    resource: &str,
    location: &ResourceLocation,
    reason: impl Into<String>,
) -> ResourceReloadError {
    ResourceReloadError::InvalidPngMetadata {
        resource: resource.to_owned(),
        path: location.path.clone(),
        reason: reason.into(),
    }
}

#[derive(Error, Debug)]
pub enum ResourceReloadError {
    #[error("missing client resource `{0}`")]
    MissingResource(String),
    #[error("failed to read client resource `{resource}` at `{path}`")]
    ReadResource {
        resource: String,
        path: PathBuf,
        source: io::Error,
    },
    #[error("failed to parse client resource json `{resource}` at `{path}`")]
    ParseResourceJson {
        resource: String,
        path: PathBuf,
        source: serde_json::Error,
    },
    #[error(
        "invalid png signature for client resource `{resource}` at `{path}` ({byte_count} bytes)"
    )]
    InvalidPngSignature {
        resource: String,
        path: PathBuf,
        byte_count: usize,
    },
    #[error("invalid png metadata for client resource `{resource}` at `{path}`: {reason}")]
    InvalidPngMetadata {
        resource: String,
        path: PathBuf,
        reason: String,
    },
    #[error("invalid font definition `{resource}` from pack `{pack_id}`: {reason}")]
    InvalidFontDefinition {
        resource: String,
        pack_id: String,
        reason: String,
    },
    #[error("invalid atlas sources `{resource}` from pack `{pack_id}`: {reason}")]
    InvalidAtlasSources {
        resource: String,
        pack_id: String,
        reason: String,
    },
    #[error("invalid sound events resource `{resource}` at `{path}`: {reason}")]
    InvalidSoundEvents {
        resource: String,
        path: PathBuf,
        reason: String,
    },
    #[error("invalid particle manifest `{resource}` from pack `{pack_id}`: {reason}")]
    InvalidParticleManifest {
        resource: String,
        pack_id: String,
        reason: String,
    },
    #[error("invalid waypoint style manifest `{resource}` from pack `{pack_id}`: {reason}")]
    InvalidWaypointStyleManifest {
        resource: String,
        pack_id: String,
        reason: String,
    },
    #[error("invalid equipment asset `{resource}` from pack `{pack_id}`: {reason}")]
    InvalidEquipmentAsset {
        resource: String,
        pack_id: String,
        reason: String,
    },
    #[error("invalid shader source `{resource}` at `{path}`: {reason}")]
    InvalidShaderSource {
        resource: String,
        path: PathBuf,
        reason: String,
    },
    #[error("invalid model dependency `{resource}` from pack `{pack_id}`: {reason}")]
    InvalidModelDependency {
        resource: String,
        pack_id: String,
        reason: String,
    },
    #[error("invalid blockstate model references `{resource}` from pack `{pack_id}`: {reason}")]
    InvalidBlockstateModelReferences {
        resource: String,
        pack_id: String,
        reason: String,
    },
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        io::{Cursor, Write},
        path::{Path, PathBuf},
        sync::atomic::{AtomicU64, Ordering},
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::*;

    #[test]
    fn committed_vanilla_stack_finds_key_assets() {
        let stack = ClientResourceStack::vanilla();

        for resource in DEFAULT_REQUIRED_VANILLA_ASSETS {
            let location = stack
                .find_resource(resource)
                .unwrap_or_else(|| panic!("expected committed vanilla asset {resource}"));
            assert_eq!(location.pack_id, VANILLA_PACK_ID);
            assert!(location.path.ends_with(resource));
        }
    }

    #[test]
    fn repository_stack_always_keeps_vanilla_at_bottom() {
        let custom = TempPack::new();

        let repository = ClientResourceRepository::committed_vanilla()
            .with_available_pack(AvailableClientResourcePack::new(ClientResourcePack::new(
                "custom",
                custom.path(),
            )))
            .with_selected_pack_ids(["custom"]);

        assert_eq!(
            repository
                .stack()
                .packs()
                .iter()
                .map(ClientResourcePack::id)
                .collect::<Vec<_>>(),
            [VANILLA_PACK_ID, "custom"]
        );
    }

    #[test]
    fn repository_preserves_selected_pack_order_above_vanilla() {
        let low = TempPack::new();
        let high = TempPack::new();

        let repository = ClientResourceRepository::committed_vanilla()
            .with_available_pack(AvailableClientResourcePack::new(ClientResourcePack::new(
                "low",
                low.path(),
            )))
            .with_available_pack(AvailableClientResourcePack::new(ClientResourcePack::new(
                "high",
                high.path(),
            )))
            .with_selected_pack_ids(["high", "low"]);

        let report = repository.rebuild_stack();

        assert_eq!(
            report
                .stack()
                .packs()
                .iter()
                .map(ClientResourcePack::id)
                .collect::<Vec<_>>(),
            [VANILLA_PACK_ID, "high", "low"]
        );
        assert_eq!(
            report.selected_pack_ids(),
            ["high".to_owned(), "low".to_owned()]
        );
    }

    #[test]
    fn repository_reports_missing_selected_pack_ids_without_loading_them() {
        let present = TempPack::new();

        let repository = ClientResourceRepository::committed_vanilla()
            .with_available_pack(AvailableClientResourcePack::new(ClientResourcePack::new(
                "present",
                present.path(),
            )))
            .with_selected_pack_ids(["missing", "present"]);

        let report = repository.rebuild_stack();

        assert_eq!(report.missing_selected_pack_ids(), ["missing".to_owned()]);
        assert_eq!(
            report
                .stack()
                .packs()
                .iter()
                .map(ClientResourcePack::id)
                .collect::<Vec<_>>(),
            [VANILLA_PACK_ID, "present"]
        );
    }

    #[test]
    fn repository_higher_selected_pack_overrides_lower_pack() {
        let low = TempPack::new();
        let high = TempPack::new();
        low.write("assets/minecraft/lang/en_us.json", r#"{"menu.play":"Low"}"#);
        high.write(
            "assets/minecraft/lang/en_us.json",
            r#"{"menu.play":"High"}"#,
        );

        let stack = ClientResourceRepository::committed_vanilla()
            .with_available_pack(AvailableClientResourcePack::new(ClientResourcePack::new(
                "low",
                low.path(),
            )))
            .with_available_pack(AvailableClientResourcePack::new(ClientResourcePack::new(
                "high",
                high.path(),
            )))
            .with_selected_pack_ids(["low", "high"])
            .stack();

        let location = stack
            .find_resource("assets/minecraft/lang/en_us.json")
            .expect("selected language resource should resolve");
        let contents = location
            .read_to_string()
            .expect("selected language resource should read");

        assert_eq!(location.pack_id, "high");
        assert_eq!(contents, r#"{"menu.play":"High"}"#);
    }

    #[test]
    fn repository_known_pack_ids_include_vanilla() {
        let custom = TempPack::new();

        let repository = ClientResourceRepository::committed_vanilla()
            .with_available_pack(
                AvailableClientResourcePack::new(ClientResourcePack::new("custom", custom.path()))
                    .with_known_pack_id(KnownPackId::new("example", "custom", "1")),
            )
            .with_selected_pack_ids(["custom"]);

        assert_eq!(
            repository.known_pack_ids(),
            [
                KnownPackId::vanilla(),
                KnownPackId::new("example", "custom", "1")
            ]
        );
    }

    #[test]
    fn repository_recognizes_only_offered_known_packs_in_server_order() {
        let custom = TempPack::new();

        let repository = ClientResourceRepository::committed_vanilla()
            .with_available_pack(
                AvailableClientResourcePack::new(ClientResourcePack::new("custom", custom.path()))
                    .with_known_pack_id(KnownPackId::new("example", "custom", "1")),
            )
            .with_selected_pack_ids(["custom"]);

        let offered = [
            KnownPackId::new("unknown", "missing", "1"),
            KnownPackId::new("example", "custom", "1"),
            KnownPackId::vanilla(),
        ];

        assert_eq!(
            repository.recognized_known_pack_ids(&offered),
            [
                KnownPackId::new("example", "custom", "1"),
                KnownPackId::vanilla(),
            ]
        );
    }

    #[test]
    fn reload_plan_uses_simple_reload_weights() {
        let plan = ResourceReloadPlan::new(["a", "b"]);

        assert_eq!(plan.listeners(), ["a".to_owned(), "b".to_owned()]);
        assert_eq!(plan.initial_task_weight(), 2);
        assert_eq!(plan.total_weight(), 12);
        assert_eq!(ResourceReloadStep::InitialPreparation.weight(), 2);
        assert_eq!(ResourceReloadStep::Preparation.weight(), 2);
        assert_eq!(ResourceReloadStep::Reload.weight(), 2);
        assert_eq!(ResourceReloadStep::ListenerComplete.weight(), 1);
    }

    #[test]
    fn reload_progress_snapshot_starts_with_ordered_prepare_apply_entries() {
        let plan = ResourceReloadPlan::new(["lang", "splashes"]);
        let snapshot = plan.progress_snapshot(0);

        assert_eq!(snapshot.completed_weight(), 0);
        assert_eq!(snapshot.total_weight(), 12);
        assert_eq!(snapshot.listener_count(), 2);
        assert_eq!(snapshot.started_prepare_tasks(), 0);
        assert_eq!(snapshot.actual_progress(), 0.0);
        assert_eq!(
            snapshot
                .entries()
                .iter()
                .map(|entry| (
                    entry.listener(),
                    entry.step(),
                    entry.progress(),
                    entry.weight()
                ))
                .collect::<Vec<_>>(),
            [
                (
                    INITIAL_RELOAD_TASK_NAME,
                    ResourceReloadStep::InitialPreparation,
                    0.0,
                    2
                ),
                ("lang", ResourceReloadStep::Preparation, 0.0, 2),
                ("lang", ResourceReloadStep::Reload, 0.0, 2),
                ("lang", ResourceReloadStep::ListenerComplete, 0.0, 1),
                ("splashes", ResourceReloadStep::Preparation, 0.0, 2),
                ("splashes", ResourceReloadStep::Reload, 0.0, 2),
                ("splashes", ResourceReloadStep::ListenerComplete, 0.0, 1),
            ]
        );
    }

    #[test]
    fn reload_progress_snapshot_tracks_partial_weighted_progress() {
        let plan = ResourceReloadPlan::new(["lang"]);
        let mut state = ResourceReloadState::new(plan);
        state.finish_initial_task();
        state.finish_step("lang", ResourceReloadStep::Preparation);
        let snapshot = state.progress_snapshot();

        assert_eq!(snapshot.completed_weight(), 4);
        assert_eq!(snapshot.total_weight(), 7);
        assert_eq!(snapshot.listener_count(), 1);
        assert_eq!(snapshot.started_prepare_tasks(), 1);
        assert_eq!(snapshot.finished_prepare_tasks(), 1);
        assert_eq!(snapshot.started_reload_tasks(), 0);
        assert_eq!(snapshot.finished_reload_tasks(), 0);
        assert_eq!(snapshot.completed_listeners(), 0);
        assert!((snapshot.actual_progress() - (2.0 / 3.0)).abs() < f32::EPSILON);
        assert_eq!(
            snapshot
                .entries()
                .iter()
                .map(|entry| (entry.listener(), entry.step(), entry.progress()))
                .collect::<Vec<_>>(),
            [
                (
                    INITIAL_RELOAD_TASK_NAME,
                    ResourceReloadStep::InitialPreparation,
                    1.0
                ),
                ("lang", ResourceReloadStep::Preparation, 1.0),
                ("lang", ResourceReloadStep::Reload, 0.0),
                ("lang", ResourceReloadStep::ListenerComplete, 0.0),
            ]
        );
    }

    #[test]
    fn reload_progress_snapshot_finishes_at_one_for_completed_run() {
        let stack = ClientResourceStack::new(Vec::new());
        let report = ResourceReloadManager::new(stack)
            .run()
            .expect("initial-only reload should complete");
        let snapshot = report.state().progress_snapshot();

        assert_eq!(snapshot.completed_weight(), snapshot.total_weight());
        assert_eq!(snapshot.actual_progress(), 1.0);
        assert_eq!(
            snapshot
                .entries()
                .iter()
                .map(|entry| (entry.listener(), entry.step(), entry.progress()))
                .collect::<Vec<_>>(),
            [(
                INITIAL_RELOAD_TASK_NAME,
                ResourceReloadStep::InitialPreparation,
                1.0
            )]
        );
    }

    #[test]
    fn manager_reports_initial_task_before_listener_work() {
        let stack = ClientResourceStack::new(Vec::new());
        let manager = ResourceReloadManager::new(stack);

        let report = manager.run().expect("initial task should complete");

        assert_eq!(report.state().completed_weight(), 2);
        assert_eq!(report.state().progress(), 1.0);
        assert_eq!(
            report
                .events()
                .iter()
                .map(|event| (&event.listener, event.step, event.completed_weight))
                .collect::<Vec<_>>(),
            [(
                &INITIAL_RELOAD_TASK_NAME.to_owned(),
                ResourceReloadStep::InitialPreparation,
                2
            )]
        );
    }

    #[test]
    fn optional_server_pack_can_decline_but_required_pack_cannot() {
        let optional_id = resource_pack_id(1);
        let required_id = resource_pack_id(2);
        let mut optional =
            ServerResourcePackApplyState::new(server_pack_request(optional_id, false));
        let mut required =
            ServerResourcePackApplyState::new(server_pack_request(required_id, true));

        assert_eq!(
            optional.decline().expect("optional pack can be declined"),
            ServerResourcePackAck {
                id: optional_id,
                action: ServerResourcePackAckAction::Declined,
            }
        );
        assert_eq!(optional.status(), ServerResourcePackStatus::Declined);

        assert_eq!(
            required.decline(),
            Err(ServerResourcePackApplyError::RequiredPackCannotBeDeclined { id: required_id })
        );
        assert_eq!(required.status(), ServerResourcePackStatus::Pending);
    }

    #[test]
    fn server_pack_request_validation_distinguishes_url_states() {
        let id = resource_pack_id(20);
        let lowercase_hash = "0123456789abcdef0123456789abcdef01234567";
        let valid_http = ServerResourcePackRequest::new(
            id,
            "http://example.test/resource-pack.zip",
            lowercase_hash,
            false,
            None,
        );
        let valid_https = ServerResourcePackRequest::new(
            id,
            "https://example.test:8080/resource-pack.zip?sha1=known",
            lowercase_hash,
            false,
            None,
        );
        let invalid = ServerResourcePackRequest::new(id, "https://", lowercase_hash, false, None);
        let unsupported_scheme = ServerResourcePackRequest::new(
            id,
            "ftp://example.test/resource-pack.zip",
            lowercase_hash,
            false,
            None,
        );

        assert_eq!(valid_http.validate_url(), Ok(()));
        assert_eq!(valid_https.validate_url(), Ok(()));
        assert_eq!(
            invalid.validate_url(),
            Err(ServerResourcePackValidationError::InvalidUrl)
        );
        assert_eq!(
            unsupported_scheme.validate_url(),
            Err(ServerResourcePackValidationError::InvalidUrl)
        );
    }

    #[test]
    fn server_pack_request_validation_distinguishes_hash_states() {
        let id = resource_pack_id(21);
        let valid_lowercase = ServerResourcePackRequest::new(
            id,
            "https://example.test/resource-pack.zip",
            "0123456789abcdef0123456789abcdef01234567",
            false,
            None,
        );
        let valid_uppercase = ServerResourcePackRequest::new(
            id,
            "https://example.test/resource-pack.zip",
            "0123456789ABCDEF0123456789ABCDEF01234567",
            false,
            None,
        );
        let empty = ServerResourcePackRequest::new(
            id,
            "https://example.test/resource-pack.zip",
            "",
            false,
            None,
        );
        let invalid = ServerResourcePackRequest::new(
            id,
            "https://example.test/resource-pack.zip",
            "not-a-sha1",
            false,
            None,
        );

        assert_eq!(
            valid_lowercase.parsed_sha1_hash(),
            Some("0123456789abcdef0123456789abcdef01234567".to_owned())
        );
        assert_eq!(
            valid_uppercase.parsed_sha1_hash(),
            Some("0123456789abcdef0123456789abcdef01234567".to_owned())
        );
        assert_eq!(empty.parsed_sha1_hash(), None);
        assert_eq!(invalid.parsed_sha1_hash(), None);
    }

    #[test]
    fn server_pack_apply_state_exposes_request_validation_without_status_change() {
        let id = resource_pack_id(22);
        let mut pack = ServerResourcePackApplyState::new(ServerResourcePackRequest::new(
            id,
            "not a url",
            "",
            false,
            None,
        ));

        assert_eq!(
            pack.validation(),
            ServerResourcePackValidation {
                url: Err(ServerResourcePackValidationError::InvalidUrl),
                sha1_hash: None,
            }
        );
        assert_eq!(pack.status(), ServerResourcePackStatus::Pending);

        pack.invalid_url();

        assert_eq!(
            pack.validate_url(),
            Err(ServerResourcePackValidationError::InvalidUrl)
        );
        assert_eq!(
            pack.status(),
            ServerResourcePackStatus::Failed(ServerResourcePackFailure::InvalidUrl)
        );
    }

    #[test]
    fn accepted_server_pack_no_longer_reports_loaded_without_open_apply() {
        let id = resource_pack_id(3);
        let mut pack = ServerResourcePackApplyState::new(server_pack_request(id, true));

        let accepted = pack.accept();
        pack.start_download();
        let downloaded = pack.download_succeeded();
        let failed_reload = pack.apply_downloaded();

        assert_eq!(
            [accepted, downloaded, failed_reload],
            [
                ServerResourcePackAck {
                    id,
                    action: ServerResourcePackAckAction::Accepted,
                },
                ServerResourcePackAck {
                    id,
                    action: ServerResourcePackAckAction::Downloaded,
                },
                ServerResourcePackAck {
                    id,
                    action: ServerResourcePackAckAction::FailedReload,
                },
            ]
        );
        assert_eq!(
            pack.status(),
            ServerResourcePackStatus::Failed(ServerResourcePackFailure::Reload)
        );
    }

    #[test]
    fn server_pack_download_rejects_hash_mismatch() {
        let id = resource_pack_id(23);
        let mut pack = ServerResourcePackApplyState::new(ServerResourcePackRequest::new(
            id,
            "https://example.test/resource-pack.zip",
            "0000000000000000000000000000000000000000",
            true,
            None,
        ));

        pack.accept();
        pack.start_download();
        let failed_download = pack
            .download_bytes_succeeded(b"abc")
            .expect_err("wrong sha1 should fail the download");

        assert_eq!(
            failed_download,
            ServerResourcePackAck {
                id,
                action: ServerResourcePackAckAction::FailedDownload,
            }
        );
        assert_eq!(
            pack.status(),
            ServerResourcePackStatus::Failed(ServerResourcePackFailure::HashMismatch)
        );
        assert_eq!(pack.downloaded(), None);
    }

    #[test]
    fn server_pack_cache_write_marks_downloaded_with_path_metadata() {
        let id = resource_pack_id(26);
        let bytes = b"cached server pack";
        let expected_sha1 = server_resource_pack_sha1_hex(bytes);
        let cache = TempPack::new();
        let mut pack = ServerResourcePackApplyState::new(ServerResourcePackRequest::new(
            id,
            "https://example.test/resource-pack.zip",
            &expected_sha1,
            true,
            None,
        ));

        pack.accept();
        pack.start_download();
        let ack = pack
            .cache_downloaded_bytes(cache.path(), bytes)
            .expect("matching pack bytes should write to cache");

        let expected_path = cache.path().join(id.to_string()).join(&expected_sha1);
        assert_eq!(
            ack,
            ServerResourcePackAck {
                id,
                action: ServerResourcePackAckAction::Downloaded,
            }
        );
        assert_eq!(pack.status(), ServerResourcePackStatus::Downloaded);
        assert_eq!(
            fs::read(&expected_path).expect("cached pack bytes should be readable"),
            bytes
        );
        assert_eq!(
            pack.downloaded(),
            Some(&ServerResourcePackDownload::Path {
                path: expected_path,
                len: bytes.len(),
                sha1: expected_sha1,
            })
        );
    }

    #[test]
    fn server_pack_cache_rejects_oversized_download_len_without_allocating_bytes() {
        let id = resource_pack_id(2601);
        let mut pack = ServerResourcePackApplyState::new(server_pack_request(id, true));

        pack.accept();
        pack.start_download();
        pack.validate_download_size(SERVER_RESOURCE_PACK_MAX_DOWNLOAD_BYTES)
            .expect("vanilla max-sized resource pack should be allowed");
        let err = pack
            .validate_download_size(SERVER_RESOURCE_PACK_MAX_DOWNLOAD_BYTES + 1)
            .expect_err("oversized resource pack should fail before buffering bytes");

        assert_eq!(
            err.ack(),
            ServerResourcePackAck {
                id,
                action: ServerResourcePackAckAction::FailedDownload,
            }
        );
        assert!(
            matches!(err, ServerResourcePackCacheError::TooLarge { len, max, .. }
                if len == SERVER_RESOURCE_PACK_MAX_DOWNLOAD_BYTES + 1
                    && max == SERVER_RESOURCE_PACK_MAX_DOWNLOAD_BYTES)
        );
        assert_eq!(
            pack.status(),
            ServerResourcePackStatus::Failed(ServerResourcePackFailure::Download)
        );
        assert_eq!(pack.downloaded(), None);
    }

    #[test]
    fn server_pack_download_content_length_guard_allows_missing_or_max_len() {
        assert!(server_resource_pack_content_length_is_allowed(None));
        assert!(server_resource_pack_content_length_is_allowed(Some(
            SERVER_RESOURCE_PACK_MAX_DOWNLOAD_BYTES as u64
        )));
        assert!(!server_resource_pack_content_length_is_allowed(Some(
            SERVER_RESOURCE_PACK_MAX_DOWNLOAD_BYTES as u64 + 1
        )));
    }

    #[test]
    fn server_pack_download_chunk_len_guard_rejects_overflow_or_oversize() {
        assert_eq!(
            server_resource_pack_download_len_after_chunk(
                SERVER_RESOURCE_PACK_MAX_DOWNLOAD_BYTES - 1,
                1
            ),
            Some(SERVER_RESOURCE_PACK_MAX_DOWNLOAD_BYTES)
        );
        assert_eq!(
            server_resource_pack_download_len_after_chunk(
                SERVER_RESOURCE_PACK_MAX_DOWNLOAD_BYTES,
                1
            ),
            None
        );
        assert_eq!(
            server_resource_pack_download_len_after_chunk(usize::MAX, 1),
            None
        );
    }

    #[test]
    fn server_pack_cache_reuses_matching_cached_file() {
        let id = resource_pack_id(2602);
        let bytes = b"cached server pack";
        let expected_sha1 = server_resource_pack_sha1_hex(bytes);
        let cache = TempPack::new();
        let mut first_pack = ServerResourcePackApplyState::new(ServerResourcePackRequest::new(
            id,
            "https://example.test/resource-pack.zip",
            &expected_sha1,
            true,
            None,
        ));
        let mut second_pack = ServerResourcePackApplyState::new(ServerResourcePackRequest::new(
            id,
            "https://example.test/resource-pack.zip",
            &expected_sha1,
            true,
            None,
        ));

        first_pack.accept();
        first_pack.start_download();
        first_pack
            .cache_downloaded_bytes(cache.path(), bytes)
            .expect("first matching pack bytes should write to cache");

        second_pack.accept();
        second_pack.start_download();
        let report = second_pack
            .cache_downloaded_bytes_with_report(cache.path(), bytes)
            .expect("matching cached pack should be reused");

        assert!(report.reused_existing_file);
        assert_eq!(
            report.ack,
            ServerResourcePackAck {
                id,
                action: ServerResourcePackAckAction::Downloaded,
            }
        );
        assert_eq!(
            second_pack.downloaded(),
            Some(&ServerResourcePackDownload::Path {
                path: report.path.clone(),
                len: bytes.len(),
                sha1: expected_sha1,
            })
        );
    }

    #[test]
    fn server_pack_cache_overwrites_stale_file_at_expected_path() {
        let id = resource_pack_id(2603);
        let bytes = b"cached server pack";
        let stale_bytes = b"stale server pack";
        let expected_sha1 = server_resource_pack_sha1_hex(bytes);
        let cache = TempPack::new();
        let expected_path = cache.path().join(id.to_string()).join(&expected_sha1);
        fs::create_dir_all(
            expected_path
                .parent()
                .expect("cache path should have a parent"),
        )
        .expect("cache parent should be created");
        fs::write(&expected_path, stale_bytes).expect("stale cache file should be written");
        let mut pack = ServerResourcePackApplyState::new(ServerResourcePackRequest::new(
            id,
            "https://example.test/resource-pack.zip",
            &expected_sha1,
            true,
            None,
        ));

        pack.accept();
        pack.start_download();
        let report = pack
            .cache_downloaded_bytes_with_report(cache.path(), bytes)
            .expect("stale cache file should be replaced by matching bytes");

        assert!(!report.reused_existing_file);
        assert_eq!(
            fs::read(&expected_path).expect("cache file should be readable"),
            bytes
        );
        assert_eq!(
            pack.downloaded(),
            Some(&ServerResourcePackDownload::Path {
                path: expected_path,
                len: bytes.len(),
                sha1: expected_sha1,
            })
        );
    }

    #[test]
    fn server_pack_cache_write_rejects_hash_mismatch_without_file() {
        let id = resource_pack_id(27);
        let bytes = b"cached server pack";
        let cache = TempPack::new();
        let mut pack = ServerResourcePackApplyState::new(ServerResourcePackRequest::new(
            id,
            "https://example.test/resource-pack.zip",
            "0000000000000000000000000000000000000000",
            true,
            None,
        ));

        pack.accept();
        pack.start_download();
        let err = pack
            .cache_downloaded_bytes(cache.path(), bytes)
            .expect_err("wrong sha1 should reject cached bytes");

        assert_eq!(
            err.ack(),
            ServerResourcePackAck {
                id,
                action: ServerResourcePackAckAction::FailedDownload,
            }
        );
        assert_eq!(
            pack.status(),
            ServerResourcePackStatus::Failed(ServerResourcePackFailure::HashMismatch)
        );
        assert_eq!(pack.downloaded(), None);
        assert!(!cache.path().join(id.to_string()).exists());
    }

    #[test]
    fn server_pack_cache_write_reports_invalid_cache_dir() {
        let id = resource_pack_id(28);
        let bytes = b"cached server pack";
        let cache = TempPack::new();
        cache.write("not-a-directory", "");
        let mut pack = ServerResourcePackApplyState::new(server_pack_request(id, true));

        pack.accept();
        pack.start_download();
        let err = pack
            .cache_downloaded_bytes(cache.path().join("not-a-directory"), bytes)
            .expect_err("file cache root should fail before writing");

        assert_eq!(
            err.ack(),
            ServerResourcePackAck {
                id,
                action: ServerResourcePackAckAction::FailedDownload,
            }
        );
        assert!(matches!(err, ServerResourcePackCacheError::Io { .. }));
        assert_eq!(
            pack.status(),
            ServerResourcePackStatus::Failed(ServerResourcePackFailure::Download)
        );
        assert_eq!(pack.downloaded(), None);
    }

    #[test]
    fn server_pack_cache_path_is_deterministic() {
        let id = resource_pack_id(29);
        let cache = TempPack::new();
        let request_hash = "0123456789ABCDEF0123456789ABCDEF01234567";
        let actual_hash = "fedcba9876543210fedcba9876543210fedcba98";
        let pack = ServerResourcePackApplyState::new(ServerResourcePackRequest::new(
            id,
            "https://example.test/resource-pack.zip",
            request_hash,
            true,
            None,
        ));

        let first = pack.deterministic_cache_path(cache.path(), actual_hash);
        let second = pack.deterministic_cache_path(cache.path(), actual_hash);

        assert_eq!(first, second);
        assert_eq!(first, cache.path().join(id.to_string()).join(actual_hash));
    }

    #[test]
    fn server_pack_cache_without_declared_hash_uses_computed_sha1_path() {
        let id = resource_pack_id(30);
        let bytes = b"cached server pack without declared hash";
        let expected_sha1 = server_resource_pack_sha1_hex(bytes);
        let cache = TempPack::new();
        let mut pack = ServerResourcePackApplyState::new(ServerResourcePackRequest::new(
            id,
            "https://example.test/resource-pack.zip",
            "",
            true,
            None,
        ));

        pack.accept();
        pack.start_download();
        pack.cache_downloaded_bytes(cache.path(), bytes)
            .expect("pack bytes without declared hash should write to cache");

        let expected_path = cache.path().join(id.to_string()).join(&expected_sha1);
        assert_eq!(
            pack.downloaded().and_then(ServerResourcePackDownload::path),
            Some(expected_path.as_path())
        );
    }

    #[test]
    fn server_pack_directory_path_requires_mcmeta_open_then_apply_for_success() {
        let id = resource_pack_id(24);
        let temp = TempPack::new();
        temp.write(
            "pack.mcmeta",
            r#"{"pack":{"min_format":84,"max_format":84,"description":"test"}}"#,
        );
        let mut pack = ServerResourcePackApplyState::new(server_pack_request(id, true));

        let accepted = pack.accept();
        pack.start_download();
        let downloaded = pack
            .download_path_succeeded(temp.path())
            .expect("directory pack without enforced sha1 should download");
        pack.open_downloaded()
            .expect("directory pack with pack.mcmeta should open");
        let loaded = pack.apply_opened();

        assert_eq!(
            [accepted, downloaded, loaded],
            [
                ServerResourcePackAck {
                    id,
                    action: ServerResourcePackAckAction::Accepted,
                },
                ServerResourcePackAck {
                    id,
                    action: ServerResourcePackAckAction::Downloaded,
                },
                ServerResourcePackAck {
                    id,
                    action: ServerResourcePackAckAction::SuccessfullyLoaded,
                },
            ]
        );
        assert_eq!(pack.status(), ServerResourcePackStatus::Applied);
        assert_eq!(
            pack.downloaded().and_then(ServerResourcePackDownload::path),
            Some(temp.path())
        );
    }

    #[test]
    fn server_pack_directory_open_requires_pack_mcmeta() {
        let id = resource_pack_id(25);
        let temp = TempPack::new();
        let mut pack = ServerResourcePackApplyState::new(server_pack_request(id, true));

        pack.accept();
        pack.start_download();
        pack.download_path_succeeded(temp.path())
            .expect("test request has no enforced sha1");
        let failed_reload = pack
            .open_downloaded()
            .expect_err("directory pack without pack.mcmeta should fail to open");

        assert_eq!(
            failed_reload,
            ServerResourcePackAck {
                id,
                action: ServerResourcePackAckAction::FailedReload,
            }
        );
        assert_eq!(
            pack.status(),
            ServerResourcePackStatus::Failed(ServerResourcePackFailure::Open)
        );
    }

    #[test]
    fn server_pack_directory_open_requires_valid_pack_mcmeta_json() {
        let id = resource_pack_id(2501);
        let temp = TempPack::new();
        temp.write("pack.mcmeta", r#"{"pack":"not an object"}"#);
        let mut pack = ServerResourcePackApplyState::new(server_pack_request(id, true));

        pack.accept();
        pack.start_download();
        pack.download_path_succeeded(temp.path())
            .expect("test request has no enforced sha1");
        let failed_reload = pack
            .open_downloaded()
            .expect_err("directory pack with invalid pack metadata should fail to open");

        assert_eq!(
            failed_reload,
            ServerResourcePackAck {
                id,
                action: ServerResourcePackAckAction::FailedReload,
            }
        );
        assert_eq!(
            pack.status(),
            ServerResourcePackStatus::Failed(ServerResourcePackFailure::Open)
        );
    }

    #[test]
    fn server_pack_directory_open_requires_pack_section() {
        let id = resource_pack_id(2502);
        let temp = TempPack::new();
        temp.write("pack.mcmeta", r#"{"metadata":{}}"#);
        let mut pack = ServerResourcePackApplyState::new(server_pack_request(id, true));

        pack.accept();
        pack.start_download();
        pack.download_path_succeeded(temp.path())
            .expect("test request has no enforced sha1");
        let failed_reload = pack
            .open_downloaded()
            .expect_err("directory pack without pack section should fail to open");

        assert_eq!(
            failed_reload,
            ServerResourcePackAck {
                id,
                action: ServerResourcePackAckAction::FailedReload,
            }
        );
        assert_eq!(
            pack.status(),
            ServerResourcePackStatus::Failed(ServerResourcePackFailure::Open)
        );
    }

    #[test]
    fn server_pack_directory_open_requires_description() {
        let id = resource_pack_id(2503);
        let temp = TempPack::new();
        temp.write(
            "pack.mcmeta",
            r#"{"pack":{"min_format":84,"max_format":84}}"#,
        );
        let mut pack = ServerResourcePackApplyState::new(server_pack_request(id, true));

        pack.accept();
        pack.start_download();
        pack.download_path_succeeded(temp.path())
            .expect("test request has no enforced sha1");
        let failed_reload = pack
            .open_downloaded()
            .expect_err("directory pack without description should fail to open");

        assert_eq!(
            failed_reload,
            ServerResourcePackAck {
                id,
                action: ServerResourcePackAckAction::FailedReload,
            }
        );
        assert_eq!(
            pack.status(),
            ServerResourcePackStatus::Failed(ServerResourcePackFailure::Open)
        );
    }

    #[test]
    fn server_pack_directory_open_requires_supported_client_format() {
        let id = resource_pack_id(2504);
        let temp = TempPack::new();
        temp.write(
            "pack.mcmeta",
            r#"{"pack":{"min_format":1,"max_format":1,"description":"test"}}"#,
        );
        let mut pack = ServerResourcePackApplyState::new(server_pack_request(id, true));

        pack.accept();
        pack.start_download();
        pack.download_path_succeeded(temp.path())
            .expect("test request has no enforced sha1");
        let failed_reload = pack
            .open_downloaded()
            .expect_err("directory pack with unsupported format should fail to open");

        assert_eq!(
            failed_reload,
            ServerResourcePackAck {
                id,
                action: ServerResourcePackAckAction::FailedReload,
            }
        );
        assert_eq!(
            pack.status(),
            ServerResourcePackStatus::Failed(ServerResourcePackFailure::Open)
        );
    }

    #[test]
    fn server_pack_cached_zip_open_requires_valid_pack_mcmeta() {
        let id = resource_pack_id(2505);
        let zip_bytes = server_pack_zip_bytes(
            r#"{"pack":{"min_format":84,"max_format":84,"description":"test"}}"#,
        );
        let expected_sha1 = server_resource_pack_sha1_hex(&zip_bytes);
        let cache = TempPack::new();
        let mut pack = ServerResourcePackApplyState::new(ServerResourcePackRequest::new(
            id,
            "https://example.test/resource-pack.zip",
            &expected_sha1,
            true,
            None,
        ));

        pack.accept();
        pack.start_download();
        pack.cache_downloaded_bytes(cache.path(), &zip_bytes)
            .expect("valid zip resource pack should cache");

        pack.open_downloaded()
            .expect("cached zip resource pack with pack.mcmeta should open");
        assert_eq!(pack.status(), ServerResourcePackStatus::Opened);
    }

    #[test]
    fn applied_server_directory_pack_overrides_vanilla_and_selected_packs() {
        let id = resource_pack_id(2601);
        let high = TempPack::new();
        let server = TempPack::new();
        high.write(
            "assets/minecraft/lang/en_us.json",
            r#"{"menu.play":"High"}"#,
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
            ClientResourcePack::new("high", high.path()),
        ]));

        let pack = model.receive(ServerResourcePackRequest::new(
            id,
            "https://example.test/resource-pack",
            "",
            true,
            None,
        ));
        pack.accept();
        pack.start_download();
        pack.download_path_succeeded(server.path())
            .expect("server directory pack should download");
        pack.open_downloaded()
            .expect("server directory pack should open");
        pack.apply_opened();

        let stack = model.resource_stack();
        let location = stack
            .find_resource("assets/minecraft/lang/en_us.json")
            .expect("server resource should resolve");

        assert_eq!(location.pack_id, format!("server:{id}"));
        assert_eq!(
            location
                .read_to_string()
                .expect("server language resource should read"),
            r#"{"menu.play":"Server"}"#
        );
    }

    #[test]
    fn applied_server_zip_pack_can_override_json_resource() {
        let id = resource_pack_id(2602);
        let cache = TempPack::new();
        let zip_path = cache.path().join("server.zip");
        let zip_bytes = server_pack_zip_bytes_with_entries(
            r#"{"pack":{"min_format":84,"max_format":84,"description":"server"}}"#,
            [("assets/minecraft/lang/en_us.json", r#"{"menu.play":"Zip"}"#)],
        );
        fs::write(&zip_path, zip_bytes).expect("server zip pack should be written");
        let mut model = ServerResourcePackApplyModel::with_vanilla();

        let pack = model.receive(ServerResourcePackRequest::new(
            id,
            "https://example.test/resource-pack.zip",
            "",
            true,
            None,
        ));
        pack.accept();
        pack.start_download();
        pack.download_path_succeeded(&zip_path)
            .expect("server zip pack should download");
        pack.open_downloaded().expect("server zip pack should open");
        pack.apply_opened();

        let resource =
            load_client_json_resource(&model.resource_stack(), "assets/minecraft/lang/en_us.json")
                .expect("json resource should load from server zip");

        assert_eq!(
            resource.report().loaded_resource_pack(),
            format!("assets/minecraft/lang/en_us.json@server:{id}")
        );
        assert_eq!(resource.value()["menu.play"], "Zip");
    }

    #[test]
    fn applied_server_pack_missing_resource_falls_back_to_vanilla() {
        let id = resource_pack_id(2603);
        let server = TempPack::new();
        server.write(
            "pack.mcmeta",
            r#"{"pack":{"min_format":84,"max_format":84,"description":"server"}}"#,
        );
        let mut model = ServerResourcePackApplyModel::with_vanilla();

        let pack = model.receive(ServerResourcePackRequest::new(
            id,
            "https://example.test/resource-pack",
            "",
            true,
            None,
        ));
        pack.accept();
        pack.start_download();
        pack.download_path_succeeded(server.path())
            .expect("server directory pack should download");
        pack.open_downloaded()
            .expect("server directory pack should open");
        pack.apply_opened();

        let location = model
            .resource_stack()
            .find_resource(SPLASHES_RESOURCE)
            .expect("vanilla fallback resource should resolve");

        assert_eq!(location.pack_id, VANILLA_PACK_ID);
    }

    #[test]
    fn opened_only_server_pack_is_not_in_resource_stack() {
        let id = resource_pack_id(2604);
        let server = TempPack::new();
        server.write(
            "pack.mcmeta",
            r#"{"pack":{"min_format":84,"max_format":84,"description":"server"}}"#,
        );
        server.write(
            "assets/minecraft/lang/en_us.json",
            r#"{"menu.play":"Server"}"#,
        );
        let mut model = ServerResourcePackApplyModel::with_vanilla();

        let pack = model.receive(ServerResourcePackRequest::new(
            id,
            "https://example.test/resource-pack",
            "",
            true,
            None,
        ));
        pack.accept();
        pack.start_download();
        pack.download_path_succeeded(server.path())
            .expect("server directory pack should download");
        pack.open_downloaded()
            .expect("server directory pack should open");

        let location = model
            .resource_stack()
            .find_resource("assets/minecraft/lang/en_us.json")
            .expect("vanilla language resource should still resolve");

        assert_eq!(location.pack_id, VANILLA_PACK_ID);
    }

    #[test]
    fn server_pack_directory_open_rejects_pack_format_without_modern_range() {
        let id = resource_pack_id(2506);
        let temp = TempPack::new();
        temp.write(
            "pack.mcmeta",
            r#"{"pack":{"pack_format":84,"description":"test"}}"#,
        );
        let mut pack = ServerResourcePackApplyState::new(server_pack_request(id, true));

        pack.accept();
        pack.start_download();
        pack.download_path_succeeded(temp.path())
            .expect("test request has no enforced sha1");
        let failed_reload = pack
            .open_downloaded()
            .expect_err("pack_format alone should not open for 26.1.2 client resources");

        assert_eq!(
            failed_reload,
            ServerResourcePackAck {
                id,
                action: ServerResourcePackAckAction::FailedReload,
            }
        );
        assert_eq!(
            pack.status(),
            ServerResourcePackStatus::Failed(ServerResourcePackFailure::Open)
        );
    }

    #[test]
    fn failed_server_pack_reports_failure_ack_sequence() {
        let download_id = resource_pack_id(4);
        let reload_id = resource_pack_id(5);
        let mut download_failure =
            ServerResourcePackApplyState::new(server_pack_request(download_id, true));
        let mut reload_failure =
            ServerResourcePackApplyState::new(server_pack_request(reload_id, true));

        let download_acks = [
            download_failure.accept(),
            download_failure.download_failed(),
        ];
        let reload_acks = {
            let accepted = reload_failure.accept();
            reload_failure.start_download();
            let downloaded = reload_failure
                .download_bytes_succeeded(b"abc")
                .expect("test pack has no enforced hash");
            [accepted, downloaded, reload_failure.reload_failed()]
        };

        assert_eq!(
            download_acks,
            [
                ServerResourcePackAck {
                    id: download_id,
                    action: ServerResourcePackAckAction::Accepted,
                },
                ServerResourcePackAck {
                    id: download_id,
                    action: ServerResourcePackAckAction::FailedDownload,
                },
            ]
        );
        assert_eq!(
            reload_acks,
            [
                ServerResourcePackAck {
                    id: reload_id,
                    action: ServerResourcePackAckAction::Accepted,
                },
                ServerResourcePackAck {
                    id: reload_id,
                    action: ServerResourcePackAckAction::Downloaded,
                },
                ServerResourcePackAck {
                    id: reload_id,
                    action: ServerResourcePackAckAction::FailedReload,
                },
            ]
        );
    }

    #[test]
    fn applied_server_packs_stay_above_vanilla_in_priority_order() {
        let first_id = resource_pack_id(6);
        let second_id = resource_pack_id(7);
        let mut model = ServerResourcePackApplyModel::with_vanilla();

        model
            .receive(server_pack_request(first_id, true))
            .apply_test_server_pack();
        model
            .receive(server_pack_request(resource_pack_id(8), false))
            .decline()
            .expect("optional middle pack can be declined");
        model
            .receive(server_pack_request(second_id, true))
            .apply_test_server_pack();

        let stack = model.resource_stack();
        let pack_ids = stack
            .packs()
            .iter()
            .map(|pack| pack.id().to_owned())
            .collect::<Vec<_>>();

        assert_eq!(
            pack_ids,
            [
                VANILLA_PACK_ID.to_owned(),
                format!("server:{first_id}"),
                format!("server:{second_id}"),
            ]
        );
    }

    #[test]
    fn server_pack_pop_removes_matching_applied_pack_from_stack() {
        let removed_id = resource_pack_id(9);
        let kept_id = resource_pack_id(10);
        let mut model = ServerResourcePackApplyModel::with_vanilla();

        model
            .receive(server_pack_request(removed_id, true))
            .apply_test_server_pack();
        model
            .receive(server_pack_request(kept_id, true))
            .apply_test_server_pack();

        assert!(model.pop(removed_id));
        assert!(!model.pop(resource_pack_id(11)));

        let pack_ids = model
            .resource_stack()
            .packs()
            .iter()
            .map(|pack| pack.id().to_owned())
            .collect::<Vec<_>>();

        assert_eq!(
            pack_ids,
            [VANILLA_PACK_ID.to_owned(), format!("server:{kept_id}")]
        );
    }

    #[test]
    fn server_pack_pop_all_keeps_vanilla_only() {
        let mut model = ServerResourcePackApplyModel::with_vanilla();

        model
            .receive(server_pack_request(resource_pack_id(12), true))
            .apply_test_server_pack();
        model
            .receive(server_pack_request(resource_pack_id(13), true))
            .apply_test_server_pack();

        assert!(model.pop_all());
        assert!(!model.pop_all());

        assert_eq!(
            model
                .resource_stack()
                .packs()
                .iter()
                .map(ClientResourcePack::id)
                .collect::<Vec<_>>(),
            [VANILLA_PACK_ID]
        );
    }

    #[test]
    fn manager_runs_listing_listeners_deterministically() {
        let temp = TempPack::new();
        temp.write("assets/minecraft/lang/en_us.json", "{}");
        temp.write("assets/minecraft/texts/splashes.txt", "hello");

        let stack = ClientResourceStack::new(vec![ClientResourcePack::new("test", temp.path())]);
        let manager = ResourceReloadManager::new(stack)
            .with_listener(ListingResourceReloadListener::new(
                "lang",
                ["assets/minecraft/lang/en_us.json"],
            ))
            .with_listener(ListingResourceReloadListener::new(
                "splashes",
                ["assets/minecraft/texts/splashes.txt"],
            ));

        let report = manager.run().expect("mock reload should succeed");

        assert_eq!(report.state().completed_weight(), 12);
        assert_eq!(report.state().progress(), 1.0);
        assert_eq!(
            report
                .events()
                .iter()
                .map(|event| (&event.listener, event.step, event.completed_weight))
                .collect::<Vec<_>>(),
            [
                (
                    &INITIAL_RELOAD_TASK_NAME.to_owned(),
                    ResourceReloadStep::InitialPreparation,
                    2
                ),
                (&"lang".to_owned(), ResourceReloadStep::Preparation, 4),
                (&"lang".to_owned(), ResourceReloadStep::Reload, 6),
                (&"lang".to_owned(), ResourceReloadStep::ListenerComplete, 7),
                (&"splashes".to_owned(), ResourceReloadStep::Preparation, 9),
                (&"splashes".to_owned(), ResourceReloadStep::Reload, 11),
                (
                    &"splashes".to_owned(),
                    ResourceReloadStep::ListenerComplete,
                    12
                ),
            ]
        );
        assert_eq!(report.listener_reports().len(), 2);
        assert_eq!(
            report.listener_reports()[0].reload.items(),
            ["assets/minecraft/lang/en_us.json@test".to_owned()]
        );
    }

    #[test]
    fn required_vanilla_asset_listener_loads_lightweight_assets() {
        let manager = ResourceReloadManager::with_default_vanilla_assets();

        let report = manager
            .run()
            .expect("committed vanilla resources should load");

        assert_eq!(report.state().progress(), 1.0);
        assert_eq!(report.listener_reports().len(), 1);

        let listener = &report.listener_reports()[0];
        assert_eq!(listener.name, "vanilla_required_assets");
        assert_eq!(listener.preparation.items().len(), 4);
        assert_eq!(listener.reload.items().len(), 4);
        for resource in DEFAULT_REQUIRED_VANILLA_ASSETS {
            assert!(
                listener
                    .reload
                    .items()
                    .iter()
                    .any(|item| item.starts_with(resource)),
                "reload report should include {resource}"
            );
        }
    }

    #[test]
    fn default_client_resources_plan_uses_deterministic_startup_order() {
        let manager = ResourceReloadManager::with_default_vanilla_client_resources();

        assert_eq!(
            manager.plan().listeners(),
            [
                "client_languages",
                "texture_metadata",
                "headless_shader_sources",
                "splashes",
                "atlas_sources",
                "font_definitions",
                "colormaps",
                "model_dependencies",
                "equipment_assets",
                "particle_manifests",
                "waypoint_style_manifests",
                "cloud_texture",
                "gpu_warnlist",
                "regional_compliancies",
            ]
        );
    }

    #[test]
    fn default_client_resources_plan_excludes_smoke_and_duplicate_listeners() {
        let manager = ResourceReloadManager::with_default_vanilla_client_resources();

        for excluded in [
            "vanilla_required_assets",
            "listing",
            "model_entry_smoke",
            "blockstate_model_references",
            "atlas_manifests",
        ] {
            assert!(
                !manager
                    .plan()
                    .listeners()
                    .iter()
                    .any(|name| name == excluded),
                "default CLIENT_RESOURCES reload should not include {excluded}"
            );
        }
    }

    #[test]
    fn committed_vanilla_default_client_resources_run_succeeds() {
        let manager = ResourceReloadManager::with_default_vanilla_client_resources();

        let report = manager
            .run()
            .expect("committed vanilla default client resources should load");

        assert_eq!(report.state().progress(), 1.0);
        assert_eq!(
            report.listener_reports().len(),
            manager.plan().listeners().len()
        );
        assert_eq!(
            report
                .listener_reports()
                .iter()
                .map(|listener| listener.name.as_str())
                .collect::<Vec<_>>(),
            manager
                .plan()
                .listeners()
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn default_client_resources_progress_events_reach_complete() {
        let manager = ResourceReloadManager::with_default_vanilla_client_resources();
        let mut progress_events = Vec::new();

        let report = manager
            .run_with_events(|event| {
                progress_events.push(event.progress_snapshot.actual_progress());
            })
            .expect("committed vanilla default client resources should load");

        assert_eq!(progress_events.len(), report.events().len());
        assert!(
            progress_events
                .first()
                .is_some_and(|progress| *progress < 1.0)
        );
        assert_eq!(progress_events.last().copied(), Some(1.0));
        assert_eq!(report.state().progress_snapshot().actual_progress(), 1.0);
    }

    #[test]
    fn splashes_listener_counts_vanilla_trimmed_lines_from_highest_priority_pack() {
        let base = TempPack::new();
        let override_pack = TempPack::new();
        base.write(SPLASHES_RESOURCE, "base one\nbase two\nbase three\n");
        override_pack.write(SPLASHES_RESOURCE, "\ncustom one\n \ncustom two\n");

        let stack = ClientResourceStack::new(vec![
            ClientResourcePack::new("base", base.path()),
            ClientResourcePack::new("override", override_pack.path()),
        ]);
        let manager =
            ResourceReloadManager::new(stack).with_listener(SplashesReloadListener::default());

        let report = manager.run().expect("splashes reload should succeed");

        let listener = &report.listener_reports()[0];
        assert_eq!(listener.name, "splashes");
        assert_eq!(listener.preparation.items(), [SPLASHES_RESOURCE.to_owned()]);
        assert_eq!(
            listener.reload.items(),
            [format!("{SPLASHES_RESOURCE}@override:4 splashes")]
        );
    }

    #[test]
    fn splashes_resource_loads_highest_priority_vanilla_trimmed_lines() {
        let base = TempPack::new();
        let override_pack = TempPack::new();
        base.write(SPLASHES_RESOURCE, "base one\nbase two\nbase three\n");
        override_pack.write(
            SPLASHES_RESOURCE,
            "\n custom one \n\t\nThis message will never appear on the splash screen, isn't that weird?\ncustom two\r\n",
        );

        let stack = ClientResourceStack::new(vec![
            ClientResourcePack::new("base", base.path()),
            ClientResourcePack::new("override", override_pack.path()),
        ]);

        let splashes = load_client_splashes_resource(&stack, SPLASHES_RESOURCE)
            .expect("splashes resource should load");

        assert_eq!(splashes.lines(), ["", "custom one", "", "custom two"]);
        assert_eq!(splashes.count(), 4);
        assert_eq!(splashes.report().resource(), SPLASHES_RESOURCE);
        assert_eq!(splashes.report().pack_id(), "override");
        assert_eq!(
            splashes.report().loaded_resource_pack(),
            format!("{SPLASHES_RESOURCE}@override")
        );
        assert_eq!(splashes.report().splash_count(), 4);
    }

    #[test]
    fn atlas_manifest_listener_reports_loaded_manifests_by_resolved_pack() {
        let base = TempPack::new();
        let override_pack = TempPack::new();
        base.write("assets/minecraft/atlases/blocks.json", r#"{"sources":[]}"#);
        base.write("assets/minecraft/atlases/items.json", r#"{"sources":[]}"#);
        base.write(
            "assets/minecraft/atlases/particles.json",
            r#"{"sources":[]}"#,
        );
        override_pack.write("assets/minecraft/atlases/items.json", r#"{"sources":[{}]}"#);

        let stack = ClientResourceStack::new(vec![
            ClientResourcePack::new("base", base.path()),
            ClientResourcePack::new("override", override_pack.path()),
        ]);
        let manager =
            ResourceReloadManager::new(stack).with_listener(AtlasManifestReloadListener::default());

        let report = manager.run().expect("atlas manifest reload should succeed");

        let listener = &report.listener_reports()[0];
        assert_eq!(listener.name, "atlas_manifests");
        assert_eq!(listener.preparation.items(), DEFAULT_ATLAS_MANIFESTS);
        assert_eq!(
            listener.reload.items(),
            [
                "assets/minecraft/atlases/blocks.json@base".to_owned(),
                "assets/minecraft/atlases/items.json@override".to_owned(),
                "assets/minecraft/atlases/particles.json@base".to_owned(),
            ]
        );
    }

    #[test]
    fn committed_vanilla_splashes_listener_reports_a_non_empty_count() {
        let stack = ClientResourceStack::vanilla();
        let splashes = load_client_splashes_resource(&stack, SPLASHES_RESOURCE)
            .expect("committed vanilla splashes resource should load");
        assert!(splashes.count() > 0, "vanilla splashes should not be empty");
        assert!(
            splashes
                .lines()
                .iter()
                .all(|line| { java_string_hash_code(line) != VANILLA_EXCLUDED_SPLASH_JAVA_HASH })
        );
        assert_eq!(splashes.report().resource(), SPLASHES_RESOURCE);
        assert_eq!(splashes.report().pack_id(), VANILLA_PACK_ID);
        assert_eq!(splashes.report().splash_count(), splashes.count());

        let manager =
            ResourceReloadManager::new(stack).with_listener(SplashesReloadListener::default());

        let report = manager
            .run()
            .expect("committed vanilla splashes should load");

        let item = &report.listener_reports()[0].reload.items()[0];
        let count = item
            .strip_prefix(&format!("{SPLASHES_RESOURCE}@{VANILLA_PACK_ID}:"))
            .and_then(|value| value.strip_suffix(" splashes"))
            .expect("report should include vanilla splash count")
            .parse::<usize>()
            .expect("splash count should be numeric");
        assert_eq!(count, splashes.count());
    }

    #[test]
    fn committed_vanilla_atlas_manifest_listener_loads_default_manifest_set() {
        let manager = ResourceReloadManager::new(ClientResourceStack::vanilla())
            .with_listener(AtlasManifestReloadListener::default());

        let report = manager
            .run()
            .expect("committed vanilla atlas manifests should load");

        let listener = &report.listener_reports()[0];
        assert_eq!(listener.name, "atlas_manifests");
        assert_eq!(listener.preparation.items(), DEFAULT_ATLAS_MANIFESTS);
        assert_eq!(
            listener.reload.items(),
            [
                "assets/minecraft/atlases/blocks.json@vanilla".to_owned(),
                "assets/minecraft/atlases/items.json@vanilla".to_owned(),
                "assets/minecraft/atlases/particles.json@vanilla".to_owned(),
            ]
        );
    }

    #[test]
    fn atlas_source_listener_reports_source_counts_from_stack_order() {
        let base = TempPack::new();
        let override_pack = TempPack::new();
        base.write(
            "assets/minecraft/atlases/blocks.json",
            r#"{"sources":[{"type":"minecraft:directory"}]}"#,
        );
        base.write(
            "assets/minecraft/atlases/items.json",
            r#"{"sources":[{"type":"minecraft:directory"}]}"#,
        );
        base.write(
            "assets/minecraft/atlases/particles.json",
            r#"{"sources":[{"type":"minecraft:directory"}]}"#,
        );
        override_pack.write(
            "assets/minecraft/atlases/items.json",
            r#"{"sources":[{"type":"minecraft:directory"},{"type":"minecraft:single"}]}"#,
        );

        let stack = ClientResourceStack::new(vec![
            ClientResourcePack::new("base", base.path()),
            ClientResourcePack::new("override", override_pack.path()),
        ]);
        let manager =
            ResourceReloadManager::new(stack).with_listener(AtlasSourceReloadListener::default());

        let report = manager.run().expect("atlas source reload should succeed");

        let listener = &report.listener_reports()[0];
        assert_eq!(listener.name, "atlas_sources");
        assert_eq!(listener.preparation.items(), DEFAULT_ATLAS_MANIFESTS);
        assert_eq!(
            listener.reload.items(),
            [
                "assets/minecraft/atlases/blocks.json@base:1 sources".to_owned(),
                "assets/minecraft/atlases/items.json@base:1 sources".to_owned(),
                "assets/minecraft/atlases/items.json@override:2 sources".to_owned(),
                "assets/minecraft/atlases/particles.json@base:1 sources".to_owned(),
            ]
        );
    }

    #[test]
    fn committed_vanilla_atlas_source_listener_reports_default_source_counts() {
        let manager = ResourceReloadManager::new(ClientResourceStack::vanilla())
            .with_listener(AtlasSourceReloadListener::default());

        let report = manager
            .run()
            .expect("committed vanilla atlas sources should load");

        let listener = &report.listener_reports()[0];
        assert_eq!(listener.name, "atlas_sources");
        assert_eq!(
            listener.reload.items(),
            [
                "assets/minecraft/atlases/blocks.json@vanilla:4 sources".to_owned(),
                "assets/minecraft/atlases/items.json@vanilla:2 sources".to_owned(),
                "assets/minecraft/atlases/particles.json@vanilla:1 sources".to_owned(),
            ]
        );
    }

    #[test]
    fn atlas_source_listener_rejects_invalid_sources_shape() {
        let temp = TempPack::new();
        temp.write(
            "assets/minecraft/atlases/blocks.json",
            r#"{"sources":[{"type":"minecraft:directory"}]}"#,
        );
        temp.write(
            "assets/minecraft/atlases/items.json",
            r#"{"sources":["not an object"]}"#,
        );
        temp.write(
            "assets/minecraft/atlases/particles.json",
            r#"{"sources":[{"type":"minecraft:directory"}]}"#,
        );

        let stack = ClientResourceStack::new(vec![ClientResourcePack::new("test", temp.path())]);
        let manager =
            ResourceReloadManager::new(stack).with_listener(AtlasSourceReloadListener::default());

        assert!(matches!(
            manager.run(),
            Err(ResourceReloadError::InvalidAtlasSources {
                resource,
                pack_id,
                reason,
            }) if resource == "assets/minecraft/atlases/items.json"
                && pack_id == "test"
                && reason == "source 0 is not an object"
        ));
    }

    #[test]
    fn atlas_source_listener_rejects_missing_source_type_field() {
        let temp = TempPack::new();
        temp.write(
            "assets/minecraft/atlases/blocks.json",
            r#"{"sources":[{"type":"minecraft:directory"}]}"#,
        );
        temp.write("assets/minecraft/atlases/items.json", r#"{"sources":[{}]}"#);
        temp.write(
            "assets/minecraft/atlases/particles.json",
            r#"{"sources":[{"type":"minecraft:directory"}]}"#,
        );

        let stack = ClientResourceStack::new(vec![ClientResourcePack::new("test", temp.path())]);
        let manager =
            ResourceReloadManager::new(stack).with_listener(AtlasSourceReloadListener::default());

        assert!(matches!(
            manager.run(),
            Err(ResourceReloadError::InvalidAtlasSources {
                resource,
                pack_id,
                reason,
            }) if resource == "assets/minecraft/atlases/items.json"
                && pack_id == "test"
                && reason == "source 0 is missing a type field"
        ));
    }

    #[test]
    fn font_definition_listener_reports_priority_pack_and_provider_types() {
        let base = TempPack::new();
        let override_pack = TempPack::new();
        base.write(
            "assets/minecraft/font/default.json",
            r#"{"providers":[{"type":"reference","id":"minecraft:include/default"}]}"#,
        );
        base.write(
            "assets/minecraft/font/uniform.json",
            r#"{"providers":[{"type":"reference","id":"minecraft:include/unifont"}]}"#,
        );
        base.write(
            "assets/minecraft/font/alt.json",
            r#"{"providers":[{"type":"reference","id":"minecraft:include/alt"}]}"#,
        );
        override_pack.write(
            "assets/minecraft/font/default.json",
            r#"{"providers":[{"type":"reference","id":"minecraft:include/space"},{"type":"bitmap","file":"minecraft:font/ascii.png","ascent":7,"chars":["abc"]}]}"#,
        );

        let stack = ClientResourceStack::new(vec![
            ClientResourcePack::new("base", base.path()),
            ClientResourcePack::new("override", override_pack.path()),
        ]);
        let definitions = FontDefinitionsReloadListener::default()
            .load(&stack)
            .expect("font definitions should load");
        let default_definition = definitions
            .definitions()
            .iter()
            .find(|definition| {
                definition.resource().report().loaded_resource_pack()
                    == "assets/minecraft/font/default.json@override"
            })
            .expect("override default font definition should load");
        assert_eq!(
            default_definition
                .resource()
                .report()
                .loaded_resource_pack(),
            "assets/minecraft/font/default.json@override"
        );
        assert_eq!(default_definition.provider_count(), 2);
        assert_eq!(
            default_definition.provider_types().collect::<Vec<_>>(),
            ["reference", "bitmap"]
        );

        let report = ResourceReloadManager::new(stack)
            .with_listener(FontDefinitionsReloadListener::default())
            .run()
            .expect("font definitions reload should succeed");

        let listener = &report.listener_reports()[0];
        assert_eq!(listener.name, "font_definitions");
        assert_eq!(
            listener.preparation.items(),
            [
                "assets/minecraft/font/alt.json".to_owned(),
                "assets/minecraft/font/default.json".to_owned(),
                "assets/minecraft/font/uniform.json".to_owned(),
            ]
        );
        assert_eq!(
            listener.reload.items(),
            [
                "assets/minecraft/font/alt.json@base:providers:1:types:reference".to_owned(),
                "assets/minecraft/font/default.json@base:providers:1:types:reference".to_owned(),
                "assets/minecraft/font/default.json@override:providers:2:types:reference,bitmap"
                    .to_owned(),
                "assets/minecraft/font/uniform.json@base:providers:1:types:reference".to_owned(),
            ]
        );
    }

    #[test]
    fn committed_vanilla_font_definition_listener_loads_representative_definitions() {
        let report = ResourceReloadManager::new(ClientResourceStack::vanilla())
            .with_listener(FontDefinitionsReloadListener::default())
            .run()
            .expect("committed vanilla font definitions should load");

        let listener = &report.listener_reports()[0];
        assert_eq!(listener.name, "font_definitions");
        assert_eq!(
            listener.preparation.items(),
            [
                "assets/minecraft/font/alt.json".to_owned(),
                "assets/minecraft/font/default.json".to_owned(),
                "assets/minecraft/font/illageralt.json".to_owned(),
                "assets/minecraft/font/include/default.json".to_owned(),
                "assets/minecraft/font/include/space.json".to_owned(),
                "assets/minecraft/font/include/unifont.json".to_owned(),
                "assets/minecraft/font/uniform.json".to_owned(),
            ]
        );
        assert_eq!(
            listener.reload.items(),
            [
                "assets/minecraft/font/alt.json@vanilla:providers:2:types:reference,bitmap"
                    .to_owned(),
                "assets/minecraft/font/default.json@vanilla:providers:3:types:reference,reference,reference"
                    .to_owned(),
                "assets/minecraft/font/illageralt.json@vanilla:providers:2:types:reference,bitmap"
                    .to_owned(),
                "assets/minecraft/font/include/default.json@vanilla:providers:3:types:bitmap,bitmap,bitmap"
                    .to_owned(),
                "assets/minecraft/font/include/space.json@vanilla:providers:1:types:space"
                    .to_owned(),
                "assets/minecraft/font/include/unifont.json@vanilla:providers:0:types:"
                    .to_owned(),
                "assets/minecraft/font/uniform.json@vanilla:providers:2:types:reference,reference"
                    .to_owned(),
            ]
        );
    }

    #[test]
    fn font_definition_reload_rejects_invalid_provider_type() {
        let temp = TempPack::new();
        temp.write(
            "assets/minecraft/font/default.json",
            r#"{"providers":[{"type":"bogus"}]}"#,
        );

        let stack = ClientResourceStack::new(vec![ClientResourcePack::new("test", temp.path())]);
        let error = FontDefinitionsReloadListener::new(["assets/minecraft/font/default.json"])
            .load(&stack)
            .expect_err("unsupported font provider type should fail");

        assert!(
            matches!(error, ResourceReloadError::InvalidFontDefinition { resource, reason, .. } if resource == "assets/minecraft/font/default.json" && reason == "provider 0 has unsupported type `bogus`")
        );
    }

    #[test]
    fn font_definition_reload_rejects_invalid_json() {
        let temp = TempPack::new();
        temp.write("assets/minecraft/font/default.json", "{not json");

        let stack = ClientResourceStack::new(vec![ClientResourcePack::new("test", temp.path())]);
        let error = FontDefinitionsReloadListener::new(["assets/minecraft/font/default.json"])
            .load(&stack)
            .expect_err("invalid font definition json should fail");

        assert!(
            matches!(error, ResourceReloadError::ParseResourceJson { resource, .. } if resource == "assets/minecraft/font/default.json")
        );
    }

    #[test]
    fn font_definition_reload_rejects_missing_providers_array() {
        let temp = TempPack::new();
        temp.write("assets/minecraft/font/default.json", r#"{"providers":{}}"#);

        let stack = ClientResourceStack::new(vec![ClientResourcePack::new("test", temp.path())]);
        let error = FontDefinitionsReloadListener::new(["assets/minecraft/font/default.json"])
            .load(&stack)
            .expect_err("font definition without providers array should fail");

        assert!(
            matches!(error, ResourceReloadError::InvalidFontDefinition { resource, reason, .. } if resource == "assets/minecraft/font/default.json" && reason == "missing providers array")
        );
    }

    #[test]
    fn committed_vanilla_headless_shader_source_listener_loads_representative_sources() {
        let report = ResourceReloadManager::new(ClientResourceStack::vanilla())
            .with_listener(HeadlessShaderSourceReloadListener::default())
            .run()
            .expect("committed vanilla shader sources should load");

        let listener = &report.listener_reports()[0];
        assert_eq!(listener.name, "headless_shader_sources");
        assert_eq!(
            listener.preparation.items().len(),
            DEFAULT_REPRESENTATIVE_SHADER_SOURCES.len() + DEFAULT_SHADER_INCLUDE_SOURCES.len()
        );
        assert_eq!(
            listener.preparation.items(),
            DEFAULT_REPRESENTATIVE_SHADER_SOURCES
                .iter()
                .chain(DEFAULT_SHADER_INCLUDE_SOURCES.iter())
                .map(|resource| (*resource).to_owned())
                .collect::<Vec<_>>()
        );

        for resource in DEFAULT_REPRESENTATIVE_SHADER_SOURCES
            .iter()
            .chain(DEFAULT_SHADER_INCLUDE_SOURCES.iter())
        {
            let item = listener
                .reload
                .items()
                .iter()
                .find(|item| item.starts_with(&format!("{resource}@{VANILLA_PACK_ID}:")))
                .unwrap_or_else(|| panic!("reload report should include {resource}"));
            let byte_count = item
                .strip_prefix(&format!("{resource}@{VANILLA_PACK_ID}:"))
                .and_then(|suffix| suffix.strip_suffix(" bytes"))
                .and_then(|bytes| bytes.parse::<usize>().ok())
                .expect("shader reload report should include a byte count");
            assert!(byte_count > 0, "shader report should count bytes");
        }
    }

    #[test]
    fn shader_source_reload_uses_highest_priority_pack_and_reports_bytes() {
        let base = TempPack::new();
        let override_pack = TempPack::new();
        let source = "assets/minecraft/shaders/core/position_tex.vsh";
        let include = "assets/minecraft/shaders/include/dynamictransforms.glsl";
        base.write(source, "#version 150\nvoid main() {}\n");
        base.write(include, "#define BASE 1\n");
        override_pack.write(source, "#version 150\n// override\nvoid main() {}\n");

        let stack = ClientResourceStack::new(vec![
            ClientResourcePack::new("base", base.path()),
            ClientResourcePack::new("override", override_pack.path()),
        ]);
        let report = ResourceReloadManager::new(stack)
            .with_listener(HeadlessShaderSourceReloadListener::new([source], [include]))
            .run()
            .expect("shader source reload should succeed");

        let listener = &report.listener_reports()[0];
        assert_eq!(
            listener.reload.items(),
            [
                format!(
                    "{source}@override:{} bytes",
                    "#version 150\n// override\nvoid main() {}\n".len()
                ),
                format!("{include}@base:{} bytes", "#define BASE 1\n".len()),
            ]
        );
    }

    #[test]
    fn shader_source_reload_fails_when_representative_source_is_missing() {
        let temp = TempPack::new();
        temp.write(
            "assets/minecraft/shaders/include/dynamictransforms.glsl",
            "#define PRESENT 1\n",
        );

        let stack = ClientResourceStack::new(vec![ClientResourcePack::new("test", temp.path())]);
        let error = ResourceReloadManager::new(stack)
            .with_listener(HeadlessShaderSourceReloadListener::new(
                ["assets/minecraft/shaders/core/position_tex.vsh"],
                ["assets/minecraft/shaders/include/dynamictransforms.glsl"],
            ))
            .run()
            .expect_err("missing representative shader source should fail");

        assert!(
            matches!(error, ResourceReloadError::MissingResource(resource) if resource == "assets/minecraft/shaders/core/position_tex.vsh")
        );
    }

    #[test]
    fn shader_source_reload_fails_when_include_source_is_missing() {
        let temp = TempPack::new();
        temp.write(
            "assets/minecraft/shaders/core/position_tex.vsh",
            "#version 150\nvoid main() {}\n",
        );

        let stack = ClientResourceStack::new(vec![ClientResourcePack::new("test", temp.path())]);
        let error = ResourceReloadManager::new(stack)
            .with_listener(HeadlessShaderSourceReloadListener::new(
                ["assets/minecraft/shaders/core/position_tex.vsh"],
                ["assets/minecraft/shaders/include/dynamictransforms.glsl"],
            ))
            .run()
            .expect_err("missing shader include source should fail");

        assert!(
            matches!(error, ResourceReloadError::MissingResource(resource) if resource == "assets/minecraft/shaders/include/dynamictransforms.glsl")
        );
    }

    #[test]
    fn client_json_reload_applies_pack_priority_overrides() {
        let base = TempPack::new();
        let override_pack = TempPack::new();
        base.write(
            GPU_WARNLIST_RESOURCE,
            r#"{"renderer":["base"],"version":[],"vendor":[]}"#,
        );
        base.write(REGIONAL_COMPLIANCIES_RESOURCE, r#"{"BASE":[]}"#);
        override_pack.write(
            GPU_WARNLIST_RESOURCE,
            r#"{"renderer":[],"vendor":["override"],"version":[]}"#,
        );
        override_pack.write(REGIONAL_COMPLIANCIES_RESOURCE, r#"{"OVERRIDE":[]}"#);

        let stack = ClientResourceStack::new(vec![
            ClientResourcePack::new("base", base.path()),
            ClientResourcePack::new("override", override_pack.path()),
        ]);

        let gpu_warnlist = GpuWarnlistReloadListener::default()
            .load(&stack)
            .expect("gpu warnlist should load from highest priority pack");
        let compliancies = RegionalComplianciesReloadListener::default()
            .load(&stack)
            .expect("regional compliancies should load from highest priority pack");

        assert_eq!(gpu_warnlist.report().pack_id(), "override");
        assert_eq!(
            gpu_warnlist.report().top_level_shape(),
            &ClientJsonTopLevelShape::Object {
                keys: vec![
                    "renderer".to_owned(),
                    "vendor".to_owned(),
                    "version".to_owned()
                ],
            }
        );
        assert_eq!(
            gpu_warnlist.value()["vendor"][0],
            serde_json::Value::String("override".to_owned())
        );
        assert_eq!(compliancies.report().pack_id(), "override");
        assert_eq!(
            compliancies.report().top_level_shape(),
            &ClientJsonTopLevelShape::Object {
                keys: vec!["OVERRIDE".to_owned()],
            }
        );
    }

    #[test]
    fn client_json_listener_reports_loaded_pack_and_top_level_shape() {
        let temp = TempPack::new();
        temp.write(GPU_WARNLIST_RESOURCE, r#"["warn","list"]"#);

        let stack = ClientResourceStack::new(vec![ClientResourcePack::new("test", temp.path())]);
        let report = ResourceReloadManager::new(stack)
            .with_listener(GpuWarnlistReloadListener::default())
            .run()
            .expect("json listener should run");

        assert_eq!(report.listener_reports()[0].name, "gpu_warnlist");
        assert_eq!(
            report.listener_reports()[0].preparation.items(),
            [GPU_WARNLIST_RESOURCE.to_owned()]
        );
        assert_eq!(
            report.listener_reports()[0].reload.items(),
            [format!("{GPU_WARNLIST_RESOURCE}@test:array len:2")]
        );
    }

    #[test]
    fn sound_events_listener_reports_highest_priority_pack_and_event_count() {
        let base = TempPack::new();
        let override_pack = TempPack::new();
        base.write(
            SOUND_EVENTS_RESOURCE,
            r#"{"block.stone.break":{"sounds":["dig.stone"]}}"#,
        );
        override_pack.write(
            SOUND_EVENTS_RESOURCE,
            r#"{"block.stone.break":{"sounds":["override.stone"]},"entity.player.hurt":{"sounds":[{"name":"damage.hit","type":"sound"}]}}"#,
        );

        let stack = ClientResourceStack::new(vec![
            ClientResourcePack::new("base", base.path()),
            ClientResourcePack::new("override", override_pack.path()),
        ]);
        let report = ResourceReloadManager::new(stack)
            .with_listener(SoundEventsReloadListener::default())
            .run()
            .expect("sound events reload should succeed");

        let listener = &report.listener_reports()[0];
        assert_eq!(listener.name, "sound_events");
        assert_eq!(
            listener.preparation.items(),
            [SOUND_EVENTS_RESOURCE.to_owned()]
        );
        assert_eq!(
            listener.reload.items(),
            [format!("{SOUND_EVENTS_RESOURCE}@override:2 events")]
        );
    }

    #[test]
    fn sound_events_reload_rejects_invalid_event_shape() {
        let temp = TempPack::new();
        temp.write(SOUND_EVENTS_RESOURCE, r#"{"block.stone.break":[]}"#);

        let stack = ClientResourceStack::new(vec![ClientResourcePack::new("test", temp.path())]);
        let error = SoundEventsReloadListener::default()
            .load(&stack)
            .expect_err("sound events with non-object event values should fail");

        assert!(
            matches!(error, ResourceReloadError::InvalidSoundEvents { resource, reason, .. } if resource == SOUND_EVENTS_RESOURCE && reason == "sound event `block.stone.break` must be an object")
        );
    }

    #[test]
    fn sound_events_reload_rejects_missing_sounds_array() {
        let temp = TempPack::new();
        temp.write(
            SOUND_EVENTS_RESOURCE,
            r#"{"block.stone.break":{"subtitle":"subtitles.block.generic.break"}}"#,
        );

        let stack = ClientResourceStack::new(vec![ClientResourcePack::new("test", temp.path())]);
        let error = SoundEventsReloadListener::default()
            .load(&stack)
            .expect_err("sound events without sounds arrays should fail");

        assert!(
            matches!(error, ResourceReloadError::InvalidSoundEvents { resource, reason, .. } if resource == SOUND_EVENTS_RESOURCE && reason == "sound event `block.stone.break` must define a sounds array")
        );
    }

    #[test]
    fn client_json_reload_rejects_invalid_json() {
        let temp = TempPack::new();
        temp.write(GPU_WARNLIST_RESOURCE, "{not json");

        let stack = ClientResourceStack::new(vec![ClientResourcePack::new("test", temp.path())]);
        let error = GpuWarnlistReloadListener::default()
            .load(&stack)
            .expect_err("invalid gpu warnlist json should fail");

        assert!(
            matches!(error, ResourceReloadError::ParseResourceJson { resource, .. } if resource == GPU_WARNLIST_RESOURCE)
        );
    }

    #[test]
    fn particle_manifest_listener_reports_highest_priority_override_textures() {
        let base = TempPack::new();
        let override_pack = TempPack::new();
        base.write(
            "assets/minecraft/particles/rain.json",
            r#"{"textures":["minecraft:base_rain_0","minecraft:base_rain_1"]}"#,
        );
        base.write(
            "assets/minecraft/particles/splash.json",
            r#"{"textures":["minecraft:base_splash"]}"#,
        );
        override_pack.write(
            "assets/minecraft/particles/rain.json",
            r#"{"textures":["minecraft:override_rain_0","minecraft:override_rain_1"],"override":true}"#,
        );

        let stack = ClientResourceStack::new(vec![
            ClientResourcePack::new("base", base.path()),
            ClientResourcePack::new("override", override_pack.path()),
        ]);
        let report = ResourceReloadManager::new(stack)
            .with_listener(ParticleManifestReloadListener::default())
            .run()
            .expect("particle manifest reload should succeed");

        let listener = &report.listener_reports()[0];
        assert_eq!(listener.name, "particle_manifests");
        assert_eq!(
            listener.preparation.items(),
            [
                "assets/minecraft/particles/rain.json".to_owned(),
                "assets/minecraft/particles/splash.json".to_owned(),
            ]
        );
        assert_eq!(
            listener.reload.items(),
            [
                "assets/minecraft/particles/rain.json@override:2 sprites:minecraft:override_rain_0,minecraft:override_rain_1"
                    .to_owned(),
                "assets/minecraft/particles/splash.json@base:1 sprites:minecraft:base_splash"
                    .to_owned(),
            ]
        );
    }

    #[test]
    fn waypoint_style_manifest_listener_reports_priority_override_sprites_and_distances() {
        let base = TempPack::new();
        let override_pack = TempPack::new();
        base.write(
            "assets/minecraft/waypoint_style/default.json",
            r#"{"far_distance":192,"sprites":["minecraft:default_0"]}"#,
        );
        base.write(
            "assets/minecraft/waypoint_style/bowtie.json",
            r#"{"sprites":["minecraft:bowtie"]}"#,
        );
        override_pack.write(
            "assets/minecraft/waypoint_style/bowtie.json",
            r#"{"far_distance":192,"near_distance":64,"sprites":["minecraft:override_bowtie","custom:pin"]}"#,
        );

        let stack = ClientResourceStack::new(vec![
            ClientResourcePack::new("base", base.path()),
            ClientResourcePack::new("override", override_pack.path()),
        ]);
        let report = ResourceReloadManager::new(stack)
            .with_listener(WaypointStyleManifestReloadListener::default())
            .run()
            .expect("waypoint style manifest reload should succeed");

        let listener = &report.listener_reports()[0];
        assert_eq!(listener.name, "waypoint_style_manifests");
        assert_eq!(
            listener.preparation.items(),
            [
                "assets/minecraft/waypoint_style/default.json".to_owned(),
                "assets/minecraft/waypoint_style/bowtie.json".to_owned(),
            ]
        );
        assert_eq!(
            listener.reload.items(),
            [
                "assets/minecraft/waypoint_style/default.json@base:near:128 far:192 sprites:1:minecraft:default_0 locations:minecraft:hud/locator_bar_dot/default_0"
                    .to_owned(),
                "assets/minecraft/waypoint_style/bowtie.json@override:near:64 far:192 sprites:2:minecraft:override_bowtie,custom:pin locations:minecraft:hud/locator_bar_dot/override_bowtie,custom:hud/locator_bar_dot/pin"
                    .to_owned(),
            ]
        );
    }

    #[test]
    fn waypoint_style_manifest_reload_rejects_invalid_sprite_shape() {
        let temp = TempPack::new();
        temp.write(
            "assets/minecraft/waypoint_style/default.json",
            r#"{"sprites":"minecraft:default_0"}"#,
        );

        let stack = ClientResourceStack::new(vec![ClientResourcePack::new("test", temp.path())]);
        let error = WaypointStyleManifestReloadListener::new(["default"])
            .load(&stack)
            .expect_err("invalid waypoint style sprite shape should fail");

        assert!(matches!(
            error,
            ResourceReloadError::InvalidWaypointStyleManifest {
                resource,
                pack_id,
                reason
            } if resource == "assets/minecraft/waypoint_style/default.json"
                && pack_id == "test"
                && reason == "sprites must be an array of resource ids"
        ));
    }

    #[test]
    fn waypoint_style_manifest_reload_rejects_invalid_sprite_identifier() {
        let temp = TempPack::new();
        temp.write(
            "assets/minecraft/waypoint_style/default.json",
            r#"{"sprites":["Minecraft:Uppercase"]}"#,
        );

        let stack = ClientResourceStack::new(vec![ClientResourcePack::new("test", temp.path())]);
        let error = WaypointStyleManifestReloadListener::new(["default"])
            .load(&stack)
            .expect_err("invalid waypoint style sprite identifier should fail");

        assert!(matches!(
            error,
            ResourceReloadError::InvalidWaypointStyleManifest {
                resource,
                pack_id,
                reason
            } if resource == "assets/minecraft/waypoint_style/default.json"
                && pack_id == "test"
                && reason == "sprites[0] is not a valid resource id"
        ));
    }

    #[test]
    fn waypoint_style_manifest_reload_rejects_empty_sprites() {
        let temp = TempPack::new();
        temp.write(
            "assets/minecraft/waypoint_style/default.json",
            r#"{"sprites":[]}"#,
        );

        let stack = ClientResourceStack::new(vec![ClientResourcePack::new("test", temp.path())]);
        let error = WaypointStyleManifestReloadListener::new(["default"])
            .load(&stack)
            .expect_err("empty waypoint style sprites should fail");

        assert!(matches!(
            error,
            ResourceReloadError::InvalidWaypointStyleManifest {
                resource,
                pack_id,
                reason
            } if resource == "assets/minecraft/waypoint_style/default.json"
                && pack_id == "test"
                && reason == "sprites must not be empty"
        ));
    }

    #[test]
    fn waypoint_style_manifest_reload_rejects_invalid_distances() {
        let temp = TempPack::new();
        temp.write(
            "assets/minecraft/waypoint_style/default.json",
            r#"{"near_distance":64,"far_distance":64,"sprites":["minecraft:default_0"]}"#,
        );

        let stack = ClientResourceStack::new(vec![ClientResourcePack::new("test", temp.path())]);
        let error = WaypointStyleManifestReloadListener::new(["default"])
            .load(&stack)
            .expect_err("invalid waypoint style distances should fail");

        assert!(matches!(
            error,
            ResourceReloadError::InvalidWaypointStyleManifest {
                resource,
                pack_id,
                reason
            } if resource == "assets/minecraft/waypoint_style/default.json"
                && pack_id == "test"
                && reason == "far_distance must be greater than near_distance"
        ));
    }

    #[test]
    fn equipment_assets_listener_reports_priority_pack_and_layers() {
        let base = TempPack::new();
        let override_pack = TempPack::new();
        base.write(
            "assets/minecraft/equipment/diamond.json",
            r#"{"layers":{"humanoid":[{"texture":"minecraft:diamond"}]}}"#,
        );
        base.write(
            "assets/minecraft/equipment/elytra.json",
            r#"{"layers":{"wings":[{"texture":"minecraft:elytra","use_player_texture":true}]}}"#,
        );
        override_pack.write(
            "assets/minecraft/equipment/diamond.json",
            r#"{"layers":{"humanoid":[{"texture":"minecraft:custom_diamond","dyeable":{}}]},"override":true}"#,
        );

        let stack = ClientResourceStack::new(vec![
            ClientResourcePack::new("base", base.path()),
            ClientResourcePack::new("override", override_pack.path()),
        ]);
        let report = ResourceReloadManager::new(stack)
            .with_listener(EquipmentAssetsReloadListener::default())
            .run()
            .expect("equipment asset reload should succeed");

        let listener = &report.listener_reports()[0];
        assert_eq!(listener.name, "equipment_assets");
        assert_eq!(
            listener.preparation.items(),
            [
                "assets/minecraft/equipment/diamond.json".to_owned(),
                "assets/minecraft/equipment/elytra.json".to_owned(),
            ]
        );
        assert_eq!(
            listener.reload.items(),
            [
                "assets/minecraft/equipment/diamond.json@override:layers:1 entries:1 layer_types:humanoid textures:minecraft:custom_diamond texture_locations:minecraft:textures/entity/equipment/humanoid/custom_diamond.png dyeable:1 player_textures:0".to_owned(),
                "assets/minecraft/equipment/elytra.json@base:layers:1 entries:1 layer_types:wings textures:minecraft:elytra texture_locations:minecraft:textures/entity/equipment/wings/elytra.png dyeable:0 player_textures:1".to_owned(),
            ]
        );
    }

    #[test]
    fn model_entry_smoke_listener_reports_priority_pack_and_shape() {
        let base = TempPack::new();
        let override_pack = TempPack::new();
        base.write(
            "assets/minecraft/models/block/stone.json",
            r#"{"parent":"block/cube_all","textures":{}}"#,
        );
        base.write(
            "assets/minecraft/models/item/stick.json",
            r#"{"parent":"item/generated","textures":{}}"#,
        );
        base.write(
            "assets/minecraft/blockstates/stone.json",
            r#"{"variants":{}}"#,
        );
        base.write(
            "assets/minecraft/items/stone.json",
            r#"{"model":{"type":"minecraft:model"}}"#,
        );
        override_pack.write(
            "assets/minecraft/models/item/stick.json",
            r#"{"parent":"item/generated","textures":{},"override":true}"#,
        );

        let stack = ClientResourceStack::new(vec![
            ClientResourcePack::new("base", base.path()),
            ClientResourcePack::new("override", override_pack.path()),
        ]);
        let report = ResourceReloadManager::new(stack)
            .with_listener(ModelEntrySmokeReloadListener::default())
            .run()
            .expect("model entry smoke reload should succeed");

        let listener = &report.listener_reports()[0];
        assert_eq!(listener.name, "model_entry_smoke");
        assert_eq!(listener.preparation.items(), DEFAULT_MODEL_SMOKE_RESOURCES);
        assert_eq!(
            listener.reload.items(),
            [
                "assets/minecraft/models/block/stone.json@base:object keys:parent,textures"
                    .to_owned(),
                "assets/minecraft/models/item/stick.json@override:object keys:override,parent,textures"
                    .to_owned(),
                "assets/minecraft/blockstates/stone.json@base:object keys:variants".to_owned(),
                "assets/minecraft/items/stone.json@base:object keys:model".to_owned(),
            ]
        );
    }

    #[test]
    fn blockstate_model_reference_extraction_reads_variant_object_and_string_forms() {
        let blockstate = serde_json::json!({
            "variants": {
                "": { "model": "minecraft:block/stone" },
                "powered=true": "minecraft:block/redstone_torch"
            }
        });

        assert_eq!(
            extract_blockstate_model_references(&blockstate)
                .expect("variant model references should extract"),
            std::collections::BTreeSet::from([
                "minecraft:block/redstone_torch".to_owned(),
                "minecraft:block/stone".to_owned(),
            ])
        );
    }

    #[test]
    fn blockstate_model_reference_extraction_reads_variant_array_form() {
        let blockstate = serde_json::json!({
            "variants": {
                "axis=y": [
                    { "model": "minecraft:block/oak_log" },
                    { "model": "minecraft:block/oak_log_horizontal" }
                ]
            }
        });

        assert_eq!(
            extract_blockstate_model_references(&blockstate)
                .expect("variant array model references should extract"),
            std::collections::BTreeSet::from([
                "minecraft:block/oak_log".to_owned(),
                "minecraft:block/oak_log_horizontal".to_owned(),
            ])
        );
    }

    #[test]
    fn blockstate_model_reference_extraction_reads_multipart_apply_object_and_array_forms() {
        let blockstate = serde_json::json!({
            "multipart": [
                {
                    "when": { "north": "true" },
                    "apply": { "model": "minecraft:block/oak_fence_side" }
                },
                {
                    "apply": [
                        { "model": "minecraft:block/oak_fence_post" },
                        { "model": "minecraft:block/oak_fence_inventory" }
                    ]
                }
            ]
        });

        assert_eq!(
            extract_blockstate_model_references(&blockstate)
                .expect("multipart apply model references should extract"),
            std::collections::BTreeSet::from([
                "minecraft:block/oak_fence_inventory".to_owned(),
                "minecraft:block/oak_fence_post".to_owned(),
                "minecraft:block/oak_fence_side".to_owned(),
            ])
        );
    }

    #[test]
    fn blockstate_model_reference_extraction_rejects_invalid_json_shape() {
        let blockstate = serde_json::json!({
            "variants": [
                { "model": "minecraft:block/stone" }
            ]
        });

        assert_eq!(
            extract_blockstate_model_references(&blockstate)
                .expect_err("invalid blockstate shape should fail"),
            "blockstate variants must be an object"
        );
    }

    #[test]
    fn item_root_model_dependency_extraction_reads_direct_model_and_special_base() {
        let direct = serde_json::json!({
            "model": {
                "type": "minecraft:model",
                "model": "minecraft:item/stick"
            }
        });
        let special = serde_json::json!({
            "model": {
                "type": "minecraft:special",
                "base": "minecraft:item/shield",
                "model": {
                    "type": "minecraft:shield"
                }
            }
        });

        assert_eq!(
            extract_item_root_model_dependencies(&direct)
                .expect("direct item model dependency should extract"),
            std::collections::BTreeSet::from(["minecraft:item/stick".to_owned()])
        );
        assert_eq!(
            extract_item_root_model_dependencies(&special)
                .expect("special base model dependency should extract"),
            std::collections::BTreeSet::from(["minecraft:item/shield".to_owned()])
        );
    }

    #[test]
    fn item_root_model_dependency_extraction_recurses_condition_select_range_and_composite() {
        let item_root = serde_json::json!({
            "model": {
                "type": "minecraft:condition",
                "on_true": {
                    "type": "minecraft:select",
                    "cases": [
                        {
                            "when": "minecraft:bow",
                            "model": {
                                "type": "minecraft:model",
                                "model": "minecraft:item/bow"
                            }
                        }
                    ],
                    "fallback": {
                        "type": "minecraft:range_dispatch",
                        "entries": [
                            {
                                "threshold": 0.5,
                                "model": {
                                    "type": "minecraft:model",
                                    "model": "minecraft:item/bow_pulling_0"
                                }
                            }
                        ],
                        "fallback": {
                            "type": "minecraft:model",
                            "model": "minecraft:item/bow_standby"
                        }
                    }
                },
                "on_false": {
                    "type": "minecraft:composite",
                    "models": [
                        {
                            "type": "minecraft:model",
                            "model": "minecraft:item/bow_overlay"
                        },
                        {
                            "type": "minecraft:empty"
                        },
                        {
                            "type": "minecraft:bundle/selected_item"
                        }
                    ]
                }
            }
        });

        assert_eq!(
            extract_item_root_model_dependencies(&item_root)
                .expect("nested item root model dependencies should extract"),
            std::collections::BTreeSet::from([
                "minecraft:item/bow".to_owned(),
                "minecraft:item/bow_overlay".to_owned(),
                "minecraft:item/bow_pulling_0".to_owned(),
                "minecraft:item/bow_standby".to_owned(),
            ])
        );
    }

    #[test]
    fn item_root_model_dependency_extraction_rejects_invalid_json_shape() {
        let item_root = serde_json::json!({
            "model": {
                "type": "minecraft:model",
                "model": 42
            }
        });

        assert_eq!(
            extract_item_root_model_dependencies(&item_root)
                .expect_err("invalid item root model dependency shape should fail"),
            "minecraft:model item model model must be a string"
        );
    }

    #[test]
    fn blockstate_model_reference_listener_reports_priority_pack_and_models() {
        let base = TempPack::new();
        let override_pack = TempPack::new();
        base.write(
            "assets/minecraft/blockstates/stone.json",
            r#"{"variants":{"":{"model":"minecraft:block/stone"}}}"#,
        );
        base.write(
            "assets/minecraft/blockstates/oak_fence.json",
            r#"{"multipart":[{"apply":{"model":"minecraft:block/oak_fence_post"}}]}"#,
        );
        override_pack.write(
            "assets/minecraft/blockstates/oak_fence.json",
            r#"{"multipart":[{"apply":[{"model":"minecraft:block/oak_fence_post"},{"model":"minecraft:block/oak_fence_side"}]}]}"#,
        );

        let stack = ClientResourceStack::new(vec![
            ClientResourcePack::new("base", base.path()),
            ClientResourcePack::new("override", override_pack.path()),
        ]);
        let report = ResourceReloadManager::new(stack)
            .with_listener(HeadlessBlockstateModelReferenceReloadListener::new([
                "stone",
                "oak_fence",
            ]))
            .run()
            .expect("blockstate model reference reload should succeed");

        let listener = &report.listener_reports()[0];
        assert_eq!(listener.name, "blockstate_model_references");
        assert_eq!(
            listener.preparation.items(),
            [
                "assets/minecraft/blockstates/stone.json".to_owned(),
                "assets/minecraft/blockstates/oak_fence.json".to_owned(),
            ]
        );
        assert_eq!(
            listener.reload.items(),
            [
                "assets/minecraft/blockstates/stone.json@base:minecraft:block/stone".to_owned(),
                "assets/minecraft/blockstates/oak_fence.json@override:minecraft:block/oak_fence_post,minecraft:block/oak_fence_side".to_owned(),
            ]
        );
    }

    #[test]
    fn blockstate_model_reference_listener_rejects_invalid_json_shape() {
        let temp = TempPack::new();
        temp.write(
            "assets/minecraft/blockstates/stone.json",
            r#"{"variants":[{"model":"minecraft:block/stone"}]}"#,
        );

        let stack = ClientResourceStack::new(vec![ClientResourcePack::new("test", temp.path())]);
        let error = ResourceReloadManager::new(stack)
            .with_listener(HeadlessBlockstateModelReferenceReloadListener::new([
                "stone",
            ]))
            .run()
            .expect_err("invalid blockstate shape should fail");

        assert!(matches!(
            error,
            ResourceReloadError::InvalidBlockstateModelReferences {
                resource,
                pack_id,
                reason
            } if resource == "assets/minecraft/blockstates/stone.json"
                && pack_id == "test"
                && reason == "blockstate variants must be an object"
        ));
    }

    #[test]
    fn particle_manifest_reload_skips_requested_ids_that_are_not_present() {
        let temp = TempPack::new();
        temp.write(
            "assets/minecraft/particles/rain.json",
            r#"{"textures":["minecraft:rain"]}"#,
        );

        let stack = ClientResourceStack::new(vec![ClientResourcePack::new("test", temp.path())]);
        let listener = ParticleManifestReloadListener::new(["rain", "missing"]);
        let descriptions = listener
            .load(&stack)
            .expect("present particle manifests should load");

        assert_eq!(descriptions.descriptions().len(), 1);
        assert_eq!(descriptions.descriptions()[0].id(), "rain");
        assert_eq!(descriptions.descriptions()[0].sprites(), ["minecraft:rain"]);
        assert_eq!(descriptions.descriptions()[0].report().pack_id(), "test");
    }

    #[test]
    fn particle_manifest_reload_rejects_invalid_texture_shape() {
        let temp = TempPack::new();
        temp.write(
            "assets/minecraft/particles/rain.json",
            r#"{"textures":"minecraft:rain"}"#,
        );

        let stack = ClientResourceStack::new(vec![ClientResourcePack::new("test", temp.path())]);
        let error = ParticleManifestReloadListener::new(["rain"])
            .load(&stack)
            .expect_err("invalid particle manifest texture shape should fail");

        assert!(matches!(
            error,
            ResourceReloadError::InvalidParticleManifest {
                resource,
                pack_id,
                reason
            } if resource == "assets/minecraft/particles/rain.json"
                && pack_id == "test"
                && reason == "textures must be an array of resource ids"
        ));
    }

    #[test]
    fn particle_manifest_reload_allows_missing_textures_array() {
        let temp = TempPack::new();
        temp.write("assets/minecraft/particles/rain.json", r#"{"custom":true}"#);

        let stack = ClientResourceStack::new(vec![ClientResourcePack::new("test", temp.path())]);
        let descriptions = ParticleManifestReloadListener::new(["rain"])
            .load(&stack)
            .expect("particle description without textures should load");

        assert_eq!(descriptions.descriptions().len(), 1);
        assert!(descriptions.descriptions()[0].sprites().is_empty());
        assert_eq!(descriptions.descriptions()[0].report().sprite_count(), 0);
    }

    #[test]
    fn particle_manifest_reload_rejects_invalid_texture_identifier() {
        let temp = TempPack::new();
        temp.write(
            "assets/minecraft/particles/rain.json",
            r#"{"textures":["Minecraft:Uppercase"]}"#,
        );

        let stack = ClientResourceStack::new(vec![ClientResourcePack::new("test", temp.path())]);
        let error = ParticleManifestReloadListener::new(["rain"])
            .load(&stack)
            .expect_err("invalid texture identifier should fail");

        assert!(matches!(
            error,
            ResourceReloadError::InvalidParticleManifest {
                resource,
                pack_id,
                reason
            } if resource == "assets/minecraft/particles/rain.json"
                && pack_id == "test"
                && reason == "textures[0] is not a valid resource id"
        ));
    }

    #[test]
    fn equipment_asset_reload_rejects_invalid_present_manifest() {
        let temp = TempPack::new();
        temp.write("assets/minecraft/equipment/diamond.json", "{not json");

        let stack = ClientResourceStack::new(vec![ClientResourcePack::new("test", temp.path())]);
        let error = EquipmentAssetsReloadListener::default()
            .load(&stack)
            .expect_err("invalid equipment asset json should fail");

        assert!(
            matches!(error, ResourceReloadError::ParseResourceJson { resource, .. } if resource == "assets/minecraft/equipment/diamond.json")
        );
    }

    #[test]
    fn equipment_asset_reload_rejects_invalid_layers_shape() {
        let temp = TempPack::new();
        temp.write(
            "assets/minecraft/equipment/diamond.json",
            r#"{"layers":["humanoid"]}"#,
        );

        let stack = ClientResourceStack::new(vec![ClientResourcePack::new("test", temp.path())]);
        let error = EquipmentAssetsReloadListener::default()
            .load(&stack)
            .expect_err("invalid equipment layers shape should fail");

        assert!(
            matches!(error, ResourceReloadError::InvalidEquipmentAsset { resource, pack_id, reason }
                if resource == "assets/minecraft/equipment/diamond.json"
                    && pack_id == "test"
                    && reason == "layers must be an object")
        );
    }

    #[test]
    fn equipment_asset_reload_rejects_invalid_layer_texture_id() {
        let temp = TempPack::new();
        temp.write(
            "assets/minecraft/equipment/diamond.json",
            r#"{"layers":{"humanoid":[{"texture":"Minecraft:Diamond"}]}}"#,
        );

        let stack = ClientResourceStack::new(vec![ClientResourcePack::new("test", temp.path())]);
        let error = EquipmentAssetsReloadListener::default()
            .load(&stack)
            .expect_err("invalid equipment layer texture id should fail");

        assert!(
            matches!(error, ResourceReloadError::InvalidEquipmentAsset { resource, pack_id, reason }
                if resource == "assets/minecraft/equipment/diamond.json"
                    && pack_id == "test"
                    && reason == "layers.humanoid[0].texture is not a valid resource id")
        );
    }

    #[test]
    fn model_entry_smoke_reload_rejects_invalid_json() {
        let temp = TempPack::new();
        temp.write("assets/minecraft/models/block/stone.json", "{not json");
        temp.write("assets/minecraft/models/item/stick.json", "{}");
        temp.write("assets/minecraft/blockstates/stone.json", "{}");
        temp.write("assets/minecraft/items/stone.json", "{}");

        let stack = ClientResourceStack::new(vec![ClientResourcePack::new("test", temp.path())]);
        let error = ResourceReloadManager::new(stack)
            .with_listener(ModelEntrySmokeReloadListener::default())
            .run()
            .expect_err("invalid model entry json should fail");

        assert!(
            matches!(error, ResourceReloadError::ParseResourceJson { resource, .. } if resource == "assets/minecraft/models/block/stone.json")
        );
    }

    #[test]
    fn committed_vanilla_model_dependency_listener_loads_stone_and_stick_resources() {
        let report = ResourceReloadManager::new(ClientResourceStack::vanilla())
            .with_listener(ModelDependencyReloadListener::default())
            .run()
            .expect("committed vanilla model dependency resources should load");

        let listener = &report.listener_reports()[0];
        assert_eq!(listener.name, "model_dependencies");
        assert_eq!(
            listener.preparation.items(),
            [
                "assets/minecraft/blockstates/stone.json".to_owned(),
                "assets/minecraft/models/block/stone.json".to_owned(),
                "assets/minecraft/models/item/stick.json".to_owned(),
                "assets/minecraft/items/stone.json".to_owned(),
            ]
        );
        assert_eq!(
            listener.reload.items(),
            [
                "assets/minecraft/blockstates/stone.json@vanilla:blockstates".to_owned(),
                "assets/minecraft/models/block/stone.json@vanilla:block_models".to_owned(),
                "assets/minecraft/models/item/stick.json@vanilla:item_models".to_owned(),
                "assets/minecraft/items/stone.json@vanilla:item_roots".to_owned(),
                "pack:vanilla:blockstates:1 block_models:1 item_models:1 item_roots:1".to_owned(),
                "blockstate_models:minecraft:block/stone,minecraft:block/stone_mirrored".to_owned(),
                "item_root_models:minecraft:block/stone".to_owned(),
                "parents:minecraft:block/cube_all,minecraft:item/handheld".to_owned(),
                "textures:minecraft:block/stone,minecraft:item/stick".to_owned(),
            ]
        );
    }

    #[test]
    fn model_dependency_listener_reports_counts_by_top_priority_pack() {
        let base = TempPack::new();
        let override_pack = TempPack::new();
        base.write(
            "assets/minecraft/blockstates/stone.json",
            r#"{"variants":{"":{"model":"minecraft:block/stone"}}}"#,
        );
        base.write(
            "assets/minecraft/models/block/stone.json",
            r#"{"parent":"minecraft:block/cube_all","textures":{"all":"minecraft:block/stone"}}"#,
        );
        base.write(
            "assets/minecraft/models/item/stick.json",
            r#"{"parent":"minecraft:item/handheld","textures":{"layer0":"minecraft:item/stick"}}"#,
        );
        base.write(
            "assets/minecraft/items/stone.json",
            r#"{"model":{"type":"minecraft:model","model":"minecraft:block/stone"}}"#,
        );
        override_pack.write(
            "assets/minecraft/models/item/stick.json",
            r#"{"parent":"minecraft:item/generated","textures":{"layer0":"minecraft:item/custom_stick"}}"#,
        );

        let stack = ClientResourceStack::new(vec![
            ClientResourcePack::new("base", base.path()),
            ClientResourcePack::new("override", override_pack.path()),
        ]);
        let report = ResourceReloadManager::new(stack)
            .with_listener(ModelDependencyReloadListener::default())
            .run()
            .expect("model dependency reload should succeed");

        let listener = &report.listener_reports()[0];
        assert_eq!(
            listener.reload.items(),
            [
                "assets/minecraft/blockstates/stone.json@base:blockstates".to_owned(),
                "assets/minecraft/models/block/stone.json@base:block_models".to_owned(),
                "assets/minecraft/models/item/stick.json@override:item_models".to_owned(),
                "assets/minecraft/items/stone.json@base:item_roots".to_owned(),
                "pack:base:blockstates:1 block_models:1 item_models:0 item_roots:1".to_owned(),
                "pack:override:blockstates:0 block_models:0 item_models:1 item_roots:0".to_owned(),
                "blockstate_models:minecraft:block/stone".to_owned(),
                "item_root_models:minecraft:block/stone".to_owned(),
                "parents:minecraft:block/cube_all,minecraft:item/generated".to_owned(),
                "textures:minecraft:block/stone,minecraft:item/custom_stick".to_owned(),
            ]
        );
    }

    #[test]
    fn model_dependency_listener_rejects_invalid_model_shape() {
        let temp = TempPack::new();
        temp.write(
            "assets/minecraft/blockstates/stone.json",
            r#"{"variants":{"":{"model":"minecraft:block/stone"}}}"#,
        );
        temp.write(
            "assets/minecraft/models/block/stone.json",
            r#"{"parent":42,"textures":{"all":"minecraft:block/stone"}}"#,
        );
        temp.write(
            "assets/minecraft/models/item/stick.json",
            r#"{"parent":"minecraft:item/handheld","textures":{"layer0":"minecraft:item/stick"}}"#,
        );
        temp.write(
            "assets/minecraft/items/stone.json",
            r#"{"model":{"type":"minecraft:model","model":"minecraft:block/stone"}}"#,
        );

        let stack = ClientResourceStack::new(vec![ClientResourcePack::new("test", temp.path())]);
        let error = ResourceReloadManager::new(stack)
            .with_listener(ModelDependencyReloadListener::default())
            .run()
            .expect_err("invalid model dependency shape should fail");

        assert!(
            matches!(error, ResourceReloadError::InvalidModelDependency { resource, pack_id, reason } if resource == "assets/minecraft/models/block/stone.json" && pack_id == "test" && reason == "model parent must be a string")
        );
    }

    #[test]
    fn committed_vanilla_gpu_warnlist_loads() {
        let resource =
            load_client_json_resource(&ClientResourceStack::vanilla(), GPU_WARNLIST_RESOURCE)
                .expect("committed vanilla gpu warnlist should parse");

        assert_eq!(resource.report().pack_id(), VANILLA_PACK_ID);
        assert_eq!(
            resource.report().top_level_shape(),
            &ClientJsonTopLevelShape::Object {
                keys: vec![
                    "renderer".to_owned(),
                    "vendor".to_owned(),
                    "version".to_owned()
                ],
            }
        );
    }

    #[test]
    fn committed_vanilla_particle_manifest_listener_loads_default_manifest_set() {
        let stack = ClientResourceStack::vanilla();
        let descriptions = ParticleManifestReloadListener::default()
            .load(&stack)
            .expect("committed vanilla particle descriptions should load");
        let total_sprites = descriptions
            .descriptions()
            .iter()
            .map(|description| description.sprites().len())
            .sum::<usize>();

        assert_eq!(descriptions.descriptions().len(), 106);
        assert_eq!(total_sprites, 460);
        assert!(descriptions.descriptions().iter().any(|description| {
            description.report().resource() == "assets/minecraft/particles/rain.json"
                && description.sprites().iter().map(String::as_str).eq([
                    "minecraft:splash_0",
                    "minecraft:splash_1",
                    "minecraft:splash_2",
                    "minecraft:splash_3",
                ])
        }));

        let report = ResourceReloadManager::new(ClientResourceStack::vanilla())
            .with_listener(ParticleManifestReloadListener::default())
            .run()
            .expect("committed vanilla particle manifests should load");

        let listener = &report.listener_reports()[0];
        assert_eq!(listener.name, "particle_manifests");
        assert_eq!(listener.preparation.items().len(), 106);
        assert_eq!(listener.reload.items().len(), 106);
        assert!(
            listener
                .preparation
                .items()
                .contains(&"assets/minecraft/particles/firework.json".to_owned())
        );
        assert!(listener.reload.items().contains(
            &"assets/minecraft/particles/firework.json@vanilla:8 sprites:minecraft:spark_7,minecraft:spark_6,minecraft:spark_5,minecraft:spark_4,minecraft:spark_3,minecraft:spark_2,minecraft:spark_1,minecraft:spark_0".to_owned()
        ));
    }

    #[test]
    fn committed_vanilla_waypoint_style_manifest_listener_loads_default_manifest_set() {
        let stack = ClientResourceStack::vanilla();
        let styles = WaypointStyleManifestReloadListener::default()
            .load(&stack)
            .expect("committed vanilla waypoint styles should load");
        let default_style = styles
            .styles()
            .iter()
            .find(|style| style.id() == "default")
            .expect("default waypoint style should load");
        let bowtie_style = styles
            .styles()
            .iter()
            .find(|style| style.id() == "bowtie")
            .expect("bowtie waypoint style should load");

        assert_eq!(default_style.near_distance(), 128);
        assert_eq!(default_style.far_distance(), 332);
        assert_eq!(
            default_style.sprite_locations(),
            [
                "minecraft:hud/locator_bar_dot/default_0",
                "minecraft:hud/locator_bar_dot/default_1",
                "minecraft:hud/locator_bar_dot/default_2",
                "minecraft:hud/locator_bar_dot/default_3",
            ]
        );
        assert_eq!(bowtie_style.near_distance(), 64);
        assert_eq!(bowtie_style.far_distance(), 332);

        let report = ResourceReloadManager::new(ClientResourceStack::vanilla())
            .with_listener(WaypointStyleManifestReloadListener::default())
            .run()
            .expect("committed vanilla waypoint style manifests should load");

        let listener = &report.listener_reports()[0];
        assert_eq!(listener.name, "waypoint_style_manifests");
        assert_eq!(
            listener.preparation.items(),
            [
                "assets/minecraft/waypoint_style/default.json".to_owned(),
                "assets/minecraft/waypoint_style/bowtie.json".to_owned(),
            ]
        );
        assert_eq!(
            listener.reload.items(),
            [
                "assets/minecraft/waypoint_style/default.json@vanilla:near:128 far:332 sprites:4:minecraft:default_0,minecraft:default_1,minecraft:default_2,minecraft:default_3 locations:minecraft:hud/locator_bar_dot/default_0,minecraft:hud/locator_bar_dot/default_1,minecraft:hud/locator_bar_dot/default_2,minecraft:hud/locator_bar_dot/default_3"
                    .to_owned(),
                "assets/minecraft/waypoint_style/bowtie.json@vanilla:near:64 far:332 sprites:5:minecraft:bowtie,minecraft:default_0,minecraft:default_1,minecraft:default_2,minecraft:default_3 locations:minecraft:hud/locator_bar_dot/bowtie,minecraft:hud/locator_bar_dot/default_0,minecraft:hud/locator_bar_dot/default_1,minecraft:hud/locator_bar_dot/default_2,minecraft:hud/locator_bar_dot/default_3"
                    .to_owned(),
            ]
        );
    }

    #[test]
    fn committed_vanilla_equipment_asset_listener_loads_representative_manifests() {
        let report = ResourceReloadManager::new(ClientResourceStack::vanilla())
            .with_listener(EquipmentAssetsReloadListener::default())
            .run()
            .expect("committed vanilla equipment assets should load");

        let listener = &report.listener_reports()[0];
        assert_eq!(listener.name, "equipment_assets");
        assert!(listener.preparation.items().len() >= 40);
        assert_eq!(
            listener.preparation.items().len(),
            listener.reload.items().len()
        );
        assert!(listener.reload.items().contains(
            &"assets/minecraft/equipment/diamond.json@vanilla:layers:5 entries:5 layer_types:horse_body,humanoid,humanoid_baby,humanoid_leggings,nautilus_body textures:minecraft:diamond texture_locations:minecraft:textures/entity/equipment/horse_body/diamond.png,minecraft:textures/entity/equipment/humanoid/diamond.png,minecraft:textures/entity/equipment/humanoid_baby/diamond.png,minecraft:textures/entity/equipment/humanoid_leggings/diamond.png,minecraft:textures/entity/equipment/nautilus_body/diamond.png dyeable:0 player_textures:0".to_owned()
        ));
        assert!(listener.reload.items().contains(
            &"assets/minecraft/equipment/elytra.json@vanilla:layers:1 entries:1 layer_types:wings textures:minecraft:elytra texture_locations:minecraft:textures/entity/equipment/wings/elytra.png dyeable:0 player_textures:1".to_owned()
        ));
    }

    #[test]
    fn committed_vanilla_model_entry_smoke_listener_loads_default_resources() {
        let report = ResourceReloadManager::new(ClientResourceStack::vanilla())
            .with_listener(ModelEntrySmokeReloadListener::default())
            .run()
            .expect("committed vanilla model entry smoke resources should load");

        let listener = &report.listener_reports()[0];
        assert_eq!(listener.name, "model_entry_smoke");
        assert_eq!(listener.preparation.items(), DEFAULT_MODEL_SMOKE_RESOURCES);
        assert_eq!(
            listener.reload.items(),
            [
                "assets/minecraft/models/block/stone.json@vanilla:object keys:parent,textures"
                    .to_owned(),
                "assets/minecraft/models/item/stick.json@vanilla:object keys:parent,textures"
                    .to_owned(),
                "assets/minecraft/blockstates/stone.json@vanilla:object keys:variants".to_owned(),
                "assets/minecraft/items/stone.json@vanilla:object keys:model".to_owned(),
            ]
        );
    }

    #[test]
    fn committed_vanilla_regional_compliancies_loads() {
        let resource = load_client_json_resource(
            &ClientResourceStack::vanilla(),
            REGIONAL_COMPLIANCIES_RESOURCE,
        )
        .expect("committed vanilla regional compliancies should parse");

        assert_eq!(resource.report().pack_id(), VANILLA_PACK_ID);
        assert_eq!(
            resource.report().top_level_shape(),
            &ClientJsonTopLevelShape::Object {
                keys: vec!["KOR".to_owned()],
            }
        );
    }

    #[test]
    fn colormap_listener_reports_highest_priority_pack_pixels() {
        let base = TempPack::new();
        let override_pack = TempPack::new();
        let base_png = encode_test_rgba_png(1, 1, &[0, 0, 0, 255]);
        let override_png = encode_test_rgba_png(
            2,
            2,
            &[
                255, 0, 0, 255, //
                0, 255, 0, 255, //
                0, 0, 255, 128, //
                0, 0, 0, 0,
            ],
        );
        base.write_bytes(GRASS_COLORMAP_RESOURCE, &base_png);
        override_pack.write_bytes(GRASS_COLORMAP_RESOURCE, &override_png);

        let stack = ClientResourceStack::new(vec![
            ClientResourcePack::new("base", base.path()),
            ClientResourcePack::new("override", override_pack.path()),
        ]);
        let listener = ColormapReloadListener::new([GRASS_COLORMAP_RESOURCE]);

        let colormaps = listener.load(&stack).expect("colormap load should succeed");
        let report = colormaps[0].report();
        assert_eq!(report.resource(), GRASS_COLORMAP_RESOURCE);
        assert_eq!(report.pack_id(), "override");
        assert_eq!(report.byte_count(), override_png.len());
        assert_eq!(report.width(), 2);
        assert_eq!(report.height(), 2);
        assert_eq!(report.pixel_count(), 4);
        assert_eq!(
            colormaps[0].pixels(),
            [0xffff0000, 0xff00ff00, 0x800000ff, 0x00000000]
        );

        let manager = ResourceReloadManager::new(stack).with_listener(listener);
        let reload_report = manager.run().expect("colormap reload should succeed");

        let listener = &reload_report.listener_reports()[0];
        assert_eq!(listener.name, "colormaps");
        assert_eq!(
            listener.preparation.items(),
            [GRASS_COLORMAP_RESOURCE.to_owned()]
        );
        assert_eq!(
            listener.reload.items(),
            [format!(
                "{GRASS_COLORMAP_RESOURCE}@override:{} bytes:rgba8:2x2:4 pixels",
                override_png.len()
            )]
        );
    }

    #[test]
    fn texture_metadata_listener_reports_highest_priority_pack_bytes_and_mcmeta_shape() {
        let base = TempPack::new();
        let override_pack = TempPack::new();
        let texture = "assets/minecraft/textures/block/test.png";
        base.write_bytes(texture, MINIMAL_PNG);
        base.write(
            &format!("{texture}.mcmeta"),
            r#"{"animation":{"frametime":2}}"#,
        );
        override_pack.write_bytes(texture, OVERRIDE_MINIMAL_PNG);
        override_pack.write(&format!("{texture}.mcmeta"), r#"{"custom":true}"#);

        let stack = ClientResourceStack::new(vec![
            ClientResourcePack::new("base", base.path()),
            ClientResourcePack::new("override", override_pack.path()),
        ]);
        let report = ResourceReloadManager::new(stack)
            .with_listener(TextureMetadataReloadListener::new([texture]))
            .run()
            .expect("texture metadata reload should succeed");

        let listener = &report.listener_reports()[0];
        assert_eq!(listener.name, "texture_metadata");
        assert_eq!(listener.preparation.items(), [texture.to_owned()]);
        assert_eq!(
            listener.reload.items(),
            [format!(
                "{texture}@override:{} bytes:png-signature-ok:mcmeta@override:object keys:custom",
                OVERRIDE_MINIMAL_PNG.len()
            )]
        );
    }

    #[test]
    fn texture_metadata_reload_rejects_invalid_mcmeta_json() {
        let temp = TempPack::new();
        let texture = "assets/minecraft/textures/block/test.png";
        temp.write_bytes(texture, MINIMAL_PNG);
        temp.write(&format!("{texture}.mcmeta"), "{not json");

        let stack = ClientResourceStack::new(vec![ClientResourcePack::new("test", temp.path())]);
        let error = TextureMetadataReloadListener::new([texture])
            .load(&stack)
            .expect_err("invalid texture mcmeta json should fail");

        assert!(
            matches!(error, ResourceReloadError::ParseResourceJson { resource, .. } if resource == format!("{texture}.mcmeta"))
        );
    }

    #[test]
    fn committed_vanilla_texture_metadata_listener_loads_representative_textures() {
        let report = ResourceReloadManager::new(ClientResourceStack::vanilla())
            .with_listener(TextureMetadataReloadListener::default())
            .run()
            .expect("committed vanilla representative textures should load");

        let listener = &report.listener_reports()[0];
        assert_eq!(listener.name, "texture_metadata");
        assert_eq!(
            listener.preparation.items(),
            DEFAULT_REPRESENTATIVE_TEXTURES
        );
        assert_eq!(
            listener.reload.items().len(),
            DEFAULT_REPRESENTATIVE_TEXTURES.len()
        );

        for resource in DEFAULT_REPRESENTATIVE_TEXTURES {
            let prefix = format!("{resource}@{VANILLA_PACK_ID}:");
            let item = listener
                .reload
                .items()
                .iter()
                .find(|item| item.starts_with(&prefix))
                .unwrap_or_else(|| panic!("reload report should include {resource}"));
            let byte_count = item
                .strip_prefix(&prefix)
                .and_then(|value| value.split_once(" bytes:png-signature-ok:"))
                .map(|(byte_count, _)| byte_count)
                .expect("report should include byte count and signature status")
                .parse::<usize>()
                .expect("byte count should be numeric");
            assert!(byte_count > PNG_SIGNATURE.len());
        }

        let pumpkinblur = listener
            .reload
            .items()
            .iter()
            .find(|item| {
                item.starts_with("assets/minecraft/textures/misc/pumpkinblur.png@vanilla:")
            })
            .expect("pumpkinblur should be included");
        assert!(
            pumpkinblur.ends_with(" bytes:png-signature-ok:mcmeta@vanilla:object keys:texture")
        );
    }

    #[test]
    fn committed_vanilla_colormap_listener_loads_default_colormap_set() {
        let stack = ClientResourceStack::vanilla();
        let colormaps = ColormapReloadListener::default()
            .load(&stack)
            .expect("committed vanilla colormaps should load");

        assert_eq!(colormaps.len(), 3);
        for colormap in &colormaps {
            let report = colormap.report();
            assert!(DEFAULT_COLORMAPS.contains(&report.resource()));
            assert_eq!(report.pack_id(), VANILLA_PACK_ID);
            assert!(report.byte_count() > PNG_SIGNATURE.len());
            assert_eq!(report.width(), 256);
            assert_eq!(report.height(), 256);
            assert_eq!(report.pixel_count(), 65_536);
            assert_eq!(colormap.pixels().len(), 65_536);
        }

        let manager =
            ResourceReloadManager::new(stack).with_listener(ColormapReloadListener::default());

        let report = manager
            .run()
            .expect("committed vanilla colormaps should reload");

        let listener = &report.listener_reports()[0];
        assert_eq!(listener.name, "colormaps");
        assert_eq!(listener.preparation.items(), DEFAULT_COLORMAPS);
        assert_eq!(listener.reload.items().len(), 3);

        for resource in DEFAULT_COLORMAPS {
            let prefix = format!("{resource}@{VANILLA_PACK_ID}:");
            let item = listener
                .reload
                .items()
                .iter()
                .find(|item| item.starts_with(&prefix))
                .unwrap_or_else(|| panic!("reload report should include {resource}"));
            let byte_count = item
                .strip_prefix(&prefix)
                .and_then(|value| value.strip_suffix(" bytes:rgba8:256x256:65536 pixels"))
                .expect("report should include byte count and decoded dimensions")
                .parse::<usize>()
                .expect("byte count should be numeric");
            assert!(byte_count > PNG_SIGNATURE.len());
        }
    }

    #[test]
    fn cloud_texture_listener_reports_highest_priority_pack_bytes_and_png_signature() {
        let base = TempPack::new();
        let override_pack = TempPack::new();
        base.write_bytes(CLOUDS_TEXTURE_RESOURCE, MINIMAL_PNG);
        override_pack.write_bytes(CLOUDS_TEXTURE_RESOURCE, OVERRIDE_MINIMAL_PNG);

        let stack = ClientResourceStack::new(vec![
            ClientResourcePack::new("base", base.path()),
            ClientResourcePack::new("override", override_pack.path()),
        ]);
        let manager =
            ResourceReloadManager::new(stack).with_listener(CloudTextureReloadListener::default());

        let report = manager.run().expect("cloud texture reload should succeed");

        let listener = &report.listener_reports()[0];
        assert_eq!(listener.name, "cloud_texture");
        assert_eq!(
            listener.preparation.items(),
            [CLOUDS_TEXTURE_RESOURCE.to_owned()]
        );
        assert_eq!(
            listener.reload.items(),
            [format!(
                "{CLOUDS_TEXTURE_RESOURCE}@override:{} bytes:png-signature-ok",
                OVERRIDE_MINIMAL_PNG.len()
            )]
        );
    }

    #[test]
    fn committed_vanilla_cloud_texture_listener_loads_cloud_png() {
        let manager = ResourceReloadManager::new(ClientResourceStack::vanilla())
            .with_listener(CloudTextureReloadListener::default());

        let report = manager
            .run()
            .expect("committed vanilla cloud texture should load");

        let listener = &report.listener_reports()[0];
        assert_eq!(listener.name, "cloud_texture");
        assert_eq!(
            listener.preparation.items(),
            [CLOUDS_TEXTURE_RESOURCE.to_owned()]
        );

        let prefix = format!("{CLOUDS_TEXTURE_RESOURCE}@{VANILLA_PACK_ID}:");
        let item = &listener.reload.items()[0];
        let byte_count = item
            .strip_prefix(&prefix)
            .and_then(|value| value.strip_suffix(" bytes:png-signature-ok"))
            .expect("report should include byte count and signature status")
            .parse::<usize>()
            .expect("byte count should be numeric");
        assert!(byte_count > PNG_SIGNATURE.len());
    }

    #[test]
    fn cloud_texture_reload_rejects_invalid_png_signature() {
        let temp = TempPack::new();
        temp.write_bytes(CLOUDS_TEXTURE_RESOURCE, b"not a png");

        let stack = ClientResourceStack::new(vec![ClientResourcePack::new("test", temp.path())]);
        let error = ResourceReloadManager::new(stack)
            .with_listener(CloudTextureReloadListener::default())
            .run()
            .expect_err("cloud texture with invalid png signature should fail");

        assert!(
            matches!(error, ResourceReloadError::InvalidPngSignature { resource, byte_count, .. } if resource == CLOUDS_TEXTURE_RESOURCE && byte_count == 9)
        );
    }

    #[test]
    fn language_reload_applies_pack_priority_overrides() {
        let low = TempPack::new();
        low.write(
            "assets/minecraft/lang/en_us.json",
            r#"{"menu.play":"Play","shared":"low en","only.low":"Low"}"#,
        );
        low.write(
            "assets/minecraft/lang/pirate.json",
            r#"{"menu.play":"Sail","shared":"low pirate"}"#,
        );

        let high = TempPack::new();
        high.write(
            "assets/minecraft/lang/en_us.json",
            r#"{"shared":"high en"}"#,
        );
        high.write(
            "assets/minecraft/lang/pirate.json",
            r#"{"shared":"high pirate","only.high":"High"}"#,
        );

        let stack = ClientResourceStack::new(vec![
            ClientResourcePack::new("low", low.path()),
            ClientResourcePack::new("high", high.path()),
        ]);

        let resources = ClientLanguageReloadListener::new("pirate")
            .load(&stack)
            .expect("language resources should load");

        assert_eq!(
            resources.translations().get("menu.play"),
            Some(&"Sail".to_owned())
        );
        assert_eq!(
            resources.translations().get("shared"),
            Some(&"high pirate".to_owned())
        );
        assert_eq!(
            resources.translations().get("only.low"),
            Some(&"Low".to_owned())
        );
        assert_eq!(
            resources.translations().get("only.high"),
            Some(&"High".to_owned())
        );
        assert_eq!(
            resources.report().loaded_files(),
            [
                "assets/minecraft/lang/en_us.json@low".to_owned(),
                "assets/minecraft/lang/en_us.json@high".to_owned(),
                "assets/minecraft/lang/pirate.json@low".to_owned(),
                "assets/minecraft/lang/pirate.json@high".to_owned(),
            ]
        );
        assert_eq!(resources.report().translation_count(), 4);
    }

    #[test]
    fn language_reload_uses_en_us_before_selected_language() {
        let temp = TempPack::new();
        temp.write(
            "assets/minecraft/lang/en_us.json",
            r#"{"menu.play":"Play","menu.quit":"Quit"}"#,
        );
        temp.write(
            "assets/minecraft/lang/pirate.json",
            r#"{"menu.play":"Sail"}"#,
        );

        let stack = ClientResourceStack::new(vec![ClientResourcePack::new("test", temp.path())]);
        let resources = load_client_language_resources(&stack, "pirate")
            .expect("language resources should load");

        assert_eq!(
            resources.translations().get("menu.play"),
            Some(&"Sail".to_owned())
        );
        assert_eq!(
            resources.translations().get("menu.quit"),
            Some(&"Quit".to_owned())
        );
        assert_eq!(resources.report().language_code(), "pirate");
        assert_eq!(resources.report().translation_count(), 2);
    }

    #[test]
    fn language_reload_listener_reports_file_and_translation_counts() {
        let temp = TempPack::new();
        temp.write(
            "assets/minecraft/lang/en_us.json",
            r#"{"menu.play":"Play"}"#,
        );

        let stack = ClientResourceStack::new(vec![ClientResourcePack::new("test", temp.path())]);
        let report = ResourceReloadManager::new(stack)
            .with_listener(ClientLanguageReloadListener::new(DEFAULT_LANGUAGE_CODE))
            .run()
            .expect("language reload listener should run");

        assert_eq!(report.listener_reports().len(), 1);
        assert_eq!(report.listener_reports()[0].name, "client_languages");
        assert_eq!(
            report.listener_reports()[0].reload.items(),
            [
                "language:en_us".to_owned(),
                "files:1".to_owned(),
                "translations:1".to_owned(),
            ]
        );
    }

    #[test]
    fn committed_vanilla_en_us_language_loads() {
        let resources = load_client_language_resources(&ClientResourceStack::vanilla(), "en_us")
            .expect("committed vanilla en_us should load");

        assert_eq!(
            resources.translations().get("language.code"),
            Some(&"en_us".to_owned())
        );
        assert!(resources.report().translation_count() > 1000);
        assert_eq!(
            resources.report().loaded_files(),
            ["assets/minecraft/lang/en_us.json@vanilla".to_owned()]
        );
    }

    #[test]
    fn manager_event_callback_reports_events_as_reload_runs() {
        let stack = ClientResourceStack::new(Vec::new());
        let manager = ResourceReloadManager::new(stack)
            .with_listener(ListingResourceReloadListener::new(
                "first",
                std::iter::empty::<&str>(),
            ))
            .with_listener(ListingResourceReloadListener::new(
                "second",
                std::iter::empty::<&str>(),
            ));
        let mut callback_events = Vec::new();

        let report = manager
            .run_with_events(|event| {
                callback_events.push((
                    event.listener.clone(),
                    event.step,
                    event.progress_snapshot.actual_progress(),
                ));
            })
            .expect("reload should complete");

        assert_eq!(callback_events.len(), report.events().len());
        assert_eq!(
            callback_events,
            report
                .events()
                .iter()
                .map(|event| (
                    event.listener.clone(),
                    event.step,
                    event.progress_snapshot.actual_progress()
                ))
                .collect::<Vec<_>>()
        );
        assert!(callback_events[0].2 < 1.0);
        assert_eq!(callback_events.last().map(|event| event.2), Some(1.0));
    }

    #[test]
    fn missing_asset_fails_before_reload_finishes() {
        let temp = TempPack::new();
        let stack = ClientResourceStack::new(vec![ClientResourcePack::new("test", temp.path())]);
        let manager = ResourceReloadManager::new(stack).with_listener(
            ListingResourceReloadListener::new("missing", ["assets/minecraft/lang/en_us.json"]),
        );

        let error = manager.run().expect_err("missing asset should fail");

        assert!(
            matches!(error, ResourceReloadError::MissingResource(resource) if resource == "assets/minecraft/lang/en_us.json")
        );
    }

    struct TempPack {
        root: PathBuf,
    }

    static TEMP_PACK_COUNTER: AtomicU64 = AtomicU64::new(0);

    const MINIMAL_PNG: &[u8] = &[
        137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 1, 0, 0, 0, 1, 8, 6,
        0, 0, 0, 31, 21, 196, 137, 0, 0, 0, 13, 73, 68, 65, 84, 120, 156, 99, 248, 15, 4, 0, 9,
        251, 3, 253, 5, 67, 69, 202, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130,
    ];
    const OVERRIDE_MINIMAL_PNG: &[u8] = &[
        137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 1, 0, 0, 0, 1, 8, 6,
        0, 0, 0, 31, 21, 196, 137, 0, 0, 0, 13, 73, 68, 65, 84, 120, 156, 99, 248, 207, 192, 240,
        31, 0, 5, 0, 1, 255, 137, 153, 61, 29, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130,
    ];

    fn encode_test_rgba_png(width: u32, height: u32, rgba: &[u8]) -> Vec<u8> {
        assert_eq!(rgba.len(), width as usize * height as usize * 4);
        let mut bytes = Vec::new();
        {
            let mut encoder = png::Encoder::new(&mut bytes, width, height);
            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);
            let mut writer = encoder.write_header().expect("png header should write");
            writer
                .write_image_data(rgba)
                .expect("png image data should write");
        }
        bytes
    }

    fn resource_pack_id(seed: u128) -> Uuid {
        Uuid::from_u128(seed)
    }

    fn server_pack_request(id: Uuid, required: bool) -> ServerResourcePackRequest {
        ServerResourcePackRequest::new(
            id,
            format!("https://example.test/{id}.zip"),
            id.simple().to_string(),
            required,
            None,
        )
    }

    fn server_pack_zip_bytes(pack_metadata: &str) -> Vec<u8> {
        server_pack_zip_bytes_with_entries(pack_metadata, std::iter::empty::<(&str, &str)>())
    }

    fn server_pack_zip_bytes_with_entries<'a>(
        pack_metadata: &str,
        entries: impl IntoIterator<Item = (&'a str, &'a str)>,
    ) -> Vec<u8> {
        let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
        writer
            .start_file(
                PACK_MCMETA_RESOURCE,
                zip::write::SimpleFileOptions::default()
                    .compression_method(zip::CompressionMethod::Stored),
            )
            .expect("pack.mcmeta zip entry should start");
        writer
            .write_all(pack_metadata.as_bytes())
            .expect("pack.mcmeta zip entry should be written");
        for (name, contents) in entries {
            writer
                .start_file(
                    name,
                    zip::write::SimpleFileOptions::default()
                        .compression_method(zip::CompressionMethod::Stored),
                )
                .expect("server pack zip entry should start");
            writer
                .write_all(contents.as_bytes())
                .expect("server pack zip entry should be written");
        }
        writer
            .finish()
            .expect("server pack zip should finish")
            .into_inner()
    }

    trait ServerResourcePackApplyStateTestExt {
        fn apply_test_server_pack(&mut self) -> ServerResourcePackAck;
    }

    impl ServerResourcePackApplyStateTestExt for ServerResourcePackApplyState {
        fn apply_test_server_pack(&mut self) -> ServerResourcePackAck {
            self.start_download();
            self.download_bytes_succeeded(b"test server pack")
                .expect("test request has no enforced sha1");
            self.open_downloaded()
                .expect("downloaded test bytes should open");
            self.apply_opened()
        }
    }

    impl TempPack {
        fn new() -> Self {
            let nanos = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system clock should be after unix epoch")
                .as_nanos();
            let root = std::env::temp_dir().join(format!(
                "azalea-client-resource-test-{}-{nanos}-{}",
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
            self.write_bytes(resource, contents.as_bytes());
        }

        fn write_bytes(&self, resource: &str, contents: &[u8]) {
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

//! Headless client resource reload foundation.
//!
//! This keeps Minecraft client-resource reload shape visible without pulling in
//! rendering, audio, or packet handling.

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt, fs, io,
    path::{Path, PathBuf},
};

use azalea_chat::FormattedText;
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
pub const GPU_WARNLIST_RESOURCE: &str = "assets/minecraft/gpu_warnlist.json";
pub const REGIONAL_COMPLIANCIES_RESOURCE: &str = "assets/minecraft/regional_compliancies.json";
pub const EQUIPMENT_MANIFEST_DIR: &str = "assets/minecraft/equipment";
pub const PARTICLE_MANIFEST_DIR: &str = "assets/minecraft/particles";
pub const WAYPOINT_STYLE_MANIFEST_DIR: &str = "assets/minecraft/waypoint_style";

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
const DEFAULT_MODEL_SMOKE_RESOURCES: &[&str] = &[
    "assets/minecraft/models/block/stone.json",
    "assets/minecraft/models/item/stick.json",
    "assets/minecraft/blockstates/stone.json",
    "assets/minecraft/items/stone.json",
];
const DEFAULT_PARTICLE_MANIFEST_IDS: &[&str] = &["rain", "firework", "splash"];
const DEFAULT_WAYPOINT_STYLE_MANIFEST_IDS: &[&str] = &["default", "bowtie"];
const PNG_SIGNATURE: &[u8; 8] = b"\x89PNG\r\n\x1a\n";

pub type ResourceReloadResult<T> = Result<T, ResourceReloadError>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClientResourcePack {
    id: String,
    root: PathBuf,
}

impl ClientResourcePack {
    pub fn new(id: impl Into<String>, root: impl Into<PathBuf>) -> Self {
        Self {
            id: id.into(),
            root: root.into(),
        }
    }

    pub fn vanilla() -> Self {
        Self::new(VANILLA_PACK_ID, VANILLA_PACK_PATH)
    }

    pub fn server_placeholder(id: Uuid) -> Self {
        Self::new(
            format!("server:{id}"),
            PathBuf::from(format!("<server-resource-pack:{id}>")),
        )
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
        self.resource_path(resource).is_file()
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
        self.packs.iter().rev().find_map(|pack| {
            let path = pack.resource_path(resource);
            path.is_file().then(|| ResourceLocation {
                pack_id: pack.id.clone(),
                path,
            })
        })
    }

    pub fn resource_stack(&self, resource: impl AsRef<Path>) -> Vec<ResourceLocation> {
        let resource = resource.as_ref();
        self.packs
            .iter()
            .filter_map(|pack| {
                let path = pack.resource_path(resource);
                path.is_file().then(|| ResourceLocation {
                    pack_id: pack.id.clone(),
                    path,
                })
            })
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
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServerResourcePackStatus {
    Pending,
    Accepted,
    Downloading,
    Downloaded,
    Applied,
    Failed(ServerResourcePackFailure),
    Declined,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServerResourcePackFailure {
    Download,
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
}

impl ServerResourcePackApplyState {
    pub fn new(request: ServerResourcePackRequest) -> Self {
        Self {
            request,
            status: ServerResourcePackStatus::Pending,
        }
    }

    pub fn request(&self) -> &ServerResourcePackRequest {
        &self.request
    }

    pub fn status(&self) -> ServerResourcePackStatus {
        self.status
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
    }

    pub fn download_succeeded(&mut self) {
        self.status = ServerResourcePackStatus::Downloaded;
    }

    pub fn apply_downloaded(&mut self) -> ServerResourcePackAck {
        self.status = ServerResourcePackStatus::Applied;
        self.ack(ServerResourcePackAckAction::SuccessfullyLoaded)
    }

    pub fn download_failed(&mut self) -> ServerResourcePackAck {
        self.status = ServerResourcePackStatus::Failed(ServerResourcePackFailure::Download);
        self.ack(ServerResourcePackAckAction::FailedDownload)
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
        self.ack(ServerResourcePackAckAction::Discarded)
    }

    pub fn placeholder_pack(&self) -> ClientResourcePack {
        ClientResourcePack::server_placeholder(self.request.id)
    }

    fn ack(&self, action: ServerResourcePackAckAction) -> ServerResourcePackAck {
        ServerResourcePackAck {
            id: self.request.id,
            action,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ServerResourcePackApplyError {
    RequiredPackCannotBeDeclined { id: Uuid },
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
                .map(ServerResourcePackApplyState::placeholder_pack),
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
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceReloadState {
    plan: ResourceReloadPlan,
    completed_weight: u32,
    current_listener: Option<String>,
    current_step: Option<ResourceReloadStep>,
}

impl ResourceReloadState {
    pub fn new(plan: ResourceReloadPlan) -> Self {
        Self {
            plan,
            completed_weight: 0,
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

    fn finish_step(&mut self, listener: &str, step: ResourceReloadStep) {
        self.current_listener = Some(listener.to_owned());
        self.current_step = Some(step);
        self.completed_weight += step.weight();
    }

    fn finish_initial_task(&mut self) {
        self.finish_step(
            INITIAL_RELOAD_TASK_NAME,
            ResourceReloadStep::InitialPreparation,
        );
    }
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

    pub fn with_listener(mut self, listener: impl ResourceReloadListener + 'static) -> Self {
        self.listeners.push(Box::new(listener));
        self
    }

    pub fn plan(&self) -> ResourceReloadPlan {
        ResourceReloadPlan::from_listeners(&self.listeners)
    }

    pub fn run(&self) -> ResourceReloadResult<ResourceReloadReport> {
        let mut state = ResourceReloadState::new(self.plan());
        let mut events = Vec::new();
        let mut listener_reports = Vec::new();

        state.finish_initial_task();
        events.push(ResourceReloadEvent::from_state(&state));

        for listener in &self.listeners {
            let name = listener.name();

            let preparation = listener.prepare(&self.stack)?;
            state.finish_step(name, ResourceReloadStep::Preparation);
            events.push(ResourceReloadEvent::from_state(&state));

            let reload = listener.reload(&self.stack)?;
            state.finish_step(name, ResourceReloadStep::Reload);
            events.push(ResourceReloadEvent::from_state(&state));

            state.finish_step(name, ResourceReloadStep::ListenerComplete);
            events.push(ResourceReloadEvent::from_state(&state));

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

#[derive(Clone, Debug, PartialEq)]
pub struct ResourceReloadEvent {
    pub listener: String,
    pub step: ResourceReloadStep,
    pub completed_weight: u32,
    pub progress: f32,
}

impl ResourceReloadEvent {
    fn from_state(state: &ResourceReloadState) -> Self {
        Self {
            listener: state.current_listener.clone().unwrap_or_default(),
            step: state
                .current_step
                .unwrap_or(ResourceReloadStep::ListenerComplete),
            completed_weight: state.completed_weight,
            progress: state.progress(),
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
        fs::read_to_string(&location.path).map_err(|source| ResourceReloadError::ReadResource {
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

    pub fn load(&self, stack: &ClientResourceStack) -> ResourceReloadResult<ClientJsonManifestSet> {
        load_client_json_manifest_set(stack, PARTICLE_MANIFEST_DIR, &self.ids)
    }
}

impl Default for ParticleManifestReloadListener {
    fn default() -> Self {
        Self::new(DEFAULT_PARTICLE_MANIFEST_IDS.iter().copied())
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
        Ok(ResourceReloadTaskReport::new(available_manifest_paths(
            stack,
            PARTICLE_MANIFEST_DIR,
            &self.ids,
        )))
    }

    fn reload(
        &self,
        stack: &ClientResourceStack,
    ) -> ResourceReloadResult<ResourceReloadTaskReport> {
        let manifests = self.load(stack)?;
        Ok(ResourceReloadTaskReport::new(manifest_report_items(
            &manifests,
        )))
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

    pub fn load(&self, stack: &ClientResourceStack) -> ResourceReloadResult<ClientJsonManifestSet> {
        load_client_json_manifest_set(stack, WAYPOINT_STYLE_MANIFEST_DIR, &self.ids)
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
        let manifests = self.load(stack)?;
        Ok(ResourceReloadTaskReport::new(manifest_report_items(
            &manifests,
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

    pub fn load(&self, stack: &ClientResourceStack) -> ResourceReloadResult<ClientJsonManifestSet> {
        load_client_json_manifest_directory(stack, &self.directory)
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
        let manifests = self.load(stack)?;
        Ok(ResourceReloadTaskReport::new(manifest_report_items(
            &manifests,
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

fn manifest_report_items(manifests: &ClientJsonManifestSet) -> Vec<String> {
    manifests.reports().map(json_resource_report_item).collect()
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
        fs::read_to_string(&location.path).map_err(|source| ResourceReloadError::ReadResource {
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
                fs::read(&location.path).map_err(|source| ResourceReloadError::ReadResource {
                    resource: resource.clone(),
                    path: location.path,
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
        let location = stack.require_resource(&self.resource)?;
        let contents = fs::read_to_string(&location.path).map_err(|source| {
            ResourceReloadError::ReadResource {
                resource: self.resource.clone(),
                path: location.path.clone(),
                source,
            }
        })?;
        let splash_count = contents
            .lines()
            .filter(|line| !line.trim().is_empty())
            .count();

        Ok(ResourceReloadTaskReport::new([format!(
            "{}@{}:{splash_count} splashes",
            self.resource, location.pack_id
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
            let contents = fs::read_to_string(&location.path).map_err(|source| {
                ResourceReloadError::ReadResource {
                    resource: manifest.clone(),
                    path: location.path.clone(),
                    source,
                }
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
        let mut loaded = Vec::with_capacity(self.colormaps.len());
        for colormap in &self.colormaps {
            let location = stack.require_resource(colormap)?;
            let bytes =
                fs::read(&location.path).map_err(|source| ResourceReloadError::ReadResource {
                    resource: colormap.clone(),
                    path: location.path.clone(),
                    source,
                })?;

            validate_png_signature(colormap, &location, &bytes)?;

            loaded.push(format!(
                "{colormap}@{}:{} bytes:png-signature-ok",
                location.pack_id,
                bytes.len()
            ));
        }

        Ok(ResourceReloadTaskReport::new(loaded))
    }
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
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
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
        let contents =
            fs::read_to_string(location.path).expect("selected language resource should read");

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
    fn accepted_server_pack_reports_downloaded_then_applied_ack_sequence() {
        let id = resource_pack_id(3);
        let mut pack = ServerResourcePackApplyState::new(server_pack_request(id, true));

        let accepted = pack.accept();
        pack.start_download();
        pack.download_succeeded();
        let loaded = pack.apply_downloaded();

        assert_eq!(
            [accepted, loaded],
            [
                ServerResourcePackAck {
                    id,
                    action: ServerResourcePackAckAction::Accepted,
                },
                ServerResourcePackAck {
                    id,
                    action: ServerResourcePackAckAction::SuccessfullyLoaded,
                },
            ]
        );
        assert_eq!(pack.status(), ServerResourcePackStatus::Applied);
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
            reload_failure.download_succeeded();
            [accepted, reload_failure.reload_failed()]
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
            .apply_downloaded();
        model
            .receive(server_pack_request(resource_pack_id(8), false))
            .decline()
            .expect("optional middle pack can be declined");
        model
            .receive(server_pack_request(second_id, true))
            .apply_downloaded();

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
            .apply_downloaded();
        model
            .receive(server_pack_request(kept_id, true))
            .apply_downloaded();

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
            .apply_downloaded();
        model
            .receive(server_pack_request(resource_pack_id(13), true))
            .apply_downloaded();

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
    fn splashes_listener_counts_non_empty_lines_from_highest_priority_pack() {
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
            [format!("{SPLASHES_RESOURCE}@override:2 splashes")]
        );
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
        let manager = ResourceReloadManager::new(ClientResourceStack::vanilla())
            .with_listener(SplashesReloadListener::default());

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
        assert!(count > 0, "vanilla splashes should not be empty");
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
    fn particle_manifest_listener_reports_priority_pack_and_shape() {
        let base = TempPack::new();
        let override_pack = TempPack::new();
        base.write(
            "assets/minecraft/particles/rain.json",
            r#"{"textures":["minecraft:base_rain"]}"#,
        );
        base.write(
            "assets/minecraft/particles/splash.json",
            r#"{"textures":["minecraft:base_splash"]}"#,
        );
        override_pack.write(
            "assets/minecraft/particles/rain.json",
            r#"{"textures":["minecraft:override_rain"],"override":true}"#,
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
                "assets/minecraft/particles/rain.json@override:object keys:override,textures"
                    .to_owned(),
                "assets/minecraft/particles/splash.json@base:object keys:textures".to_owned(),
            ]
        );
    }

    #[test]
    fn waypoint_style_manifest_listener_reports_priority_pack_and_shape() {
        let base = TempPack::new();
        let override_pack = TempPack::new();
        base.write(
            "assets/minecraft/waypoint_style/default.json",
            r#"{"sprites":["minecraft:default_0"]}"#,
        );
        base.write(
            "assets/minecraft/waypoint_style/bowtie.json",
            r#"{"sprites":["minecraft:bowtie"]}"#,
        );
        override_pack.write(
            "assets/minecraft/waypoint_style/bowtie.json",
            r#"{"near_distance":64,"sprites":["minecraft:override_bowtie"]}"#,
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
                "assets/minecraft/waypoint_style/default.json@base:object keys:sprites"
                    .to_owned(),
                "assets/minecraft/waypoint_style/bowtie.json@override:object keys:near_distance,sprites"
                    .to_owned(),
            ]
        );
    }

    #[test]
    fn equipment_assets_listener_reports_priority_pack_and_shape() {
        let base = TempPack::new();
        let override_pack = TempPack::new();
        base.write(
            "assets/minecraft/equipment/diamond.json",
            r#"{"layers":{"humanoid":[]}}"#,
        );
        base.write(
            "assets/minecraft/equipment/elytra.json",
            r#"{"layers":{"wings":[]}}"#,
        );
        override_pack.write(
            "assets/minecraft/equipment/diamond.json",
            r#"{"layers":{"humanoid":[]},"override":true}"#,
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
                "assets/minecraft/equipment/diamond.json@override:object keys:layers,override"
                    .to_owned(),
                "assets/minecraft/equipment/elytra.json@base:object keys:layers".to_owned(),
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
    fn json_manifest_reload_skips_requested_ids_that_are_not_present() {
        let temp = TempPack::new();
        temp.write(
            "assets/minecraft/particles/rain.json",
            r#"{"textures":["minecraft:rain"]}"#,
        );

        let stack = ClientResourceStack::new(vec![ClientResourcePack::new("test", temp.path())]);
        let listener = ParticleManifestReloadListener::new(["rain", "missing"]);
        let manifests = listener
            .load(&stack)
            .expect("present particle manifests should load");

        assert_eq!(manifests.manifests().len(), 1);
        assert_eq!(manifests.manifests()[0].id(), "rain");
        assert_eq!(
            manifests.manifests()[0].resource().report().pack_id(),
            "test"
        );
    }

    #[test]
    fn json_manifest_reload_rejects_invalid_present_manifest() {
        let temp = TempPack::new();
        temp.write("assets/minecraft/particles/rain.json", "{not json");

        let stack = ClientResourceStack::new(vec![ClientResourcePack::new("test", temp.path())]);
        let error = ParticleManifestReloadListener::new(["rain"])
            .load(&stack)
            .expect_err("invalid particle manifest json should fail");

        assert!(
            matches!(error, ResourceReloadError::ParseResourceJson { resource, .. } if resource == "assets/minecraft/particles/rain.json")
        );
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
        let report = ResourceReloadManager::new(ClientResourceStack::vanilla())
            .with_listener(ParticleManifestReloadListener::default())
            .run()
            .expect("committed vanilla particle manifests should load");

        let listener = &report.listener_reports()[0];
        assert_eq!(listener.name, "particle_manifests");
        assert_eq!(
            listener.preparation.items(),
            [
                "assets/minecraft/particles/rain.json".to_owned(),
                "assets/minecraft/particles/firework.json".to_owned(),
                "assets/minecraft/particles/splash.json".to_owned(),
            ]
        );
        assert_eq!(
            listener.reload.items(),
            [
                "assets/minecraft/particles/rain.json@vanilla:object keys:textures".to_owned(),
                "assets/minecraft/particles/firework.json@vanilla:object keys:textures".to_owned(),
                "assets/minecraft/particles/splash.json@vanilla:object keys:textures".to_owned(),
            ]
        );
    }

    #[test]
    fn committed_vanilla_waypoint_style_manifest_listener_loads_default_manifest_set() {
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
                "assets/minecraft/waypoint_style/default.json@vanilla:object keys:sprites"
                    .to_owned(),
                "assets/minecraft/waypoint_style/bowtie.json@vanilla:object keys:near_distance,sprites"
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
            &"assets/minecraft/equipment/diamond.json@vanilla:object keys:layers".to_owned()
        ));
        assert!(listener.reload.items().contains(
            &"assets/minecraft/equipment/elytra.json@vanilla:object keys:layers".to_owned()
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
    fn colormap_listener_reports_highest_priority_pack_bytes_and_png_signature() {
        let base = TempPack::new();
        let override_pack = TempPack::new();
        base.write_bytes(GRASS_COLORMAP_RESOURCE, MINIMAL_PNG);
        override_pack.write_bytes(GRASS_COLORMAP_RESOURCE, OVERRIDE_MINIMAL_PNG);

        let stack = ClientResourceStack::new(vec![
            ClientResourcePack::new("base", base.path()),
            ClientResourcePack::new("override", override_pack.path()),
        ]);
        let manager = ResourceReloadManager::new(stack)
            .with_listener(ColormapReloadListener::new([GRASS_COLORMAP_RESOURCE]));

        let report = manager.run().expect("colormap reload should succeed");

        let listener = &report.listener_reports()[0];
        assert_eq!(listener.name, "colormaps");
        assert_eq!(
            listener.preparation.items(),
            [GRASS_COLORMAP_RESOURCE.to_owned()]
        );
        assert_eq!(
            listener.reload.items(),
            [format!(
                "{GRASS_COLORMAP_RESOURCE}@override:{} bytes:png-signature-ok",
                OVERRIDE_MINIMAL_PNG.len()
            )]
        );
    }

    #[test]
    fn committed_vanilla_colormap_listener_loads_default_colormap_set() {
        let manager = ResourceReloadManager::new(ClientResourceStack::vanilla())
            .with_listener(ColormapReloadListener::default());

        let report = manager
            .run()
            .expect("committed vanilla colormaps should load");

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
                .and_then(|value| value.strip_suffix(" bytes:png-signature-ok"))
                .expect("report should include byte count and signature status")
                .parse::<usize>()
                .expect("byte count should be numeric");
            assert!(byte_count > PNG_SIGNATURE.len());
        }
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

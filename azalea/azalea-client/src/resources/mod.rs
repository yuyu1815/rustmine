//! Headless client resource reload foundation.
//!
//! This keeps Minecraft client-resource reload shape visible without pulling in
//! rendering, audio, or packet handling.

use std::{
    collections::BTreeMap,
    fmt, fs, io,
    path::{Path, PathBuf},
};

use thiserror::Error;

pub const VANILLA_PACK_ID: &str = "vanilla";
pub const VANILLA_PACK_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../assets/vanilla-pack");
pub const INITIAL_RELOAD_TASK_NAME: &str = "initial";
pub const DEFAULT_LANGUAGE_CODE: &str = "en_us";
pub const SPLASHES_RESOURCE: &str = "assets/minecraft/texts/splashes.txt";

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClientResourceRepository {
    vanilla_pack: ClientResourcePack,
}

impl ClientResourceRepository {
    pub fn new(vanilla_pack: ClientResourcePack) -> Self {
        Self { vanilla_pack }
    }

    pub fn committed_vanilla() -> Self {
        Self::new(ClientResourcePack::vanilla())
    }

    pub fn vanilla_pack(&self) -> &ClientResourcePack {
        &self.vanilla_pack
    }

    pub fn stack(&self) -> ClientResourceStack {
        ClientResourceStack::new(vec![self.vanilla_pack.clone()])
    }
}

impl Default for ClientResourceRepository {
    fn default() -> Self {
        Self::committed_vanilla()
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
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::{Path, PathBuf},
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

    impl TempPack {
        fn new() -> Self {
            let nanos = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system clock should be after unix epoch")
                .as_nanos();
            let root = std::env::temp_dir().join(format!(
                "azalea-client-resource-test-{}-{nanos}",
                std::process::id()
            ));
            fs::create_dir_all(&root).expect("temp resource pack directory should be created");
            Self { root }
        }

        fn path(&self) -> &Path {
            &self.root
        }

        fn write(&self, resource: &str, contents: &str) {
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

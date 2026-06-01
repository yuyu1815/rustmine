//! Headless client resource reload foundation.
//!
//! This keeps Minecraft client-resource reload shape visible without pulling in
//! rendering, audio, or packet handling.

use std::{
    fmt, fs, io,
    path::{Path, PathBuf},
};

use thiserror::Error;

pub const VANILLA_PACK_ID: &str = "vanilla";
pub const VANILLA_PACK_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../assets/vanilla-pack");
pub const INITIAL_RELOAD_TASK_NAME: &str = "initial";

const DEFAULT_REQUIRED_VANILLA_ASSETS: &[&str] = &[
    "assets/minecraft/lang/en_us.json",
    "assets/minecraft/textures/gui/title/mojangstudios.png",
    "assets/minecraft/texts/splashes.txt",
    "assets/minecraft/atlases/blocks.json",
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

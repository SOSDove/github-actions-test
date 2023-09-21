use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Global {
    namespace: String,
    argoproject: String,
    charts: Charts,
    pipelines: Pipelines,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Charts {
    app: AppChart,
    pipeline: PipelineChart,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppChart {
    targetRevision: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PipelineChart {
    targetRevision: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pipelines {
    imageRegistryNamespace: String,
    targets: Vec<Target>,
    image: Image,
    teamsOnFailureNotifyUrl: String,
    gitUpdateCommitStatus: bool,
    mavenBundledSteps: bool,
    pipelineServiceAccount: PipelineServiceAccount,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Target {
    namespace: String,
    branch: String,
    gitConfig: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Image {
    graal: String,
    openjdk: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PipelineServiceAccount {
    secrets: Vec<Secret>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Secret {
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Deployment {
    values: DeploymentValues,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeploymentValues {
    framework: String,
    replicaCount: u32,
    image: DeploymentImage,
    routes: Vec<Route>,
    app: App,
    extraEnv: Vec<Env>,
    extraEnvFrom: Option<Vec<EnvFrom>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeploymentImage {
    repository: String,
    tag: String,
    pullSecret: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Route {
    host: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct App {
    profile: String,
    config: AppConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    properties: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Env {
   pub name: String,
    pub value: String,
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[Name]: {} - [Value]: {}", self.name, self.value)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnvFrom {
    secretRef: SecretRef,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecretRef {
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApplicationValues {
    global: Global,
    deployments: BTreeMap<String, Deployment>,
}

impl ApplicationValues {
    pub(crate) fn to_applications_map(&self) -> ApplicationsMap {
        let mut apps = ApplicationsMap::new();
        for (index, (app_name, deployment)) in self.deployments.iter().enumerate() {
            apps.insert(index as i32, (app_name.clone(), Application {
                framework: deployment.values.framework.clone(),
                replicaCount: deployment.values.replicaCount,
                image: deployment.values.image.clone(),
                routes: deployment.values.routes.clone(),
                app: deployment.values.app.clone(),
                extraEnv: deployment.values.extraEnv.clone(),
                extraEnvFrom: deployment.values.extraEnvFrom.clone(),
            }));
        }
        apps
    }
}

type ApplicationsMap = BTreeMap<i32, (String, Application)>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Application {
    pub(crate) framework: String,
    pub replicaCount: u32,
    pub image: DeploymentImage,
    pub routes: Vec<Route>,
    pub app: App,
    pub extraEnv: Vec<Env>,
    pub extraEnvFrom: Option<Vec<EnvFrom>>,
}

pub struct Choice {
    pub(crate) short: char,
    pub(crate) name: String,
}

impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Extracting the substring from the name (excluding the first character)
        let substring = &self.name[1..];

        write!(f, "({}){}", self.short, substring)
    }
}
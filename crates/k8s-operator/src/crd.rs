use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Struct corresponding to the Specification (`spec`) part of the `Echo` resource, directly
/// reflects context of the `echoes.example.com.yaml` file to be found in this repository.
/// The `Echo` struct will be generated by the `CustomResource` derive macro.
#[derive(CustomResource, Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
#[kube(
    group = "bionic-gpt.com",
    version = "v1",
    kind = "Bionic",
    plural = "bionics",
    derive = "PartialEq",
    namespaced
)]
pub struct BionicSpec {
    pub replicas: i32,
    #[serde(rename = "bionicgpt-image")]
    pub bionicgpt_image: String,
    #[serde(rename = "bionicgpt-pipeline-job-image")]
    pub bionicgpt_pipeline_job_image: String,
    #[serde(rename = "bionicgpt-db-migrations-image")]
    pub bionicgpt_db_migrations_image: String,

    #[serde(rename = "keycloak-image")]
    pub keycloak_image: String,
}
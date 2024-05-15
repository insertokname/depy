use crate::{env_var::EnvVar, parse_json_manifest, path::Path};

/// Struct version of a manifest.json
#[derive(Debug)]
pub struct Manifest {
    pub bin_paths: Vec<Path>,
    pub added_paths: Vec<Path>,
    pub env_vars: Vec<EnvVar>,
    pub version: String,
}

impl Manifest {
    pub fn new(manifest_value: &serde_json::Value) -> Result<Manifest, Box<dyn std::error::Error>> {
        let bin_paths = parse_json_manifest::find_all_bin(manifest_value)?;
        let added_paths = parse_json_manifest::find_all_added_paths(manifest_value)?;
        let env_vars = parse_json_manifest::get_env_variables(manifest_value)?;
        let version = parse_json_manifest::get_version(manifest_value)?;
        Ok(Manifest {
            bin_paths,
            added_paths,
            env_vars,
            version,
        })
    }

    pub fn from_str(manifest: &str) -> Result<Manifest, Box<dyn std::error::Error>> {
        let parsed_json: serde_json::Value = serde_json::from_str(manifest)?;
        Ok(Manifest::new(&parsed_json)?)
    }
}

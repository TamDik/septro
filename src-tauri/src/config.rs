use std::fs;
use std::io::Read;
use std::io::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Namespace {
    id: String,
    #[serde(rename = "rootDir")]
    root_dir: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MasterConfig {
    namespace: Vec<Namespace>,
}

impl Default for MasterConfig {
    fn default() -> Self {
        Self { namespace: vec![] }
    }
}

pub fn setup(config: &tauri::Config) -> Result<MasterConfig, String> {
    let app_dir = tauri::api::path::app_dir(config).unwrap();
    fs::create_dir_all(&app_dir).unwrap_or_else(|why| println!("{:?}", why.kind()));
    let config_path = app_dir.join("config.json");
    match fs::File::open(&config_path) {
        Ok(mut file) => {
            let mut buffer = String::new();
            file.read_to_string(&mut buffer).map_err(|e| format!("{:?}", e))?;
            let config = serde_json::from_str(&buffer).map_err(|e| format!("{:?}", e))?;
            Ok(config)
        },
        Err(_) => {
            let config = MasterConfig::default();
            let config_json = serde_json::to_string(&config).unwrap();
            let file = fs::File::create(&config_path).unwrap();
            write!(&file, "{}", config_json).unwrap();
            Ok(config)
        }
    }
}

use std::fs;
use std::io::Read;
use std::io::prelude::Write;
use serde::{ Serialize, Deserialize };
use crate::wikilink::DEFAULT_NAMESPACE;
use crate::{ category, history };
use rand::Rng;


fn generate_random_string(len: i8) -> String {
    let chars: Vec<char> = "0123456789abcdefghijklmnopqrstuvwxyz".chars().collect();
    let chars_len = chars.len();
    let mut chosen: String = String::new();
    for _ in 1..=len {
        let rand_num = rand::thread_rng().gen_range(0..chars_len);
        chosen.push(chars[rand_num]);
    }
    chosen
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag="type")]
enum NamespaceReference {
    #[serde(rename="internal")]
    Internal {
        id: String,
    },
    #[serde(rename="external")]
    External {
        id: String,
        #[serde(rename="rootDir")]
        root_dir: String,
    }
}

impl NamespaceReference {
    fn internal() -> Self {
        Self::Internal { id: generate_random_string(6) }
    }

    fn external(root_dir: String) -> Self {
        Self::External { id: generate_random_string(6), root_dir }
    }

    fn root_dir(&self, config: &tauri::Config) -> std::path::PathBuf {
        match self {
            NamespaceReference::Internal { id } => {
                let app_dir = tauri::api::path::app_dir(config).unwrap();
                app_dir.join("Data").join(id)
            },
            NamespaceReference::External { id: _, root_dir } => root_dir.into()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag="type")]
enum SideMenuItem {
    #[serde(rename="text")]
    Text {
        value: String,
    },
    #[serde(rename="link")]
    Link {
        text: String,
        path: String,
    },
}

type SideMenuSection = Vec<SideMenuItem>;

#[derive(Serialize, Deserialize, Debug)]
struct SideMenuSubSection {
    title: String,
    data: SideMenuSection,
}

#[derive(Serialize, Deserialize, Debug)]
struct SideMenu {
    main: SideMenuSection,
    sub: Vec<SideMenuSubSection>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MasterConfig {
    namespace: Vec<NamespaceReference>,
    sidemenu: SideMenu,
}

impl Default for MasterConfig {
    fn default() -> Self {
        Self {
            namespace: vec![
                NamespaceReference::Internal { id: "0".to_string() }
            ],
            sidemenu: SideMenu {
                main: vec![],
                sub: vec![]
            }
        }
    }
}

impl MasterConfig {
    fn default_namespace_reference(&self) -> &NamespaceReference {
        &self.namespace[0]
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct NamespaceConfig {
    id: String,
    name: String,
}

impl NamespaceConfig {
    fn new(name: &str) -> Self {
        Self {
            id: generate_random_string(6),
            name: name.to_string(),
        }
    }
}

fn write_json<T: Serialize>(path: std::path::PathBuf, data: T) -> Result<(), std::io::Error> {
    let json_string = serde_json::to_string(&data)?;
    let file = fs::File::create(path)?;
    write!(&file, "{}", json_string)?;
    Ok(())
}

fn new_namespace(name: &str, root_path: std::path::PathBuf) -> Result<(), std::io::Error> {
    fs::create_dir_all(&root_path)?;
    write_json(root_path.join("config.json"), NamespaceConfig::new(name))?;

    history::History::new().save(root_path.join("Page"))?;
    history::History::new().save(root_path.join("File"))?;
    history::History::new().save(root_path.join("FileDescription"))?;
    history::History::new().save(root_path.join("Category"))?;

    category::CategoryReference::new().save(root_path)?;
    Ok(())
}

pub fn setup(app_config: &tauri::Config) -> Result<MasterConfig, std::io::Error> {
    let app_dir = tauri::api::path::app_dir(app_config).unwrap();
    fs::create_dir_all(&app_dir)?;
    let config_path = app_dir.join("config.json");
    let config = match fs::File::open(&config_path) {
        Ok(mut file) => {
            let mut buffer = String::new();
            file.read_to_string(&mut buffer)?;
            serde_json::from_str(&buffer)?
        },
        Err(_) => {
            let config = MasterConfig::default();
            write_json(config_path, &config)?;
            new_namespace(DEFAULT_NAMESPACE, config.default_namespace_reference().root_dir(app_config))?;
            config
        }
    };

    for namespace in &config.namespace {
        let root_dir = namespace.root_dir(app_config);
        println!("{:?}", root_dir);
        history::History::read(root_dir.join("Page"))?;
        category::CategoryReference::read(root_dir)?;
    }
    Ok(config)
}

use std::fs;
use std::path;
use std::io::Read;
use std::io::prelude::Write;
use serde::{ Serialize, Deserialize };
use crate::wikilink::DEFAULT_NAMESPACE;
use crate::{ category, history };
use crate::utils::generate_random_string;


#[derive(Serialize, Deserialize, Debug)]
#[serde(tag="type")]
pub enum NamespaceReference {
    #[serde(rename="internal")]
    Internal {
        id: String,
    },
    #[serde(rename="external")]
    External {
        id: String,
        #[serde(rename="rootDir")]
        root_dir: path::PathBuf,
    }
}

impl NamespaceReference {
    pub fn root_dir(&self, app_dir: &path::PathBuf) -> path::PathBuf {
        match self {
            NamespaceReference::Internal { id } => {
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
    pub fn default_namespace_reference(&self) -> &NamespaceReference {
        &self.namespace[0]
    }

    pub fn internal_namespace(&mut self) -> NamespaceReference {
        let id = generate_random_string(6);
        self.namespace.push(NamespaceReference::Internal { id: id.to_string() });
        NamespaceReference::Internal { id: format!("{}", id) }
    }

    pub fn external_namespace(&mut self, root_dir: path::PathBuf) -> NamespaceReference {
        let id = generate_random_string(6);
        self.namespace.push(NamespaceReference::External { id: id.to_string(), root_dir: root_dir.to_owned() });
        NamespaceReference::External { id, root_dir }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NamespaceConfig {
    id: String,
    name: String,
}

impl NamespaceConfig {
    pub fn new(name: &str) -> Self {
        Self {
            id: generate_random_string(6),
            name: name.to_string(),
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}


fn write_json<T: Serialize>(path: path::PathBuf, data: T) -> Result<(), std::io::Error> {
    let json_string = serde_json::to_string(&data)?;
    let file = fs::File::create(path)?;
    write!(&file, "{}", json_string)?;
    Ok(())
}

fn new_namespace(name: &str, root_path: path::PathBuf) -> Result<(), std::io::Error> {
    fs::create_dir_all(&root_path)?;
    write_json(root_path.join("config.json"), NamespaceConfig::new(name))?;

    history::History::new(root_path.join("Page")).save()?;
    history::History::new(root_path.join("File")).save()?;
    history::History::new(root_path.join("FileDescription")).save()?;
    history::History::new(root_path.join("Category")).save()?;

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
            new_namespace(DEFAULT_NAMESPACE, config.default_namespace_reference().root_dir(&app_dir))?;
            config
        }
    };

    for namespace in &config.namespace {
        let root_dir = namespace.root_dir(&app_dir);
        println!("{:?}", root_dir);
        history::History::read(root_dir.join("Page"))?;
        category::CategoryReference::read(root_dir)?;
    }
    Ok(config)
}

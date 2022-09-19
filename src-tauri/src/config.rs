use std::path;
use serde::{ Serialize, Deserialize };
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
    pub namespace: Vec<NamespaceReference>,
    sidemenu: SideMenu,
}

impl Default for MasterConfig {
    fn default() -> Self {
        Self {
            namespace: vec![],
            sidemenu: SideMenu {
                main: vec![],
                sub: vec![]
            }
        }
    }
}

impl MasterConfig {
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

    pub fn config_path(app_dir: &path::PathBuf) -> path::PathBuf {
        app_dir.join("config.json")
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

    pub fn config_path(root_dir: &path::PathBuf) -> path::PathBuf {
        root_dir.join("config.json")
    }
}

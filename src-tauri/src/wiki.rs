use crate::config::{ MasterConfig, NamespaceConfig };
use crate::content::{ self, Content };
use crate::history::History;
use crate::wikilink::{ DEFAULT_NAMESPACE, WikiLink, WikiType };
use std::fs;
use std::path;
use std::io::Read;
use std::io::prelude::Write;
use serde::{ Serialize, Deserialize };


fn write_json<T: Serialize>(path: path::PathBuf, data: T) -> Result<(), std::io::Error> {
    let json_string = serde_json::to_string(&data)?;
    let file = fs::File::create(path)?;
    write!(&file, "{}", json_string)?;
    Ok(())
}

fn read_json<'a, T: Deserialize<'a>>(path: &path::PathBuf, buffer: &'a mut String) -> Result<T, std::io::Error> {
    let mut file = fs::File::open(path)?;
    file.read_to_string(buffer)?;
    Ok(serde_json::from_str(buffer)?)
}

#[derive(Debug)]
struct Histories {
    page: History,
    file: History,
    file_description: History,
    category: History,
}

#[derive(Debug)]
struct Namespace {
    config: NamespaceConfig,
    history: Histories,
}

impl Namespace {
    fn new(name: &str, root_dir: &path::PathBuf) -> Self {
        let config = NamespaceConfig::new(name);
        fs::create_dir_all(&root_dir).unwrap_or_else(|why| panic!("{:?}", why.kind()));
        write_json(NamespaceConfig::config_path(root_dir), &config).unwrap_or_else(|why| panic!("{:?}", why.kind()));
        let page = History::new(root_dir.join("Page"));
        let file = History::new(root_dir.join("File"));
        let file_description = History::new(root_dir.join("FileDescription"));
        let category = History::new(root_dir.join("Category"));

        page.save().unwrap_or_else(|why| panic!("{:?}", why.kind()));
        file.save().unwrap_or_else(|why| panic!("{:?}", why.kind()));
        file_description.save().unwrap_or_else(|why| panic!("file_description {:?}", why.kind()));
        category.save().unwrap_or_else(|why| panic!("{:?}", why.kind()));
        Self {
            config,
            history: Histories { page, file, file_description, category }
        }
    }

    fn from_config_file(root_dir: &path::PathBuf) -> Result<Self, std::io::Error> {
        let config: NamespaceConfig = read_json(&NamespaceConfig::config_path(root_dir), &mut String::new())?;
        Ok(Self {
            config,
            history: Histories {
                page: History::read(root_dir.join("Page"))?,
                file: History::read(root_dir.join("File"))?,
                file_description: History::read(root_dir.join("FileDescription"))?,
                category: History::read(root_dir.join("Category"))?,
            }
        })
    }

    fn get_name(&self) -> &String {
        self.config.get_name()
    }

    pub fn add_page(&mut self, name: &str) {
        self.history.page.add(name);
    }

    pub fn add_category(&mut self, name: &str) {
        self.history.category.add(name);
    }

    pub fn add_file(&mut self, name: &str) {
        self.history.file.add(name);
        self.history.file_description.add(name);
    }

    pub fn get_content(&self, wikilink: WikiLink) -> Box<dyn Content> {
        match wikilink.wiki_type {
            WikiType::Page => {
                if let Some(path) = self.history.page.get_file_path_by_name(&wikilink.name) {
                    println!("{:?}", path);
                    Box::new(content::Page::new(wikilink))
                } else {
                    Box::new(content::UnknownPage::new(wikilink))
                }
            },
            WikiType::File => Box::new(content::Page::new(wikilink)),
            WikiType::Category => Box::new(content::Page::new(wikilink)),
            WikiType::Special => Box::new(content::Page::new(wikilink)),
        }
    }
}

#[derive(Debug)]
pub struct Wiki {
    app_dir: path::PathBuf,
    config: MasterConfig,
    namespaces: Vec<Namespace>,
}

impl Wiki {
    pub fn new(app_dir: &path::PathBuf) -> Self {
        fs::create_dir_all(&app_dir).unwrap_or_else(|why| panic!("{:?}", why.kind()));
        let config = MasterConfig::default();
        let mut wiki = Self {
            app_dir: app_dir.to_path_buf(),
            config,
            namespaces: vec![],
        };
        wiki.internal_namespace(DEFAULT_NAMESPACE);
        wiki
    }

    pub fn from_master_config_file(app_dir: &path::PathBuf) -> Result<Self, std::io::Error> {
        let config: MasterConfig = read_json(&MasterConfig::config_path(app_dir), &mut String::new())?;

        let mut namespaces: Vec<Namespace> = vec![];
        for namespace in &config.namespace {
            // FIXME: ひとつでも壊れると全てのデータが消えてしまう
            let root_dir = namespace.root_dir(&app_dir);
            namespaces.push(Namespace::from_config_file(&root_dir)?);
            // category::CategoryReference::read(root_dir)?;
        }
        Ok(Self { app_dir: app_dir.to_path_buf(), config, namespaces })
    }

    pub fn internal_namespace(&mut self, name: &str) {
        let reference = self.config.internal_namespace();
        let root_dir = reference.root_dir(&self.app_dir);
        let namespace = Namespace::from_config_file(&root_dir).unwrap_or(Namespace::new(name, &root_dir));
        self.new_namespace(namespace);
    }

    pub fn external_namespace(&mut self, name: &str, root_dir: path::PathBuf) {
        let reference = self.config.external_namespace(root_dir);
        let root_dir = reference.root_dir(&self.app_dir);
        let namespace = Namespace::from_config_file(&root_dir).unwrap_or(Namespace::new(name, &root_dir));
        self.new_namespace(namespace);
    }

    fn new_namespace(&mut self, namespace: Namespace) -> Result<(), std::io::Error> {
        self.namespaces.push(namespace);
        write_json(MasterConfig::config_path(&self.app_dir), &self.config)
    }

    fn get_mut_namespace(&mut self, namespace: &str) -> Option<&mut Namespace> {
        self.namespaces.iter_mut().find(|ns| ns.get_name() == namespace)
    }

    fn get_namespace(&self, namespace: &str) -> Option<&Namespace> {
        self.namespaces.iter().find(|ns| ns.get_name() == namespace)
    }

    pub fn add_page(&mut self, namespace: &str, name: &str) -> Result<WikiLink, ()> {
        match self.get_mut_namespace(namespace) {
            Some(ns) => {
                ns.add_page(&name);
                Ok(WikiLink::new(namespace, WikiType::Page, name))
            },
            None => Err(())
        }
    }

    pub fn add_category(&mut self, namespace: &str, name: &str) -> Result<WikiLink, ()> {
        match self.get_mut_namespace(namespace) {
            Some(ns) => {
                ns.add_category(&name);
                Ok(WikiLink::new(namespace, WikiType::Category, name))
            },
            None => Err(())
        }
    }

    pub fn add_file(&mut self, namespace: &str, name: &str) -> Result<WikiLink, ()> {
        match self.get_mut_namespace(namespace) {
            Some(ns) => {
                ns.add_file(&name);
                Ok(WikiLink::new(namespace, WikiType::File, name))
            },
            None => Err(())
        }
    }

    pub fn get_content(&self, wikilink: WikiLink) -> Box<dyn Content> {
        match self.get_namespace(&wikilink.namespace) {
            None => Box::new(content::UnknownNamespace {wikilink}),
            Some(namespace) => namespace.get_content(wikilink),
        }
    }
}

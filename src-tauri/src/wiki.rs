use crate::content::{self, Content};
use crate::config::{ MasterConfig, NamespaceConfig };
use crate::history::History;
use crate::wikilink::{ DEFAULT_NAMESPACE, WikiLink, WikiType };
use std::path;


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
    fn new(name: &str, root_dir: path::PathBuf) -> Self {
        Self {
            config: NamespaceConfig::new(name),
            history: Histories {
                page: History::new(root_dir.join("Page")),
                file: History::new(root_dir.join("File")),
                file_description: History::new(root_dir.join("FileDescription")),
                category: History::new(root_dir.join("Category")),
            }
        }
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
            WikiType::Page => Box::new(content::Page::new(wikilink)),
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
    pub fn new(app_dir: path::PathBuf) -> Self {
        let config = MasterConfig::default();
        let root_dir = config.default_namespace_reference().root_dir(&app_dir);
        Self {
            app_dir,
            config,
            namespaces: vec![Namespace {
                config: NamespaceConfig::new(DEFAULT_NAMESPACE),
                history: Histories {
                    page: History::new(root_dir.join("Page")),
                    file: History::new(root_dir.join("File")),
                    file_description: History::new(root_dir.join("FileDescription")),
                    category: History::new(root_dir.join("Category")),
                }
            }]
        }
    }

    pub fn internal_namespace(&mut self, name: &str) {
        let refrence = self.config.internal_namespace();
        let namespace = Namespace::new(name, refrence.root_dir(&self.app_dir));
        self.namespaces.push(namespace);
    }

    pub fn external_namespace(&mut self, name: &str, root_dir: path::PathBuf) {
        let refrence = self.config.external_namespace(root_dir);
        let namespace = Namespace::new(name, refrence.root_dir(&self.app_dir));
        self.namespaces.push(namespace);
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
                Ok(WikiLink::page(namespace, name))
            },
            None => Err(())
        }
    }

    pub fn add_category(&mut self, namespace: &str, name: &str) -> Result<WikiLink, ()> {
        match self.get_mut_namespace(namespace) {
            Some(ns) => {
                ns.add_category(&name);
                Ok(WikiLink::category(namespace, name))
            },
            None => Err(())
        }
    }

    pub fn add_file(&mut self, namespace: &str, name: &str) -> Result<WikiLink, ()> {
        match self.get_mut_namespace(namespace) {
            Some(ns) => {
                ns.add_file(&name);
                Ok(WikiLink::file(namespace, name))
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

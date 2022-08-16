use std::fs;
use std::path;
use std::io::{ Write, Read };
use crate::wikilink::WikiLink;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
struct Category {
    #[serde(rename="category")]
    wikilink: WikiLink,  // FIXME:
    refered: Vec<WikiLink>,
}

pub struct CategoryReference {
    categories: Vec<Category>,
}

impl CategoryReference {
    pub fn new() -> Self {
        Self { categories: Vec::new() }
    }

    pub fn read(root_dir: path::PathBuf) -> Result<Self, std::io::Error> {
        let mut file = fs::File::open(root_dir.join("categories.json"))?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        let categories: Vec<Category> = serde_json::from_str(&buffer)?;
        Ok(Self { categories })
    }

    pub fn save(&self, root_dir: path::PathBuf) -> Result<(), std::io::Error> {
        let path = root_dir.join("categories.json");
        let json_string = serde_json::to_string(&self.categories)?;
        let file = fs::File::create(path)?;
        write!(&file, "{}", json_string)?;
        Ok(())
    }
}

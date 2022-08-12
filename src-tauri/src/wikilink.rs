use serde::Serialize;


#[derive(Serialize, Debug)]
pub enum WikiType {
    Page,
    File,
    Category,
    Special
}

impl Default for WikiType {
    fn default() -> Self {
        Self::Page
    }
}

#[derive(Serialize, Debug)]
pub struct WikiLink {
    pub namespace: String,
    pub wiki_type: WikiType,
    pub name: String,
}

impl Default for WikiLink {
    fn default() -> Self {
        Self {
            namespace: "Main".into(),
            wiki_type: WikiType::default(),
            name: "Main".into(),
        }
    }
}

impl WikiLink {
    pub fn parse(url: String) -> Result<Self, ()> {
        Ok(WikiLink { namespace: "Main".into(), wiki_type: WikiType::Page, name: url })
    }
}

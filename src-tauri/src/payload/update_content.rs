use crate::content::{ Tab as ContentTab, Script };

#[derive(Clone, serde::Serialize)]
pub struct UpdateContent {
    pub href: String,
    pub body: String,
    pub tabs: Vec<Tab>,
    pub scripts: Vec<String>
}

#[derive(Clone, serde::Serialize)]
pub struct Tab {
    pub title: String,
    pub selected: bool,
    pub href: String,
}

impl From<&ContentTab> for Tab {
    fn from(tab: &ContentTab) -> Self {
        Self {
            title: tab.title.to_string(),
            selected: tab.selected,
            href: tab.wikilink.href(),
        }
    }
}

impl From<&Script> for String {
    fn from(script: &Script) -> Self {
        match *script {
            Script::MarkdownEditor => "markdownEditor",
        }.into()
    }
}

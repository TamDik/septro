use crate::wikilink::WikiLink;
use crate::content;

#[derive(serde::Deserialize)]
pub struct PageTransitionPayload {
    pub wikilink: WikiLink,
}

#[derive(Clone, serde::Serialize)]
pub struct CoreErrorPayload {
    pub message: String,
}

#[derive(Clone, serde::Serialize)]
pub struct TabPayload {
    pub title: String,
    pub selected: bool,
    pub href: String,
}

impl From<&content::Tab> for TabPayload {
    fn from(tab: &content::Tab) -> Self {
        Self {
            title: tab.title.to_string(),
            selected: tab.selected,
            href: tab.wikilink.href(),
        }
    }
}

#[derive(Clone, serde::Serialize)]
pub struct UpdateContentPayload {
    pub href: String,
    pub body: String,
    pub tabs: Vec<TabPayload>,
}

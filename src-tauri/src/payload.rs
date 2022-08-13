use crate::wikilink;

#[derive(serde::Deserialize)]
pub struct PageTransitionPayload {
    pub wikilink: wikilink::WikiLink,
}

#[derive(Clone, serde::Serialize)]
pub struct CoreErrorPayload {
    pub message: String,
}

#[derive(Clone, serde::Serialize)]
pub struct UpdateContentPayload {
    pub body: String,
}

use crate::wikilink::WikiLink;


#[derive(serde::Deserialize)]
pub struct PageTransition {
    pub wikilink: WikiLink,
}

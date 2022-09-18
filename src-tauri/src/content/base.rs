use crate::wikilink::WikiLink;
use std::str::FromStr;


pub fn header(text: impl Into<String>) -> String {
    format!(r#"<h1 id="content-head">{}</h1>"#, text.into())
}

pub fn body(text: impl Into<String>) -> String {
    format!(r#"<div id="content-body">{}</div>"#, text.into())
}


pub trait Content {
    fn content(&self) -> String;

    fn tabs(&self) -> Vec<Tab>;

    fn scripts(&self) -> Vec<Script>;
}

#[derive(Debug)]
pub struct Tab {
    pub wikilink: WikiLink,
    pub title: String,
    pub selected: bool,
}

impl Tab {
    pub fn selected(wikilink: WikiLink, title: impl Into<String>) -> Self {
        Self { wikilink, title: title.into(), selected: true }
    }

    pub fn not_selected(wikilink: WikiLink, title: impl Into<String>) -> Self {
        Self { wikilink, title: title.into(), selected: false }
    }
}

#[derive(Debug)]
pub enum Mode {
    Read,
    Edit,
    History,
}

impl FromStr for Mode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "read" => Ok(Mode::Read),
            "edit" => Ok(Mode::Edit),
            "history" => Ok(Mode::History),
            _ => Err(()),
        }
    }
}

impl ToString for Mode {
    fn to_string(&self) -> String {
        match self {
            Mode::Read => "read".to_string(),
            Mode::Edit => "edit".to_string(),
            Mode::History => "history".to_string(),
        }
    }
}

impl Into<String> for Mode {
    fn into(self) -> String {
        self.to_string()
    }
}

impl From<&WikiLink> for Mode {
    fn from(wikilink: &WikiLink) -> Self {
        let mode = wikilink.get_query("mode").map_or(Mode::Read.to_string(), |mode| mode.to_string());
        match Mode::from_str(&mode) {
            Ok(mode) => mode,
            Err(()) => Mode::Read,
        }
    }
}


#[derive(Debug)]
pub enum Script {
    MarkdownEditor,
}

use crate::wikilink;
use std::str::FromStr;
use wikilink::{ WikiLink, WikiType };

pub trait Content {
    fn content(&self) -> String;

    fn tabs(&self) -> Vec<Tab>;
}

#[derive(Debug)]
pub struct Tab {
    pub wikilink: WikiLink,
    pub title: String,
    pub selected: bool,
}

impl Tab {
    fn selected(wikilink: WikiLink, title: impl Into<String>) -> Self {
        Self { wikilink, title: title.into(), selected: true }
    }

    fn not_selected(wikilink: WikiLink, title: impl Into<String>) -> Self {
        Self { wikilink, title: title.into(), selected: false }
    }
}

#[derive(Debug)]
enum Mode {
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

pub struct Page {
    mode: Mode,
    wikilink: WikiLink,
}

impl Page {
    pub fn new(wikilink: WikiLink) -> Self {
        let mode = Mode::from(&wikilink);
        Self { mode, wikilink }
    }
}

fn header(text: impl Into<String>) -> String {
    format!(r#"<h1 id="content-head">{}</h1>"#, text.into())
}

fn body(text: impl Into<String>) -> String {
    format!(r#"<div id="content-body">{}</div>"#, text.into())
}

impl Content for Page {
    fn content(&self) -> String {
        let page_name = self.wikilink.base();
        let header = match &self.mode {
            Mode::Read => {
                header(page_name) + &body("body")
            },
            Mode::Edit => {
                header(format!("editing {}", page_name)) + &body("body")
            },
            Mode::History => {
                header(format!(r#"Revision history of "{}""#, page_name)) + &body("body")
            },
        };
        header
    }

    fn tabs(&self) -> Vec<Tab> {
        let mut read = WikiLink::new(&self.wikilink.namespace, WikiType::Page, &self.wikilink.name);
        let mut edit = WikiLink::new(&self.wikilink.namespace, WikiType::Page, &self.wikilink.name);
        let mut hist = WikiLink::new(&self.wikilink.namespace, WikiType::Page, &self.wikilink.name);
        read.add_query("mode", Mode::Read);
        edit.add_query("mode", Mode::Edit);
        hist.add_query("mode", Mode::History);
        match &self.mode {
            Mode::Read => vec![
                Tab::selected(read, "Read"),
                Tab::not_selected(edit, "Edit"),
                Tab::not_selected(hist, "History"),
            ],
            Mode::Edit => vec![
                Tab::not_selected(read, "Read"),
                Tab::selected(edit, "Edit"),
                Tab::not_selected(hist, "History"),
            ],
            Mode::History => vec![
                Tab::not_selected(read, "Read"),
                Tab::not_selected(edit, "Edit"),
                Tab::selected(hist, "History"),
            ],
        }
    }
}


pub struct UnknownNamespace {
    pub wikilink: WikiLink
}

impl Content for UnknownNamespace {
    fn tabs(&self) -> Vec<Tab> {
        let wikilink = WikiLink::new(&self.wikilink.namespace, self.wikilink.wiki_type, &self.wikilink.name);
        vec![
            Tab::selected(wikilink, "Read"),
        ]
    }

    fn content(&self) -> String {
        header(self.wikilink.base()) +
        &body("The namespace you are looking for doesn't exist or an other error occurred. Choose a new direction, or you can create this namespace.")
    }
}

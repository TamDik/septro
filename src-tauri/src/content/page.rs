use crate::wikilink::WikiLink;
use crate::content::{ Tab, Content, Mode, Script, header, body };


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

impl Content for Page {
    fn content(&self) -> String {
        let page_name = self.wikilink.base();
        let header = match &self.mode {
            Mode::Read => {
                header(page_name) + &body("body1")
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
        let mut read = WikiLink::base_of(&self.wikilink);
        let mut edit = WikiLink::base_of(&self.wikilink);
        let mut hist = WikiLink::base_of(&self.wikilink);
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

    fn scripts(&self) -> Vec<Script> {
        vec![]
    }
}

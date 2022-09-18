use crate::wikilink::WikiLink;
use crate::content::{ Tab, Content, Mode, Script, header, body };


pub struct UnknownPage {
    pub wikilink: WikiLink
}

impl UnknownPage {
    pub fn new(wikilink: WikiLink) -> Self {
        UnknownPage { wikilink }
    }
}

impl Content for UnknownPage {
    fn tabs(&self) -> Vec<Tab> {
        let mut read = WikiLink::base_of(&self.wikilink);
        let mut edit = WikiLink::base_of(&self.wikilink);
        let mut hist = WikiLink::base_of(&self.wikilink);
        read.add_query("mode", Mode::Read);
        edit.add_query("mode", Mode::Edit);
        hist.add_query("mode", Mode::History);
        match Mode::from(&self.wikilink) {
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

    fn content(&self) -> String {
        let text = if let Mode::Edit = Mode::from(&self.wikilink) {
            include_str!("../static/editor.html").to_string()
        } else {
            let mut wikilink = WikiLink::base_of(&self.wikilink);
            wikilink.add_query("mode", Mode::Edit);
            format!(r##"There is currently no text in this page. You can <a href="#" data-wikilink="{}">create this page</a>."##, wikilink.href())
        };
        header(self.wikilink.base()) + &body(text)
    }

    fn scripts(&self) -> Vec<Script> {
        if let Mode::Edit = Mode::from(&self.wikilink) {
            vec![Script::MarkdownEditor]
        } else {
            vec![]
        }
    }
}

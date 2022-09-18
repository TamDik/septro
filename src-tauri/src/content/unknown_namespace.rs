use crate::wikilink::WikiLink;
use crate::content::{ Tab, Content, Script, header, body };


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

    fn scripts(&self) -> Vec<Script> {
        vec![]
    }
}

use pulldown_cmark::{html, Parser};


pub fn parse(text: &str, buf: &mut String) {
    let parser = Parser::new(text);
    html::push_html(buf, parser);
}

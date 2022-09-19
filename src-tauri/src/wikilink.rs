use serde::{ Serialize, Deserialize };
use std::str::FromStr;
use std::collections::HashMap;
use percent_encoding::{ utf8_percent_encode, NON_ALPHANUMERIC };


pub const DEFAULT_NAMESPACE: &str = "Main";
pub const DEFAULT_NAME: &str = "Main";


#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum WikiType {
    Page,
    File,
    Category,
    Special
}

impl FromStr for WikiType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Page" => Ok(WikiType::Page),
            "File" => Ok(WikiType::File),
            "Category" => Ok(WikiType::Category),
            "Special" => Ok(WikiType::Special),
            _ => Err(()),
        }
    }
}

impl ToString for WikiType {
    fn to_string(&self) -> String {
        match self {
            WikiType::Page => "Page".to_string(),
            WikiType::File => "File".to_string(),
            WikiType::Category => "Category".to_string(),
            WikiType::Special => "Special".to_string(),
        }
    }
}

impl Default for WikiType {
    fn default() -> Self {
        Self::Page
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WikiLink {
    pub namespace: String,
    pub wiki_type: WikiType,
    pub name: String,
    queries: HashMap<String, String>,
    fragment: Option<String>
}

impl Default for WikiLink {
    fn default() -> Self {
        Self::new(DEFAULT_NAMESPACE, WikiType::Page, DEFAULT_NAME)
    }
}

fn utf8_encode(input: &str) -> String {
    utf8_percent_encode(input, NON_ALPHANUMERIC).to_string()
}

impl WikiLink {
    pub fn new(namespace: impl Into<String>, wiki_type: WikiType, name: impl Into<String>) -> Self {
        Self {
            namespace: namespace.into(),
            wiki_type,
            name: name.into(),
            queries: HashMap::new(),
            fragment: None,
        }
    }

    pub fn base_of(wikilink: &Self) -> Self {
        Self::new(&wikilink.namespace, wikilink.wiki_type, &wikilink.name)
    }

    pub fn get_query(&self, key: &str) -> Option<&String> {
        self.queries.get(&key.to_string())
    }

    pub fn add_query(&mut self, key: impl Into<String>, val: impl Into<String>) {
        self.queries.insert(key.into(), val.into());
    }

    fn set_fragment(&mut self, fragment: &str) {
        self.fragment = Some(fragment.to_string());
    }

    pub fn base(&self) -> String {
        let mut base = self.name.to_string();
        if self.wiki_type != WikiType::Page {
            base = format!("{}:{}", self.wiki_type.to_string(), base)
        }
        if self.namespace != DEFAULT_NAMESPACE {
            base = format!("{}:{}", self.namespace, base);
        }
        base
    }

    pub fn href(&self) -> String {
        let base = self.base();
        let mut queries: Vec<String> = Vec::new();
        let mut query_map: Vec<(&String, &String)> = self.queries.iter().collect();
        query_map.sort_by(|a, b| a.0.cmp(&b.0));
        for (key, value) in query_map {
            if key == "mode" && value == "read" {
                continue;
            }
            queries.push(format!("{}={}", utf8_encode(key), utf8_encode(value)));
        }
        let mut href = String::from(base);
        if queries.len() != 0 {
            href = format!("{}?{}", href, queries.join("&"));
        }
        if let Some(fragment) = &self.fragment {
            href = format!("{}#{}", href, utf8_encode(fragment));
        }
        href
    }

    pub fn parse(uri: impl Into<String>) -> Self {
        // base?queries#fragment
        let mut base = String::new();
        let mut queries: Option<String> = None;
        let mut fragment: Option<String> = None;
        for c in uri.into().chars() {
            match fragment {
                Some(old) => {
                    fragment = Some(old + &c.to_string());
                },
                None => {
                    if c == '#' {
                        fragment = Some("".to_string());
                        continue;
                    }
                    match queries {
                        Some(old) => {
                            queries = Some(old + &c.to_string());
                        },
                        None => {
                            if c == '?' {
                                queries = Some("".to_string());
                            } else {
                                base += &c.to_string();
                            }
                        }
                    }
                }
            }
        }

        // base
        let arr: Vec<&str> = base.split(":").collect();
        let mut wikilink = match &arr[..] {
            &[a] if a == "" => Self::default(),
            &[a] => match WikiType::from_str(a) {
                Ok(a) => Self::new(DEFAULT_NAMESPACE, a, DEFAULT_NAME),
                Err(()) => Self::new(DEFAULT_NAMESPACE, WikiType::Page, a),
            },
            &[a, b] => match [WikiType::from_str(a), WikiType::from_str(b)] {
                [Ok(a), _] => Self::new(DEFAULT_NAMESPACE, a, b),
                [_, Ok(b)] => Self::new(a, b, DEFAULT_NAME),
                _ => Self::new(a, WikiType::Page, b),
            }
            _ => {
                let mut split_off_i = 0;
                let mut wiki_type = WikiType::Page;
                for (i, a) in arr.iter().enumerate() {
                    if let Ok(a) = WikiType::from_str(a) {
                        wiki_type = a;
                        split_off_i = i + 1;
                        break;
                    }
                }
                if split_off_i == 0 {
                    Self::new(arr[0], wiki_type, arr[1..].join(":"))
                } else if split_off_i == 1 {
                    Self::new(DEFAULT_NAMESPACE, wiki_type, arr[1..].join(":"))
                } else if split_off_i == arr.len() {
                    Self::new(arr[..split_off_i - 1].join(":"), wiki_type, DEFAULT_NAME)
                } else {
                    Self::new(arr[..split_off_i - 1].join(":"), wiki_type, arr[split_off_i..].join(":"))
                }
            }
        };

        // queries
        if let Some(queries) = queries {
            let queries: Vec<&str> = queries.split("&").collect();
            for query in queries {
                let key_value: Vec<&str> = query.split("=").collect();
                match &key_value[..] {
                    &[k] if k.len() != 0 => wikilink.add_query(k, ""),
                    &[_] => (),
                    &[k, v] => wikilink.add_query(k, v),
                    _ => wikilink.add_query(key_value[0], &key_value[1..].join("="))
                }
            }
        }

        // fragment
        if let Some(fragment) = fragment {
            wikilink.set_fragment(&fragment);
        }

        wikilink
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_no_colon() {
        assert_eq!(WikiLink::parse(""), WikiLink::new(DEFAULT_NAMESPACE, WikiType::Page, DEFAULT_NAME));
        assert_eq!(WikiLink::parse("a"), WikiLink::new(DEFAULT_NAMESPACE, WikiType::Page, "a"));
        assert_eq!(WikiLink::parse("Category"), WikiLink::new(DEFAULT_NAMESPACE, WikiType::Category, DEFAULT_NAME));
    }

    #[test]
    fn parse_one_colon() {
        assert_eq!(WikiLink::parse("a:b"), WikiLink::new("a", WikiType::Page, "b"));
        assert_eq!(WikiLink::parse("Category:b"), WikiLink::new(DEFAULT_NAMESPACE, WikiType::Category, "b"));
        assert_eq!(WikiLink::parse("a:Category"), WikiLink::new("a", WikiType::Category, DEFAULT_NAME));
    }
    #[test]
    fn parse_more_than_two_colons() {
        assert_eq!(WikiLink::parse("a:b:c"), WikiLink::new("a", WikiType::Page, "b:c"));
        assert_eq!(WikiLink::parse("File:b:c"), WikiLink::new(DEFAULT_NAMESPACE, WikiType::File, "b:c"));
        assert_eq!(WikiLink::parse("a:File:c"), WikiLink::new("a", WikiType::File, "c"));
        assert_eq!(WikiLink::parse("a:b:File"), WikiLink::new("a:b", WikiType::File, DEFAULT_NAME));
    }

    #[test]
    fn parse_pueries() {
        let mut expected = WikiLink::new(DEFAULT_NAMESPACE, WikiType::Page, "a");
        expected.add_query("k1", "v1");
        expected.add_query("k2", "v2");
        expected.add_query("k3", "");
        assert_eq!(WikiLink::parse("a?k1=v1&k2=v2&k3"), expected);

        let mut expected = WikiLink::new(DEFAULT_NAMESPACE, WikiType::Page, DEFAULT_NAME);
        expected.add_query("k1", "v1");
        assert_eq!(WikiLink::parse("?k1=v1"), expected);
    }

    #[test]
    fn parse_fragment() {
        let mut expected = WikiLink::new(DEFAULT_NAMESPACE, WikiType::Page, "a");
        expected.set_fragment("abc");
        assert_eq!(WikiLink::parse("a#abc"), expected);
    }
}

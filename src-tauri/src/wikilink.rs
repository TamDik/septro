use serde::Serialize;
use std::str::FromStr;


#[derive(Serialize, Debug, Eq, PartialEq)]
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

impl Default for WikiType {
    fn default() -> Self {
        Self::Page
    }
}

#[derive(Serialize, Debug, Eq, PartialEq)]
pub struct WikiLink {
    pub namespace: String,
    pub wiki_type: WikiType,
    pub name: String,
}

impl Default for WikiLink {
    fn default() -> Self {
        Self::page("Main".to_string(), "Main".to_string())
    }
}

impl WikiLink {
    fn new(namespace: String, wiki_type: WikiType, name: String) -> Self {
        Self { namespace, wiki_type, name }
    }

    fn page(namespace: String, name: String) -> Self {
        Self { namespace, wiki_type: WikiType::Page, name }
    }

    fn file(namespace: String, name: String) -> Self {
        Self { namespace, wiki_type: WikiType::File, name }
    }

    fn category(namespace: String, name: String) -> Self {
        Self { namespace, wiki_type: WikiType::Category, name }
    }

    fn special(namespace: String, name: String) -> Self {
        Self { namespace, wiki_type: WikiType::Special, name }
    }

    pub fn parse(url: String) -> Self {
        let arr: Vec<&str> = url.split(":").collect();
        match &arr[..] {
            &[a] if a == "" => Self::default(),
            &[a] => match WikiType::from_str(a) {
                Ok(a) => Self::new("Main".to_string(), a, "Main".to_string()),
                Err(()) => Self::page("Main".to_string(), a.to_string()),
            },
            &[a, b] => match [WikiType::from_str(a), WikiType::from_str(b)] {
                [Ok(a), _] => Self::new("Main".to_string(), a, b.to_string()),
                [_, Ok(b)] => Self::new(a.to_string(), b, "Main".to_string()),
                _ => Self::page(a.to_string(), b.to_string()),
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
                    Self::new(arr[0].to_string(), wiki_type, arr[1..].join(":"))
                } else if split_off_i == 1 {
                    Self::new("Main".to_string(), wiki_type, arr[1..].join(":"))
                } else if split_off_i == arr.len() {
                    Self::new(arr[..split_off_i - 1].join(":"), wiki_type, "Main".to_string())
                } else {
                    Self::new(arr[..split_off_i - 1].join(":"), wiki_type, arr[split_off_i..].join(":"))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::wikilink::WikiLink;

    #[test]
    fn parse_no_colon() {
        assert_eq!(WikiLink::parse("".to_string()), WikiLink::page("Main".to_string(), "Main".to_string()));
        assert_eq!(WikiLink::parse("a".to_string()), WikiLink::page("Main".to_string(), "a".to_string()));
        assert_eq!(WikiLink::parse("Category".to_string()), WikiLink::category("Main".to_string(), "Main".to_string()));
    }

    #[test]
    fn parse_one_colon() {
        assert_eq!(WikiLink::parse("a:b".to_string()), WikiLink::page("a".to_string(), "b".to_string()));
        assert_eq!(WikiLink::parse("Category:b".to_string()), WikiLink::category("Main".to_string(), "b".to_string()));
        assert_eq!(WikiLink::parse("a:Category".to_string()), WikiLink::category("a".to_string(), "Main".to_string()));
    }
    #[test]
    fn parse_more_than_two_colons() {
        assert_eq!(WikiLink::parse("a:b:c:d".to_string()), WikiLink::page("a".to_string(), "b:c:d".to_string()));
        assert_eq!(WikiLink::parse("File:b:c:d".to_string()), WikiLink::file("Main".to_string(), "b:c:d".to_string()));
        assert_eq!(WikiLink::parse("a:File:c:d".to_string()), WikiLink::file("a".to_string(), "c:d".to_string()));
        assert_eq!(WikiLink::parse("a:b:File:d".to_string()), WikiLink::file("a:b".to_string(), "d".to_string()));
        assert_eq!(WikiLink::parse("a:b:c:File".to_string()), WikiLink::file("a:b:c".to_string(), "Main".to_string()));
    }
}

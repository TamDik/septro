use std::fs;
use std::path;
use std::io::{ Write, Read };
use crate::wikilink::{ WikiLink, WikiType };
use serde::{ Serialize, Deserialize, ser::SerializeStruct, de::{ self, Visitor, MapAccess } };

#[derive(Serialize, Deserialize, Debug)]
struct Category {
    #[serde(rename="category")]
    #[serde(serialize_with="serialize_wikilink")]
    #[serde(deserialize_with="deserialize_wikilink")]
    wikilink: WikiLink,

    #[serde(serialize_with="serialize_vec_wikilink")]
    #[serde(deserialize_with="deserialize_vec_wikilink")]
    refered: Vec<WikiLink>,
}

fn serialize_wikilink<S: serde::Serializer>(wikilink: &WikiLink, s: S) -> Result<S::Ok, S::Error> {
    let mut serializer = s.serialize_struct("WikiLink", 3)?;
    serializer.serialize_field("namespace", &wikilink.namespace)?;
    serializer.serialize_field("type", &wikilink.wiki_type)?;
    serializer.serialize_field("name", &wikilink.name)?;
    serializer.end()
}

struct WikiLinkVistor;

const WIKI_LINK_FIELDS: &[&str; 3] = &["namespace", "type", "name"];

impl<'de> Visitor<'de> for WikiLinkVistor {
    type Value = WikiLink;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("struct WikiLink")
    }

    fn visit_map<V>(self, mut map: V) -> Result<WikiLink, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut namespace = None;
        let mut name = None;
        let mut wikitype: Option<WikiType> = None;
        while let Some(key) = map.next_key()? {
            match key {
                "namespace" => {
                    if namespace.is_some() {
                        return Err(de::Error::duplicate_field("namespace"));
                    }
                    namespace = Some(map.next_value()?);
                }
                "type" => {
                    if wikitype.is_some() {
                        return Err(de::Error::duplicate_field("type"));
                    }
                    wikitype =  map.next_value()?;
                }
                "name" => {
                    if name.is_some() {
                        return Err(de::Error::duplicate_field("name"));
                    }
                    name = Some(map.next_value()?);
                }
                &_ => return Err(de::Error::unknown_field(key, WIKI_LINK_FIELDS))
            }
        }
        let namespace: &str = namespace.ok_or_else(|| de::Error::missing_field("namespace"))?;
        let name: &str = name.ok_or_else(|| de::Error::missing_field("name"))?;
        let wikitype: WikiType = wikitype.ok_or_else(|| de::Error::missing_field("type"))?;
        Ok(WikiLink::new(namespace, wikitype, name))
    }
}

fn deserialize_wikilink<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<WikiLink, D::Error> {
    deserializer.deserialize_struct("WikiLink", WIKI_LINK_FIELDS, WikiLinkVistor)
}

fn serialize_vec_wikilink<S: serde::Serializer>(vec_wikilink: &Vec<WikiLink>, s: S) -> Result<S::Ok, S::Error> {
    #[derive(Serialize)]
    struct Wrapper (
        #[serde(serialize_with="serialize_wikilink")]
        WikiLink
    );
    let v: Vec<Wrapper> = vec_wikilink.into_iter().map(|w| Wrapper(WikiLink::new(&w.namespace, w.wiki_type, &w.name))).collect();
    v.serialize(s)
}

fn deserialize_vec_wikilink<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Vec<WikiLink>, D::Error> {
    #[derive(Deserialize)]
    struct Wrapper (
        #[serde(deserialize_with="deserialize_wikilink")]
        WikiLink
    );

    let v = Vec::deserialize(deserializer)?;
    Ok(v.into_iter().map(|Wrapper(w)| w).collect())
}

pub struct CategoryReference {
    categories: Vec<Category>,
}

impl CategoryReference {
    pub fn new() -> Self {
        Self { categories: Vec::new() }
    }

    pub fn read(root_dir: path::PathBuf) -> Result<Self, std::io::Error> {
        let mut file = fs::File::open(root_dir.join("categories.json"))?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        let categories: Vec<Category> = serde_json::from_str(&buffer)?;
        Ok(Self { categories })
    }

    pub fn save(&self, root_dir: path::PathBuf) -> Result<(), std::io::Error> {
        let path = root_dir.join("categories.json");
        let json_string = serde_json::to_string(&self.categories)?;
        let file = fs::File::create(path)?;
        write!(&file, "{}", json_string)?;
        Ok(())
    }
}

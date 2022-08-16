use std::{ fs, path };
use std::io::{ Write, Read };
use chrono::NaiveDateTime;
use serde::{ Serialize, Deserialize };

type HistoryVersionId = String;

#[derive(Serialize, Deserialize, Debug)]
struct CurrentReference {
    name: String,
    id: HistoryVersionId,
}

#[derive(Serialize, Deserialize, Debug)]
struct HistoryVersion {
    id: HistoryVersionId,
    name: String,
    version: i32,
    next: Option<HistoryVersionId>,
    prev: Option<HistoryVersionId>,
    #[serde(serialize_with = "serialize_naive_date_time")]
    #[serde(deserialize_with = "deserialize_naive_date_time")]
    created: NaiveDateTime,
    comment: String,
    filename: String,
}

const DATE_TIME_FORMAT: &str = "%Y/%m/%d %H:%M:%S";

fn serialize_naive_date_time<S: serde::Serializer>(date_time: &NaiveDateTime, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(&date_time.format(DATE_TIME_FORMAT).to_string())
}

fn deserialize_naive_date_time<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<NaiveDateTime, D::Error> {
    let buf = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&buf, DATE_TIME_FORMAT).map_err(serde::de::Error::custom)
}


#[derive(Debug)]
pub struct History {
    versions: Vec<HistoryVersion>,
    current: Vec<CurrentReference>,
}

impl History {
    pub fn new() -> Self {
        Self { versions: Vec::new(), current: Vec::new() }
    }

    pub fn read(root_dir: path::PathBuf) -> Result<Self, std::io::Error> {
        let mut current_file = fs::File::open(root_dir.join("current.json"))?;
        let mut history_file = fs::File::open(root_dir.join("history.json"))?;

        // read current
        let mut buffer = String::new();
        current_file.read_to_string(&mut buffer)?;
        let current: Vec<CurrentReference> = serde_json::from_str(&buffer)?;

        // read history
        let mut buffer = String::new();
        history_file.read_to_string(&mut buffer)?;
        let versions: Vec<HistoryVersion> = serde_json::from_str(&buffer)?;

        Ok(Self { current, versions })
    }

    pub fn save(&self, root_dir: path::PathBuf) -> Result<(), std::io::Error> {
        fs::create_dir_all(&root_dir).unwrap_or_else(|why| println!("{:?}", why.kind()));

        // save current.json
        let file = fs::File::create(root_dir.join("current.json"))?;
        let json_string = serde_json::to_string(&self.current)?;
        write!(&file, "{}", json_string)?;

        // save history.json
        let file = fs::File::create(root_dir.join("history.json"))?;
        let json_string = serde_json::to_string(&self.versions)?;
        write!(&file, "{}", json_string)?;

        Ok(())
    }
}

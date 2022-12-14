use std::{ fs, path };
use std::io::{ Write, Read };
use chrono::{ NaiveDateTime, offset::Utc };
use serde::{ Serialize, Deserialize };
use crate::utils::generate_random_string;


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

impl HistoryVersion {
    fn new(name: &str) -> Self {
        HistoryVersion {
            id: generate_random_string(16),
            name: name.to_string(),
            version: 1,
            next: None,
            prev: None,
            created: Utc::now().naive_utc(),
            comment: "".to_string(),
            filename: "".to_string(),
        }
    }

    fn next(version: &HistoryVersion) -> Self {
        Self {
            id: generate_random_string(16),
            name: version.name.to_owned(),
            version: version.version + 1,
            next: None,
            prev: Some(version.id.to_owned()),
            created: Utc::now().naive_utc(),
            comment: "".to_string(),
            filename: "".to_string(),
        }
    }

    fn relative_path(&self) -> path::PathBuf {
        path::Path::new(self.filename.get(..2).unwrap()).join(&self.filename)
    }
}


#[derive(Debug)]
pub struct History {
    root_dir: path::PathBuf,
    versions: Vec<HistoryVersion>,
    current: Vec<CurrentReference>,
}

impl History {
    pub fn new(root_dir: path::PathBuf) -> Self {
        Self { root_dir, versions: Vec::new(), current: Vec::new() }
    }

    pub fn add(&mut self, name: &str) {
        let current: Option<&mut CurrentReference> = self.current.iter_mut().find(|current| current.name == name);
        match current {
            None => {
                // New
                let history =  HistoryVersion::new(name);
                let current = CurrentReference {
                    id: history.id.to_owned(),
                    name: name.to_string(),
                };
                self.versions.push(history);
                self.current.push(current);
            },
            Some(current) => {
                let version: Option<&mut HistoryVersion> = self.versions.iter_mut().find(|version| version.id == current.id);
                match version {
                    Some(version) => {
                        // Update
                        let history = HistoryVersion::next(&version);
                        version.next = Some(history.id.to_owned());
                        current.id = history.id.to_owned();
                        self.versions.push(history);
                    }
                    None => {
                        unreachable!();
                    },
                }
            }
        }
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

        Ok(Self { root_dir, current, versions })
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let root_dir = &self.root_dir;
        fs::create_dir_all(&root_dir).unwrap_or_else(|why| panic!("{:?}", why.kind()));

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

    fn to_full_path(&self, version: &HistoryVersion) -> path::PathBuf {
        self.root_dir.join(version.relative_path())
    }

    fn get_current_reference(&self, name: &str) -> Option<&CurrentReference> {
        self.current.iter().find(|current| current.name == name)
    }

    fn get_current_history_version(&self, name: &str) -> Option<&HistoryVersion> {
        let current = self.get_current_reference(name)?;
        self.versions.iter().find(|version| version.id == current.id)
    }

    pub fn get_file_path_by_name(&self, name: impl Into<String>) -> Option<path::PathBuf> {
        let version = self.get_current_history_version(&name.into())?;
        Some(self.to_full_path(version))
    }

    fn rewind_history<'a>(&'a self, history: &'a HistoryVersion, version: i32) -> Option<&'a HistoryVersion> {
        if history.version == version {
            Some(history)
        } else {
            match &history.prev {
                None => None,
                Some(id) => self.rewind_history(self.versions.iter().find(|v| &v.id == id)?, version),
            }
        }
    }

    pub fn get_file_path_by_version(&self, name: impl Into<String>, version: i32) -> Option<path::PathBuf> {
        let current = self.get_current_history_version(&name.into())?;
        let history = self.rewind_history(current, version)?;
        Some(self.to_full_path(history))
    }
}

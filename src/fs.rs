use std::fmt::{self, Display};
use std::fs::{read_dir, DirEntry};

#[derive(Debug)]
pub struct Entity {
    path: String,
    name: String,
    is_dir: bool,
    children: Vec<Entity>,
}

impl Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "path: {}, name: {}, is_dir: {}",
            self.path, self.name, self.is_dir
        )
    }
}

impl Entity {
    fn new(entry: DirEntry) -> Entity {
        let name = entry.file_name().to_str().unwrap_or("").to_string();
        let path = entry.path().to_str().unwrap_or(".").to_string();
        let is_dir = match entry.file_type() {
            Ok(file_type) => file_type.is_dir(),
            Err(_) => false,
        };

        let children = match is_dir {
            true => parse_dir(&path),
            false => Vec::new(),
        };

        Entity {
            name,
            path: trim_path(path),
            is_dir,
            children,
        }
    }
}

pub fn parse_dir(path: &str) -> Vec<Entity> {
    let dir = read_dir(path).expect("couldn't read current dir");
    let mut entitites = Vec::new();

    for entry in dir.flatten() {
        entitites.push(Entity::new(entry));
    }

    entitites
}

fn trim_path(path: String) -> String {
    let mut segments = path.split('/').collect::<Vec<&str>>();
    if segments.len() >= 2 {
        segments.remove(segments.len() - 1);
    }

    segments.join("/")
}

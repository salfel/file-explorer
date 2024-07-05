use std::env;
use std::fmt::{self, Display};
use std::fs::read_dir;

#[derive(Debug)]
pub struct Entity {
    pub path: String,
    pub name: String,
    pub is_dir: bool,
    pub children: Vec<Entity>,
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
    pub fn new(name: String, path: String, is_dir: bool, children: bool) -> Entity {
        let children = if children {
            parse_children(&path)
        } else {
            Vec::new()
        };

        Entity {
            name,
            path,
            is_dir,
            children,
        }
    }

    pub fn populate_children(&mut self) {
        self.children = parse_children(&self.path);
    }
}

fn parse_children(path: &str) -> Vec<Entity> {
    let dir = read_dir(path).expect("couldn't read current dir");
    let mut entitites = Vec::new();

    for entry in dir.flatten() {
        let name = entry.file_name().to_str().unwrap_or("").to_string();
        let path = entry.path().to_str().unwrap_or(".").to_string();
        let is_dir = match entry.file_type() {
            Ok(file_type) => file_type.is_dir(),
            Err(_) => false,
        };

        entitites.push(Entity::new(name, path, is_dir, false));
    }

    entitites
}

pub fn get_current_entity() -> Entity {
    let path = env::current_dir()
        .expect("couldn't read current dir")
        .display()
        .to_string();

    let (name, _) = trim_path(&path);
    Entity::new(name, path, true, true)
}

pub fn trim_path(path: &str) -> (String, String) {
    let mut segments = path.split('/').collect::<Vec<&str>>();
    let mut name = String::new();
    if segments.len() >= 2 {
        name = segments.remove(segments.len() - 1).to_string();
    }

    (name, segments.join("/"))
}

use std::env;
use std::fmt::{self, Display};
use std::fs::read_dir;

#[derive(Debug)]
pub struct Entity {
    pub path: String,
    pub file_name: FileName,
    pub is_dir: bool,
    pub children: Vec<Entity>,
}

impl Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "path: {}, name: {}, is_dir: {}",
            self.path, self.file_name, self.is_dir
        )
    }
}

impl Entity {
    pub fn new(name: String, path: String, is_dir: bool, should_parse_children: bool) -> Entity {
        let mut entity = Entity {
            file_name: FileName::new(name),
            path,
            is_dir,
            children: Vec::new(),
        };

        if should_parse_children {
            entity.parse_relatives();
        }

        entity
    }

    pub fn parse_relatives(&mut self) {
        self.parse_children();
        self.parse_parent();
    }

    fn parse_children(&mut self) {
        let dir = read_dir(&self.path).expect("couldn't read current dir");
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

        self.children = entitites;
    }

    fn parse_parent(&mut self) {
        let (_, path) = trim_path(&self.path);

        let parent = Entity::new(String::from(".."), path, true, false);

        self.children.insert(0, parent);
    }
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

#[derive(Debug)]
pub struct FileName {
    pub name: String,
    pub extension: String,
}

impl Display for FileName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.extension.is_empty() {
            write!(f, "{}", self.name)
        } else {
            write!(f, "{}.{}", self.name, self.extension)
        }
    }
}

impl FileName {
    pub fn new(file_name: String) -> FileName {
        let mut segments = file_name
            .split('.')
            .map(|str| str.to_string())
            .collect::<Vec<String>>();

        let mut extension = String::new();

        if segments.len() >= 2 {
            extension = segments.remove(segments.len() - 1);
        }

        let name = segments.join(".");

        FileName { name, extension }
    }
}

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
    fn new(entry: DirEntry) -> Option<Entity> {
        let path = entry.path().to_str().unwrap_or("").to_string();
        let path = &Self::get_dir_from_path(path);

        let name = entry.file_name().to_str().unwrap_or("").to_string();
        let is_dir = match entry.file_type() {
            Ok(file_type) => file_type.is_dir(),
            Err(_) => return None,
        };

        let children = match is_dir {
            true => Self::parse_dir(format!("{}/{}", path, name)),
            false => Vec::new(),
        };

        Some(Entity {
            path: path.to_string(),
            name,
            is_dir,
            children,
        })
    }

    pub fn parse_dir(dir: String) -> Vec<Entity> {
        let dir = read_dir(dir).expect("couldn't read current directory");
        let mut entities = Vec::new();

        for entry in dir.flatten() {
            if let Some(entity) = Entity::new(entry) {
                entities.push(entity);
            }
        }

        entities
    }

    fn get_dir_from_path(path: String) -> String {
        let mut paths = path.split('/').collect::<Vec<&str>>();
        if paths.len() >= 2 {
            paths.remove(paths.len() - 1);
        }
        paths.join("/")
    }
}

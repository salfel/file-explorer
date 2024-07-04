use fs::Entity;

mod fs;

fn main() {
    for entity in Entity::parse_dir(String::from(".")) {
        println!("{}", entity);
    }
}

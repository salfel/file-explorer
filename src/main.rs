mod fs;

fn main() {
    for entity in fs::get_current_dir() {
        println!("{}", entity);
    }
}

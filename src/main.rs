use fs::parse_dir;

//mod cli;
mod fs;

fn main() {
    for entity in parse_dir(".") {
        println!("{}", entity);
    }
}

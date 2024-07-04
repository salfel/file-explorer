use cli::Writer;

mod cli;
mod fs;

fn main() {
    let mut writer = Writer::new();

    writer.start();
}

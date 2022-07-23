use std::{fs::File, io::Write, path::PathBuf};

fn main() {
    let file_data = include_bytes!("README.md");
    let mut target = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    target.pop();
    target.pop();
    target.push("my_file.md");
    dbg!(&target);
    File::options()
        .create(true)
        .write(true)
        .open(target)
        .unwrap()
        .write_all(file_data)
        .unwrap();
}

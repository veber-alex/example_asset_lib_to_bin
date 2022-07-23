use std::path::PathBuf;

fn main() {
    let mut file = PathBuf::from(env!("OUT_DIR"));
    file.pop();
    file.pop();
    file.push("my_file.md");
    let data = std::fs::read_to_string(&file).unwrap();
    println!("{data}");
}

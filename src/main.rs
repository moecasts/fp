use std::{fs::File, path::Path};

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    input: String,
}

fn main() {
    let cli = Cli::parse();
    let input = cli.input;
        
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let dict_path = root_dir.join("src/dict/xhyx.json");

    let f = File::open(dict_path).unwrap();
    let dict: serde_json::Value = serde_json::from_reader(f).unwrap();

    match dict[input].as_array() {
        Some(item) => {
            println!("{}", &item[0].as_str().unwrap());

            println!("===============================================");

            println!("拆　分：  {}", item[1].as_str().unwrap());
            println!("首　末：  {}  {}", item[2].as_str().unwrap(), item[3].as_str().unwrap());
            println!("编  码：  {}  {}", item[4].as_str().unwrap(), item[5].as_str().unwrap());
        },
        None => {
            println!("未收录");
        }
    }
}
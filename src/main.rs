use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    input: String,
}

fn main() {
    let cli = Cli::parse();
    let input = cli.input;
        
    let dict_bytes = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/dict/xhyx.json"));

    let dict: serde_json::Value = serde_json::from_slice(dict_bytes).unwrap();
    
    for ch in input.chars() {
        match dict[String::from(ch)].as_array() {
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
        println!("");
    }
}

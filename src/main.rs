use std::collections::HashMap;
use clap::Parser;

// TODO:
// - add support to output into file when -o / --output arugment specified

#[derive(Parser)]
struct Cli {
    input_url: url::Url,
}

fn main() {
    let cli = Cli::parse();
    if let Some(query) = cli.input_url.query() {
        let my_query: HashMap<String, String> = serde_qs::from_str(query).unwrap();
        let serialized = serde_json::to_string_pretty(&my_query).unwrap();
        println!("{serialized}");
    }
    else {
        println!("No query found");
    }
}

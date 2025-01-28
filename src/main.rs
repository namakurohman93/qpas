use std::collections::HashMap;
use url::Url;

// TODO:
// - get url from argument
// - print result in JSON format (space 2 / 4?)
// - add support to output into file when -o / --output arugment specified

fn main() {
    let url = Url::try_from("https://google.com?foo=bar&baz=320").unwrap();
    if let Some(query) = url.query() {
        let my_query: HashMap<String, String> = serde_qs::from_str(query).unwrap();
        println!("{my_query:?}");
    }
    else {
        println!("No query found");
    }
}

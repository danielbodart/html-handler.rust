extern crate scraper;
extern crate html_handler;

use std::env;
use html_handler::*;

fn main() {
    let mut argv = env::args();
    let filename = argv.nth(1).ok_or("Please provide a filename").unwrap();
    let document = doc(&filename).unwrap();
    let root = root(&document).unwrap();
    let elements = select(&root, "include");
    for element in elements {
        println!("{}", element.html());
    }
}
extern crate html_handler;

use std::env;
use std::path::Path;
use html_handler::*;

fn main() {
    let mut argv = env::args();
    let filename = argv.nth(1).ok_or("Please provide a filename").unwrap();
    let path = Path::new(&filename);
    let base_path = path.parent().unwrap();
    let document = doc(&path).unwrap();
    process_includes(base_path, &document);
}
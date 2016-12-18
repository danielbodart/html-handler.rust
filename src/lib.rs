extern crate scraper;
extern crate ego_tree;

use scraper::{Html, Selector};
use scraper::element_ref::ElementRef;
use std::fs::File;
use std::path::Path;
use std::io::{Read, Error};

pub fn doc(filename: &Path) -> Result<Html, Err> {
    let mut file = try!(File::open(filename));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents));
    Ok(Html::parse_document(&contents))
}

pub fn root<'a>(document: &'a Html) -> Option<ElementRef<'a>> {
    for node in document.tree.nodes().by_ref() {
        if let Some(element) = ElementRef::wrap(node) {
            return Some(element);
        }
    }
    None
}

pub fn select<'a>(parent: &'a ElementRef<'a>, query: &str) -> Vec<ElementRef<'a>> {
    let selector = Selector::parse(query).unwrap();

    let mut vec = Vec::new();
    vec.extend(parent.select(&selector));
    vec
}

pub trait TagProcessor {
    fn process<'a>(tag: &'a ElementRef<'a>, target:&'a ElementRef<'a>) -> ();
}

pub fn process_tag<'a>(base_path: &Path, parent: &'a ElementRef<'a>, tag_selector: &str) {
    for tag in &select(parent, tag_selector) {
        let uri = tag.value().attr("src").unwrap();
        let selector = tag.value().attr("selector").unwrap();
        println!("uri: {} selector:{}", uri, selector);
        let child_doc = doc(base_path.join(uri).as_path()).unwrap();
        let child_root = root(&child_doc).unwrap();
        let child_nodes = select(&child_root, selector);
        for child_node in child_nodes {
            process_tag(base_path, &child_node, tag_selector);
        }
    }
}

/*
    private void replace(Node old, List<Node> newNodes) {
        // Fix for siblingIndex not being correctly set by Lagarto
        Node parentNode = old.getParentNode();
        int index = list(parentNode.getChildNodes()).indexOf(old);
        parentNode.insertChild(newNodes.toArray(new Node[newNodes.size()]), index + 1);
        parentNode.removeChild(index);
    }
*/


#[derive(Debug)]
pub enum Err {
    Message(String),
}

impl From<Error> for Err {
    fn from(err: Error) -> Err {
        Err::Message(err.to_string())
    }
}

impl From<String> for Err {
    fn from(message: String) -> Err {
        Err::Message(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn it_works() {
        let path = Path::new("/home/dan/Projects/up-to-date/web/application/application.html");
        let base_path = path.parent().unwrap();
        let document = doc(&path).unwrap();
        let root = root(&document).unwrap();
        process_tag(base_path, &root, "include");
    }
}

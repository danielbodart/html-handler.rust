extern crate kuchiki;

use kuchiki::{NodeRef, NodeDataRef, ElementData};
use kuchiki::traits::*;
use std::path::Path;
use std::io::{Error};

pub fn doc(filename: &Path) -> Result<NodeRef, Error> {
    kuchiki::parse_html().from_utf8().from_file(&filename)
}

//pub trait TagProcessor {
//    fn process(tag: &NodeRef, target: &NodeRef) -> ();
//}

pub fn process_tag(base_path: &Path, parent: &NodeRef, tag_selector: &str, processor:&Fn(&NodeRef, &NodeRef)) {
    for tag in parent.select(tag_selector).unwrap() {
        let attributes = tag.attributes.borrow();
        let uri = attributes.get("src").unwrap();
        let selector = attributes.get("selector").unwrap();
        let child_doc = doc(base_path.join(uri).as_path()).unwrap();
        let child_nodes = child_doc.select(selector).unwrap().collect::<Vec<_>>();
        for child_node in &child_nodes {
            process_tag(base_path, &child_node.as_node(), tag_selector, processor);
            processor(tag.as_node(), &child_node.as_node())
        }
        replace(tag.as_node(), child_nodes);
    }
}

#[allow(unused_variables)]
pub fn process_includes(base_path: &Path, parent: &NodeRef) {
    process_tag(base_path, parent, "include", &|link, target| {
        // NO-OP
    });
}

#[allow(unused_variables)]
pub fn replace(old: &NodeRef, new_nodes: Vec<NodeDataRef<ElementData>>){
    for new_node in new_nodes {
//        old.insert_after(*new_node.as_node());
    }
}


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
        process_includes(base_path, &document);
    }
}

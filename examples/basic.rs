#![allow(unused)]
// use tree_formatter::{DisplayTree, ToDisplayTree};
use tree_formatter::*;

// fn main() {
//     println!("TODO")
// }

#[derive(Debug, Clone)]
struct DocumentSection {
    title: String,
    subsections: Vec<DocumentSection>,
}

impl DocumentSection {
    fn new(title: impl Into<String>, subsections: Vec<DocumentSection>) -> Self {
        Self {
            title: title.into(),
            subsections,
        }
    }
}

impl ToPrettyTree for DocumentSection {
    fn to_pretty_tree(&self) -> PrettyTree {
        let children = self.subsections
            .iter()
            .map(|x| x.to_pretty_tree())
            .collect::<Vec<_>>();
        if children.is_empty() {
            return PrettyTree::value(&self.title)
        }
        PrettyTree::branch_of( &self.title, &children )
    }
}


fn main() {
    // Create a sample document structure
    let section = DocumentSection::new(
        "Chapter 1: Introduction",
        vec![
            DocumentSection::new("Section 1.1: Overview", vec![]),
            DocumentSection::new(
                "Section 1.2: Background",
                vec![
                    DocumentSection::new("Subsection 1.2.1: History", vec![]),
                    DocumentSection::new("Subsection 1.2.2: Motivation", vec![]),
                ],
            ),
        ],
    );

    // Convert to a DisplayTree and pretty print
    section.print_pretty_tree();
}

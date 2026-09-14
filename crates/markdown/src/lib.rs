mod inline;
mod parser;
mod scanners;
mod types;

pub use parser::parse_markdown;
pub use types::{BlockNode, Document, HeadingLevel, InlineNode};

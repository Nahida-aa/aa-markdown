mod inline;
mod parser;
mod types;

pub use parser::parse_markdown;
pub use types::{BlockNode, Document, InlineNode};

use crate::types::InlineNode;

pub fn parse_inline(text: &str) -> Vec<InlineNode> {
    vec![InlineNode::Text {
        value: text.to_string(),
    }]
}

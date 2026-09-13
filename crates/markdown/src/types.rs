/// The root object with children: BlockNode[]
#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    // 由你决定字段是否公开以及内部如何保存
    pub children: Vec<BlockNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockNode {
    Paragraph { children: Vec<InlineNode> },
}

#[derive(Debug, Clone, PartialEq)] // ← 必须 derive 才能用 assert_eq!
pub enum InlineNode {
    Text { value: String },
}

/// The root object with children: BlockNode[]
#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    // 由你决定字段是否公开以及内部如何保存
    pub children: Vec<BlockNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockNode {
    Paragraph {
        children: Vec<InlineNode>,
    },
    Heading {
        level: HeadingLevel,
        children: Vec<InlineNode>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HeadingLevel {
    H1 = 1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidHeadingLevel(pub usize);

impl TryFrom<usize> for HeadingLevel {
    type Error = InvalidHeadingLevel;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::H1),
            2 => Ok(Self::H2),
            3 => Ok(Self::H3),
            4 => Ok(Self::H4),
            5 => Ok(Self::H5),
            6 => Ok(Self::H6),
            _ => Err(InvalidHeadingLevel(value)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)] // ← 必须 derive 才能用 assert_eq!
pub enum InlineNode {
    Text { value: String },
}

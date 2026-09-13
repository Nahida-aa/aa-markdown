use crate::{
    inline::parse_inline,
    types::{BlockNode, Document},
};

/// Parses a Markdown source string into a document.
pub fn parse_markdown(markdown: &str) -> Document {
    // split 在 Rust 里返回一个迭代器（惰性的，不会立刻分配），所以需要 .collect() 变成 Vec
    let lines: Vec<&str> = markdown.split('\n').collect();

    let mut buf: Vec<&str> = Vec::new();
    let mut children: Vec<BlockNode> = Vec::new();
    // 段落是可以有多行, 按照空行分段, 先按行切分, 然后再按空行分段
    // 先不管了 迭代 lines 再说
    for line in lines {
        // 如果发现了空行, 就要判断 是不是段落结束
        if line.trim().is_empty() {
            // 要判断段落是本身结束, 得有一个容器记录之前的行
            flush_paragraph(&mut buf, &mut children);
        } else {
            buf.push(line);
        }
    }
    flush_paragraph(&mut buf, &mut children);

    Document { children }
}

fn flush_paragraph(buf: &mut Vec<&str>, children: &mut Vec<BlockNode>) {
    if buf.is_empty() {
        return;
    }
    let text = buf.join("\n");
    buf.clear();
    children.push(BlockNode::Paragraph {
        children: parse_inline(&text),
    });
}

#[cfg(test)]
mod tests {
    use crate::types::InlineNode;

    use super::*;
    // 把上面的东西引进来
    fn para(s: &str) -> BlockNode {
        BlockNode::Paragraph {
            children: vec![InlineNode::Text { value: s.into() }],
        }
    }

    #[test]
    fn empty_input() {
        assert_eq!(parse_markdown(""), Document { children: vec![] });
    }

    #[test]
    fn single_paragraph() {
        assert_eq!(
            parse_markdown("hello"),
            Document {
                children: vec![para("hello")]
            },
        );
    }

    #[test]
    fn two_paragraphs() {
        assert_eq!(
            parse_markdown("a\n\nb"),
            Document {
                children: vec![para("a"), para("b")]
            },
        );
    }
}

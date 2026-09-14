use crate::{
    inline::parse_inline,
    scanners::{is_paragraph_interrupt, scan_atx_heading},
    types::{BlockNode, Document, HeadingLevel},
};

/// Parses a Markdown source string into a document.
pub fn parse_markdown(markdown: &str) -> Document {
    // split 在 Rust 里返回一个迭代器（惰性的，不会立刻分配），所以需要 .collect() 变成 Vec
    let lines: Vec<&str> = markdown.split('\n').collect();

    let mut children: Vec<BlockNode> = Vec::new();

    let mut i = 0;
    while i < lines.len() {
        let (node, next) = parse_block(&lines, i);
        if let Some(node) = node {
            children.push(node);
        }
        i = next;
    }
    Document { children }
}

fn parse_heading(line: &str, level: HeadingLevel) -> BlockNode {
    let hashes = level as usize;
    // 切片并去除前后空格
    let after = line[hashes..].trim_end();

    let content = if after.ends_with('#') {
        let without = after.trim_end_matches('#');
        // 只有去掉 # 后末尾是空白（或空了），才算闭合标记
        if without.is_empty() || without.ends_with([' ', '\t']) {
            without.trim_end()
        } else {
            after // # 紧贴正文，不是闭合标记
        }
    } else {
        after
    };

    let content = content.trim_start();

    BlockNode::Heading {
        level,
        children: parse_inline(content),
    }
}

/// 从 `lines[start]` 开始解析一个块。
/// 返回：(可选的节点, 下一个块的位置)。
/// 空行返回 `(None, start+1)` —— 跳过。
fn parse_block(lines: &[&str], start: usize) -> (Option<BlockNode>, usize) {
    let line = lines[start];

    // 空行：无节点，前进一行
    if line.trim().is_empty() {
        return (None, start + 1);
    }

    // 标题：单行块
    if let Some(level) = scan_atx_heading(line) {
        return (Some(parse_heading(line, level)), start + 1);
    }

    // 段落：多行块，自己消费到空行或中断
    let mut end = start + 1;
    while end < lines.len() && !lines[end].trim().is_empty() && !is_paragraph_interrupt(lines[end])
    {
        end += 1;
    }

    let text = lines[start..end].join("\n");
    (
        Some(BlockNode::Paragraph {
            children: parse_inline(&text),
        }),
        end,
    )
}

#[cfg(test)]
mod tests {
    use crate::types::InlineNode;

    use super::*;
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

    // 用于创建一个标题节点的辅助函数
    fn heading(level: HeadingLevel, text: &str) -> BlockNode {
        BlockNode::Heading {
            level,
            children: vec![InlineNode::Text { value: text.into() }],
        }
    }

    #[test]
    fn parses_h1() {
        assert_eq!(
            parse_markdown("# hello"),
            Document {
                children: vec![heading(HeadingLevel::H1, "hello")]
            },
        );
    }

    #[test]
    fn parses_h2() {
        assert_eq!(
            parse_markdown("## hello"),
            Document {
                children: vec![heading(HeadingLevel::H2, "hello")]
            },
        );
    }

    #[test]
    fn parses_empty_h2() {
        assert_eq!(
            parse_markdown("##"),
            Document {
                children: vec![heading(HeadingLevel::H2, "")]
            },
        );
    }

    #[test]
    fn accepts_multiple_spaces_after_heading_marker() {
        assert_eq!(
            parse_markdown("##    hello"),
            Document {
                children: vec![heading(HeadingLevel::H2, "hello")]
            },
        );
    }

    #[test]
    fn accepts_tab_after_heading_marker() {
        assert_eq!(
            parse_markdown("##\thello"),
            Document {
                children: vec![heading(HeadingLevel::H2, "hello")]
            },
        );
    }

    #[test]
    fn parses_h6() {
        assert_eq!(
            parse_markdown("###### hello"),
            Document {
                children: vec![heading(HeadingLevel::H6, "hello")]
            },
        );
    }

    #[test]
    fn seven_hashes_are_paragraph() {
        assert_eq!(
            parse_markdown("####### hello"),
            Document {
                children: vec![para("####### hello")]
            },
        );
    }

    #[test]
    fn missing_space_is_paragraph() {
        assert_eq!(
            parse_markdown("#hello"),
            Document {
                children: vec![para("#hello")]
            },
        );
    }

    #[test]
    fn keeps_block_order() {
        assert_eq!(
            parse_markdown("a\n# b\nc"),
            Document {
                children: vec![para("a"), heading(HeadingLevel::H1, "b"), para("c")]
            },
        );
    }
}

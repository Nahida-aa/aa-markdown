use aa_markdown::{BlockNode, HeadingLevel, InlineNode, parse_markdown};

#[test]
fn parses_through_public_api() {
    let document = parse_markdown("hello");

    assert_eq!(
        document.children,
        vec![BlockNode::Paragraph {
            children: vec![InlineNode::Text {
                value: "hello".into(),
            }],
        }]
    );
}

#[test]
fn exposes_heading_level_through_public_api() {
    let document = parse_markdown("# Hello");

    assert_eq!(
        document.children,
        vec![BlockNode::Heading {
            level: HeadingLevel::H1,
            children: vec![InlineNode::Text {
                value: "Hello".into(),
            }],
        }]
    );
}

#[test]
fn preserves_block_order_through_public_api() {
    let document = parse_markdown("before\n# Title\nafter");

    assert!(matches!(
        document.children.as_slice(),
        [
            BlockNode::Paragraph { .. },
            BlockNode::Heading {
                level: HeadingLevel::H1,
                ..
            },
            BlockNode::Paragraph { .. },
        ]
    ));
}

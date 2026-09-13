use aa_markdown::parse_markdown;

#[test]
fn parses_through_public_api() {
    let document = parse_markdown("hello");

    assert_eq!(document.children.len(), 1);
}

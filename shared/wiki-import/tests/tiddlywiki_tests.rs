use wiki_import::extract_tiddlers;

#[test]
fn test_extract_tiddler_with_text_attr() {
    let html = r#"<div title="MyPage" text="Hello world"></div>"#;
    let tiddlers = extract_tiddlers(html);
    assert_eq!(tiddlers.len(), 1);
    assert_eq!(tiddlers[0].0, "MyPage");
    assert_eq!(tiddlers[0].1, "Hello world");
}

#[test]
fn test_extract_multiple_tiddlers() {
    let html = r#"
        <div title="Page1" text="Content 1"></div>
        <div title="Page2" text="Content 2"></div>
    "#;
    let tiddlers = extract_tiddlers(html);
    assert_eq!(tiddlers.len(), 2);
}

#[test]
fn test_bold_conversion() {
    let html = r#"<div title="Test" text="''bold text''"></div>"#;
    let tiddlers = extract_tiddlers(html);
    assert_eq!(tiddlers[0].1, "**bold text**");
}

#[test]
fn test_italic_conversion() {
    let html = r#"<div title="Test" text="//italic//"></div>"#;
    let tiddlers = extract_tiddlers(html);
    assert_eq!(tiddlers[0].1, "*italic*");
}

#[test]
fn test_link_passthrough() {
    let html = r#"<div title="Test" text="[[PageName]]"></div>"#;
    let tiddlers = extract_tiddlers(html);
    assert_eq!(tiddlers[0].1, "[[PageName]]");
}

#[test]
fn test_link_order_reversal() {
    // TiddlyWiki: [[display|target]], our format: [[target|display]]
    let html = r#"<div title="Test" text="[[click here|MyPage]]"></div>"#;
    let tiddlers = extract_tiddlers(html);
    assert_eq!(tiddlers[0].1, "[[MyPage|click here]]");
}

#[test]
fn test_heading_conversion() {
    let html = r#"<div title="Test" text="!Top Level"></div>"#;
    let tiddlers = extract_tiddlers(html);
    assert_eq!(tiddlers[0].1, "# Top Level");
}

#[test]
fn test_html_entity_decoding() {
    let html = r#"<div title="Test" text="a &lt; b &amp; c &gt; d"></div>"#;
    let tiddlers = extract_tiddlers(html);
    assert_eq!(tiddlers[0].1, "a < b & c > d");
}

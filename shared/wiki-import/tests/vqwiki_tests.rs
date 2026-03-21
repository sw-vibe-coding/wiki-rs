use wiki_import::convert_vqwiki;

#[test]
fn test_headings() {
    assert_eq!(convert_vqwiki("!!!Top Level"), "# Top Level");
    assert_eq!(convert_vqwiki("!!Second Level"), "## Second Level");
    assert_eq!(convert_vqwiki("!Third Level"), "### Third Level");
}

#[test]
fn test_bold() {
    assert_eq!(convert_vqwiki("'''bold text'''"), "**bold text**");
}

#[test]
fn test_italic() {
    assert_eq!(convert_vqwiki("''italic text''"), "*italic text*");
}

#[test]
fn test_links() {
    assert_eq!(convert_vqwiki("[PageName]"), "[[PageName]]");
    assert_eq!(
        convert_vqwiki("[PageName|click here]"),
        "[[PageName|click here]]"
    );
}

#[test]
fn test_horizontal_rule() {
    assert_eq!(convert_vqwiki("----"), "---");
}

#[test]
fn test_unordered_list() {
    assert_eq!(convert_vqwiki("* item one"), "- item one");
}

#[test]
fn test_ordered_list() {
    assert_eq!(convert_vqwiki("# item one"), "1. item one");
}

#[test]
fn test_inline_code() {
    assert_eq!(convert_vqwiki("{{{some code}}}"), "`some code`");
}

#[test]
fn test_mixed_content() {
    let input = "!!!My Page\n\nSome '''bold''' and ''italic'' text.\n\n* [LinkOne]\n* [LinkTwo|Two]\n\n----\n\nDone.";
    let output = convert_vqwiki(input);
    assert!(output.contains("# My Page"));
    assert!(output.contains("**bold**"));
    assert!(output.contains("*italic*"));
    assert!(output.contains("[[LinkOne]]"));
    assert!(output.contains("[[LinkTwo|Two]]"));
    assert!(output.contains("---"));
}

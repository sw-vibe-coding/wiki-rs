use wiki_common::parser::render_wiki_content;

#[test]
fn test_simple_link() {
    let html = render_wiki_content("See [[MyPage]] for details");
    assert!(html.contains(r#"data-wiki-link="MyPage""#));
    assert!(html.contains(r#"href="/wiki/MyPage""#));
    assert!(html.contains(">MyPage</a>"));
}

#[test]
fn test_aliased_link() {
    let html = render_wiki_content("See [[MyPage|click here]]");
    assert!(html.contains(r#"data-wiki-link="MyPage""#));
    assert!(html.contains(">click here</a>"));
}

#[test]
fn test_plain_text_escaped() {
    let html = render_wiki_content("<script>alert('xss')</script>");
    assert!(!html.contains("<script>"));
}

#[test]
fn test_markdown_headings() {
    let html = render_wiki_content("# Hello\n\nSome text");
    assert!(html.contains("<h1>Hello</h1>"));
    assert!(html.contains("<p>Some text</p>"));
}

#[test]
fn test_markdown_bold_italic() {
    let html = render_wiki_content("**bold** and *italic*");
    assert!(html.contains("<strong>bold</strong>"));
    assert!(html.contains("<em>italic</em>"));
}

#[test]
fn test_markdown_list() {
    let html = render_wiki_content("- item one\n- item two");
    assert!(html.contains("<li>item one</li>"));
    assert!(html.contains("<li>item two</li>"));
}

#[test]
fn test_markdown_code_block() {
    let html = render_wiki_content("```\nlet x = 1;\n```");
    assert!(html.contains("<code>"));
    assert!(html.contains("let x = 1;"));
}

#[test]
fn test_wiki_links_inside_markdown() {
    let html = render_wiki_content("## Links\n\n- [[PageA]]\n- [[PageB|Page B]]");
    assert!(html.contains("<h2>Links</h2>"));
    assert!(html.contains(r#"data-wiki-link="PageA""#));
    assert!(html.contains(">Page B</a>"));
}

#[test]
fn test_wiki_links_not_expanded_in_backticks() {
    let html = render_wiki_content("Use `[[PageName]]` syntax");
    assert!(!html.contains("data-wiki-link"));
    assert!(html.contains("[[PageName]]"));
}

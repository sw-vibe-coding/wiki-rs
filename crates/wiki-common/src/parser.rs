/// Render wiki content to HTML.
///
/// First expands `[[PageName]]` and `[[PageName|Display Text]]` into HTML links,
/// then renders the result as Markdown via pulldown-cmark.
pub fn render_wiki_content(raw: &str) -> String {
    let with_links = expand_wiki_links(raw);
    markdown_to_html(&with_links)
}

/// Replace `[[Target]]` and `[[Target|Display]]` with HTML anchor tags.
/// These survive Markdown rendering because pulldown-cmark passes through raw HTML.
fn expand_wiki_links(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();
    let mut in_backtick = false;

    while let Some(c) = chars.next() {
        if c == '`' {
            in_backtick = !in_backtick;
            result.push(c);
        } else if !in_backtick && c == '[' && chars.peek() == Some(&'[') {
            chars.next(); // consume second '['
            let mut link_content = String::new();
            let mut closed = false;
            while let Some(lc) = chars.next() {
                if lc == ']' && chars.peek() == Some(&']') {
                    chars.next(); // consume second ']'
                    closed = true;
                    break;
                }
                link_content.push(lc);
            }
            if closed && !link_content.is_empty() {
                let (target, display) = if let Some(pos) = link_content.find('|') {
                    (&link_content[..pos], &link_content[pos + 1..])
                } else {
                    (link_content.as_str(), link_content.as_str())
                };
                result.push_str(&format!(
                    r#"<a class="wiki-link" data-wiki-link="{target}" href="/wiki/{target}">{display}</a>"#
                ));
            } else {
                // Unclosed link, output literally
                result.push_str("[[");
                result.push_str(&link_content);
            }
        } else {
            result.push(c);
        }
    }

    result
}

fn markdown_to_html(input: &str) -> String {
    use pulldown_cmark::Event;

    let parser = pulldown_cmark::Parser::new(input);

    // Filter out raw HTML *except* our own wiki-link anchors.
    // Wiki links use <a class="wiki-link" ...> which we explicitly generate.
    let filtered = parser.filter(|event| match event {
        Event::Html(html) | Event::InlineHtml(html) => {
            html.contains("class=\"wiki-link\"") || html.contains("</a>")
        }
        _ => true,
    });

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, filtered);
    html_output
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

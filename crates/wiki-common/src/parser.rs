/// Render wiki content to HTML.
/// Converts `[[PageName]]` and `[[PageName|Display Text]]` to links.
/// Everything else is plain text (formatting comes later).
pub fn render_wiki_content(raw: &str) -> String {
    let mut result = String::new();
    let mut chars = raw.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '[' && chars.peek() == Some(&'[') {
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
        } else if c == '\n' {
            result.push_str("<br>");
        } else {
            // Escape HTML
            match c {
                '<' => result.push_str("&lt;"),
                '>' => result.push_str("&gt;"),
                '&' => result.push_str("&amp;"),
                '"' => result.push_str("&quot;"),
                _ => result.push(c),
            }
        }
    }

    result
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
        assert!(html.contains("&lt;script&gt;"));
    }
}

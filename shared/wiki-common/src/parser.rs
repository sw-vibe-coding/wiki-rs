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
                result.push_str("<a class=\"wiki-link\" data-wiki-link=\"");
                result.push_str(target);
                result.push_str("\" href=\"#/wiki/");
                result.push_str(target);
                result.push_str("\">");
                result.push_str(display);
                result.push_str("</a>");
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

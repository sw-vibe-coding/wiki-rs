mod inline;
mod markup;

/// Extract tiddlers from a TiddlyWiki HTML file and convert to wiki pages.
///
/// TiddlyWiki stores tiddlers as div elements with title and text attributes:
/// `<div title="PageName" text="content...">`
/// or newer format with content inside the div.
pub fn extract_tiddlers(html: &str) -> Vec<(String, String)> {
    let mut tiddlers = Vec::new();
    for div_content in find_tiddler_divs(html) {
        if let Some((title, text)) = parse_tiddler_div(&div_content) {
            let markdown = markup::convert(&text);
            tiddlers.push((title, markdown));
        }
    }
    tiddlers
}

fn find_tiddler_divs(html: &str) -> Vec<String> {
    let mut divs = Vec::new();
    let mut search_from = 0;
    while let Some(start) = html[search_from..].find("<div title=\"") {
        let abs_start = search_from + start;
        if let Some(end) = html[abs_start..].find("</div>") {
            let abs_end = abs_start + end + 6;
            divs.push(html[abs_start..abs_end].to_string());
            search_from = abs_end;
        } else {
            break;
        }
    }
    divs
}

fn parse_tiddler_div(div: &str) -> Option<(String, String)> {
    let title = extract_attr(div, "title")?;
    // Try text attribute first (classic format)
    if let Some(text) = extract_attr(div, "text") {
        return Some((title, text));
    }
    // Fallback: content between > and </div>
    let content_start = div.find('>')? + 1;
    let content_end = div.rfind("</div>")?;
    if content_start < content_end {
        let inner = div[content_start..content_end].trim();
        if !inner.is_empty() {
            return Some((title, inner.to_string()));
        }
    }
    None
}

fn extract_attr(html: &str, attr: &str) -> Option<String> {
    let needle = format!("{attr}=\"");
    let start = html.find(&needle)? + needle.len();
    let end = html[start..].find('"')? + start;
    Some(
        html[start..end]
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&amp;", "&"),
    )
}

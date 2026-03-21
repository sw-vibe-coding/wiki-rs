use super::inline;

/// Convert TiddlyWiki markup to Markdown.
///
/// TiddlyWiki syntax:
/// - `!` / `!!` / `!!!` -> `#` / `##` / `###` (same direction as Markdown)
/// - `''bold''` -> `**bold**`
/// - `//italic//` -> `*italic*`
/// - `[[PageName]]` -> `[[PageName]]` (already wiki-link format)
/// - `[[display|PageName]]` -> `[[PageName|display]]` (reversed order!)
/// - `{{{code}}}` -> `` `code` ``
/// - `----` -> `---`
/// - `* item` -> `- item`
/// - `# item` -> `1. item`
pub fn convert(input: &str) -> String {
    input
        .lines()
        .map(convert_line)
        .collect::<Vec<_>>()
        .join("\n")
}

fn convert_line(line: &str) -> String {
    let trimmed = line.trim_start();
    let converted = convert_heading(trimmed)
        .or_else(|| convert_rule(trimmed))
        .or_else(|| convert_list(trimmed))
        .unwrap_or_else(|| trimmed.to_string());
    convert_inline(&converted)
}

fn convert_heading(line: &str) -> Option<String> {
    // TiddlyWiki: ! = h1, !! = h2, !!! = h3 (same as Markdown)
    line.strip_prefix("!!!")
        .map(|r| format!("### {}", r.trim()))
        .or_else(|| line.strip_prefix("!!").map(|r| format!("## {}", r.trim())))
        .or_else(|| line.strip_prefix('!').map(|r| format!("# {}", r.trim())))
}

fn convert_rule(line: &str) -> Option<String> {
    if line.starts_with("----") {
        Some("---".to_string())
    } else {
        None
    }
}

fn convert_list(line: &str) -> Option<String> {
    line.strip_prefix("* ")
        .map(|r| format!("- {r}"))
        .or_else(|| line.strip_prefix("# ").map(|r| format!("1. {r}")))
}

fn convert_inline(input: &str) -> String {
    let result = inline::convert_bold(input);
    let result = inline::convert_italic(&result);
    let result = inline::convert_links(&result);
    inline::convert_code(&result)
}

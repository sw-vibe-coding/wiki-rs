mod inline;

/// Convert VQWiki markup to Markdown with wiki links.
pub fn convert_vqwiki(input: &str) -> String {
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
    inline::convert(&converted)
}

fn convert_heading(line: &str) -> Option<String> {
    line.strip_prefix("!!!")
        .map(|r| format!("# {}", r.trim()))
        .or_else(|| line.strip_prefix("!!").map(|r| format!("## {}", r.trim())))
        .or_else(|| line.strip_prefix('!').map(|r| format!("### {}", r.trim())))
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

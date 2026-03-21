/// Convert ''bold'' to **bold**
pub fn convert_bold(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\'' && chars.peek() == Some(&'\'') {
            chars.next();
            let text = take_until_double_quote(&mut chars);
            result.push_str(&format!("**{text}**"));
        } else {
            result.push(c);
        }
    }
    result
}

/// Convert //italic// to *italic*
pub fn convert_italic(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '/' && chars.peek() == Some(&'/') {
            chars.next();
            let text = take_until_double_slash(&mut chars);
            result.push_str(&format!("*{text}*"));
        } else {
            result.push(c);
        }
    }
    result
}

/// Convert [[display|Target]] to [[Target|display]] (TiddlyWiki reverses order)
/// Pass through [[Target]] unchanged.
pub fn convert_links(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '[' && chars.peek() == Some(&'[') {
            chars.next();
            let content = take_until_double_bracket(&mut chars);
            if let Some(pos) = content.find('|') {
                let display = &content[..pos];
                let target = &content[pos + 1..];
                result.push_str(&format!("[[{target}|{display}]]"));
            } else {
                result.push_str(&format!("[[{content}]]"));
            }
        } else {
            result.push(c);
        }
    }
    result
}

pub fn convert_code(input: &str) -> String {
    input.replace("{{{", "`").replace("}}}", "`")
}

fn take_until_double_quote(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
    let mut buf = String::new();
    while let Some(c) = chars.next() {
        if c == '\'' && chars.peek() == Some(&'\'') {
            chars.next();
            return buf;
        }
        buf.push(c);
    }
    buf
}

fn take_until_double_slash(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
    let mut buf = String::new();
    while let Some(c) = chars.next() {
        if c == '/' && chars.peek() == Some(&'/') {
            chars.next();
            return buf;
        }
        buf.push(c);
    }
    buf
}

fn take_until_double_bracket(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
    let mut buf = String::new();
    while let Some(c) = chars.next() {
        if c == ']' && chars.peek() == Some(&']') {
            chars.next();
            return buf;
        }
        buf.push(c);
    }
    buf
}

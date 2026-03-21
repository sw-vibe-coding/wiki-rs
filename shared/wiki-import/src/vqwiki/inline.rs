pub fn convert(input: &str) -> String {
    let result = convert_bold_italic(input);
    let result = convert_links(&result);
    convert_code(&result)
}

fn convert_bold_italic(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\'' && chars.peek() == Some(&'\'') {
            chars.next();
            if chars.peek() == Some(&'\'') {
                chars.next();
                let text = take_until_seq(&mut chars, "'''");
                result.push_str(&format!("**{text}**"));
            } else {
                let text = take_until_seq(&mut chars, "''");
                result.push_str(&format!("*{text}*"));
            }
        } else {
            result.push(c);
        }
    }
    result
}

fn convert_links(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '[' && chars.peek() != Some(&'[') {
            let content = take_until_char(&mut chars, ']');
            if !content.is_empty() {
                result.push_str(&format!("[[{content}]]"));
            } else {
                result.push('[');
            }
        } else {
            result.push(c);
        }
    }
    result
}

fn convert_code(input: &str) -> String {
    input.replace("{{{", "`").replace("}}}", "`")
}

fn take_until_seq(chars: &mut std::iter::Peekable<std::str::Chars>, end: &str) -> String {
    let mut buf = String::new();
    let end_str: String = end.chars().collect();
    while let Some(&c) = chars.peek() {
        buf.push(c);
        chars.next();
        if buf.ends_with(&end_str) {
            buf.truncate(buf.len() - end_str.len());
            return buf;
        }
    }
    buf
}

fn take_until_char(chars: &mut std::iter::Peekable<std::str::Chars>, end: char) -> String {
    let mut buf = String::new();
    for c in chars.by_ref() {
        if c == end {
            return buf;
        }
        buf.push(c);
    }
    buf
}

use wiki_ui::theme::{sub_wiki_prefix, theme_css_vars, theme_for_page};

#[test]
fn prefix_extracts_before_slash() {
    assert_eq!(sub_wiki_prefix("Tech/Rust"), Some("Tech"));
    assert_eq!(sub_wiki_prefix("Nature/Birds"), Some("Nature"));
}

#[test]
fn prefix_none_without_slash() {
    assert_eq!(sub_wiki_prefix("MainPage"), None);
    assert_eq!(sub_wiki_prefix("SimplePage"), None);
}

#[test]
fn prefix_none_for_edge_cases() {
    assert_eq!(sub_wiki_prefix("/LeadingSlash"), None);
    assert_eq!(sub_wiki_prefix("TrailingSlash/"), None);
}

#[test]
fn known_prefix_returns_themed() {
    let theme = theme_for_page("Tech/Rust");
    assert_eq!(theme.name, "ocean");
}

#[test]
fn unknown_prefix_returns_default() {
    let theme = theme_for_page("Random/Page");
    assert_eq!(theme.name, "default");
}

#[test]
fn no_prefix_returns_default() {
    let theme = theme_for_page("MainPage");
    assert_eq!(theme.name, "default");
}

#[test]
fn css_vars_contain_all_properties() {
    let theme = theme_for_page("Nature/Trees");
    let vars = theme_css_vars(theme);
    assert!(vars.contains("--wiki-bg:"));
    assert!(vars.contains("--wiki-accent:"));
    assert!(vars.contains("--wiki-border:"));
    assert!(vars.contains("--wiki-nav-bg:"));
}

#[test]
fn prefix_case_insensitive() {
    let theme = theme_for_page("tech/lowercase");
    assert_eq!(theme.name, "ocean");
}

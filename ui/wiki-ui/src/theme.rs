/// A visual theme applied via CSS custom properties.
pub struct Theme {
    pub name: &'static str,
    pub bg: &'static str,
    pub accent: &'static str,
    pub border: &'static str,
    pub nav_bg: &'static str,
}

const DEFAULT: Theme = Theme {
    name: "default",
    bg: "#ffffff",
    accent: "#0645ad",
    border: "#cccccc",
    nav_bg: "#f8f8f8",
};

const FOREST: Theme = Theme {
    name: "forest",
    bg: "#f4f9f4",
    accent: "#2d6a2e",
    border: "#a3c4a3",
    nav_bg: "#e8f0e8",
};

const OCEAN: Theme = Theme {
    name: "ocean",
    bg: "#f0f6fa",
    accent: "#1a5276",
    border: "#a0c4de",
    nav_bg: "#e0eef5",
};

const SUNSET: Theme = Theme {
    name: "sunset",
    bg: "#fef6f0",
    accent: "#b45309",
    border: "#e0c4a8",
    nav_bg: "#f8ede0",
};

const LAVENDER: Theme = Theme {
    name: "lavender",
    bg: "#f6f0fa",
    accent: "#6b3fa0",
    border: "#c4a8de",
    nav_bg: "#eee0f5",
};

static THEMES: &[(&str, &Theme)] = &[
    ("Tech", &OCEAN),
    ("Nature", &FOREST),
    ("Cooking", &SUNSET),
    ("Art", &LAVENDER),
];

/// Extract the sub-wiki prefix from a page title (text before first `/`).
pub fn sub_wiki_prefix(title: &str) -> Option<&str> {
    let slash = title.find('/')?;
    if slash > 0 && slash < title.len() - 1 {
        Some(&title[..slash])
    } else {
        None
    }
}

/// Look up the theme for a given page title based on its sub-wiki prefix.
pub fn theme_for_page(title: &str) -> &'static Theme {
    let Some(prefix) = sub_wiki_prefix(title) else {
        return &DEFAULT;
    };
    THEMES
        .iter()
        .find(|(p, _)| p.eq_ignore_ascii_case(prefix))
        .map(|(_, t)| *t)
        .unwrap_or(&DEFAULT)
}

/// Generate an inline CSS `style` string that sets custom properties for the theme.
pub fn theme_css_vars(theme: &Theme) -> String {
    format!(
        "--wiki-bg:{};--wiki-accent:{};--wiki-border:{};--wiki-nav-bg:{}",
        theme.bg, theme.accent, theme.border, theme.nav_bg,
    )
}

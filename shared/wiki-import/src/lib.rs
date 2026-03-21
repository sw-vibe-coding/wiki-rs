mod vqwiki;

pub use vqwiki::convert_vqwiki;

/// Supported import formats.
pub enum ImportFormat {
    VqWiki,
}

/// Convert content from an external wiki format to Markdown + wiki links.
pub fn convert(content: &str, format: ImportFormat) -> String {
    match format {
        ImportFormat::VqWiki => convert_vqwiki(content),
    }
}

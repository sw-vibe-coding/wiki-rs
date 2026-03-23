use clap::Parser;
use std::path::PathBuf;

const VERSION_INFO: &str = "\
wiki-import-cli 0.1.0
Copyright (c) 2026 Software Wrighter LLC
License: See LICENSE file
Repository: https://github.com/sw-vibe-coding/wiki-rs
Build Host: local
Build Commit: dev
Build Time: dev";

#[derive(Parser)]
#[command(
    name = "wiki-import-cli",
    version = VERSION_INFO,
    about = "Import VQWiki or TiddlyWiki content into Markdown wiki pages",
    long_about = "Import VQWiki or TiddlyWiki content into Markdown wiki pages.\n\n\
        AI CODING AGENT INSTRUCTIONS:\n\n\
        Converts legacy wiki formats to markdown files:\n\
        - vqwiki: reads .txt files from --input directory\n\
        - tiddlywiki: reads a single .html file from --input\n\n\
        Output directory will contain one .md file per page.\n\n\
        Examples:\n\
          wiki-import-cli --format vqwiki --input ./old-wiki/ --output ./data/pages/\n\
          wiki-import-cli --format tiddlywiki --input wiki.html --output ./data/pages/"
)]
struct Cli {
    /// Import format
    #[arg(long, value_enum)]
    format: Format,

    /// Input path (directory for vqwiki, file for tiddlywiki)
    #[arg(long)]
    input: PathBuf,

    /// Output directory for converted .md files
    #[arg(long)]
    output: PathBuf,
}

#[derive(Clone, clap::ValueEnum)]
enum Format {
    Vqwiki,
    Tiddlywiki,
}

fn main() {
    let cli = Cli::parse();
    std::fs::create_dir_all(&cli.output).expect("failed to create output directory");

    let count = match cli.format {
        Format::Vqwiki => import_vqwiki(&cli.input, &cli.output),
        Format::Tiddlywiki => import_tiddlywiki(&cli.input, &cli.output),
    };

    println!("Imported {count} pages to {}", cli.output.display());
}

fn import_vqwiki(input_dir: &std::path::Path, output_dir: &std::path::Path) -> usize {
    let entries = std::fs::read_dir(input_dir).expect("failed to read input directory");
    let mut count = 0;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "txt") {
            let content = std::fs::read_to_string(&path).expect("failed to read file");
            let markdown = wiki_import::convert_vqwiki(&content);
            let title = path.file_stem().unwrap().to_string_lossy();
            let out_path = output_dir.join(format!("{title}.md"));
            std::fs::write(&out_path, &markdown).expect("failed to write output");
            println!("  {title}.txt -> {title}.md");
            count += 1;
        }
    }
    count
}

fn import_tiddlywiki(input_file: &std::path::Path, output_dir: &std::path::Path) -> usize {
    let html = std::fs::read_to_string(input_file).expect("failed to read TiddlyWiki file");
    let tiddlers = wiki_import::extract_tiddlers(&html);
    for (title, markdown) in &tiddlers {
        let safe_title: String = title
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
            .collect();
        let out_path = output_dir.join(format!("{safe_title}.md"));
        std::fs::write(&out_path, markdown).expect("failed to write output");
        println!("  {title} -> {safe_title}.md");
    }
    tiddlers.len()
}

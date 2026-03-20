# Wiki-RS

An educational Rust/Yew/WASM wiki application demonstrating how wiki
storage architectures have evolved -- from flat files to databases to
browser-based storage.

Each storage backend runs as its own independent WASM application on a
dedicated port. All backends share the same UI components and wiki engine.

## Wiki Variants

| Variant | Port | Storage | Persistence | Docs |
|---------|------|---------|-------------|------|
| [Ephemeral](docs/wiki-ephemeral.md) | 7408 | In-memory HashMap | None (lost on refresh) | [docs/wiki-ephemeral.md](docs/wiki-ephemeral.md) |
| [Browser Memory](docs/wiki-browser-memory.md) | 7409 | localStorage | Survives refreshes | [docs/wiki-browser-memory.md](docs/wiki-browser-memory.md) |
| [Export/Import File](docs/wiki-export-file.md) | 7407 | In-memory + JSON file | Manual export/import | [docs/wiki-export-file.md](docs/wiki-export-file.md) |
| Server File | 7400 | Axum + flat files | Server-side files | Planned |
| Server DB | 7401 | Axum + SQLite | Server-side database | Planned |
| Server Git | 7402 | Axum + git repo | Git commits | Planned |

## Quick Start

```bash
# Install Rust and the WASM target
rustup target add wasm32-unknown-unknown

# Install Trunk (WASM build tool)
cargo install trunk

# Run any variant
cd crates/wiki-ephemeral && trunk serve    # http://127.0.0.1:7408/
cd crates/wiki-browser-memory && trunk serve  # http://127.0.0.1:7409/
cd crates/wiki-export-file && trunk serve  # http://127.0.0.1:7407/
```

## Features

- **Wiki links:** `[[PageName]]` and `[[PageName|display text]]`
- **Markdown:** Headings, bold, italic, code blocks, lists (via pulldown-cmark)
- **Red links:** Nonexistent pages show as red links (classic wiki behavior)
- **Create on click:** Clicking a red link opens the editor for that page
- **XSS protection:** Raw HTML is filtered; wiki links inside backticks are not expanded

## Architecture

```
crates/
  wiki-common/        Shared types, parser, WikiStorage trait
  wiki-ui/            Shared Yew components (PageView, PageEdit, PageList, Nav)
  wiki-ephemeral/     Port 7408 - in-memory, lost on refresh
  wiki-browser-memory/ Port 7409 - localStorage persistence
  wiki-export-file/   Port 7407 - JSON file export/import
```

Each frontend crate is a thin wrapper (~30 lines) that implements the
`WikiStorage` trait and calls `wiki_ui::app::render_wiki()`.

## Historical Context

This project is inspired by the evolution of wiki engines:

- **WikiWikiWeb** (1995) -- flat file storage, the first wiki
- **TiKi** (Ruby, ~2002) -- early dynamic wiki with CGI
- **VQWiki** (Java, ~2001) -- servlet-based with hybrid storage
- **TiddlyWiki** (2004) -- entire wiki in a single HTML file
- **GitHub Wikis** -- git-backed markdown storage

See [docs/research.txt](docs/research.txt) for the full historical research.

## License

See [LICENSE](LICENSE) and [COPYRIGHT](COPYRIGHT).

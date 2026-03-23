# Wiki-RS

A Rust-based wiki system with 6 storage backends, built with Yew (WASM)
and Axum. Demonstrates how wiki storage architectures have evolved --
from flat files to databases to browser-based storage.

**[Live Demo](https://sw-vibe-coding.github.io/wiki-rs/)** — try the
client-side wikis in your browser, no installation needed.

## Screenshots

| Landing Page | Ephemeral Wiki | Browser Memory Wiki |
|--------------|---------------|---------------------|
| ![Landing](docs/images/live-demo.png?ts=1774221179362) | ![Ephemeral](docs/images/live-ephemeral.png?ts=1774221179362) | ![Browser Memory](docs/images/live-browser-memory.png?ts=1774221179362) |

| Main Page | Editor | Red Links |
|-----------|--------|-----------|
| ![Main Page](docs/images/main-page.png?ts=1774221179362) | ![Editor](docs/images/editing.png?ts=1774221179362) | ![Red Links](docs/images/red-link.png?ts=1774221179362) |

## Wiki Variants

| Variant | Port | Storage | Live |
|---------|------|---------|------|
| [Ephemeral](docs/wiki-ephemeral.md) | 7408 | In-memory HashMap | [Try it](https://sw-vibe-coding.github.io/wiki-rs/ephemeral/) |
| [Browser Memory](docs/wiki-browser-memory.md) | 7409 | localStorage | [Try it](https://sw-vibe-coding.github.io/wiki-rs/browser-memory/) |
| [Export/Import File](docs/wiki-export-file.md) | 7407 | JSON file export/import | [Try it](https://sw-vibe-coding.github.io/wiki-rs/export-file/) |
| Server File | 7400 | Axum + flat `.md` files | Local only |
| Server DB | 7401 | Axum + SQLite | Local only |
| Server Git | 7402 | Axum + git commits | Local only |

## Quick Start

```bash
# Install Rust and the WASM target
rustup target add wasm32-unknown-unknown
cargo install trunk

# Client-side wikis
cd frontend/wiki-ephemeral && trunk serve       # http://127.0.0.1:7408/
cd frontend/wiki-browser-memory && trunk serve  # http://127.0.0.1:7409/
cd frontend/wiki-export-file && trunk serve     # http://127.0.0.1:7407/

# Server-backed wikis (build WASM frontend first)
cd frontend/wiki-server-ui && trunk build
cargo run -p wiki-server -- --backend file --data-dir ./data/file  # port 7400
cargo run -p wiki-server -- --backend db   --data-dir ./data/db    # port 7401
cargo run -p wiki-server -- --backend git  --data-dir ./work/git   # port 7402
```

## Features

- **Wiki links:** `[[PageName]]` and `[[PageName|display text]]`
- **Markdown:** Headings, bold, italic, code blocks, lists (via pulldown-cmark)
- **Red links:** Nonexistent pages show as red links (classic wiki behavior)
- **Create on click:** Clicking a red link opens the editor for that page
- **XSS protection:** Raw HTML is filtered; wiki links inside backticks are not expanded
- **Page aging:** 5 visual tiers (Fresh to Ancient) based on last edit time
- **Sub-wiki theming:** CSS custom properties with 5 color themes by page prefix
- **Import/conversion:** VQWiki and TiddlyWiki markup converters
- **50 integration tests** across unit, API, and browser tests

## Architecture

```
shared/              2 crates
  wiki-common/         model, parser, storage traits, aging, time
  wiki-import/         VQWiki + TiddlyWiki importers

ui/                  2 crates
  wiki-ui/             shared Yew components + theming
  wiki-ui-app/         App shell, routing, render_wiki()

frontend/            4 crates
  wiki-ephemeral/      port 7408
  wiki-browser-memory/ port 7409
  wiki-export-file/    port 7407
  wiki-server-ui/      REST client (shared by all server backends)

backend/             3 crates
  wiki-server/         Axum REST server + CLI
  wiki-server-db/      SQLite backend
  wiki-server-git/     git backend
```

Each frontend crate is a thin wrapper (~30 lines) that implements the
`WikiStorage` trait and calls `render_wiki()`.

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

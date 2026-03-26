# Wiki-RS Project Status

Last updated: 2026-03-26

## Completed Features

### 1. Six Storage Backends (all working)

| Variant | Port | Crate | Storage | Status |
|---------|------|-------|---------|--------|
| Ephemeral | 7408 | frontend/wiki-ephemeral | In-memory HashMap | Done |
| Browser Memory | 7409 | frontend/wiki-browser-memory | localStorage | Done |
| Export/Import File | 7407 | frontend/wiki-export-file | JSON download/upload | Done |
| Server File | 7400 | backend/wiki-server --backend file | Flat .md files | Done |
| Server DB | 7401 | backend/wiki-server --backend db | SQLite (rusqlite) | Done |
| Server Git | 7402 | backend/wiki-server --backend git | git commits (git2) | Done |

### 2. Wiki Engine Features

- Markdown rendering via pulldown-cmark (headings, bold, italic, lists, code)
- Wiki links: `[[PageName]]` and `[[PageName|display text]]`
- Red links for nonexistent pages (classic wiki behavior)
- Click red link to create page
- XSS protection (raw HTML filtered, wiki links in backticks not expanded)
- Page delete (all pages except MainPage)
- All Pages index

### 3. Timestamps (created_at / updated_at)

- WikiPage model has created_at and updated_at (u64 Unix seconds)
- Platform-aware time::now() (js_sys for WASM, SystemTime for native)
- Shared time::file_mtime() for file/git backends
- DB backend stores/retrieves timestamps in schema
- Page editor preserves created_at on updates

### 4. Import/Conversion

- VQWiki markup converter (shared/wiki-import)
  - Headings, bold, italic, links, lists, code, horizontal rules
  - 9 integration tests
- TiddlyWiki importer (shared/wiki-import)
  - HTML tiddler extraction from TiddlyWiki files
  - Markup conversion (bold, italic, links with order reversal, headings)
  - HTML entity decoding
  - 8 integration tests

### 5. Page Aging Visual Effects

- 5 age tiers: Fresh, Recent, Stale, Old, Ancient
- CSS effects: yellowing, parchment gradients, folded corners, stain effects
- Based on updated_at timestamp
- 7 integration tests

### 6. Theming System

- CSS custom properties: --wiki-bg, --wiki-accent, --wiki-border, --wiki-nav-bg
- 5 built-in themes: Default, Forest, Ocean, Sunset, Lavender
- Sub-wiki detection by page title prefix (e.g., Tech/Rust → Ocean theme)
- Case-insensitive prefix matching
- Theme badge label on sub-wiki pages
- 8 integration tests

### 7. Server CRUD Integration Tests

- Axum router tested via tower::ServiceExt (no HTTP server needed)
- All 3 backends tested: File, SQLite, Git
- Tests: list seeded pages, get page, 404 missing, PUT/GET roundtrip, DELETE, HEAD exists check, full CRUD lifecycle
- 9 integration tests across 3 test files
- Temp directories for test isolation

### 8. CAS (Compare-and-Swap) Concurrency Control

- Optimistic concurrency for multi-agent coordination
- HTTP ETag header on GET/HEAD responses (SHA-256 of content)
- If-Match header on PUT for conditional writes
- 409 Conflict response includes current page + current ETag (no extra round-trip)
- Per-page locking via DashMap for CAS atomicity
- Unconditional PUT still works (backward compatible)
- Protocol: GET → edit locally → PUT with If-Match → retry on 409
- 10 new CAS tests across all 3 backends

### 9. Journaled Git Backend

- JournaledGitStorage wraps git backend for fast CAS coordination
- Reads/writes hit filesystem directly (sub-millisecond)
- Git commits queued via tokio::mpsc to background worker
- Eventually consistent git history
- Graceful shutdown drains commit queue via Rust drop semantics

### 10. Wiki Sync to GitHub Wiki

- CLI tool `wiki-sync-gh` exports wiki-rs pages to a GitHub Wiki git repo
- Fetches all pages via REST API, writes as `.md` files, commits and pushes
- Handles updates, deletions (removes stale `.md` files), and idempotent re-runs
- Supports custom commit messages, author identity, and work directory
- Uses git CLI for SSH key/credential compatibility
- Designed for cron or manual use after agent convergence

### 11. Agent API Guide

- `docs/agent-cas-wiki.md` — copy-paste instructions for AI agents
- Covers full API reference, CAS workflow, conflict handling, coordination rules
- Designed to be included in agent system prompts

## Remaining Items (from roadmap)

### 8. Playwright Browser Tests

- Tested file backend (port 7400) via headless Chromium
- Full CRUD flow: view MainPage, create SandBox, edit content, save, delete
- Red link verification (deleted page shows red)
- All Pages index verification
- 7 screenshots saved to docs/screenshots/
- Known limitation: page titles with `/` don't work (stripped from filenames, breaks router)

### 9. Demo Landing Page

- Static HTML page at docs/demo/index.html (no build step)
- Cards for all 6 wiki variants with ports, descriptions, live links
- Client-side / Server-backed tags
- Features grid (markdown, wiki links, aging, theming, import, XSS)
- Embedded screenshots from Playwright tests
- Running instructions with copy-pasteable commands
- Footer with copyright

## Architecture

### Component Layout

```
shared/             2 crates
  wiki-common/        model, parser, storage traits, aging, time
  wiki-import/        VQWiki + TiddlyWiki importers

ui/                 2 crates
  wiki-ui/            shared Yew components (PageView, PageEdit, etc.)
  wiki-ui-app/        App shell, routing, render_wiki()

frontend/           4 crates
  wiki-ephemeral/     port 7408
  wiki-browser-memory/ port 7409
  wiki-export-file/   port 7407
  wiki-server-ui/     REST client (shared by all server backends)

backend/            3 crates
  wiki-server/        Axum REST server + CLI
  wiki-server-db/     SQLite backend
  wiki-server-git/    git backend
```

### Key Shared Code

- AsyncWikiStorage trait: shared/wiki-common/src/async_storage.rs
  (behind "server" feature flag)
- WikiStorage trait: shared/wiki-common/src/storage.rs (sync, for WASM)
- WikiPage model: shared/wiki-common/src/model.rs
- Parser: shared/wiki-common/src/parser.rs
- Time utilities: shared/wiki-common/src/time.rs
- Age calculation: shared/wiki-common/src/aging.rs
- ETag computation: shared/wiki-common/src/etag.rs (behind "server" feature)
- Journaled git storage: backend/wiki-server-git/src/journal.rs

### Quality

- sw-checklist: 69 passed, 0 failed
- clippy: zero warnings (-D warnings)
- No #[allow(clippy::...)] directives anywhere
- 63 integration tests
- All markdown validated
- Favicons on all WASM crates
- CLI --help with AI agent instructions section

### Design Principles

- Max 5 functions per module (headroom before 7 hard limit)
- Max 5 modules per crate (headroom before 7 hard limit)
- Split "up and out" when approaching limits
- No code duplication across components (shared code in shared/)
- No code duplication across crates (shared code in shared crate)
- Each backend is its own crate for clean dependency graph

## Running

```bash
# Client-side wikis
cd frontend/wiki-ephemeral && trunk serve
cd frontend/wiki-browser-memory && trunk serve
cd frontend/wiki-export-file && trunk serve

# Server-backed wikis (build frontend first)
cd frontend/wiki-server-ui && trunk build
cargo run -p wiki-server -- --backend file --data-dir ./data/file-wiki
cargo run -p wiki-server -- --backend db --data-dir ./data/db-wiki
cargo run -p wiki-server -- --backend git --data-dir ./work/bare-git

# Quality checks
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
cargo fmt --all
cargo doc --all --no-deps
sw-checklist
```

# Wiki-RS Architecture

## Component Structure

The project is organized into four components (directory groups),
each containing one or more Cargo crates. All crates belong to a
single Cargo workspace rooted at `Cargo.toml`.

### shared/ -- Types, traits, importers (no platform deps)

| Crate | Modules | Purpose |
|-------|---------|---------|
| wiki-common | 6 (model, parser, storage, async_storage, aging, time) | Core types, WikiStorage trait, AsyncWikiStorage trait, parser, aging |
| wiki-import | 3 (lib, vqwiki/\*, tiddlywiki/\*) | Markup converters for VQWiki and TiddlyWiki |

### ui/ -- Shared Yew components (WASM libraries)

| Crate | Modules | Purpose |
|-------|---------|---------|
| wiki-ui | 6 (lib, components/\*) | StorageContext, Route, Nav, PageView, PageEdit, PageList |
| wiki-ui-app | 1 (lib) | App shell, switch(), render_wiki() entry point |

### frontend/ -- WASM binaries (one per storage variant)

| Crate | Modules | Port | Purpose |
|-------|---------|------|---------|
| wiki-ephemeral | 2 | 7408 | In-memory HashMap storage |
| wiki-browser-memory | 2 | 7409 | localStorage persistence |
| wiki-export-file | 3 | 7407 | JSON file download/upload |
| wiki-server-ui | 3 | - | REST client for server backends |

### backend/ -- Native server binaries

| Crate | Modules | Purpose |
|-------|---------|---------|
| wiki-server | 6 | Axum REST API + CLI (--backend file\|db\|git) |
| wiki-server-db | 2 | SQLite AsyncWikiStorage impl |
| wiki-server-git | 3 | git2 AsyncWikiStorage impl |

## Dependency Graph

```
wiki-common (shared types + traits)
  |
  +-- wiki-import (importers, depends on wiki-common for WikiPage)
  |
  +-- wiki-ui (Yew components, depends on wiki-common for types)
  |     |
  |     +-- wiki-ui-app (app shell, depends on wiki-ui + wiki-common)
  |
  +-- wiki-ephemeral (depends on wiki-common + wiki-ui-app)
  +-- wiki-browser-memory (depends on wiki-common + wiki-ui-app)
  +-- wiki-export-file (depends on wiki-common + wiki-ui)
  +-- wiki-server-ui (depends on wiki-common + wiki-ui-app)
  |
  +-- wiki-server-db (depends on wiki-common[server])
  +-- wiki-server-git (depends on wiki-common[server])
  +-- wiki-server (depends on wiki-common[server] + wiki-server-db + wiki-server-git)
```

## Key Design Decisions

### Two Storage Traits

- `WikiStorage` (sync) -- for WASM frontends where async is not needed
- `AsyncWikiStorage` (async) -- for server backends, behind "server"
  feature flag in wiki-common to avoid pulling in async-trait for WASM

### One Server Binary

All server backends are selected via `--backend file|db|git` CLI flag
on a single `wiki-server` binary. Each backend lives in its own crate
to keep module counts low and dependencies clean.

### Optimistic Caching (server-ui)

The REST client frontend (`wiki-server-ui`) uses optimistic local caching:
reads return cached data, writes update cache immediately and fire
async HTTP requests in the background.

### Page Aging

Age calculation is in shared/wiki-common (pure logic, no platform deps).
CSS effects are in assets/style.css. The page_view component applies
the age CSS class based on the updated_at timestamp.

## sw-checklist Compliance

Target: max 5 functions/module, max 5 modules/crate (headroom below
the hard limits of 7). When approaching limits, split "up and out":
functions to modules, modules to crates, crates to components.

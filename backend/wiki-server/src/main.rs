use clap::Parser;
use std::path::PathBuf;
use tower_http::cors::CorsLayer;
use wiki_server::storage::BackendKind;

const VERSION_INFO: &str = "\
wiki-server 0.1.0
Copyright (c) 2026 Michael A Wright
License: See LICENSE file
Repository: https://github.com/sw-vibe-coding/wiki-rs
Build Host: local
Build Commit: dev
Build Time: dev";

#[derive(Parser)]
#[command(
    name = "wiki-server",
    version = VERSION_INFO,
    about = "Wiki-RS REST server with pluggable storage backends",
    long_about = "Wiki-RS REST server with pluggable storage backends.\n\n\
        Serves a WASM wiki frontend and REST API for page CRUD operations.\n\n\
        AI CODING AGENT INSTRUCTIONS:\n\n\
        This server supports three storage backends:\n\
        - file: flat .md files in --data-dir (default port 7400)\n\
        - db: SQLite database in --data-dir (default port 7401)\n\
        - git: git repository in --data-dir (default port 7402)\n\n\
        Build the WASM frontend first:\n\
          cd crates/wiki-server-ui && trunk build\n\n\
        Then run the server:\n\
          cargo run -p wiki-server -- --backend file\n\n\
        The server serves both the REST API at /api/pages and\n\
        the WASM frontend at / from --static-dir."
)]
struct Cli {
    /// Storage backend to use
    #[arg(long, value_enum, default_value = "file")]
    backend: BackendKind,

    /// Port to listen on (default depends on backend)
    #[arg(long)]
    port: Option<u16>,

    /// Directory for storing data
    #[arg(long, default_value = "./data")]
    data_dir: PathBuf,

    /// Directory containing built WASM frontend assets
    #[arg(long, default_value = "./dist/server-ui")]
    static_dir: PathBuf,
}

fn default_port(backend: &BackendKind) -> u16 {
    match backend {
        BackendKind::File => 7400,
        BackendKind::Db => 7401,
        BackendKind::Git => 7402,
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    let port = cli.port.unwrap_or_else(|| default_port(&cli.backend));
    let store = wiki_server::seed::create_storage(cli.backend, &cli.data_dir).await;

    let mut app = wiki_server::api::build_router(store).layer(CorsLayer::permissive());

    if cli.static_dir.exists() {
        let serve = tower_http::services::ServeDir::new(&cli.static_dir).fallback(
            tower_http::services::ServeFile::new(cli.static_dir.join("index.html")),
        );
        app = app.fallback_service(serve);
    }

    let addr = format!("0.0.0.0:{port}");
    tracing::info!("wiki-server listening on {addr}");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

use clap::Parser;
use serde::Deserialize;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::Command;

const VERSION_INFO: &str = "\
wiki-sync-gh 0.1.0
Copyright (c) 2026 Michael A Wright
License: See LICENSE file
Repository: https://github.com/sw-vibe-coding/wiki-rs";

#[derive(Parser)]
#[command(
    name = "wiki-sync-gh",
    version = VERSION_INFO,
    about = "Sync wiki-rs pages to a GitHub Wiki repository",
    long_about = "Sync wiki-rs pages to a GitHub Wiki repository.\n\n\
        Fetches all pages from a running wiki-rs server and pushes them\n\
        as markdown files to a GitHub Wiki git repo. Only the latest\n\
        state is published — no intermediate edits are exposed.\n\n\
        The wiki repo URL must use SSH format for read/write access:\n\
          git@github.com:owner/repo.wiki.git\n\n\
        AI CODING AGENT INSTRUCTIONS:\n\n\
        This tool is designed for cron or manual use after AI agents\n\
        have converged on wiki content via the CAS protocol.\n\n\
        Examples:\n\
          wiki-sync-gh --server http://localhost:7402 \\\n\
            --wiki-repo git@github.com:owner/repo.wiki.git\n\
          wiki-sync-gh --server http://localhost:7402 \\\n\
            --wiki-repo git@github.com:owner/repo.wiki.git \\\n\
            --work-dir ./work/wiki-sync \\\n\
            --message \"Auto-sync after agent coordination\""
)]
struct Cli {
    /// wiki-rs server URL (e.g. http://localhost:7402)
    #[arg(long)]
    server: String,

    /// GitHub Wiki git URL (use SSH format: git@github.com:owner/repo.wiki.git)
    #[arg(long)]
    wiki_repo: String,

    /// Local directory for the wiki repo clone
    #[arg(long, default_value = "./work/wiki-sync")]
    work_dir: PathBuf,

    /// Commit message
    #[arg(long, default_value = "Sync from wiki-rs")]
    message: String,

    /// Git author name
    #[arg(long, default_value = "wiki-rs-sync")]
    author_name: String,

    /// Git author email
    #[arg(long, default_value = "wiki-rs@localhost")]
    author_email: String,
}

#[derive(Deserialize)]
struct WikiPage {
    title: String,
    content: String,
}

fn main() {
    let cli = Cli::parse();
    let server = cli.server.trim_end_matches('/');
    let client = reqwest::blocking::Client::new();

    // 1. Fetch page list
    println!("Fetching pages from {server}...");
    let titles: Vec<String> = client
        .get(format!("{server}/api/pages"))
        .send()
        .expect("failed to connect to wiki-rs server")
        .json()
        .expect("failed to parse page list");
    println!("  Found {} pages", titles.len());

    // 2. Fetch each page
    let mut pages = Vec::new();
    for title in &titles {
        let page: WikiPage = client
            .get(format!("{server}/api/pages/{title}"))
            .send()
            .unwrap_or_else(|e| panic!("failed to fetch page '{title}': {e}"))
            .json()
            .unwrap_or_else(|e| panic!("failed to parse page '{title}': {e}"));
        pages.push(page);
    }

    // 3. Clone or pull wiki repo
    clone_or_pull(&cli.wiki_repo, &cli.work_dir);

    // 4. Write pages and remove stale files
    let server_titles: HashSet<String> = titles.into_iter().collect();
    let mut written = 0;
    for page in &pages {
        let path = cli.work_dir.join(format!("{}.md", page.title));
        std::fs::write(&path, &page.content)
            .unwrap_or_else(|e| panic!("failed to write {}: {e}", path.display()));
        written += 1;
    }

    let mut removed = 0;
    let entries = std::fs::read_dir(&cli.work_dir).expect("failed to read work dir");
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "md")
            && let Some(stem) = path.file_stem().and_then(|s| s.to_str())
            && !server_titles.contains(stem)
        {
            std::fs::remove_file(&path).ok();
            println!("  Removed stale: {stem}.md");
            removed += 1;
        }
    }
    println!("  Wrote {written} pages, removed {removed} stale files");

    // 5. Stage, commit, push (if changes exist)
    git_cmd(&cli.work_dir, &["add", "-A"]);

    let status = git_output(&cli.work_dir, &["status", "--porcelain"]);
    if status.trim().is_empty() {
        println!("No changes to push — wiki is already up to date.");
        return;
    }

    println!("Changes detected, committing...");
    git_cmd(
        &cli.work_dir,
        &[
            "commit",
            "-m",
            &cli.message,
            &format!("--author={} <{}>", cli.author_name, cli.author_email),
        ],
    );

    println!("Pushing to {}...", cli.wiki_repo);
    git_cmd(&cli.work_dir, &["push"]);

    println!("Sync complete.");
}

fn clone_or_pull(repo_url: &str, work_dir: &Path) {
    if work_dir.join(".git").exists() {
        println!("Pulling latest from wiki repo...");
        git_cmd(work_dir, &["pull", "--ff-only"]);
    } else {
        println!("Cloning wiki repo...");
        std::fs::create_dir_all(work_dir).expect("failed to create work dir");
        let status = Command::new("git")
            .args(["clone", repo_url, &work_dir.to_string_lossy()])
            .status()
            .expect("failed to run git clone");
        if !status.success() {
            panic!("git clone failed with exit code {status}");
        }
    }
}

fn git_cmd(dir: &Path, args: &[&str]) {
    let status = Command::new("git")
        .current_dir(dir)
        .args(args)
        .status()
        .unwrap_or_else(|e| panic!("failed to run git {}: {e}", args[0]));
    if !status.success() {
        panic!("git {} failed with exit code {status}", args[0]);
    }
}

fn git_output(dir: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .current_dir(dir)
        .args(args)
        .output()
        .unwrap_or_else(|e| panic!("failed to run git {}: {e}", args[0]));
    String::from_utf8_lossy(&output.stdout).to_string()
}

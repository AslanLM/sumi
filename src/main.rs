mod cli;
mod core;
mod log;
mod process;
mod web;

use clap::Parser;
use cli::args::Args;
use core::app::App;
use tokio::sync::broadcast;
use web::server::Log;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    println!("✔ sumi v0.1.0");
    println!("☰ watching process: {}", args.command.join(" "));
    println!("☰ web ui: http://localhost:8080");
    println!("──────────────────────────────────");

    let (log_tx, _) = broadcast::channel::<Log>(1000);

    let web_log_tx = log_tx.clone();
    tokio::spawn(async move {
        web::server::start(web_log_tx).await;
    });

    App::run(args.command, log_tx).await
}

#![allow(dead_code)]

mod cli;
mod config;
mod core;
mod detector;
mod doctor;
mod history;
mod logger;
mod managers;
mod search;
mod ui;
mod utils;

use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init();

    let cli = Cli::parse();
    let detector = detector::Detector::new();

    match cli.command {
        Commands::Search { package } => {
            search::search(&detector, &package).await?;
        }
        Commands::Install { package } => {
            search::install(&detector, &package).await?;
        }
        Commands::Remove { package, yes } => {
            search::remove(&detector, &package, yes).await?;
        }
        Commands::Update { package } => {
            search::update(&detector, package.as_deref()).await?;
        }
        Commands::Info { package } => {
            search::info(&detector, &package).await?;
        }
        Commands::List => {
            search::list(&detector).await?;
        }
        Commands::Clean => {
            search::clean(&detector).await?;
        }
        Commands::Doctor => {
            let mut doc = doctor::Doctor::new();
            doc.run(&detector);
        }
        Commands::History => {
            history::show_history();
        }
        Commands::Config => {
            let cfg = config::Config::load();
            cfg.open_in_editor()?;
            ui::info("Opening configuration file...");
        }
        Commands::Version => {
            ui::banner();
            println!("  {}", env!("CARGO_PKG_VERSION").white().bold());
            println!();
            ui::info("Installed package managers:");
            for name in detector.manager_names() {
                ui::success(&name);
            }
        }
    }

    Ok(())
}

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "kawaii",
    about = "Universal Linux Package Manager Wrapper",
    version,
    long_about = "One command. Every package manager.\n\nKawaii provides a unified interface for all your Linux package managers."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Search for a package across all managers
    #[command(short_flag_alias = 's', visible_short_flag_alias = 's')]
    Search {
        /// Package name to search for
        package: String,
    },

    /// Install a package (smart install with manager selection)
    #[command(short_flag_alias = 'i', visible_short_flag_alias = 'i')]
    Install {
        /// Package name to install
        package: String,
    },

    /// Remove a package
    #[command(short_flag_alias = 'r', visible_short_flag_alias = 'r')]
    Remove {
        /// Package name to remove
        package: String,

        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
    },

    /// Update packages
    #[command(short_flag_alias = 'u', visible_short_flag_alias = 'u')]
    Update {
        /// Specific package to update (omit to update all)
        package: Option<String>,
    },

    /// Show detailed info about a package
    #[command(short_flag_alias = 'I', visible_short_flag_alias = 'I')]
    Info {
        /// Package name
        package: String,
    },

    /// List installed packages
    #[command(short_flag_alias = 'l', visible_short_flag_alias = 'l')]
    List,

    /// Clean package manager caches
    #[command(short_flag_alias = 'C', visible_short_flag_alias = 'C')]
    Clean,

    /// Check system health
    #[command(short_flag_alias = 'd', visible_short_flag_alias = 'd')]
    Doctor,

    /// Show command history
    #[command(short_flag_alias = 'H', visible_short_flag_alias = 'H')]
    History,

    /// Open configuration file
    #[command(short_flag_alias = 'c', visible_short_flag_alias = 'c')]
    Config,

    /// Show version information
    #[command(short_flag_alias = 'v', visible_short_flag_alias = 'v')]
    Version,
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum KawaiiError {
    #[error("Package manager '{0}' is not installed")]
    NotInstalled(String),

    #[error("Package '{0}' not found in any configured package manager")]
    PackageNotFound(String),

    #[error("Command failed: {0}")]
    CommandFailed(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("User cancelled the operation")]
    Cancelled,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML parse error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("{0}")]
    Anyhow(#[from] anyhow::Error),
}

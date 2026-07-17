pub mod error;
pub mod types;

pub use types::*;

use async_trait::async_trait;

/// Supported package manager types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum PackageManagerKind {
    Pacman,
    Yay,
    Paru,
    Apt,
    Dnf,
    Yum,
    Zypper,
    Xbps,
    Nix,
    Apk,
    Brew,
    Flatpak,
    Snap,
}

impl std::fmt::Display for PackageManagerKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pacman => write!(f, "pacman"),
            Self::Yay => write!(f, "yay"),
            Self::Paru => write!(f, "paru"),
            Self::Apt => write!(f, "apt"),
            Self::Dnf => write!(f, "dnf"),
            Self::Yum => write!(f, "yum"),
            Self::Zypper => write!(f, "zypper"),
            Self::Xbps => write!(f, "xbps"),
            Self::Nix => write!(f, "nix"),
            Self::Apk => write!(f, "apk"),
            Self::Brew => write!(f, "brew"),
            Self::Flatpak => write!(f, "flatpak"),
            Self::Snap => write!(f, "snap"),
        }
    }
}

impl PackageManagerKind {
    pub fn all() -> &'static [PackageManagerKind] {
        &[
            Self::Pacman,
            Self::Yay,
            Self::Paru,
            Self::Apt,
            Self::Dnf,
            Self::Yum,
            Self::Zypper,
            Self::Xbps,
            Self::Nix,
            Self::Apk,
            Self::Brew,
            Self::Flatpak,
            Self::Snap,
        ]
    }

    pub fn binary_name(&self) -> &'static str {
        match self {
            Self::Pacman => "pacman",
            Self::Yay => "yay",
            Self::Paru => "paru",
            Self::Apt => "apt",
            Self::Dnf => "dnf",
            Self::Yum => "yum",
            Self::Zypper => "zypper",
            Self::Xbps => "xbps-install",
            Self::Nix => "nix-env",
            Self::Apk => "apk",
            Self::Brew => "brew",
            Self::Flatpak => "flatpak",
            Self::Snap => "snap",
        }
    }
}

/// Trait that all package manager implementations must satisfy
#[async_trait]
pub trait PackageManager: Send + Sync {
    /// Human-readable name of this package manager
    fn kind(&self) -> PackageManagerKind;

    /// Check if this package manager is installed on the system
    fn is_installed(&self) -> bool;

    /// Search for a package, returning matches
    async fn search(&self, package: &str) -> anyhow::Result<Vec<PackageSearchResult>>;

    /// Install a package
    async fn install(&self, package: &str) -> anyhow::Result<InstallResult>;

    /// Remove a package
    async fn remove(&self, package: &str) -> anyhow::Result<RemoveResult>;

    /// Update all packages or a specific one
    async fn update(&self, package: Option<&str>) -> anyhow::Result<UpdateResult>;

    /// Get detailed info about a package
    async fn info(&self, package: &str) -> anyhow::Result<PackageInfo>;

    /// List installed packages
    async fn list_installed(&self) -> anyhow::Result<Vec<String>>;

    /// Clean package cache
    async fn clean(&self) -> anyhow::Result<CleanResult>;

    /// Get the version of this package manager
    fn version(&self) -> Option<String>;
}

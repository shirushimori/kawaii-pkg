use super::PackageManagerKind;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct PackageSearchResult {
    pub name: String,
    pub version: String,
    pub description: String,
    pub manager: PackageManagerKind,
}

#[derive(Debug, Clone)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub maintainer: Option<String>,
    pub homepage: Option<String>,
    pub size: Option<String>,
    pub dependencies: Vec<String>,
    pub manager: PackageManagerKind,
}

#[derive(Debug, Clone)]
pub struct InstallResult {
    pub package: String,
    pub version: Option<String>,
    pub manager: PackageManagerKind,
    pub dependencies_count: usize,
    pub disk_usage: Option<String>,
    pub duration_secs: u64,
}

#[derive(Debug, Clone)]
pub struct RemoveResult {
    pub package: String,
    pub manager: PackageManagerKind,
}

#[derive(Debug, Clone)]
pub struct UpdateResult {
    pub manager: PackageManagerKind,
    pub updated_count: usize,
    pub packages: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CleanResult {
    pub manager: PackageManagerKind,
    pub freed_bytes: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HistoryEntry {
    pub timestamp: DateTime<Utc>,
    pub action: String,
    pub package: String,
    pub manager: String,
    pub success: bool,
}

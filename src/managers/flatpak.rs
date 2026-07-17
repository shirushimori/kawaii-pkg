use async_trait::async_trait;
use crate::core::*;

pub struct Flatpak;

#[async_trait]
impl PackageManager for Flatpak {
    fn kind(&self) -> PackageManagerKind { PackageManagerKind::Flatpak }
    fn is_installed(&self) -> bool { crate::utils::binary_exists("flatpak") }

    async fn search(&self, package: &str) -> anyhow::Result<Vec<PackageSearchResult>> {
        let output = crate::utils::async_search("flatpak", &["search", package]).await;
        let mut results = Vec::new();
        for line in output.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                results.push(PackageSearchResult {
                    name: parts[0].to_string(),
                    version: parts.get(1).unwrap_or(&"").to_string(),
                    description: parts[2..].join(" "),
                    manager: PackageManagerKind::Flatpak,
                });
            }
        }
        Ok(results)
    }

    async fn install(&self, package: &str) -> anyhow::Result<InstallResult> {
        let start = std::time::Instant::now();
        let output = crate::utils::run_command_combined("flatpak", &["install", "-y", package])?;
        if output.contains("Error") || output.contains("No remote") {
            return Err(anyhow::anyhow!("flatpak install failed: {output}"));
        }
        Ok(InstallResult { package: package.to_string(), version: None, manager: PackageManagerKind::Flatpak, dependencies_count: 0, disk_usage: None, duration_secs: start.elapsed().as_secs() })
    }

    async fn remove(&self, package: &str) -> anyhow::Result<RemoveResult> {
        let output = crate::utils::run_command_combined("flatpak", &["uninstall", "-y", package])?;
        if output.contains("Error") { return Err(anyhow::anyhow!("flatpak remove failed: {output}")); }
        Ok(RemoveResult { package: package.to_string(), manager: PackageManagerKind::Flatpak })
    }

    async fn update(&self, _package: Option<&str>) -> anyhow::Result<UpdateResult> {
        let _output = crate::utils::run_command_combined("flatpak", &["update", "-y"])?;
        Ok(UpdateResult { manager: PackageManagerKind::Flatpak, updated_count: 0, packages: Vec::new() })
    }

    async fn info(&self, package: &str) -> anyhow::Result<PackageInfo> {
        let output = crate::utils::run_command("flatpak", &["info", package])?;
        let mut info = PackageInfo { name: package.to_string(), version: String::new(), description: String::new(), maintainer: None, homepage: None, size: None, dependencies: Vec::new(), manager: PackageManagerKind::Flatpak };
        for line in output.lines() {
            if let Some(val) = line.strip_prefix("Version: ") { info.version = val.to_string(); }
            else if let Some(val) = line.strip_prefix("Name: ") { info.name = val.to_string(); }
            else if let Some(val) = line.strip_prefix("Origin: ") { info.maintainer = Some(val.to_string()); }
            else if let Some(val) = line.strip_prefix("Homepage: ") { info.homepage = Some(val.to_string()); }
        }
        Ok(info)
    }

    async fn list_installed(&self) -> anyhow::Result<Vec<String>> {
        let output = crate::utils::run_command("flatpak", &["list"])?;
        Ok(output.lines().filter_map(|l| l.split_whitespace().next().map(|s| s.to_string())).collect())
    }

    async fn clean(&self) -> anyhow::Result<CleanResult> {
        let _output = crate::utils::run_command_combined("flatpak", &["uninstall", "--unused", "-y"])?;
        Ok(CleanResult { manager: PackageManagerKind::Flatpak, freed_bytes: 0 })
    }

    fn version(&self) -> Option<String> {
        crate::utils::run_command("flatpak", &["--version"]).ok().map(|s| s.trim().to_string())
    }
}

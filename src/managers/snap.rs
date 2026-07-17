use async_trait::async_trait;
use crate::core::*;

pub struct Snap;

#[async_trait]
impl PackageManager for Snap {
    fn kind(&self) -> PackageManagerKind { PackageManagerKind::Snap }
    fn is_installed(&self) -> bool { crate::utils::binary_exists("snap") }

    async fn search(&self, package: &str) -> anyhow::Result<Vec<PackageSearchResult>> {
        let output = crate::utils::async_search("snap", &["find", package]).await;
        let mut results = Vec::new();
        for line in output.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(name) = parts.first() {
                results.push(PackageSearchResult {
                    name: name.to_string(),
                    version: parts.get(1).unwrap_or(&"").to_string(),
                    description: parts.get(2).unwrap_or(&"").to_string(),
                    manager: PackageManagerKind::Snap,
                });
            }
        }
        Ok(results)
    }

    async fn install(&self, package: &str) -> anyhow::Result<InstallResult> {
        let start = std::time::Instant::now();
        let output = crate::utils::run_command_combined("snap", &["install", package])?;
        if output.contains("error") || output.contains("not found") {
            return Err(anyhow::anyhow!("snap install failed: {output}"));
        }
        Ok(InstallResult { package: package.to_string(), version: None, manager: PackageManagerKind::Snap, dependencies_count: 0, disk_usage: None, duration_secs: start.elapsed().as_secs() })
    }

    async fn remove(&self, package: &str) -> anyhow::Result<RemoveResult> {
        let output = crate::utils::run_command_combined("snap", &["remove", package])?;
        if output.contains("error") { return Err(anyhow::anyhow!("snap remove failed: {output}")); }
        Ok(RemoveResult { package: package.to_string(), manager: PackageManagerKind::Snap })
    }

    async fn update(&self, _package: Option<&str>) -> anyhow::Result<UpdateResult> {
        let _output = crate::utils::run_command_combined("snap", &["refresh"])?;
        Ok(UpdateResult { manager: PackageManagerKind::Snap, updated_count: 0, packages: Vec::new() })
    }

    async fn info(&self, package: &str) -> anyhow::Result<PackageInfo> {
        let output = crate::utils::run_command("snap", &["info", package])?;
        let mut info = PackageInfo { name: package.to_string(), version: String::new(), description: String::new(), maintainer: None, homepage: None, size: None, dependencies: Vec::new(), manager: PackageManagerKind::Snap };
        for line in output.lines() {
            if let Some(val) = line.strip_prefix("version: ") { info.version = val.trim().to_string(); }
            else if let Some(val) = line.strip_prefix("summary: ") { info.description = val.trim().to_string(); }
            else if let Some(val) = line.strip_prefix("publisher: ") { info.maintainer = Some(val.trim().to_string()); }
        }
        Ok(info)
    }

    async fn list_installed(&self) -> anyhow::Result<Vec<String>> {
        let output = crate::utils::run_command("snap", &["list"])?;
        Ok(output.lines().skip(1).filter_map(|l| l.split_whitespace().next().map(|s| s.to_string())).collect())
    }

    async fn clean(&self) -> anyhow::Result<CleanResult> {
        Ok(CleanResult { manager: PackageManagerKind::Snap, freed_bytes: 0 })
    }

    fn version(&self) -> Option<String> {
        crate::utils::run_command("snap", &["version"]).ok().and_then(|o| o.lines().next().map(|s| s.to_string()))
    }
}

use async_trait::async_trait;
use crate::core::*;

pub struct Brew;

#[async_trait]
impl PackageManager for Brew {
    fn kind(&self) -> PackageManagerKind { PackageManagerKind::Brew }
    fn is_installed(&self) -> bool { crate::utils::binary_exists("brew") }

    async fn search(&self, package: &str) -> anyhow::Result<Vec<PackageSearchResult>> {
        let output = crate::utils::async_search("brew", &["search", package]).await;
        let mut results = Vec::new();
        for line in output.lines() {
            let name = line.trim().to_string();
            if !name.is_empty() {
                results.push(PackageSearchResult {
                    name,
                    version: String::new(),
                    description: String::new(),
                    manager: PackageManagerKind::Brew,
                });
            }
        }
        Ok(results)
    }

    async fn install(&self, package: &str) -> anyhow::Result<InstallResult> {
        let start = std::time::Instant::now();
        let output = crate::utils::run_command_combined("brew", &["install", package])?;
        if output.contains("Error") || output.contains("No such keg") {
            return Err(anyhow::anyhow!("brew install failed: {output}"));
        }
        Ok(InstallResult { package: package.to_string(), version: None, manager: PackageManagerKind::Brew, dependencies_count: 0, disk_usage: None, duration_secs: start.elapsed().as_secs() })
    }

    async fn remove(&self, package: &str) -> anyhow::Result<RemoveResult> {
        let output = crate::utils::run_command_combined("brew", &["uninstall", package])?;
        if output.contains("Error") { return Err(anyhow::anyhow!("brew remove failed: {output}")); }
        Ok(RemoveResult { package: package.to_string(), manager: PackageManagerKind::Brew })
    }

    async fn update(&self, _package: Option<&str>) -> anyhow::Result<UpdateResult> {
        let _output = crate::utils::run_command_combined("brew", &["upgrade"])?;
        Ok(UpdateResult { manager: PackageManagerKind::Brew, updated_count: 0, packages: Vec::new() })
    }

    async fn info(&self, package: &str) -> anyhow::Result<PackageInfo> {
        let output = crate::utils::run_command("brew", &["info", package])?;
        let mut info = PackageInfo { name: package.to_string(), version: String::new(), description: String::new(), maintainer: None, homepage: None, size: None, dependencies: Vec::new(), manager: PackageManagerKind::Brew };
        for line in output.lines() {
            if line.starts_with("==>") {
                let rest = line.trim_start_matches("==> ").trim();
                if info.description.is_empty() {
                    info.description = rest.to_string();
                }
            }
        }
        Ok(info)
    }

    async fn list_installed(&self) -> anyhow::Result<Vec<String>> {
        let output = crate::utils::run_command("brew", &["list"])?;
        Ok(output.split_whitespace().map(|s| s.to_string()).collect())
    }

    async fn clean(&self) -> anyhow::Result<CleanResult> {
        let _output = crate::utils::run_command_combined("brew", &["cleanup"])?;
        Ok(CleanResult { manager: PackageManagerKind::Brew, freed_bytes: 0 })
    }

    fn version(&self) -> Option<String> {
        crate::utils::run_command("brew", &["--version"]).ok().and_then(|o| o.lines().next().map(|s| s.to_string()))
    }
}

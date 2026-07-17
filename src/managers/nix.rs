use async_trait::async_trait;
use crate::core::*;

pub struct Nix;

#[async_trait]
impl PackageManager for Nix {
    fn kind(&self) -> PackageManagerKind { PackageManagerKind::Nix }
    fn is_installed(&self) -> bool { crate::utils::binary_exists("nix-env") }

    async fn search(&self, package: &str) -> anyhow::Result<Vec<PackageSearchResult>> {
        let output = crate::utils::async_search("nix-env", &["-qaP", package]).await;
        let mut results = Vec::new();
        for line in output.lines() {
            if let Some((path, name)) = line.split_once(' ') {
                results.push(PackageSearchResult {
                    name: path.to_string(),
                    version: String::new(),
                    description: name.to_string(),
                    manager: PackageManagerKind::Nix,
                });
            }
        }
        Ok(results)
    }

    async fn install(&self, package: &str) -> anyhow::Result<InstallResult> {
        let start = std::time::Instant::now();
        let output = crate::utils::run_command_combined("nix-env", &["-iA", package])?;
        if output.contains("error") || output.contains("failed") {
            return Err(anyhow::anyhow!("nix install failed: {output}"));
        }
        Ok(InstallResult { package: package.to_string(), version: None, manager: PackageManagerKind::Nix, dependencies_count: 0, disk_usage: None, duration_secs: start.elapsed().as_secs() })
    }

    async fn remove(&self, package: &str) -> anyhow::Result<RemoveResult> {
        let output = crate::utils::run_command_combined("nix-env", &["-e", package])?;
        if output.contains("error") { return Err(anyhow::anyhow!("nix remove failed: {output}")); }
        Ok(RemoveResult { package: package.to_string(), manager: PackageManagerKind::Nix })
    }

    async fn update(&self, _package: Option<&str>) -> anyhow::Result<UpdateResult> {
        let _output = crate::utils::run_command_combined("nix-env", &["-u"])?;
        Ok(UpdateResult { manager: PackageManagerKind::Nix, updated_count: 0, packages: Vec::new() })
    }

    async fn info(&self, package: &str) -> anyhow::Result<PackageInfo> {
        let output = crate::utils::run_command("nix-env", &["-qaP", "--description", package])?;
        let mut info = PackageInfo { name: package.to_string(), version: String::new(), description: String::new(), maintainer: None, homepage: None, size: None, dependencies: Vec::new(), manager: PackageManagerKind::Nix };
        for line in output.lines() {
            if let Some((path, desc)) = line.split_once("  - ") {
                info.name = path.to_string();
                info.description = desc.to_string();
            }
        }
        Ok(info)
    }

    async fn list_installed(&self) -> anyhow::Result<Vec<String>> {
        let output = crate::utils::run_command("nix-env", &["-q"])?;
        Ok(output.lines().map(|s| s.to_string()).collect())
    }

    async fn clean(&self) -> anyhow::Result<CleanResult> {
        let _output = crate::utils::run_command_combined("nix-collect-garbage", &["-d"])?;
        Ok(CleanResult { manager: PackageManagerKind::Nix, freed_bytes: 0 })
    }

    fn version(&self) -> Option<String> {
        crate::utils::run_command("nix-env", &["--version"]).ok().and_then(|o| o.lines().next().map(|s| s.to_string()))
    }
}

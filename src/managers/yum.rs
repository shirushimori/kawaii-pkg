use async_trait::async_trait;
use crate::core::*;

pub struct Yum;

#[async_trait]
impl PackageManager for Yum {
    fn kind(&self) -> PackageManagerKind { PackageManagerKind::Yum }
    fn is_installed(&self) -> bool { crate::utils::binary_exists("yum") }

    async fn search(&self, package: &str) -> anyhow::Result<Vec<PackageSearchResult>> {
        let output = crate::utils::async_search("yum", &["search", package]).await;
        let mut results = Vec::new();
        for line in output.lines() {
            if let Some((name, desc)) = line.split_once(" : ") {
                let name = name.trim().to_string();
                if !name.is_empty() && !name.starts_with("===") && !name.starts_with("Name") {
                    results.push(PackageSearchResult {
                        name,
                        version: String::new(),
                        description: desc.trim().to_string(),
                        manager: PackageManagerKind::Yum,
                    });
                }
            }
        }
        Ok(results)
    }

    async fn install(&self, package: &str) -> anyhow::Result<InstallResult> {
        let start = std::time::Instant::now();
        let output = crate::utils::run_command_combined("yum", &["install", "-y", package])?;
        if output.contains("No package") || output.contains("Error") {
            return Err(anyhow::anyhow!("yum install failed: {output}"));
        }
        Ok(InstallResult { package: package.to_string(), version: None, manager: PackageManagerKind::Yum, dependencies_count: 0, disk_usage: None, duration_secs: start.elapsed().as_secs() })
    }

    async fn remove(&self, package: &str) -> anyhow::Result<RemoveResult> {
        let output = crate::utils::run_command_combined("yum", &["remove", "-y", package])?;
        if output.contains("Error") { return Err(anyhow::anyhow!("yum remove failed: {output}")); }
        Ok(RemoveResult { package: package.to_string(), manager: PackageManagerKind::Yum })
    }

    async fn update(&self, _package: Option<&str>) -> anyhow::Result<UpdateResult> {
        let _output = crate::utils::run_command_combined("yum", &["update", "-y"])?;
        Ok(UpdateResult { manager: PackageManagerKind::Yum, updated_count: 0, packages: Vec::new() })
    }

    async fn info(&self, package: &str) -> anyhow::Result<PackageInfo> {
        let output = crate::utils::run_command("yum", &["info", package])?;
        let mut info = PackageInfo { name: package.to_string(), version: String::new(), description: String::new(), maintainer: None, homepage: None, size: None, dependencies: Vec::new(), manager: PackageManagerKind::Yum };
        for line in output.lines() {
            if let Some(val) = line.strip_prefix("Version        : ") { info.version = val.to_string(); }
            else if let Some(val) = line.strip_prefix("Summary        : ") { info.description = val.to_string(); }
            else if let Some(val) = line.strip_prefix("Maintainer     : ") { info.maintainer = Some(val.to_string()); }
            else if let Some(val) = line.strip_prefix("URL            : ") { info.homepage = Some(val.to_string()); }
        }
        Ok(info)
    }

    async fn list_installed(&self) -> anyhow::Result<Vec<String>> {
        let output = crate::utils::run_command("yum", &["list", "installed"])?;
        Ok(output.lines().skip(1).filter_map(|l| l.split_whitespace().next().map(|s| s.split('.').next().unwrap_or(s).to_string())).collect())
    }

    async fn clean(&self) -> anyhow::Result<CleanResult> {
        let _output = crate::utils::run_command_combined("yum", &["clean", "all"])?;
        Ok(CleanResult { manager: PackageManagerKind::Yum, freed_bytes: 0 })
    }

    fn version(&self) -> Option<String> {
        crate::utils::run_command("yum", &["--version"]).ok().and_then(|o| o.lines().next().map(|s| s.to_string()))
    }
}

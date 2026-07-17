use async_trait::async_trait;
use crate::core::*;

pub struct Apk;

#[async_trait]
impl PackageManager for Apk {
    fn kind(&self) -> PackageManagerKind { PackageManagerKind::Apk }
    fn is_installed(&self) -> bool { crate::utils::binary_exists("apk") }

    async fn search(&self, package: &str) -> anyhow::Result<Vec<PackageSearchResult>> {
        let output = crate::utils::async_search("apk", &["search", package]).await;
        let mut results = Vec::new();
        for line in output.lines() {
            results.push(PackageSearchResult {
                name: line.to_string(),
                version: String::new(),
                description: String::new(),
                manager: PackageManagerKind::Apk,
            });
        }
        Ok(results)
    }

    async fn install(&self, package: &str) -> anyhow::Result<InstallResult> {
        let start = std::time::Instant::now();
        let output = crate::utils::run_command_combined("apk", &["add", package])?;
        if output.contains("ERROR") || output.contains("unsatisfiable") {
            return Err(anyhow::anyhow!("apk install failed: {output}"));
        }
        Ok(InstallResult { package: package.to_string(), version: None, manager: PackageManagerKind::Apk, dependencies_count: 0, disk_usage: None, duration_secs: start.elapsed().as_secs() })
    }

    async fn remove(&self, package: &str) -> anyhow::Result<RemoveResult> {
        let output = crate::utils::run_command_combined("apk", &["del", package])?;
        if output.contains("ERROR") { return Err(anyhow::anyhow!("apk remove failed: {output}")); }
        Ok(RemoveResult { package: package.to_string(), manager: PackageManagerKind::Apk })
    }

    async fn update(&self, _package: Option<&str>) -> anyhow::Result<UpdateResult> {
        let _output = crate::utils::run_command_combined("apk", &["upgrade"])?;
        Ok(UpdateResult { manager: PackageManagerKind::Apk, updated_count: 0, packages: Vec::new() })
    }

    async fn info(&self, package: &str) -> anyhow::Result<PackageInfo> {
        let output = crate::utils::run_command("apk", &["info", package])?;
        let mut info = PackageInfo { name: package.to_string(), version: String::new(), description: String::new(), maintainer: None, homepage: None, size: None, dependencies: Vec::new(), manager: PackageManagerKind::Apk };
        for line in output.lines() {
            if let Some(val) = line.strip_prefix("description: ") { info.description = val.trim().to_string(); }
        }
        Ok(info)
    }

    async fn list_installed(&self) -> anyhow::Result<Vec<String>> {
        let output = crate::utils::run_command("apk", &["list", "-I"])?;
        Ok(output.lines().filter_map(|l| l.split_whitespace().next().map(|s| s.to_string())).collect())
    }

    async fn clean(&self) -> anyhow::Result<CleanResult> {
        let _output = crate::utils::run_command_combined("apk", &["cache", "clean"])?;
        Ok(CleanResult { manager: PackageManagerKind::Apk, freed_bytes: 0 })
    }

    fn version(&self) -> Option<String> {
        crate::utils::run_command("apk", &["version"]).ok().and_then(|o| o.lines().next().map(|s| s.to_string()))
    }
}

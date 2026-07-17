use async_trait::async_trait;
use crate::core::*;

pub struct Zypper;

#[async_trait]
impl PackageManager for Zypper {
    fn kind(&self) -> PackageManagerKind { PackageManagerKind::Zypper }
    fn is_installed(&self) -> bool { crate::utils::binary_exists("zypper") }

    async fn search(&self, package: &str) -> anyhow::Result<Vec<PackageSearchResult>> {
        let output = crate::utils::async_search("zypper", &["search", package]).await;
        let mut results = Vec::new();
        for line in output.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 && (parts[0] == "i" || parts[0] == " ") {
                let name = parts[2].to_string();
                let version = parts.get(1).unwrap_or(&"").to_string();
                let description = parts[3..].join(" ");
                results.push(PackageSearchResult { name, version, description, manager: PackageManagerKind::Zypper });
            }
        }
        Ok(results)
    }

    async fn install(&self, package: &str) -> anyhow::Result<InstallResult> {
        let start = std::time::Instant::now();
        let output = crate::utils::run_command_combined("zypper", &["install", "-y", package])?;
        if output.contains("not found") || output.contains("Error") {
            return Err(anyhow::anyhow!("zypper install failed: {output}"));
        }
        Ok(InstallResult { package: package.to_string(), version: None, manager: PackageManagerKind::Zypper, dependencies_count: 0, disk_usage: None, duration_secs: start.elapsed().as_secs() })
    }

    async fn remove(&self, package: &str) -> anyhow::Result<RemoveResult> {
        let output = crate::utils::run_command_combined("zypper", &["remove", "-y", package])?;
        if output.contains("Error") { return Err(anyhow::anyhow!("zypper remove failed: {output}")); }
        Ok(RemoveResult { package: package.to_string(), manager: PackageManagerKind::Zypper })
    }

    async fn update(&self, _package: Option<&str>) -> anyhow::Result<UpdateResult> {
        let _output = crate::utils::run_command_combined("zypper", &["update", "-y"])?;
        Ok(UpdateResult { manager: PackageManagerKind::Zypper, updated_count: 0, packages: Vec::new() })
    }

    async fn info(&self, package: &str) -> anyhow::Result<PackageInfo> {
        let output = crate::utils::run_command("zypper", &["info", package])?;
        let mut info = PackageInfo { name: package.to_string(), version: String::new(), description: String::new(), maintainer: None, homepage: None, size: None, dependencies: Vec::new(), manager: PackageManagerKind::Zypper };
        for line in output.lines() {
            if let Some(val) = line.strip_prefix("Version        : ") { info.version = val.to_string(); }
            else if let Some(val) = line.strip_prefix("Description    : ") { info.description = val.to_string(); }
            else if let Some(val) = line.strip_prefix("Maintainer     : ") { info.maintainer = Some(val.to_string()); }
            else if let Some(val) = line.strip_prefix("URL            : ") { info.homepage = Some(val.to_string()); }
        }
        Ok(info)
    }

    async fn list_installed(&self) -> anyhow::Result<Vec<String>> {
        let output = crate::utils::run_command("zypper", &["packages", "-i"])?;
        Ok(output.lines().filter_map(|l| l.split_whitespace().nth(2).map(|s| s.to_string())).collect())
    }

    async fn clean(&self) -> anyhow::Result<CleanResult> {
        let _output = crate::utils::run_command_combined("zypper", &["clean"])?;
        Ok(CleanResult { manager: PackageManagerKind::Zypper, freed_bytes: 0 })
    }

    fn version(&self) -> Option<String> {
        crate::utils::run_command("zypper", &["--version"]).ok().and_then(|o| o.lines().next().map(|s| s.to_string()))
    }
}

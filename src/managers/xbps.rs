use async_trait::async_trait;
use crate::core::*;

pub struct Xbps;

#[async_trait]
impl PackageManager for Xbps {
    fn kind(&self) -> PackageManagerKind { PackageManagerKind::Xbps }
    fn is_installed(&self) -> bool { crate::utils::binary_exists("xbps-install") }

    async fn search(&self, package: &str) -> anyhow::Result<Vec<PackageSearchResult>> {
        let output = crate::utils::async_search("xbps-install", &["-Ss", package]).await;
        let mut results = Vec::new();
        for line in output.lines() {
            if let Some((name, rest)) = line.split_once('-') {
                if let Some((ver, desc)) = rest.split_once(' ') {
                    results.push(PackageSearchResult {
                        name: format!("{name}-{ver}"),
                        version: ver.trim().to_string(),
                        description: desc.trim().to_string(),
                        manager: PackageManagerKind::Xbps,
                    });
                }
            }
        }
        Ok(results)
    }

    async fn install(&self, package: &str) -> anyhow::Result<InstallResult> {
        let start = std::time::Instant::now();
        let output = crate::utils::run_command_combined("xbps-install", &["-Sy", package])?;
        if output.contains("ERROR") { return Err(anyhow::anyhow!("xbps install failed: {output}")); }
        Ok(InstallResult { package: package.to_string(), version: None, manager: PackageManagerKind::Xbps, dependencies_count: 0, disk_usage: None, duration_secs: start.elapsed().as_secs() })
    }

    async fn remove(&self, package: &str) -> anyhow::Result<RemoveResult> {
        let output = crate::utils::run_command_combined("xbps-remove", &["-R", package])?;
        if output.contains("ERROR") { return Err(anyhow::anyhow!("xbps remove failed: {output}")); }
        Ok(RemoveResult { package: package.to_string(), manager: PackageManagerKind::Xbps })
    }

    async fn update(&self, _package: Option<&str>) -> anyhow::Result<UpdateResult> {
        let _output = crate::utils::run_command_combined("xbps-install", &["-Su"])?;
        Ok(UpdateResult { manager: PackageManagerKind::Xbps, updated_count: 0, packages: Vec::new() })
    }

    async fn info(&self, package: &str) -> anyhow::Result<PackageInfo> {
        let output = crate::utils::run_command("xbps-query", &["-S", package])?;
        let mut info = PackageInfo { name: package.to_string(), version: String::new(), description: String::new(), maintainer: None, homepage: None, size: None, dependencies: Vec::new(), manager: PackageManagerKind::Xbps };
        for line in output.lines() {
            if let Some(val) = line.strip_prefix("version: ") { info.version = val.to_string(); }
            else if let Some(val) = line.strip_prefix("short_desc: ") { info.description = val.to_string(); }
            else if let Some(val) = line.strip_prefix("maintainer: ") { info.maintainer = Some(val.to_string()); }
            else if let Some(val) = line.strip_prefix("homepage: ") { info.homepage = Some(val.to_string()); }
        }
        Ok(info)
    }

    async fn list_installed(&self) -> anyhow::Result<Vec<String>> {
        let output = crate::utils::run_command("xbps-query", &["-l"])?;
        Ok(output.lines().filter_map(|l| l.split_whitespace().nth(1).map(|s| s.to_string())).collect())
    }

    async fn clean(&self) -> anyhow::Result<CleanResult> {
        let _output = crate::utils::run_command_combined("xbps-remove", &["-Oo"])?;
        Ok(CleanResult { manager: PackageManagerKind::Xbps, freed_bytes: 0 })
    }

    fn version(&self) -> Option<String> {
        crate::utils::run_command("xbps-install", &["--version"]).ok().and_then(|o| o.lines().next().map(|s| s.to_string()))
    }
}

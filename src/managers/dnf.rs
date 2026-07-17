use async_trait::async_trait;
use crate::core::*;

pub struct Dnf;

#[async_trait]
impl PackageManager for Dnf {
    fn kind(&self) -> PackageManagerKind {
        PackageManagerKind::Dnf
    }

    fn is_installed(&self) -> bool {
        crate::utils::binary_exists("dnf")
    }

    async fn search(&self, package: &str) -> anyhow::Result<Vec<PackageSearchResult>> {
        let output = crate::utils::async_search("dnf", &["search", package]).await;
        let mut results = Vec::new();
        for line in output.lines() {
            if let Some((name, rest)) = line.split_once('.') {
                if !name.starts_with(' ') && !name.starts_with('=') {
                    let description = rest.trim().to_string();
                    results.push(PackageSearchResult {
                        name: name.trim().to_string(),
                        version: String::new(),
                        description,
                        manager: PackageManagerKind::Dnf,
                    });
                }
            }
        }
        Ok(results)
    }

    async fn install(&self, package: &str) -> anyhow::Result<InstallResult> {
        let start = std::time::Instant::now();
        let output = crate::utils::run_command_combined("dnf", &["install", "-y", package])?;
        if !output.contains("Error:") && !output.contains("No match") {
            Ok(InstallResult {
                package: package.to_string(),
                version: None,
                manager: PackageManagerKind::Dnf,
                dependencies_count: 0,
                disk_usage: None,
                duration_secs: start.elapsed().as_secs(),
            })
        } else {
            Err(anyhow::anyhow!("dnf install failed: {output}"))
        }
    }

    async fn remove(&self, package: &str) -> anyhow::Result<RemoveResult> {
        let output = crate::utils::run_command_combined("dnf", &["remove", "-y", package])?;
        if output.contains("Error:") {
            return Err(anyhow::anyhow!("dnf remove failed: {output}"));
        }
        Ok(RemoveResult { package: package.to_string(), manager: PackageManagerKind::Dnf })
    }

    async fn update(&self, package: Option<&str>) -> anyhow::Result<UpdateResult> {
        if let Some(p) = package {
            let _output = crate::utils::run_command_combined("dnf", &["update", "-y", p])?;
        } else {
            let _output = crate::utils::run_command_combined("dnf", &["upgrade", "-y"])?;
        }
        Ok(UpdateResult { manager: PackageManagerKind::Dnf, updated_count: 0, packages: Vec::new() })
    }

    async fn info(&self, package: &str) -> anyhow::Result<PackageInfo> {
        let output = crate::utils::run_command("dnf", &["info", package])?;
        let mut info = PackageInfo {
            name: package.to_string(),
            version: String::new(),
            description: String::new(),
            maintainer: None,
            homepage: None,
            size: None,
            dependencies: Vec::new(),
            manager: PackageManagerKind::Dnf,
        };
        for line in output.lines() {
            if let Some(val) = line.strip_prefix("Version        : ") {
                info.version = val.to_string();
            } else if let Some(val) = line.strip_prefix("Description    : ") {
                info.description = val.to_string();
            } else if let Some(val) = line.strip_prefix("Maintainer     : ") {
                info.maintainer = Some(val.to_string());
            } else if let Some(val) = line.strip_prefix("URL            : ") {
                info.homepage = Some(val.to_string());
            } else if let Some(val) = line.strip_prefix("Installed Size : ") {
                info.size = Some(val.to_string());
            }
        }
        Ok(info)
    }

    async fn list_installed(&self) -> anyhow::Result<Vec<String>> {
        let output = crate::utils::run_command("dnf", &["list", "installed"])?;
        Ok(output.lines().skip(1).filter_map(|l| {
            l.split_whitespace().next().map(|s| {
                s.split('.').next().unwrap_or(s).to_string()
            })
        }).collect())
    }

    async fn clean(&self) -> anyhow::Result<CleanResult> {
        let _output = crate::utils::run_command_combined("dnf", &["clean", "all"])?;
        Ok(CleanResult { manager: PackageManagerKind::Dnf, freed_bytes: 0 })
    }

    fn version(&self) -> Option<String> {
        crate::utils::run_command("dnf", &["--version"])
            .ok()
            .and_then(|o| o.lines().next().map(|s| s.to_string()))
    }
}

use async_trait::async_trait;
use crate::core::*;

pub struct Apt;

#[async_trait]
impl PackageManager for Apt {
    fn kind(&self) -> PackageManagerKind {
        PackageManagerKind::Apt
    }

    fn is_installed(&self) -> bool {
        crate::utils::binary_exists("apt")
    }

    async fn search(&self, package: &str) -> anyhow::Result<Vec<PackageSearchResult>> {
        let output = crate::utils::async_search("apt", &["search", package]).await;
        let mut results = Vec::new();
        for line in output.lines() {
            if line.ends_with('/') {
                continue;
            }
            if let Some((name, rest)) = line.split_once('/') {
                let description = rest.trim().to_string();
                results.push(PackageSearchResult {
                    name: name.trim().to_string(),
                    version: String::new(),
                    description,
                    manager: PackageManagerKind::Apt,
                });
            }
        }
        Ok(results)
    }

    async fn install(&self, package: &str) -> anyhow::Result<InstallResult> {
        let start = std::time::Instant::now();
        let output = crate::utils::run_command_combined("apt", &["install", "-y", package])?;
        if !output.contains("E:") {
            Ok(InstallResult {
                package: package.to_string(),
                version: None,
                manager: PackageManagerKind::Apt,
                dependencies_count: 0,
                disk_usage: None,
                duration_secs: start.elapsed().as_secs(),
            })
        } else {
            Err(anyhow::anyhow!("apt install failed: {output}"))
        }
    }

    async fn remove(&self, package: &str) -> anyhow::Result<RemoveResult> {
        let output = crate::utils::run_command_combined("apt", &["remove", "-y", package])?;
        if output.contains("E:") {
            return Err(anyhow::anyhow!("apt remove failed: {output}"));
        }
        Ok(RemoveResult { package: package.to_string(), manager: PackageManagerKind::Apt })
    }

    async fn update(&self, package: Option<&str>) -> anyhow::Result<UpdateResult> {
        if package.is_some() {
            let _output = crate::utils::run_command_combined("apt", &["install", "--only-upgrade", "-y", package.unwrap()])?;
        } else {
            let _output = crate::utils::run_command_combined("apt", &["upgrade", "-y"])?;
        }
        Ok(UpdateResult {
            manager: PackageManagerKind::Apt,
            updated_count: 0,
            packages: Vec::new(),
        })
    }

    async fn info(&self, package: &str) -> anyhow::Result<PackageInfo> {
        let output = crate::utils::run_command("apt", &["show", package])?;
        let mut info = PackageInfo {
            name: package.to_string(),
            version: String::new(),
            description: String::new(),
            maintainer: None,
            homepage: None,
            size: None,
            dependencies: Vec::new(),
            manager: PackageManagerKind::Apt,
        };
        for line in output.lines() {
            if let Some(val) = line.strip_prefix("Version: ") {
                info.version = val.to_string();
            } else if let Some(val) = line.strip_prefix("Description: ") {
                info.description = val.to_string();
            } else if let Some(val) = line.strip_prefix("Maintainer: ") {
                info.maintainer = Some(val.to_string());
            } else if let Some(val) = line.strip_prefix("Homepage: ") {
                info.homepage = Some(val.to_string());
            } else if let Some(val) = line.strip_prefix("Installed-Size: ") {
                info.size = Some(format!("{val} kB"));
            } else if let Some(val) = line.strip_prefix("Depends: ") {
                info.dependencies = val.split(", ").map(|s| {
                    s.split_whitespace().next().unwrap_or("").to_string()
                }).filter(|s| !s.is_empty()).collect();
            }
        }
        Ok(info)
    }

    async fn list_installed(&self) -> anyhow::Result<Vec<String>> {
        let output = crate::utils::run_command("apt", &["list", "--installed"])?;
        Ok(output.lines().filter_map(|l| {
            l.split_once('/').map(|(name, _)| name.to_string())
        }).collect())
    }

    async fn clean(&self) -> anyhow::Result<CleanResult> {
        let _output = crate::utils::run_command_combined("apt", &["clean"])?;
        Ok(CleanResult { manager: PackageManagerKind::Apt, freed_bytes: 0 })
    }

    fn version(&self) -> Option<String> {
        crate::utils::run_command("apt", &["--version"])
            .ok()
            .and_then(|o| o.lines().next().map(|s| s.to_string()))
    }
}

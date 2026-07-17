use async_trait::async_trait;
use crate::core::*;

pub struct Pacman;

#[async_trait]
impl PackageManager for Pacman {
    fn kind(&self) -> PackageManagerKind {
        PackageManagerKind::Pacman
    }

    fn is_installed(&self) -> bool {
        crate::utils::binary_exists("pacman")
    }

    async fn search(&self, package: &str) -> anyhow::Result<Vec<PackageSearchResult>> {
        let output = crate::utils::async_search("pacman", &["-Ss", package]).await;
        let mut results = Vec::new();
        for line in output.lines() {
            if let Some(rest) = line.strip_prefix("core/")
                .or_else(|| line.strip_prefix("extra/"))
                .or_else(|| line.strip_prefix("community/"))
            {
                if let Some((name, rest)) = rest.split_once(' ') {
                    let version = rest.split_whitespace().next().unwrap_or("").to_string();
                    let description = rest.splitn(2, ' ').nth(1).unwrap_or("").trim().to_string();
                    results.push(PackageSearchResult {
                        name: name.to_string(),
                        version,
                        description,
                        manager: PackageManagerKind::Pacman,
                    });
                }
            }
        }
        Ok(results)
    }

    async fn install(&self, package: &str) -> anyhow::Result<InstallResult> {
        let start = std::time::Instant::now();
        let output = crate::utils::sudo_run_command("pacman", &["-S", "--noconfirm", package])?;
        if !output.contains("error") && !output.contains("failed") {
            Ok(InstallResult {
                package: package.to_string(),
                version: None,
                manager: PackageManagerKind::Pacman,
                dependencies_count: 0,
                disk_usage: None,
                duration_secs: start.elapsed().as_secs(),
            })
        } else {
            Err(anyhow::anyhow!("pacman install failed: {output}"))
        }
    }

    async fn remove(&self, package: &str) -> anyhow::Result<RemoveResult> {
        let output = crate::utils::sudo_run_command("pacman", &["-R", "--noconfirm", package])?;
        if output.contains("error") || output.contains("failed") {
            return Err(anyhow::anyhow!("pacman remove failed: {output}"));
        }
        Ok(RemoveResult {
            package: package.to_string(),
            manager: PackageManagerKind::Pacman,
        })
    }

    async fn update(&self, package: Option<&str>) -> anyhow::Result<UpdateResult> {
        let mut args = vec!["-Syu"];
        let pkg;
        if let Some(p) = package {
            pkg = p.to_string();
            args.push(&pkg);
        }
        let _output = crate::utils::sudo_run_command("pacman", &args)?;
        Ok(UpdateResult {
            manager: PackageManagerKind::Pacman,
            updated_count: 0,
            packages: Vec::new(),
        })
    }

    async fn info(&self, package: &str) -> anyhow::Result<PackageInfo> {
        let output = crate::utils::run_command("pacman", &["-Si", package])?;
        Ok(parse_info(&output, PackageManagerKind::Pacman))
    }

    async fn list_installed(&self) -> anyhow::Result<Vec<String>> {
        let output = crate::utils::run_command("pacman", &["-Q"])?;
        Ok(output.lines().map(|l| {
            l.split_whitespace().next().unwrap_or("").to_string()
        }).filter(|s| !s.is_empty()).collect())
    }

    async fn clean(&self) -> anyhow::Result<CleanResult> {
        let _output = crate::utils::sudo_run_command("pacman", &["-Scc", "--noconfirm"])?;
        Ok(CleanResult {
            manager: PackageManagerKind::Pacman,
            freed_bytes: 0,
        })
    }

    fn version(&self) -> Option<String> {
        crate::utils::run_command("pacman", &["--version"])
            .ok()
            .and_then(|o| o.lines().next().map(|s| s.to_string()))
    }
}

pub(crate) fn parse_info(output: &str, manager: PackageManagerKind) -> PackageInfo {
    let mut name = String::new();
    let mut version = String::new();
    let mut description = String::new();
    let mut maintainer = None;
    let mut homepage = None;
    let mut size = None;
    let mut dependencies = Vec::new();

    for line in output.lines() {
        if let Some(val) = line.strip_prefix("Name            : ") {
            name = val.to_string();
        } else if let Some(val) = line.strip_prefix("Version         : ") {
            version = val.to_string();
        } else if let Some(val) = line.strip_prefix("Description     : ") {
            description = val.to_string();
        } else if let Some(val) = line.strip_prefix("Maintainer      : ") {
            maintainer = Some(val.to_string());
        } else if let Some(val) = line.strip_prefix("URL             : ") {
            homepage = Some(val.to_string());
        } else if let Some(val) = line.strip_prefix("Download Size   : ") {
            size = Some(val.to_string());
        } else if let Some(val) = line.strip_prefix("Depends On      : ") {
            dependencies = val.split_whitespace().map(|s| s.to_string()).collect();
        }
    }

    PackageInfo { name, version, description, maintainer, homepage, size, dependencies, manager }
}

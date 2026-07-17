use async_trait::async_trait;
use crate::core::*;

pub struct Paru;

#[async_trait]
impl PackageManager for Paru {
    fn kind(&self) -> PackageManagerKind {
        PackageManagerKind::Paru
    }

    fn is_installed(&self) -> bool {
        crate::utils::binary_exists("paru")
    }

    async fn search(&self, package: &str) -> anyhow::Result<Vec<PackageSearchResult>> {
        let output = crate::utils::async_search("paru", &["-Ss", package]).await;
        let mut results = Vec::new();
        for line in output.lines() {
            if let Some(rest) = line.strip_prefix("core/")
                .or_else(|| line.strip_prefix("extra/"))
                .or_else(|| line.strip_prefix("aur/"))
            {
                if let Some((name, rest)) = rest.split_once(' ') {
                    let version = rest.split_whitespace().next().unwrap_or("").to_string();
                    let description = rest.splitn(2, ' ').nth(1).unwrap_or("").trim().to_string();
                    results.push(PackageSearchResult {
                        name: name.to_string(),
                        version,
                        description,
                        manager: PackageManagerKind::Paru,
                    });
                }
            }
        }
        Ok(results)
    }

    async fn install(&self, package: &str) -> anyhow::Result<InstallResult> {
        let start = std::time::Instant::now();
        let output = crate::utils::run_command_combined("paru", &["-S", "--noconfirm", package])?;
        if !output.contains("error") && !output.contains("failed") {
            Ok(InstallResult {
                package: package.to_string(),
                version: None,
                manager: PackageManagerKind::Paru,
                dependencies_count: 0,
                disk_usage: None,
                duration_secs: start.elapsed().as_secs(),
            })
        } else {
            Err(anyhow::anyhow!("paru install failed: {output}"))
        }
    }

    async fn remove(&self, package: &str) -> anyhow::Result<RemoveResult> {
        let output = crate::utils::run_command_combined("paru", &["-R", "--noconfirm", package])?;
        if output.contains("error") || output.contains("failed") {
            return Err(anyhow::anyhow!("paru remove failed: {output}"));
        }
        Ok(RemoveResult { package: package.to_string(), manager: PackageManagerKind::Paru })
    }

    async fn update(&self, package: Option<&str>) -> anyhow::Result<UpdateResult> {
        let mut args = vec!["-Syu"];
        let pkg;
        if let Some(p) = package {
            pkg = p.to_string();
            args.push(&pkg);
        }
        let _output = crate::utils::run_command_combined("paru", &args)?;
        Ok(UpdateResult {
            manager: PackageManagerKind::Paru,
            updated_count: 0,
            packages: Vec::new(),
        })
    }

    async fn info(&self, package: &str) -> anyhow::Result<PackageInfo> {
        let output = crate::utils::run_command("paru", &["-Si", package])?;
        Ok(super::pacman::parse_info(&output, PackageManagerKind::Paru))
    }

    async fn list_installed(&self) -> anyhow::Result<Vec<String>> {
        let output = crate::utils::run_command("paru", &["-Q"])?;
        Ok(output.lines().filter_map(|l| l.split_whitespace().next().map(|s| s.to_string())).collect())
    }

    async fn clean(&self) -> anyhow::Result<CleanResult> {
        let _output = crate::utils::run_command_combined("paru", &["-Scc", "--noconfirm"])?;
        Ok(CleanResult { manager: PackageManagerKind::Paru, freed_bytes: 0 })
    }

    fn version(&self) -> Option<String> {
        crate::utils::run_command("paru", &["--version"])
            .ok()
            .and_then(|o| o.lines().next().map(|s| s.to_string()))
    }
}

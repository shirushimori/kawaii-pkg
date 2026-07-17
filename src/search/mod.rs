use crate::core::*;
use crate::detector::Detector;
use crate::ui;
use colored::Colorize;
use futures::future::join_all;
use std::time::Duration;

const TOTAL_SEARCH_TIMEOUT: Duration = Duration::from_secs(5);

/// Check if a manager kind requires sudo for install/remove/update
fn needs_sudo(kind: PackageManagerKind) -> bool {
    matches!(kind,
        PackageManagerKind::Pacman |
        PackageManagerKind::Apt |
        PackageManagerKind::Dnf |
        PackageManagerKind::Yum |
        PackageManagerKind::Zypper |
        PackageManagerKind::Apk
    )
}

/// Ask for sudo credentials if needed, returns true if sudo is available
fn ensure_sudo() -> bool {
    use std::process::Command;
    let status = Command::new("sudo")
        .arg("-v")
        .status();
    match status {
        Ok(s) => s.success(),
        Err(_) => false,
    }
}

fn print_collapsible(title: &str, content: &str) {
    if content.trim().is_empty() {
        return;
    }
    let lines: Vec<&str> = content.lines().collect();
    let short = lines.len() <= 3;
    println!("  {} {}", "▸".dimmed(), title.dimmed());
    if short {
        for line in &lines {
            println!("    {}", line.dimmed());
        }
    } else {
        for line in lines.iter().take(3) {
            println!("    {}", line.dimmed());
        }
        println!("    {} ({} more lines)", "...".dimmed(), lines.len() - 3);
    }
}

pub async fn search(detector: &Detector, package: &str) -> anyhow::Result<()> {
    let managers = detector.all_managers();
    let futures: Vec<_> = managers.iter().map(|m| {
        let pkg = package.to_string();
        let mgr = std::sync::Arc::clone(m);
        async move { (mgr.kind(), mgr.search(&pkg).await) }
    }).collect();

    let results = match tokio::time::timeout(TOTAL_SEARCH_TIMEOUT, join_all(futures)).await {
        Ok(r) => r,
        Err(_) => {
            ui::error("Search timed out.");
            return Ok(());
        }
    };

    let mut found_managers = Vec::new();
    for (kind, result) in &results {
        if let Ok(found) = result {
            if !found.is_empty() {
                found_managers.push(*kind);
            }
        }
    }

    if found_managers.is_empty() {
        ui::error(&format!("'{}' not found in any package manager.", package));
        return Ok(());
    }

    println!();
    println!("  {} {} found in {} manager(s):", package.green().bold(),
        "is".dimmed(), found_managers.len().to_string().cyan());
    println!();
    for mgr in &found_managers {
        ui::success(&mgr.to_string());
    }

    println!();
    use dialoguer::Confirm;
    let install = Confirm::new()
        .with_prompt(format!("  Install '{}'?", package))
        .default(true)
        .interact()?;

    if !install {
        ui::warning("Cancelled.");
        return Ok(());
    }

    let manager_name = if found_managers.len() == 1 {
        found_managers[0]
    } else {
        println!();
        let names: Vec<String> = found_managers.iter().map(|k| k.to_string()).collect();
        let idx = ui::manager_selection(&names);
        found_managers[idx]
    };

    let manager = detector.find_by_name(&manager_name.to_string())
        .ok_or_else(|| anyhow::anyhow!("Manager not found"))?;

    // Ask for sudo if manager needs it
    if needs_sudo(manager_name) {
        ui::info(&format!("{} requires sudo privileges.", manager_name.to_string().yellow()));
        if !ensure_sudo() {
            ui::error("Sudo authentication failed or cancelled.");
            return Ok(());
        }
        ui::success("Sudo authenticated");
    }

    println!();
    ui::info(&format!("Installing {} via {}...", package.white().bold(), manager_name.to_string().yellow().bold()));
    let start = std::time::Instant::now();

    match manager.install(package).await {
        Ok(mut result) => {
            result.duration_secs = start.elapsed().as_secs();

            // Show collapsible output
            println!();
            ui::success(&format!("{} installed successfully.", result.package));
            print_collapsible("Details", &format!(
                "Package:  {}\nManager:  {}\nTime:     {}s",
                result.package, result.manager, result.duration_secs
            ));

            let history_path = crate::history::history_path();
            let entry = HistoryEntry {
                timestamp: chrono::Utc::now(),
                action: "install".to_string(),
                package: package.to_string(),
                manager: manager_name.to_string(),
                success: true,
            };
            let _ = crate::history::append_entry(&history_path, &entry);
        }
        Err(e) => {
            ui::error(&format!("Installation failed: {e}"));
            print_collapsible("Error output", &e.to_string());
            let entry = HistoryEntry {
                timestamp: chrono::Utc::now(),
                action: "install".to_string(),
                package: package.to_string(),
                manager: manager_name.to_string(),
                success: false,
            };
            let history_path = crate::history::history_path();
            let _ = crate::history::append_entry(&history_path, &entry);
        }
    }

    Ok(())
}

pub async fn install(detector: &Detector, package: &str) -> anyhow::Result<()> {
    let managers = detector.all_managers();
    let futures: Vec<_> = managers.iter().map(|m| {
        let pkg = package.to_string();
        let mgr = std::sync::Arc::clone(m);
        async move { (mgr.kind(), mgr.search(&pkg).await) }
    }).collect();

    let results = match tokio::time::timeout(TOTAL_SEARCH_TIMEOUT, join_all(futures)).await {
        Ok(r) => r,
        Err(_) => {
            ui::error("Search timed out.");
            return Ok(());
        }
    };

    let mut found_in = Vec::new();
    for (kind, result) in results {
        if let Ok(found) = result {
            if !found.is_empty() {
                found_in.push(kind);
            }
        }
    }

    if found_in.is_empty() {
        ui::error(&format!("'{}' not found in any package manager.", package));
        return Ok(());
    }

    println!();
    let manager_name = if found_in.len() == 1 {
        found_in[0]
    } else {
        println!("  {} {} found in {} managers:", package.green().bold(),
            "is".dimmed(), found_in.len().to_string().cyan());
        println!();
        let names: Vec<String> = found_in.iter().map(|k| k.to_string()).collect();
        let idx = ui::manager_selection(&names);
        found_in[idx]
    };

    let manager = detector.find_by_name(&manager_name.to_string())
        .ok_or_else(|| anyhow::anyhow!("Manager not found"))?;

    // Ask for sudo if manager needs it
    if needs_sudo(manager_name) {
        ui::info(&format!("{} requires sudo privileges.", manager_name.to_string().yellow()));
        if !ensure_sudo() {
            ui::error("Sudo authentication failed or cancelled.");
            return Ok(());
        }
        ui::success("Sudo authenticated");
    }

    println!();
    ui::info(&format!("Installing {} via {}...", package.white().bold(), manager_name.to_string().yellow().bold()));
    let start = std::time::Instant::now();

    match manager.install(package).await {
        Ok(mut result) => {
            result.duration_secs = start.elapsed().as_secs();

            println!();
            ui::success(&format!("{} installed successfully.", result.package));
            print_collapsible("Details", &format!(
                "Package:  {}\nManager:  {}\nTime:     {}s",
                result.package, result.manager, result.duration_secs
            ));

            let history_path = crate::history::history_path();
            let entry = HistoryEntry {
                timestamp: chrono::Utc::now(),
                action: "install".to_string(),
                package: package.to_string(),
                manager: manager_name.to_string(),
                success: true,
            };
            let _ = crate::history::append_entry(&history_path, &entry);
        }
        Err(e) => {
            ui::error(&format!("Installation failed: {e}"));
            print_collapsible("Error output", &e.to_string());
            let entry = HistoryEntry {
                timestamp: chrono::Utc::now(),
                action: "install".to_string(),
                package: package.to_string(),
                manager: manager_name.to_string(),
                success: false,
            };
            let history_path = crate::history::history_path();
            let _ = crate::history::append_entry(&history_path, &entry);
        }
    }

    Ok(())
}

pub async fn remove(detector: &Detector, package: &str, yes: bool) -> anyhow::Result<()> {
    if !yes {
        use dialoguer::Confirm;
        let confirmed = Confirm::new()
            .with_prompt(format!("Remove '{}'?", package))
            .default(false)
            .interact()?;
        if !confirmed {
            ui::warning("Operation cancelled.");
            return Ok(());
        }
    }

    let manager = detector.primary_manager()
        .ok_or_else(|| anyhow::anyhow!("No package manager found"))?;

    // Ask for sudo if manager needs it
    if needs_sudo(manager.kind()) {
        ui::info(&format!("{} requires sudo privileges.", manager.kind().to_string().yellow()));
        if !ensure_sudo() {
            ui::error("Sudo authentication failed or cancelled.");
            return Ok(());
        }
        ui::success("Sudo authenticated");
    }

    ui::info(&format!("Removing '{}' via {}...", package, manager.kind()));
    let start = std::time::Instant::now();

    match manager.remove(package).await {
        Ok(_) => {
            let elapsed = start.elapsed().as_secs();
            ui::success(&format!("'{}' removed successfully.", package));
            print_collapsible("Details", &format!(
                "Package:  {}\nManager:  {}\nTime:     {}s",
                package, manager.kind(), elapsed
            ));
            let entry = HistoryEntry {
                timestamp: chrono::Utc::now(),
                action: "remove".to_string(),
                package: package.to_string(),
                manager: manager.kind().to_string(),
                success: true,
            };
            let history_path = crate::history::history_path();
            let _ = crate::history::append_entry(&history_path, &entry);
        }
        Err(e) => {
            ui::error(&format!("Removal failed: {e}"));
            print_collapsible("Error output", &e.to_string());
        }
    }

    Ok(())
}

pub async fn update(detector: &Detector, package: Option<&str>) -> anyhow::Result<()> {
    let manager = detector.primary_manager()
        .ok_or_else(|| anyhow::anyhow!("No package manager found"))?;

    // Ask for sudo if manager needs it
    if needs_sudo(manager.kind()) {
        ui::info(&format!("{} requires sudo privileges.", manager.kind().to_string().yellow()));
        if !ensure_sudo() {
            ui::error("Sudo authentication failed or cancelled.");
            return Ok(());
        }
        ui::success("Sudo authenticated");
    }

    if let Some(pkg) = package {
        ui::info(&format!("Updating '{}' via {}...", pkg, manager.kind()));
    } else {
        ui::header("Updating all packages...");
    }

    let start = std::time::Instant::now();

    match manager.update(package).await {
        Ok(result) => {
            let elapsed = start.elapsed().as_secs();
            ui::success(&format!("Update completed via {}", result.manager));
            print_collapsible("Details", &format!(
                "Manager:  {}\nTime:     {}s",
                result.manager, elapsed
            ));
            let entry = HistoryEntry {
                timestamp: chrono::Utc::now(),
                action: "update".to_string(),
                package: package.unwrap_or("all").to_string(),
                manager: result.manager.to_string(),
                success: true,
            };
            let history_path = crate::history::history_path();
            let _ = crate::history::append_entry(&history_path, &entry);
        }
        Err(e) => {
            ui::error(&format!("Update failed: {e}"));
            print_collapsible("Error output", &e.to_string());
        }
    }

    Ok(())
}

pub async fn info(detector: &Detector, package: &str) -> anyhow::Result<()> {
    let manager = detector.primary_manager()
        .ok_or_else(|| anyhow::anyhow!("No package manager found"))?;

    match manager.info(package).await {
        Ok(info) => {
            println!();
            ui::header(&format!("Package: {}", info.name));
            ui::separator();
            println!("  {:<20} {}", "Version:".dimmed(), info.version.white());
            println!("  {:<20} {}", "Description:".dimmed(), info.description.white());
            if let Some(ref m) = info.maintainer {
                println!("  {:<20} {}", "Maintainer:".dimmed(), m.white());
            }
            if let Some(ref h) = info.homepage {
                println!("  {:<20} {}", "Homepage:".dimmed(), h.white());
            }
            if let Some(ref s) = info.size {
                println!("  {:<20} {}", "Size:".dimmed(), s.white());
            }
            if !info.dependencies.is_empty() {
                println!("  {:<20} {}", "Dependencies:".dimmed(), info.dependencies.len().to_string().white());
            }
            println!("  {:<20} {}", "Manager:".dimmed(), info.manager.to_string().yellow());
            ui::separator();
            println!();
        }
        Err(e) => {
            ui::error(&format!("Could not get info: {e}"));
        }
    }

    Ok(())
}

pub async fn list(detector: &Detector) -> anyhow::Result<()> {
    ui::header("Installed packages:");
    println!();

    for manager in detector.all_managers() {
        match manager.list_installed().await {
            Ok(packages) => {
                ui::success(&format!("{} ({} packages)", manager.kind(), packages.len()));
                for pkg in packages.iter().take(20) {
                    ui::info(pkg);
                }
                if packages.len() > 20 {
                    ui::info(&format!("... and {} more", packages.len() - 20));
                }
                println!();
            }
            Err(e) => {
                ui::error(&format!("{}: {e}", manager.kind()));
            }
        }
    }

    Ok(())
}

pub async fn clean(detector: &Detector) -> anyhow::Result<()> {
    ui::header("Cleaning caches...");
    println!();

    for manager in detector.all_managers() {
        // Ask for sudo if manager needs it
        if needs_sudo(manager.kind()) {
            ui::info(&format!("{} requires sudo for cache cleanup.", manager.kind().to_string().yellow()));
            if !ensure_sudo() {
                ui::warning(&format!("Skipping {} (no sudo)", manager.kind()));
                continue;
            }
        }

        match manager.clean().await {
            Ok(_) => ui::success(&format!("{} cleaned", manager.kind())),
            Err(e) => ui::error(&format!("{}: {e}", manager.kind())),
        }
    }

    ui::success("Cache cleanup complete.");
    Ok(())
}

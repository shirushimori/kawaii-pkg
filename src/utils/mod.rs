use std::process::Command;
use anyhow::{Context, Result};
use tracing::debug;
use std::time::Duration;

/// Run a command and return its stdout
pub fn run_command(bin: &str, args: &[&str]) -> Result<String> {
    debug!("Running: {} {}", bin, args.join(" "));
    let output = Command::new(bin)
        .args(args)
        .output()
        .with_context(|| format!("Failed to execute '{bin}'"))?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Run a command and return stdout + stderr combined
pub fn run_command_combined(bin: &str, args: &[&str]) -> Result<String> {
    debug!("Running: {} {}", bin, args.join(" "));
    let output = Command::new(bin)
        .args(args)
        .output()
        .with_context(|| format!("Failed to execute '{bin}'"))?;
    let mut result = String::from_utf8_lossy(&output.stdout).to_string();
    result.push_str(&String::from_utf8_lossy(&output.stderr));
    Ok(result)
}

/// Run a command and return whether it succeeded
pub fn run_command_status(bin: &str, args: &[&str]) -> Result<bool> {
    debug!("Running: {} {}", bin, args.join(" "));
    let status = Command::new(bin)
        .args(args)
        .status()
        .with_context(|| format!("Failed to execute '{bin}'"))?;
    Ok(status.success())
}

/// Check if a binary exists on PATH
pub fn binary_exists(name: &str) -> bool {
    which::which(name).is_ok()
}

/// Run a command with sudo and return stdout + stderr combined
pub fn sudo_run_command(bin: &str, args: &[&str]) -> Result<String> {
    debug!("Running: sudo {} {}", bin, args.join(" "));
    let output = Command::new("sudo")
        .arg(bin)
        .args(args)
        .output()
        .with_context(|| format!("Failed to execute 'sudo {bin}'"))?;
    let mut result = String::from_utf8_lossy(&output.stdout).to_string();
    result.push_str(&String::from_utf8_lossy(&output.stderr));
    Ok(result)
}

const SEARCH_TIMEOUT: Duration = Duration::from_secs(3);

/// Async search with timeout — runs a command and returns stdout, or empty string on timeout/error
pub async fn async_search(bin: &str, args: &[&str]) -> String {
    let args_owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    let bin = bin.to_string();
    match tokio::time::timeout(SEARCH_TIMEOUT, tokio::process::Command::new(&bin)
        .args(&args_owned)
        .output()
    ).await {
        Ok(Ok(o)) => String::from_utf8_lossy(&o.stdout).to_string(),
        _ => String::new(),
    }
}

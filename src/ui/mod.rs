use colored::*;
use console::style;

pub fn header(text: &str) {
    println!("{}", style(text).cyan().bold());
}

pub fn success(text: &str) {
    println!("  {} {}", "✓".green().bold(), text);
}

pub fn error(text: &str) {
    println!("  {} {}", "✗".red().bold(), text);
}

pub fn warning(text: &str) {
    println!("  {} {}", "!".yellow().bold(), text);
}

pub fn info(text: &str) {
    println!("  {} {}", "→".blue().bold(), text);
}

pub fn banner() {
    let banner = r#"
  ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
  ┃        ⬡ Kawaii Manager ⬡         ┃
  ┃   One command. Every package.      ┃
  ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛"#;
    println!("{}", banner.cyan());
}

pub fn separator() {
    println!("{}", "─".dimmed());
}

pub fn install_summary(result: &crate::core::InstallResult) {
    println!();
    separator();
    println!("  {} {}", "✓".green().bold(), format!("{} installed successfully.", result.package).green());
    println!("  {:<16} {}", "Package:".dimmed(), result.package.white().bold());
    if let Some(ref v) = result.version {
        println!("  {:<16} {}", "Version:".dimmed(), v.white());
    }
    println!("  {:<16} {}", "Manager:".dimmed(), result.manager.to_string().yellow().bold());
    println!("  {:<16} {}s", "Time:".dimmed(), result.duration_secs.to_string().white());
    separator();
    println!();
}

pub fn manager_selection(managers: &[String]) -> usize {
    use dialoguer::FuzzySelect;

    let selection = FuzzySelect::new()
        .with_prompt("  Choose package manager")
        .items(managers)
        .default(0)
        .interact()
        .unwrap_or(0);

    selection
}

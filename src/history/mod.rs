use crate::core::HistoryEntry;
use std::path::PathBuf;

pub fn history_path() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("kawaii")
        .join("history.json")
}

pub fn load_entries(path: &PathBuf) -> Vec<HistoryEntry> {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn save_entries(path: &PathBuf, entries: &[HistoryEntry]) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, serde_json::to_string_pretty(entries)?)?;
    Ok(())
}

pub fn append_entry(path: &PathBuf, entry: &HistoryEntry) -> anyhow::Result<()> {
    let mut entries = load_entries(path);
    entries.push(entry.clone());
    save_entries(path, &entries)
}

pub fn show_history() {
    use crate::ui;
    use colored::*;

    let path = history_path();
    let entries = load_entries(&path);

    if entries.is_empty() {
        ui::info("No history yet.");
        return;
    }

    ui::header("Command History");
    ui::separator();

    for entry in entries.iter().rev().take(30) {
        let time = entry.timestamp.format("%Y-%m-%d %H:%M");
        let action_color = if entry.success {
            entry.action.green()
        } else {
            entry.action.red()
        };
        println!(
            "  {} {} {} {}",
            time.to_string().dimmed(),
            action_color,
            entry.package.white(),
            format!("({})", entry.manager).yellow(),
        );
    }
    ui::separator();
}

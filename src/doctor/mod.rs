use crate::detector::Detector;
use crate::ui;

pub struct Doctor {
    issues: Vec<String>,
    warnings: Vec<String>,
}

impl Doctor {
    pub fn new() -> Self {
        Self { issues: Vec::new(), warnings: Vec::new() }
    }

    pub fn run(&mut self, detector: &Detector) {
        ui::header("Running diagnostics...");
        ui::separator();

        self.check_managers(detector);
        self.check_broken_packages();
        self.check_mirror_list();
        self.check_cache();
        self.check_config();

        self.print_report();
    }

    fn check_managers(&mut self, detector: &Detector) {
        ui::info("Checking package managers...");
        let installed = detector.manager_names();

        if installed.is_empty() {
            self.issues.push("No package managers found.".to_string());
        } else {
            for name in &installed {
                ui::success(&format!("{name} found"));
            }
        }

        let has_aur = installed.iter().any(|n| n == "yay" || n == "paru");
        let has_pacman = installed.contains(&"pacman".to_string());
        if has_pacman && !has_aur {
            self.warnings.push("Consider installing yay or paru for AUR support.".to_string());
        }
    }

    fn check_broken_packages(&mut self) {
        ui::info("Checking for broken packages...");
        if crate::utils::binary_exists("pacman") {
            // Check for broken deps (read-only, no sudo needed)
            let output = crate::utils::run_command("pacman", &["-Dk"]);
            match output {
                Ok(out) if out.contains("error") => {
                    ui::warning("Broken packages found. Fixing...");
                    // Actually fix with sudo
                    let fix = crate::utils::sudo_run_command("pacman", &["-Dk", "--noconfirm"]);
                    match fix {
                        Ok(fix_out) => {
                            if fix_out.contains("error") {
                                self.issues.push(format!("Could not fix all broken packages. Output: {}", fix_out.trim()));
                            } else {
                                ui::success("Broken packages fixed");
                            }
                        }
                        Err(e) => {
                            self.issues.push(format!("Failed to run sudo pacman -Dk: {e}"));
                        }
                    }
                }
                Ok(_) => {
                    ui::success("No broken packages");
                }
                Err(e) => {
                    self.warnings.push(format!("Could not check packages: {e}"));
                }
            }
        }
    }

    fn check_mirror_list(&mut self) {
        ui::info("Checking mirror list...");
        if crate::utils::binary_exists("pacman") {
            ui::info("Syncing databases...");
            let output = crate::utils::sudo_run_command("pacman", &["-Syy", "--noconfirm"]);
            match output {
                Ok(out) => {
                    if out.contains("error") || out.contains("failed") {
                        self.warnings.push("Mirror sync issues detected.".to_string());
                    } else {
                        ui::success("Databases synced");
                    }
                }
                Err(e) => {
                    self.warnings.push(format!("Mirror sync failed: {e}"));
                }
            }
        }
    }

    fn check_cache(&mut self) {
        ui::info("Checking cache...");
        if let Some(cache_dir) = dirs::cache_dir() {
            let kawaii_cache = cache_dir.join("kawaii");
            if kawaii_cache.exists() {
                ui::success("Kawaii cache exists");
            } else {
                ui::info("No kawaii cache yet");
            }
        }
    }

    fn check_config(&mut self) {
        use crate::config::Config;
        ui::info("Checking configuration...");
        match Config::load() {
            config if config.search_order.is_empty() => {
                self.warnings.push("Search order is empty.".to_string());
            }
            _ => {
                ui::success("Configuration OK");
            }
        }
    }

    fn print_report(&self) {
        ui::separator();
        if self.issues.is_empty() && self.warnings.is_empty() {
            ui::success("All checks passed!");
        } else {
            for issue in &self.issues {
                ui::error(issue);
            }
            for warn in &self.warnings {
                ui::warning(warn);
            }
        }
        ui::separator();
    }
}

impl Default for Doctor {
    fn default() -> Self {
        Self::new()
    }
}

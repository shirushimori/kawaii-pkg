use crate::core::{PackageManager, PackageManagerKind};
use crate::managers::create_manager;
use std::sync::Arc;
use tracing::info;

pub struct Detector {
    managers: Vec<Arc<dyn PackageManager>>,
}

impl Detector {
    pub fn new() -> Self {
        info!("Scanning for installed package managers...");
        let kinds = PackageManagerKind::all().to_vec();

        // Check all binaries in parallel using threads
        let installed: Vec<PackageManagerKind> = std::thread::scope(|s| {
            let handles: Vec<_> = kinds
                .iter()
                .map(|kind| {
                    s.spawn(|| {
                        let name = kind.binary_name().to_string();
                        let k = *kind;
                        (k, which::which(&name).is_ok())
                    })
                })
                .collect();

            handles
                .into_iter()
                .filter_map(|h| h.join().ok())
                .filter(|(_, installed)| *installed)
                .map(|(kind, _)| kind)
                .collect()
        });

        let managers: Vec<Arc<dyn PackageManager>> = installed
            .into_iter()
            .map(create_manager)
            .collect();

        info!("Found {} installed package managers", managers.len());
        Self { managers }
    }

    pub fn all_managers(&self) -> &[Arc<dyn PackageManager>] {
        &self.managers
    }

    pub fn find_by_name(&self, name: &str) -> Option<Arc<dyn PackageManager>> {
        self.managers
            .iter()
            .find(|m| m.kind().to_string() == name)
            .cloned()
    }

    pub fn manager_names(&self) -> Vec<String> {
        self.managers.iter().map(|m| m.kind().to_string()).collect()
    }

    pub fn primary_manager(&self) -> Option<Arc<dyn PackageManager>> {
        let preferred_order = [
            PackageManagerKind::Yay,
            PackageManagerKind::Paru,
            PackageManagerKind::Pacman,
            PackageManagerKind::Dnf,
            PackageManagerKind::Apt,
            PackageManagerKind::Zypper,
            PackageManagerKind::Xbps,
            PackageManagerKind::Nix,
            PackageManagerKind::Apk,
            PackageManagerKind::Brew,
            PackageManagerKind::Flatpak,
            PackageManagerKind::Snap,
        ];
        for kind in &preferred_order {
            if let Some(m) = self.managers.iter().find(|m| m.kind() == *kind) {
                return Some(Arc::clone(m));
            }
        }
        self.managers.first().cloned()
    }
}

impl Default for Detector {
    fn default() -> Self {
        Self::new()
    }
}

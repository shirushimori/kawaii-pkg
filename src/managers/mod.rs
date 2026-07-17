pub mod apt;
pub mod apk;
pub mod brew;
pub mod dnf;
pub mod flatpak;
pub mod nix;
pub mod pacman;
pub mod paru;
pub mod snap;
pub mod xbps;
pub mod yay;
pub mod yum;
pub mod zypper;

use crate::core::{PackageManager, PackageManagerKind};
use std::sync::Arc;

/// Create a boxed PackageManager for the given kind
pub fn create_manager(kind: PackageManagerKind) -> Arc<dyn PackageManager> {
    match kind {
        PackageManagerKind::Pacman => Arc::new(pacman::Pacman),
        PackageManagerKind::Yay => Arc::new(yay::Yay),
        PackageManagerKind::Paru => Arc::new(paru::Paru),
        PackageManagerKind::Apt => Arc::new(apt::Apt),
        PackageManagerKind::Dnf => Arc::new(dnf::Dnf),
        PackageManagerKind::Yum => Arc::new(yum::Yum),
        PackageManagerKind::Zypper => Arc::new(zypper::Zypper),
        PackageManagerKind::Xbps => Arc::new(xbps::Xbps),
        PackageManagerKind::Nix => Arc::new(nix::Nix),
        PackageManagerKind::Apk => Arc::new(apk::Apk),
        PackageManagerKind::Brew => Arc::new(brew::Brew),
        PackageManagerKind::Flatpak => Arc::new(flatpak::Flatpak),
        PackageManagerKind::Snap => Arc::new(snap::Snap),
    }
}

// Package Module - Package manager
pub mod manager;

pub use manager::{PackageManager, Package, VersionConstraint, PackageError, LockFile};

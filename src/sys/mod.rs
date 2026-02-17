//! Platform-specific system icon metadata.
//!
//! This module provides platform-specific information about system folder icons,
//! such as content bounds (the region within an icon image that contains the
//! actual visual content, excluding padding/margins).

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "linux")]
pub mod linux;

// Re-export the platform-specific implementation under a common alias
#[cfg(target_os = "windows")]
pub use windows::get_folder_icon_content_bounds;
#[cfg(target_os = "windows")]
pub use windows::SURFACE_COLOR;

#[cfg(target_os = "macos")]
pub use macos::get_folder_icon_content_bounds;

#[cfg(target_os = "linux")]
pub use linux::get_folder_icon_content_bounds;

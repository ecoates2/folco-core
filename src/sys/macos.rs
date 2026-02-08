//! macOS-specific system icon metadata.

use folco_renderer::RectPx;

/// Returns the content bounds for a macOS system folder icon.
///
/// macOS folder icons may have specific content regions depending on
/// the icon size and scale factor.
///
/// # Arguments
///
/// * `width` - The width of the icon image in pixels
/// * `height` - The height of the icon image in pixels
///
/// # Returns
///
/// A `RectPx` describing the region containing the actual icon content.
pub fn get_folder_icon_content_bounds(width: u32, height: u32) -> RectPx {
    // TODO: Determine actual content bounds for macOS folder icons
    unimplemented!(
        "macOS folder icon content bounds not yet implemented for {}x{}",
        width,
        height
    )
}

#[cfg(test)]
mod tests {
    // Tests will be added once bounds are implemented
}

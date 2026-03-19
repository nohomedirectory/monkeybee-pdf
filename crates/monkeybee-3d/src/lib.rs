//! Monkeybee PDF 3D: PRC/U3D parsing, scene graph construction, and 3D rendering surfaces.

/// Placeholder 3D engine surface until the dedicated PRC/U3D implementation lands.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ThreeDStatus;

impl ThreeDStatus {
    /// Returns the placeholder status for the current crate revision.
    pub fn placeholder() -> Self {
        Self
    }
}

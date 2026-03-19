//! Monkeybee PDF GPU backend: optional GPU-accelerated 2D rendering surfaces.

/// Placeholder GPU backend surface until the wgpu implementation lands.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct GpuBackendStatus;

impl GpuBackendStatus {
    /// Returns the placeholder status for the current crate revision.
    pub fn placeholder() -> Self {
        Self
    }
}

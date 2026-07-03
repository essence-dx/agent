pub mod protocol;

// TODO(openclaw): re-enable when openclaw sidecar is ready for shipping
// #[cfg(feature = "openclaw")]
// pub mod openclaw;

// Native Rust hermes auto-update — replaces the Python sidecar.
#[cfg(feature = "hermes")]
pub mod hermes;

// TODO(deerflow): re-enable when deer-flow sidecar is ready for shipping
// #[cfg(feature = "deerflow")]
// pub mod deerflow;

// TODO(sidecars): re-export once any sidecar module is un-commented
// pub use protocol::SidecarProcess;

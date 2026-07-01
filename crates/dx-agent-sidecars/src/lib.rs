pub mod protocol;

#[cfg(feature = "openclaw")]
pub mod openclaw;

#[cfg(feature = "hermes")]
pub mod hermes;

#[cfg(feature = "deerflow")]
pub mod deerflow;

pub use protocol::SidecarProcess;

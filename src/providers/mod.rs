//! ModelProvider subsystem — re-exported from `dx-agent-providers`.

pub use dx_agent_providers::*;

// Keep traits.rs as a file module so its #[cfg(test)] block compiles.
#[path = "traits.rs"]
pub mod traits;

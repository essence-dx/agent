// ═══════════════════════════════════════════════════════════════════════════
// Deer-Flow Sidecar — COMMENTED OUT for shipping focus.
// Re-enable when deer-flow integration is ready:
//   1. Uncomment `pub mod deerflow;` in lib.rs
//   2. Ensure the `deerflow` feature flag is set
//   3. Point `spawn()` at the correct binary path
//
// The sidecar uses JSON-RPC 2.0 over stdio to communicate with an external
// Deer-Flow process (Python) for lead agent factory, middleware chain,
// subagent executor, sandbox, and run management.
// ═══════════════════════════════════════════════════════════════════════════

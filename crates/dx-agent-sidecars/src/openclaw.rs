// ═══════════════════════════════════════════════════════════════════════════
// OpenClaw Sidecar — COMMENTED OUT for shipping focus.
// Re-enable when openclaw integration is ready:
//   1. Uncomment `pub mod openclaw;` in lib.rs
//   2. Ensure the `openclaw` feature flag is set
//   3. Point `spawn()` at the correct binary path
//
// The sidecar uses JSON-RPC 2.0 over stdio to communicate with an external
// OpenClaw process (Node.js) for channel adapters, pairing, security,
// secrets, and protocol schemas.
// ═══════════════════════════════════════════════════════════════════════════

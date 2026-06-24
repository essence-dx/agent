# API Reference

Full rustdoc for every public type in the workspace, auto-generated from the `///` comments on each type, function, and module. Use this when you need to know the exact shape of a struct, the methods on a trait, or what a function returns — anything the generated reference exposes better than prose can.

**[Open the rustdoc →](../api/zeroclaw/index.html)**

## How to navigate it

- The sidebar on the left lists every crate in the workspace
- Click `dx-agent-api` first — that's where the public traits (`Provider`, `Channel`, `Tool`) live
- Use `cmd/ctrl+F` in the rustdoc page to search within a crate
- Click on any trait to see implementors across the workspace

## Crate index

| Crate | What it exposes |
|---|---|
| [`zeroclaw`](../api/zeroclaw/index.html) | Top-level umbrella with re-exports |
| [`dx-agent-api`](../api/dx_agent_api/index.html) | Public traits: `Provider`, `Channel`, `Tool`, `StreamEvent` |
| [`dx-agent-config`](../api/dx_agent_config/index.html) | Config schema, autonomy types, secrets |
| [`dx-agent-runtime`](../api/dx_agent_runtime/index.html) | Agent loop, security, SOP, onboarding |
| [`dx-agent-providers`](../api/dx_agent_providers/index.html) | Every LLM-provider implementation |
| [`dx-agent-channels`](../api/dx_agent_channels/index.html) | Messaging integrations |
| [`dx-agent-gateway`](../api/dx_agent_gateway/index.html) | HTTP/WebSocket gateway |
| [`dx-agent-tools`](../api/dx_agent_tools/index.html) | Agent-callable tools |
| [`dx-agent-memory`](../api/dx_agent_memory/index.html) | Conversation memory, embeddings |
| [`dx-agent-plugins`](../api/dx_agent_plugins/index.html) | WASM plugin host |
| [`dx-agent-hardware`](../api/dx_agent_hardware/index.html) | GPIO / I2C / SPI / USB |
| [`dx-agent-infra`](../api/dx_agent_infra/index.html) | Tracing, metrics |

See [Architecture → Crates](./architecture/crates.md) for a plain-English description of how the crates fit together.

## Regenerating the API reference

The rustdoc ships with every doc deploy. For local builds:

```bash
cargo mdbook refs     # generates CLI + config reference + rustdoc
cargo mdbook build    # rebuilds the full book including rustdoc bridge
```

See [Maintainers → Docs & Translations](./maintainers/docs-and-translations.md).

//! V0.8.0 env-var override mechanism.
//!
//! Grammar: `DX_AGENTS_<dotted_path_with_double_underscores>=<value>`.
//! Each `__` (double underscore) is a path separator (`.` in the TOML); each
//! single `_` is either a snake-case joiner inside a field name (which the
//! walker converts to kebab `-` for `set_prop`) or a literal char inside an
//! alias key.
//! The legacy `DX_AGENT_` prefix remains accepted for migration, but the DX
//! prefix wins when both names target the same schema path.
//!
//! Schema-derived: [`map_key_sections`] gives HashMap positions (one alias
//! token consumed; alias chars are `[a-z0-9_]`); [`prop_fields`] gives every
//! other leaf path. No string-literal pattern matching, no hardcoded family
//! names.
//!
//! Bootstrap exception: `DX_AGENTS_CONFIG_DIR`, `DX_AGENTS_DATA_DIR`,
//! `DX_AGENTS_WORKSPACE`, and their legacy `DX_AGENT_*` aliases keep their
//! UPPERCASE form. The case rule (lowercase tail = config-tree,
//! uppercase tail = bootstrap) does the disambiguation work without an
//! exemption list.
//!
//! Persistence boundary: each overridden path's pre-override raw value is
//! snapshotted (post-`decrypt_secrets`, so secrets are plaintext) and used
//! by [`mask_env_overrides_for_save`] to restore disk-or-default values
//! before `encrypt_secrets()` runs. Env-injected values never reach disk.
//!
//! [`map_key_sections`]: crate::schema::Config::map_key_sections
//! [`prop_fields`]: crate::schema::Config::prop_fields

use crate::schema::Config;
use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

const PRIMARY_PREFIX: &str = "DX_AGENTS_";
const LEGACY_PREFIX: &str = "DX_AGENT_";
const SEP: &str = "__";

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum EnvPrefix {
    Legacy,
    Primary,
}

impl EnvPrefix {
    const fn priority(self) -> u8 {
        match self {
            Self::Legacy => 0,
            Self::Primary => 1,
        }
    }
}

#[derive(Debug)]
struct EnvOverrideCandidate {
    env_name: String,
    value: String,
    tail: String,
    prefix: EnvPrefix,
}

#[derive(Debug)]
struct ResolvedEnvOverride {
    env_name: String,
    value: String,
    path: String,
    prefix: EnvPrefix,
}

/// Paths that the schema exposes via `prop_fields()` but that operators must
/// not override at runtime. Currently just `schema_version` (snake form, as
/// emitted by `prop_fields()`) — the migration engine sets it from the
/// on-disk file's value, and an env override would either skip needed
/// migrations or trigger a no-op rerun. O(1) HashSet lookup so adding more
/// reserved paths stays cheap.
static NON_OVERRIDABLE_PATHS: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| HashSet::from(["schema_version"]));

/// Outcome of [`apply_env_overrides`]: the set of overridden paths plus the
/// per-path snapshot of pre-override raw values. The snapshot drives
/// [`mask_env_overrides_for_save`] so secret fields recover their original
/// plaintext (which `encrypt_secrets()` then re-encrypts), and non-secret
/// fields recover their disk-or-default value.
#[derive(Debug, Default, Clone)]
pub struct AppliedOverrides {
    pub paths: HashSet<String>,
    pub snapshots: HashMap<String, String>,
}

/// Apply every schema-mirror env var to `config`. Returns the set of
/// dotted prop-paths that were overridden plus the pre-override raw values
/// for each. Hard-errors on any env var that doesn't resolve to a known
/// schema path or whose alias fails validation.
pub fn apply_env_overrides(config: &mut Config) -> Result<AppliedOverrides> {
    let mut entries: Vec<EnvOverrideCandidate> = std::env::vars()
        .filter_map(|(key, value)| candidate_from_env_var(key, value))
        .collect();
    entries.sort_by(|a, b| a.env_name.cmp(&b.env_name));

    let mut selected: HashMap<String, ResolvedEnvOverride> = HashMap::new();
    for candidate in entries {
        let path = resolve_path(&candidate.tail, config)
            .with_context(|| format!("{} did not resolve to a schema path", candidate.env_name))?;
        if NON_OVERRIDABLE_PATHS.contains(path.as_str()) {
            ::dx_agent_log::record!(
                WARN,
                ::dx_agent_log::Event::new(module_path!(), ::dx_agent_log::Action::Reject)
                    .with_outcome(::dx_agent_log::EventOutcome::Failure)
                    .with_attrs(::serde_json::json!({"env_var": candidate.env_name, "path": path})),
                "env override rejected: field is not overridable"
            );
            anyhow::bail!(
                "{} -> {path}: this field is not overridable via env vars",
                candidate.env_name
            );
        }

        match selected.get(&path) {
            Some(existing) if existing.prefix.priority() > candidate.prefix.priority() => {}
            Some(existing) if existing.prefix.priority() == candidate.prefix.priority() => {}
            _ => {
                selected.insert(
                    path.clone(),
                    ResolvedEnvOverride {
                        env_name: candidate.env_name,
                        value: candidate.value,
                        path,
                        prefix: candidate.prefix,
                    },
                );
            }
        }
    }

    let mut entries: Vec<ResolvedEnvOverride> = selected.into_values().collect();
    entries.sort_by(|a, b| a.path.cmp(&b.path));

    let mut paths: HashSet<String> = HashSet::with_capacity(entries.len());
    let mut snapshots: HashMap<String, String> = HashMap::with_capacity(entries.len());
    for entry in entries {
        // Snapshot the pre-override raw value via TOML serde walk. Bypasses
        // `Config::get_prop`'s unconditional secret mask: secret fields on
        // `config` carry plaintext (post-`decrypt_secrets`), so the snapshot
        // captures the real value that should be restored at save time.
        let snapshot = raw_value_for_path(config, &entry.path).unwrap_or_default();
        snapshots.insert(entry.path.clone(), snapshot);

        config
            .set_prop(&entry.path, &entry.value)
            .with_context(|| format!("{} → {}", entry.env_name, entry.path))?;
        if Config::prop_is_secret(&entry.path) {
            ::dx_agent_log::record!(
                WARN,
                ::dx_agent_log::Event::new(module_path!(), ::dx_agent_log::Action::Note)
                    .with_outcome(::dx_agent_log::EventOutcome::Unknown)
                    .with_attrs(
                        ::serde_json::json!({"path": entry.path, "env_var": entry.env_name})
                    ),
                "Secret applied from env override"
            );
        } else {
            ::dx_agent_log::record!(
                DEBUG,
                ::dx_agent_log::Event::new(module_path!(), ::dx_agent_log::Action::Note)
                    .with_attrs(
                        ::serde_json::json!({"path": entry.path, "env_var": entry.env_name})
                    ),
                "Env override applied"
            );
        }
        paths.insert(entry.path);
    }
    if !paths.is_empty() {
        ::dx_agent_log::record!(
            INFO,
            ::dx_agent_log::Event::new(module_path!(), ::dx_agent_log::Action::Note)
                .with_attrs(::serde_json::json!({"count": paths.len()})),
            "Applied env-var config overrides"
        );
    }
    Ok(AppliedOverrides { paths, snapshots })
}

fn candidate_from_env_var(key: String, value: String) -> Option<EnvOverrideCandidate> {
    let (prefix, tail) = key
        .strip_prefix(PRIMARY_PREFIX)
        .map(|tail| (EnvPrefix::Primary, tail))
        .or_else(|| {
            key.strip_prefix(LEGACY_PREFIX)
                .map(|tail| (EnvPrefix::Legacy, tail))
        })?;
    let tail = tail.to_string();

    (!tail.is_empty()
        && tail
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_'))
    .then(|| EnvOverrideCandidate {
        env_name: key,
        value,
        tail,
        prefix,
    })
}

/// Walk an env-var tail against the schema. Map-keyed positions consume one
/// `__`-delimited alias token (which may contain single `_` per the alias
/// validator); everything else resolves via `prop_fields()` lookup.
fn resolve_path(tail: &str, config: &mut Config) -> Result<String> {
    let mut sections = Config::map_key_sections();
    sections.sort_by_key(|s| std::cmp::Reverse(s.path.len()));
    for section in sections {
        let env_pfx: String = section.path.replace('.', SEP);
        let with_sep = format!("{env_pfx}{SEP}");
        let Some(rest) = tail.strip_prefix(&with_sep) else {
            continue;
        };
        let mut parts = rest.splitn(2, SEP);
        let alias = parts.next().filter(|s| !s.is_empty()).ok_or_else(|| {
            ::dx_agent_log::record!(
                WARN,
                ::dx_agent_log::Event::new(module_path!(), ::dx_agent_log::Action::Reject)
                    .with_outcome(::dx_agent_log::EventOutcome::Failure)
                    .with_attrs(::serde_json::json!({"section": section.path, "tail": tail})),
                "env override path missing alias segment"
            );
            anyhow::Error::msg(format!("missing alias after `{}`", section.path))
        })?;
        let inner = parts.next().unwrap_or("");
        // Propagate the alias-validator's specific error so operators see
        // *why* their alias was rejected (leading underscore, uppercase, …)
        // instead of the generic "Unknown property" that would surface from
        // a downstream `set_prop` against a non-existent map key.
        config.create_map_key(section.path, alias).map_err(|e| {
            ::dx_agent_log::record!(
                WARN,
                ::dx_agent_log::Event::new(module_path!(), ::dx_agent_log::Action::Reject)
                    .with_outcome(::dx_agent_log::EventOutcome::Failure)
                    .with_attrs(::serde_json::json!({
                        "section": section.path,
                        "alias": alias,
                        "error": format!("{}", e),
                    })),
                "env override alias rejected by validator"
            );
            anyhow::Error::msg(format!(
                "invalid alias `{alias}` for `{}`: {e}",
                section.path
            ))
        })?;
        let path = if inner.is_empty() {
            format!("{}.{}", section.path, alias)
        } else {
            // Inner segments are `__`-separated snake-case field names — the
            // same casing the prop-path uses, so join them verbatim.
            let inner_path = inner.split(SEP).collect::<Vec<_>>().join(".");
            format!("{}.{}.{}", section.path, alias, inner_path)
        };
        return Ok(path);
    }

    // Non-map path: prop_fields() entries are dotted snake-case field
    // names. Convert to env-form (`.` → `__`) and compare.
    config
        .prop_fields()
        .into_iter()
        .find(|f| f.name.replace('.', SEP) == tail)
        .map(|f| f.name)
        .ok_or_else(|| {
            ::dx_agent_log::record!(
                WARN,
                ::dx_agent_log::Event::new(module_path!(), ::dx_agent_log::Action::Reject)
                    .with_outcome(::dx_agent_log::EventOutcome::Failure)
                    .with_attrs(::serde_json::json!({"tail": tail})),
                "env override path does not match any schema field"
            );
            anyhow::Error::msg(format!("no schema field has env-form `{tail}`"))
        })
}

/// Read the raw string value at a dotted (kebab-cased) prop path from a
/// serializable Config struct, bypassing the `is_secret` masking that
/// `Config::get_prop` applies. Returns `None` when the path doesn't resolve
/// (e.g. the alias entry hasn't been created yet on disk).
///
/// Walks the TOML serialization. Each segment is resolved value-aware:
/// tried verbatim first so hyphenated map keys (aliases, model names like
/// `claude-opus-4-8`) survive, then snake-cased only as a fallback for a
/// kebab field segment. Used by [`apply_env_overrides`] so the pre-override
/// snapshot of a secret field captures the real plaintext rather than the
/// display mask.
fn raw_value_for_path(source: &Config, path: &str) -> Option<String> {
    let table = toml::Value::try_from(source).ok()?;
    let mut current: &toml::Value = &table;
    for segment in path.split('.') {
        let tbl = current.as_table()?;
        current = match tbl.get(segment) {
            Some(v) => v,
            None => tbl.get(&segment.replace('-', "_"))?,
        };
    }
    Some(match current {
        toml::Value::String(s) => s.clone(),
        other => other.to_string(),
    })
}

/// Restore env-overridden paths in a save-bound clone to their pre-override
/// snapshots, so env-injected values never reach `encrypt_secrets()` or the
/// on-disk TOML.
///
/// Snapshots come from [`apply_env_overrides`] which captures the
/// post-`decrypt_secrets` plaintext for secret fields. After this restore,
/// `encrypt_secrets()` re-encrypts the recovered plaintext to fresh
/// ciphertext that decrypts to the same value — preserving the operator's
/// real on-disk credential across env-override + save cycles.
pub fn mask_env_overrides_for_save(
    config_to_save: &mut Config,
    snapshots: &HashMap<String, String>,
) -> Result<()> {
    for (path, value) in snapshots {
        if let Err(err) = config_to_save.set_prop(path, value) {
            ::dx_agent_log::record!(
                WARN,
                ::dx_agent_log::Event::new(module_path!(), ::dx_agent_log::Action::Note)
                    .with_outcome(::dx_agent_log::EventOutcome::Unknown)
                    .with_attrs(::serde_json::json!({"path": path, "error": format!("{}", err)})),
                "Save-mask reset failed; field retains default"
            );
        }
    }
    Ok(())
}

/// Process-wide lock for env-mutating tests. Both `env_overrides::tests`
/// and `schema::tests` race on `DX_AGENT_*` env vars and must serialize on
/// the same mutex; defining it once here and re-exporting `pub(crate)`
/// keeps a single coordinator. `#[cfg(test)]` so it never lands in
/// production builds.
#[cfg(test)]
pub(crate) async fn env_test_lock() -> tokio::sync::MutexGuard<'static, ()> {
    static LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());
    LOCK.lock().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::Config;

    /// RAII-ish helper: removes the named DX_AGENT_* var on drop so failed
    /// asserts don't leak state into sibling tests.
    struct EnvVarGuard(&'static str);
    impl EnvVarGuard {
        fn set(name: &'static str, value: &str) -> Self {
            // SAFETY: tests serialize on `env_test_lock()`.
            unsafe { std::env::set_var(name, value) };
            Self(name)
        }
    }
    impl Drop for EnvVarGuard {
        fn drop(&mut self) {
            // SAFETY: tests serialize on `env_test_lock()`.
            unsafe { std::env::remove_var(self.0) };
        }
    }

    #[tokio::test]
    async fn walker_resolves_typed_family_alias_default() {
        let _guard = super::env_test_lock().await;
        let _v = EnvVarGuard::set(
            "DX_AGENT_providers__models__anthropic__default__api_key",
            "sk-ant-fixture",
        );

        let mut config = Config::default();
        let applied = apply_env_overrides(&mut config).expect("apply succeeds");

        assert!(
            applied
                .paths
                .contains("providers.models.anthropic.default.api_key"),
            "kebab-translated path should be recorded: {:?}",
            applied.paths,
        );
        // Secret field round-trips through set_prop into the typed alias.
        assert_eq!(
            config
                .providers
                .models
                .anthropic
                .get("default")
                .and_then(|c| c.base.api_key.as_deref()),
            Some("sk-ant-fixture"),
        );
    }

    #[tokio::test]
    async fn walker_accepts_dx_agents_schema_mirror_prefix() {
        let _guard = super::env_test_lock().await;
        let _v = EnvVarGuard::set("DX_AGENTS_gateway__request_timeout_secs", "121");

        let mut config = Config::default();
        let applied = apply_env_overrides(&mut config).expect("apply succeeds");

        assert!(applied.paths.contains("gateway.request_timeout_secs"));
        assert_eq!(config.gateway.request_timeout_secs, 121);
    }

    #[tokio::test]
    async fn dx_agents_schema_mirror_prefix_wins_over_legacy_zeroclaw_prefix() {
        let _guard = super::env_test_lock().await;
        let _legacy = EnvVarGuard::set("DX_AGENT_gateway__request_timeout_secs", "120");
        let _dx = EnvVarGuard::set("DX_AGENTS_gateway__request_timeout_secs", "122");

        let mut config = Config::default();
        let applied = apply_env_overrides(&mut config).expect("apply succeeds");

        assert_eq!(
            applied.paths.len(),
            1,
            "same config path should be applied only once: {:?}",
            applied.paths
        );
        assert!(applied.paths.contains("gateway.request_timeout_secs"));
        assert_eq!(config.gateway.request_timeout_secs, 122);
    }

    #[tokio::test]
    async fn walker_accepts_alias_with_underscore() {
        let _guard = super::env_test_lock().await;
        let _v1 = EnvVarGuard::set(
            "DX_AGENT_providers__models__openrouter__prod_v2__api_key",
            "sk-or-fixture",
        );
        let _v2 = EnvVarGuard::set(
            "DX_AGENT_providers__models__openrouter__prod_v2__model",
            "anthropic/claude-sonnet-4-6",
        );

        let mut config = Config::default();
        let applied = apply_env_overrides(&mut config).expect("apply succeeds");

        assert!(
            applied
                .paths
                .contains("providers.models.openrouter.prod_v2.api_key"),
        );
        assert!(
            applied
                .paths
                .contains("providers.models.openrouter.prod_v2.model"),
        );
        let entry = config
            .providers
            .models
            .openrouter
            .get("prod_v2")
            .expect("alias created");
        assert_eq!(entry.base.api_key.as_deref(), Some("sk-or-fixture"));
        assert_eq!(
            entry.base.model.as_deref(),
            Some("anthropic/claude-sonnet-4-6"),
        );
    }

    #[tokio::test]
    async fn walker_resolves_non_map_gateway_path() {
        let _guard = super::env_test_lock().await;
        let _v = EnvVarGuard::set("DX_AGENT_gateway__request_timeout_secs", "120");

        let mut config = Config::default();
        let applied = apply_env_overrides(&mut config).expect("apply succeeds");

        assert!(applied.paths.contains("gateway.request_timeout_secs"));
        assert_eq!(config.gateway.request_timeout_secs, 120);
    }

    #[tokio::test]
    async fn walker_rejects_unknown_path() {
        let _guard = super::env_test_lock().await;
        let _v = EnvVarGuard::set("DX_AGENT_no__such__field", "x");

        let mut config = Config::default();
        let err = apply_env_overrides(&mut config).expect_err("must hard-error");
        let msg = format!("{err:#}");
        assert!(
            msg.contains("DX_AGENT_no__such__field") && msg.contains("did not resolve"),
            "error must name the env var and the failure: {msg}",
        );
    }

    #[tokio::test]
    async fn walker_propagates_alias_validator_error() {
        let _guard = super::env_test_lock().await;
        // `_invalid` starts with `_`, which the alias validator rejects.
        // The walker's tail filter accepts `[a-z0-9_]+` so this gets past
        // the prefilter, and the failure must surface as the validator's
        // specific message — not a generic "Unknown property".
        let _v = EnvVarGuard::set(
            "DX_AGENT_providers__models__anthropic___invalid__api_key",
            "x",
        );

        let mut config = Config::default();
        let err = apply_env_overrides(&mut config).expect_err("must hard-error");
        let msg = format!("{err:#}");
        assert!(
            msg.contains("invalid alias") && msg.contains("_invalid"),
            "error must surface the alias validator's message: {msg}",
        );
    }

    #[tokio::test]
    async fn mask_restores_pre_override_snapshot_for_non_secret() {
        let _guard = super::env_test_lock().await;
        let _v = EnvVarGuard::set("DX_AGENT_gateway__request_timeout_secs", "999");

        let mut config = Config::default();
        let original_timeout = config.gateway.request_timeout_secs;
        let applied = apply_env_overrides(&mut config).expect("apply succeeds");
        assert_eq!(config.gateway.request_timeout_secs, 999);

        let mut to_save = config.clone();
        mask_env_overrides_for_save(&mut to_save, &applied.snapshots).expect("mask succeeds");
        assert_eq!(
            to_save.gateway.request_timeout_secs, original_timeout,
            "non-secret path resets to pre-override snapshot",
        );
        // In-memory config is unchanged — env value still effective for the
        // running process.
        assert_eq!(config.gateway.request_timeout_secs, 999);
    }

    #[tokio::test]
    async fn mask_restores_pre_override_plaintext_for_secret() {
        let _guard = super::env_test_lock().await;
        let _v = EnvVarGuard::set(
            "DX_AGENT_providers__models__anthropic__default__api_key",
            "sk-ant-from-env",
        );

        // Pre-existing alias with a real plaintext credential (the state
        // after `Config::load_or_init` calls `decrypt_secrets`).
        let mut config = Config::default();
        config
            .providers
            .models
            .ensure("anthropic", "default")
            .expect("typed slot")
            .api_key = Some("sk-ant-on-disk".to_string());

        let applied = apply_env_overrides(&mut config).expect("apply succeeds");
        assert!(
            applied
                .paths
                .contains("providers.models.anthropic.default.api_key"),
        );
        // Env value is live in memory.
        assert_eq!(
            config
                .providers
                .models
                .anthropic
                .get("default")
                .and_then(|c| c.base.api_key.as_deref()),
            Some("sk-ant-from-env"),
        );

        // Save-bound clone restores the pre-override plaintext, NOT the
        // display mask. This is the regression bar for the data-loss bug
        // identified in PR #6523 review.
        let mut to_save = config.clone();
        mask_env_overrides_for_save(&mut to_save, &applied.snapshots).expect("mask succeeds");
        assert_eq!(
            to_save
                .providers
                .models
                .anthropic
                .get("default")
                .and_then(|c| c.base.api_key.as_deref()),
            Some("sk-ant-on-disk"),
            "secret resets to pre-override plaintext (not the `**** (encrypted)` mask)",
        );
        assert_ne!(
            to_save
                .providers
                .models
                .anthropic
                .get("default")
                .and_then(|c| c.base.api_key.as_deref()),
            Some("**** (encrypted)"),
            "must not corrupt the field with the display mask",
        );
    }

    #[tokio::test]
    async fn schema_version_override_rejected() {
        let _guard = super::env_test_lock().await;
        let _v = EnvVarGuard::set("DX_AGENT_schema_version", "99");

        let mut config = Config::default();
        let err = apply_env_overrides(&mut config).expect_err("must hard-error");
        let msg = format!("{err:#}");
        assert!(
            msg.contains("schema_version") && msg.contains("not overridable"),
            "error must name the path and the reason: {msg}",
        );
    }
}

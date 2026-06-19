use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use anyhow::{Result, anyhow};
use serde::Serialize;
use zeroclaw_config::schema::{Config, ModelProviderConfig};
use zeroclaw_providers::provider_catalog::ModelProviderCatalogListing;

const LIVE_MODEL_LIST_TIMEOUT_SECS: u64 = 10;
const LIVE_PROVIDER_TIMEOUT_SECS: u64 = 10;
const LIVE_PROVIDER_MAX_TOKENS: u32 = 8;

#[derive(Clone, Copy, Debug, Serialize)]
pub(crate) struct LiveProbeCapabilities {
    pub(crate) streaming: bool,
    pub(crate) tool_calling: bool,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct ProviderLiveProbeResult {
    pub(crate) requested: bool,
    pub(crate) attempted: bool,
    pub(crate) completed: bool,
    pub(crate) status: &'static str,
    pub(crate) alias: String,
    pub(crate) model: Option<String>,
    pub(crate) endpoint: &'static str,
    pub(crate) model_list: &'static str,
    pub(crate) chat_canary: &'static str,
    pub(crate) streaming: &'static str,
    pub(crate) tool_calling: &'static str,
    pub(crate) live_model_count: Option<usize>,
    pub(crate) duration_ms: Option<u64>,
    pub(crate) error_kind: Option<&'static str>,
    pub(crate) error_summary: Option<String>,
}

impl ProviderLiveProbeResult {
    pub(crate) fn ready(
        alias: &str,
        model: Option<String>,
        live_model_count: Option<usize>,
        duration_ms: Option<u64>,
        capabilities: LiveProbeCapabilities,
    ) -> Self {
        Self {
            requested: true,
            attempted: true,
            completed: true,
            status: "ready",
            alias: alias.to_string(),
            model,
            endpoint: "pass",
            model_list: "pass",
            chat_canary: "not_requested",
            streaming: capability_status(capabilities.streaming),
            tool_calling: capability_status(capabilities.tool_calling),
            live_model_count,
            duration_ms,
            error_kind: None,
            error_summary: None,
        }
    }

    fn skipped(
        alias: &str,
        model: Option<String>,
        error_kind: &'static str,
        error_summary: impl Into<String>,
    ) -> Self {
        Self {
            requested: true,
            attempted: false,
            completed: true,
            status: "skipped",
            alias: alias.to_string(),
            model,
            endpoint: "skipped",
            model_list: "skipped",
            chat_canary: "not_requested",
            streaming: "not_requested",
            tool_calling: "not_requested",
            live_model_count: None,
            duration_ms: None,
            error_kind: Some(error_kind),
            error_summary: Some(error_summary.into()),
        }
    }

    fn failed(
        alias: &str,
        model: Option<String>,
        duration_ms: Option<u64>,
        error_kind: &'static str,
        error_summary: impl Into<String>,
    ) -> Self {
        Self {
            requested: true,
            attempted: true,
            completed: true,
            status: live_status_for_error(error_kind),
            alias: alias.to_string(),
            model,
            endpoint: check_status_for_error(error_kind),
            model_list: check_status_for_error(error_kind),
            chat_canary: "not_requested",
            streaming: "not_requested",
            tool_calling: "not_requested",
            live_model_count: None,
            duration_ms,
            error_kind: Some(error_kind),
            error_summary: Some(error_summary.into()),
        }
    }
}

pub(crate) fn validate_provider_health_request(provider: Option<&str>, live: bool) -> Result<()> {
    if live && normalized_provider_filter(provider).is_none() {
        anyhow::bail!(
            "providers health --live requires --provider <provider> to avoid broad live API probes"
        );
    }
    Ok(())
}

pub(crate) async fn run_live_provider_probes(
    config: &Config,
    provider: Option<&str>,
    listing: &ModelProviderCatalogListing,
) -> Result<BTreeMap<String, ProviderLiveProbeResult>> {
    let provider = normalized_provider_filter(provider)
        .ok_or_else(|| anyhow!("live provider health requires a provider filter"))?;
    let (family, _) = provider_filter_parts(&provider);
    let local = listing
        .entries
        .iter()
        .find(|entry| entry.name == family)
        .is_some_and(|entry| entry.local);
    let result = match resolve_live_probe_target(config, &provider, local) {
        LiveProbeTargetResolution::Target(target) => probe_provider_target(config, target).await,
        LiveProbeTargetResolution::Resolved(result) => result,
    };

    Ok(BTreeMap::from([(family.to_string(), result)]))
}

#[derive(Clone)]
struct ProviderLiveProbeTarget {
    family: String,
    alias: String,
    model: Option<String>,
    api_key: Option<String>,
}

impl std::fmt::Debug for ProviderLiveProbeTarget {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ProviderLiveProbeTarget")
            .field("family", &self.family)
            .field("alias", &self.alias)
            .field("model", &self.model)
            .field("api_key", &self.api_key.as_ref().map(|_| "[REDACTED]"))
            .finish()
    }
}

enum LiveProbeTargetResolution {
    Target(ProviderLiveProbeTarget),
    Resolved(ProviderLiveProbeResult),
}

fn resolve_live_probe_target(
    config: &Config,
    provider_filter: &str,
    local: bool,
) -> LiveProbeTargetResolution {
    let (family, requested_alias) = provider_filter_parts(provider_filter);
    let configured_aliases = config
        .providers
        .models
        .iter_entries()
        .filter(|(configured_family, _, _)| *configured_family == family)
        .map(|(_, alias, provider)| (alias.to_string(), provider.clone()))
        .collect::<Vec<_>>();

    if let Some(alias) = requested_alias {
        let Some((_, provider)) = configured_aliases
            .into_iter()
            .find(|(configured_alias, _)| configured_alias == alias)
        else {
            return LiveProbeTargetResolution::Resolved(ProviderLiveProbeResult::skipped(
                alias,
                None,
                "missing_config",
                format!("No configured provider alias found for {family}.{alias}."),
            ));
        };

        return live_probe_target_from_config(family, alias, provider, local);
    }

    match configured_aliases.as_slice() {
        [] => LiveProbeTargetResolution::Resolved(ProviderLiveProbeResult::skipped(
            "default",
            None,
            "missing_config",
            format!("Add a {family} provider alias before running live health checks."),
        )),
        [(alias, provider)] => {
            live_probe_target_from_config(family, alias, provider.clone(), local)
        }
        aliases => LiveProbeTargetResolution::Resolved(ProviderLiveProbeResult::skipped(
            "multiple",
            None,
            "ambiguous_alias",
            format!(
                "Provider {family} has {} aliases. Run with --provider {family}.<alias>.",
                aliases.len()
            ),
        )),
    }
}

fn live_probe_target_from_config(
    family: &str,
    alias: &str,
    provider: ModelProviderConfig,
    local: bool,
) -> LiveProbeTargetResolution {
    let api_key = provider
        .api_key
        .clone()
        .filter(|value| !value.trim().is_empty());
    if !local && api_key.is_none() {
        return LiveProbeTargetResolution::Resolved(ProviderLiveProbeResult::skipped(
            alias,
            provider.model,
            "missing_credentials",
            format!("Configure credentials for {family}.{alias} before running live probes."),
        ));
    }

    LiveProbeTargetResolution::Target(ProviderLiveProbeTarget {
        family: family.to_string(),
        alias: alias.to_string(),
        model: provider.model,
        api_key,
    })
}

async fn probe_provider_target(
    config: &Config,
    target: ProviderLiveProbeTarget,
) -> ProviderLiveProbeResult {
    let started = Instant::now();
    let mut options = zeroclaw_providers::provider_runtime_options_for_alias(
        config,
        &target.family,
        &target.alias,
    );
    options.provider_timeout_secs = Some(LIVE_PROVIDER_TIMEOUT_SECS);
    options.provider_max_tokens = Some(LIVE_PROVIDER_MAX_TOKENS);

    let provider = match zeroclaw_providers::create_model_provider_for_alias(
        config,
        &target.family,
        &target.alias,
        target.api_key.as_deref(),
        &options,
    ) {
        Ok(provider) => provider,
        Err(error) => {
            let error_text = zeroclaw_providers::format_error_chain(error.as_ref());
            let error_kind = classify_live_probe_error(&error_text);
            return ProviderLiveProbeResult::failed(
                &target.alias,
                target.model,
                Some(elapsed_ms(started)),
                error_kind,
                error_summary_for_kind(error_kind),
            );
        }
    };

    let capabilities = LiveProbeCapabilities {
        streaming: provider.supports_streaming(),
        tool_calling: provider.supports_native_tools(),
    };
    let model_list = tokio::time::timeout(
        Duration::from_secs(LIVE_MODEL_LIST_TIMEOUT_SECS),
        provider.list_models(),
    )
    .await;

    match model_list {
        Ok(Ok(models)) => ProviderLiveProbeResult::ready(
            &target.alias,
            target.model,
            Some(models.len()),
            Some(elapsed_ms(started)),
            capabilities,
        ),
        Ok(Err(error)) => {
            let error_text = zeroclaw_providers::format_error_chain(error.as_ref());
            let error_kind = classify_live_probe_error(&error_text);
            ProviderLiveProbeResult::failed(
                &target.alias,
                target.model,
                Some(elapsed_ms(started)),
                error_kind,
                error_summary_for_kind(error_kind),
            )
        }
        Err(_) => ProviderLiveProbeResult::failed(
            &target.alias,
            target.model,
            Some(elapsed_ms(started)),
            "timeout",
            format!("Model list probe timed out after {LIVE_MODEL_LIST_TIMEOUT_SECS}s."),
        ),
    }
}

fn normalized_provider_filter(provider: Option<&str>) -> Option<String> {
    provider
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

fn provider_filter_parts(provider_filter: &str) -> (&str, Option<&str>) {
    provider_filter
        .split_once('.')
        .map_or((provider_filter, None), |(family, alias)| {
            (family, Some(alias))
        })
}

fn capability_status(supported: bool) -> &'static str {
    if supported {
        "supported"
    } else {
        "unsupported"
    }
}

fn elapsed_ms(started: Instant) -> u64 {
    started.elapsed().as_millis().min(u128::from(u64::MAX)) as u64
}

fn classify_live_probe_error(error_text: &str) -> &'static str {
    let lower = error_text.to_lowercase();
    if lower.contains("live model listing is not supported")
        || lower.contains("not supported for this model_provider")
    {
        return "unsupported_probe";
    }
    if lower.contains("timeout") || lower.contains("timed out") {
        return "timeout";
    }
    if lower.contains("429") || lower.contains("rate limit") || lower.contains("too many requests")
    {
        return "rate_limited";
    }
    if lower.contains("401") || lower.contains("unauthorized") || lower.contains("api key") {
        return "auth_failed";
    }
    if lower.contains("403") || lower.contains("forbidden") {
        return "access_denied";
    }
    if lower.contains("dns") || lower.contains("resolve") {
        return "dns_error";
    }
    if lower.contains("tls") || lower.contains("certificate") {
        return "tls_error";
    }
    if lower.contains("connect") || lower.contains("network") {
        return "network_error";
    }
    if lower.contains("500")
        || lower.contains("502")
        || lower.contains("503")
        || lower.contains("504")
    {
        return "http_5xx";
    }
    "internal_error"
}

fn live_status_for_error(error_kind: &str) -> &'static str {
    match error_kind {
        "auth_failed" => "auth_failed",
        "access_denied" => "auth_failed",
        "rate_limited" => "rate_limited",
        "timeout" => "timeout",
        "unsupported_probe" => "unsupported",
        _ => "error",
    }
}

fn check_status_for_error(error_kind: &str) -> &'static str {
    match error_kind {
        "auth_failed" => "auth_failed",
        "access_denied" => "auth_failed",
        "rate_limited" => "rate_limited",
        "timeout" => "timeout",
        "unsupported_probe" => "unsupported",
        _ => "error",
    }
}

fn error_summary_for_kind(error_kind: &str) -> String {
    match error_kind {
        "auth_failed" => "Provider rejected the configured credentials.".to_string(),
        "access_denied" => "Provider credentials do not have access to the model list.".to_string(),
        "rate_limited" => "Provider rate-limited the model list probe.".to_string(),
        "timeout" => "Provider model list probe timed out.".to_string(),
        "unsupported_probe" => {
            "This provider does not expose a supported live model-list probe.".to_string()
        }
        "dns_error" => "Provider endpoint DNS lookup failed.".to_string(),
        "tls_error" => "Provider endpoint TLS validation failed.".to_string(),
        "network_error" => "Provider endpoint could not be reached.".to_string(),
        "http_5xx" => "Provider endpoint returned a server error.".to_string(),
        _ => "Provider live probe failed before model listing completed.".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn live_probe_target_debug_redacts_credentials() {
        let target = ProviderLiveProbeTarget {
            family: "nvidia".to_string(),
            alias: "default".to_string(),
            model: Some("nvidia/example".to_string()),
            api_key: Some("sk-live-probe-secret".to_string()),
        };

        let rendered = format!("{target:?}");

        assert!(rendered.contains("api_key"));
        assert!(
            rendered.contains("[REDACTED]"),
            "debug output should show that a credential exists: {rendered}"
        );
        assert!(
            !rendered.contains("sk-live-probe-secret"),
            "debug output must not include credential material: {rendered}"
        );
    }
}

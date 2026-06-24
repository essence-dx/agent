use std::collections::BTreeMap;

#[cfg(test)]
use crate::cli_provider_health_live::LiveProbeCapabilities;
use crate::cli_provider_health_live::{
    ProviderLiveProbeResult, run_live_provider_probes, validate_provider_health_request,
};
use anyhow::Result;
use serde::Serialize;
use dx_agent_config::schema::{Config, ModelProviderConfig};
use dx_agent_providers::provider_catalog::{
    DxProvidersCatalogLoadDiagnostic, ModelProviderCatalogEntry, ModelProviderCatalogListing,
    ModelProviderCatalogSource,
};

const PROVIDERS_HEALTH_SCHEMA: &str = "dx.agents.zed.providers_health.v1";

#[derive(Debug, Serialize)]
struct ProviderHealthCatalogReport {
    loaded: bool,
    path: Option<String>,
    provider_count: Option<usize>,
    model_count: Option<usize>,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
struct ProviderHealthReport {
    schema_version: &'static str,
    status: &'static str,
    live_probe: bool,
    provider_filter: Option<String>,
    provider_count: usize,
    configured_provider_count: usize,
    ready_provider_count: usize,
    live_ready_provider_count: usize,
    live_problem_provider_count: usize,
    needs_key_count: usize,
    catalog: ProviderHealthCatalogReport,
    providers: Vec<ProviderHealthRow>,
}

#[derive(Debug, Serialize)]
struct ProviderHealthRow {
    id: String,
    display_name: String,
    source: ModelProviderCatalogSource,
    local: bool,
    model_count: usize,
    configured: bool,
    configured_aliases: Vec<String>,
    credential_state: &'static str,
    health_state: &'static str,
    live_health_state: &'static str,
    checks: ProviderHealthChecks,
    probe_result: Option<ProviderLiveProbeResult>,
    next_action: Option<String>,
}

#[derive(Debug, Serialize)]
struct ProviderHealthChecks {
    catalog: &'static str,
    configuration: &'static str,
    credentials: &'static str,
    endpoint: &'static str,
    model_list: &'static str,
    chat_canary: &'static str,
    streaming: &'static str,
    tool_calling: &'static str,
}

pub async fn print_providers_health_json(
    config: &Config,
    provider: Option<&str>,
    live: bool,
) -> Result<()> {
    validate_provider_health_request(provider, live)?;
    let listing = dx_agent_providers::provider_catalog::list_model_provider_catalog_listing();
    let live_results = if live {
        run_live_provider_probes(config, provider, &listing).await?
    } else {
        BTreeMap::new()
    };
    let report = if live {
        build_provider_health_report_with_live_results(listing, config, provider, live_results)
    } else {
        build_provider_health_report(listing, config, provider)
    };

    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub async fn print_providers_health(
    config: &Config,
    provider: Option<&str>,
    live: bool,
) -> Result<()> {
    validate_provider_health_request(provider, live)?;
    let listing = dx_agent_providers::provider_catalog::list_model_provider_catalog_listing();
    let live_results = if live {
        run_live_provider_probes(config, provider, &listing).await?
    } else {
        BTreeMap::new()
    };
    let report = if live {
        build_provider_health_report_with_live_results(listing, config, provider, live_results)
    } else {
        build_provider_health_report(listing, config, provider)
    };

    println!(
        "Provider health matrix: {} providers, {} configured, {} ready, {} need keys",
        report.provider_count,
        report.configured_provider_count,
        report.ready_provider_count,
        report.needs_key_count
    );
    println!("  ID                    STATE       CREDENTIALS       MODELS");
    println!("  ───────────────────── ─────────── ───────────────── ──────");
    for row in report.providers {
        println!(
            "  {:<21} {:<11} {:<17} {}",
            row.id, row.health_state, row.credential_state, row.model_count
        );
    }
    Ok(())
}

fn build_provider_health_report(
    listing: ModelProviderCatalogListing,
    config: &Config,
    provider_filter: Option<&str>,
) -> ProviderHealthReport {
    build_provider_health_report_inner(listing, config, provider_filter, BTreeMap::new(), false)
}

fn build_provider_health_report_with_live_results(
    listing: ModelProviderCatalogListing,
    config: &Config,
    provider_filter: Option<&str>,
    live_results: BTreeMap<String, ProviderLiveProbeResult>,
) -> ProviderHealthReport {
    build_provider_health_report_inner(listing, config, provider_filter, live_results, true)
}

fn build_provider_health_report_inner(
    listing: ModelProviderCatalogListing,
    config: &Config,
    provider_filter: Option<&str>,
    live_results: BTreeMap<String, ProviderLiveProbeResult>,
    live_probe: bool,
) -> ProviderHealthReport {
    let catalog = provider_health_catalog_report(listing.dx_providers_catalog);
    let configured = configured_provider_entries(config);
    let provider_filter = normalized_provider_filter(provider_filter);
    let providers = listing
        .entries
        .into_iter()
        .filter(|entry| {
            provider_filter
                .as_deref()
                .is_none_or(|filter| provider_matches_filter(&entry.name, filter))
        })
        .map(|entry| {
            let live_result = live_results.get(&entry.name).cloned();
            provider_health_row(entry, &configured, live_result)
        })
        .collect::<Vec<_>>();
    let configured_provider_count = providers
        .iter()
        .filter(|provider| provider.configured)
        .count();
    let ready_provider_count = providers
        .iter()
        .filter(|provider| provider.health_state == "configured")
        .count();
    let live_ready_provider_count = providers
        .iter()
        .filter(|provider| provider.live_health_state == "ready")
        .count();
    let live_problem_provider_count = providers
        .iter()
        .filter(|provider| {
            !matches!(
                provider.live_health_state,
                "not_tested" | "ready" | "skipped"
            )
        })
        .count();
    let needs_key_count = providers
        .iter()
        .filter(|provider| provider.health_state == "needs_key")
        .count();

    ProviderHealthReport {
        schema_version: PROVIDERS_HEALTH_SCHEMA,
        status: if catalog.error.is_some() {
            "partial"
        } else {
            "ok"
        },
        live_probe,
        provider_filter,
        provider_count: providers.len(),
        configured_provider_count,
        ready_provider_count,
        live_ready_provider_count,
        live_problem_provider_count,
        needs_key_count,
        catalog,
        providers,
    }
}

fn provider_health_row(
    entry: ModelProviderCatalogEntry,
    configured: &BTreeMap<String, Vec<ConfiguredProviderAlias>>,
    live_result: Option<ProviderLiveProbeResult>,
) -> ProviderHealthRow {
    let configured_aliases = configured.get(&entry.name).cloned().unwrap_or_default();
    let alias_labels = configured_aliases
        .iter()
        .map(|alias| alias.name.clone())
        .collect::<Vec<_>>();
    let configured = !configured_aliases.is_empty();
    let credential_state = credential_state(&entry, &configured_aliases);
    let health_state = health_state(configured, credential_state);
    let live_health_state = live_result
        .as_ref()
        .map(|result| result.status)
        .unwrap_or("not_tested");

    ProviderHealthRow {
        checks: provider_health_checks(configured, credential_state, live_result.as_ref()),
        next_action: provider_next_action(&entry.name, health_state, live_result.as_ref()),
        id: entry.name,
        display_name: entry.display_name,
        source: entry.source,
        local: entry.local,
        model_count: entry.model_count,
        configured,
        configured_aliases: alias_labels,
        credential_state,
        health_state,
        live_health_state,
        probe_result: live_result,
    }
}

fn provider_health_checks(
    configured: bool,
    credential_state: &'static str,
    live_result: Option<&ProviderLiveProbeResult>,
) -> ProviderHealthChecks {
    if let Some(live_result) = live_result {
        return ProviderHealthChecks {
            catalog: "pass",
            configuration: if configured { "pass" } else { "missing" },
            credentials: credential_state,
            endpoint: live_result.endpoint,
            model_list: live_result.model_list,
            chat_canary: live_result.chat_canary,
            streaming: live_result.streaming,
            tool_calling: live_result.tool_calling,
        };
    }

    ProviderHealthChecks {
        catalog: "pass",
        configuration: if configured { "pass" } else { "missing" },
        credentials: credential_state,
        endpoint: "not_tested_offline",
        model_list: "not_tested_offline",
        chat_canary: "not_tested_offline",
        streaming: "not_tested_offline",
        tool_calling: "not_tested_offline",
    }
}

fn provider_next_action(
    provider_id: &str,
    health_state: &str,
    live_result: Option<&ProviderLiveProbeResult>,
) -> Option<String> {
    if let Some(live_result) = live_result {
        return match live_result.status {
            "ready" => Some(format!(
                "Live probe passed for {provider_id}.{}.",
                live_result.alias
            )),
            "skipped" => live_result.error_summary.clone(),
            _ => Some(format!(
                "Live probe for {provider_id}.{} reported {}.",
                live_result.alias, live_result.status
            )),
        };
    }

    match health_state {
        "configured" => Some(format!(
            "Offline checks passed. Run with --live --provider {provider_id} to probe the endpoint."
        )),
        "needs_key" => Some(format!(
            "Configure credentials for {provider_id} before running live health checks."
        )),
        _ => Some(format!(
            "Add a {provider_id} provider alias before running live health checks."
        )),
    }
}

fn health_state(configured: bool, credential_state: &str) -> &'static str {
    match (configured, credential_state) {
        (true, "configured" | "local_runtime" | "not_required") => "configured",
        (_, "needs_key") => "needs_key",
        _ => "cataloged",
    }
}

fn credential_state(
    entry: &ModelProviderCatalogEntry,
    aliases: &[ConfiguredProviderAlias],
) -> &'static str {
    if entry.local {
        return "local_runtime";
    }
    if aliases.iter().any(|alias| alias.has_api_key) {
        return "configured";
    }
    if entry
        .freemium()
        .is_some_and(|metadata| !metadata.env_vars.is_empty())
    {
        "needs_key"
    } else {
        "not_required"
    }
}

#[derive(Clone, Debug)]
struct ConfiguredProviderAlias {
    name: String,
    has_api_key: bool,
}

fn configured_provider_entries(config: &Config) -> BTreeMap<String, Vec<ConfiguredProviderAlias>> {
    let mut entries: BTreeMap<String, Vec<ConfiguredProviderAlias>> = BTreeMap::new();
    for (family, alias, provider) in config.providers.models.iter_entries() {
        entries
            .entry(family.to_string())
            .or_default()
            .push(configured_provider_alias(alias, provider));
    }
    for aliases in entries.values_mut() {
        aliases.sort_by(|left, right| left.name.cmp(&right.name));
    }
    entries
}

fn configured_provider_alias(
    alias: &str,
    provider: &ModelProviderConfig,
) -> ConfiguredProviderAlias {
    ConfiguredProviderAlias {
        name: alias.to_string(),
        has_api_key: provider
            .api_key
            .as_deref()
            .is_some_and(|value| !value.trim().is_empty()),
    }
}

fn provider_health_catalog_report(
    diagnostic: Option<DxProvidersCatalogLoadDiagnostic>,
) -> ProviderHealthCatalogReport {
    diagnostic.map_or(
        ProviderHealthCatalogReport {
            loaded: false,
            path: None,
            provider_count: None,
            model_count: None,
            error: None,
        },
        |diagnostic| ProviderHealthCatalogReport {
            loaded: diagnostic.loaded(),
            path: Some(diagnostic.path.display().to_string()),
            provider_count: diagnostic.provider_count,
            model_count: diagnostic.model_count,
            error: diagnostic.error,
        },
    )
}

fn normalized_provider_filter(provider: Option<&str>) -> Option<String> {
    provider
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

fn provider_matches_filter(provider_id: &str, filter: &str) -> bool {
    filter.split_once('.').map_or(filter, |(family, _)| family) == provider_id
}

#[cfg(test)]
#[path = "cli_provider_health_tests.rs"]
mod cli_provider_health_tests;

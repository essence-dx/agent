use std::path::PathBuf;

use super::*;
use dx_agent_config::schema::Config;
use dx_agent_providers::provider_catalog::{
    DxProvidersCatalogLoadDiagnostic, ModelProviderCatalogEntry, ModelProviderCatalogListing,
    ModelProviderCatalogSource,
};

fn catalog_entry(id: &str, display_name: &str, local: bool) -> ModelProviderCatalogEntry {
    ModelProviderCatalogEntry {
        name: id.to_string(),
        display_name: display_name.to_string(),
        local,
        source: ModelProviderCatalogSource::DxProvidersCatalog,
        model_count: 120,
    }
}

fn catalog_listing() -> ModelProviderCatalogListing {
    ModelProviderCatalogListing {
        entries: vec![
            catalog_entry("nvidia", "NVIDIA NIM", false),
            catalog_entry("ollama", "Ollama", true),
        ],
        dx_providers_catalog: Some(DxProvidersCatalogLoadDiagnostic {
            path: PathBuf::from("G:\\Dx\\providers\\data\\providers.rkyv"),
            provider_count: Some(184),
            model_count: Some(6245),
            error: None,
        }),
    }
}

#[test]
fn provider_health_report_marks_configured_rows_without_exporting_secrets() {
    let mut config = Config::default();
    let nvidia = config
        .providers
        .models
        .ensure("nvidia", "default")
        .expect("nvidia should be a known provider family");
    nvidia.api_key = Some("secret-nvidia-key".to_string());

    let report = build_provider_health_report(catalog_listing(), &config, None);
    let nvidia = report
        .providers
        .iter()
        .find(|row| row.id == "nvidia")
        .expect("nvidia health row should exist");

    assert_eq!(report.schema_version, "dx.agents.zed.providers_health.v1");
    assert_eq!(report.catalog.provider_count, Some(184));
    assert_eq!(report.provider_count, 2);
    assert_eq!(report.configured_provider_count, 1);
    assert!(!report.live_probe);
    assert_eq!(nvidia.health_state, "configured");
    assert_eq!(nvidia.credential_state, "configured");
    assert_eq!(nvidia.configured_aliases, vec!["default".to_string()]);
    assert_eq!(nvidia.checks.chat_canary, "not_tested_offline");
    assert_eq!(
        nvidia.next_action.as_deref(),
        Some("Offline checks passed. Run with --live --provider nvidia to probe the endpoint.")
    );

    let serialized =
        serde_json::to_string(&report).expect("health report should serialize to JSON");
    assert!(!serialized.contains("secret-nvidia-key"));
}

#[test]
fn provider_health_report_filters_to_one_provider() {
    let report =
        build_provider_health_report(catalog_listing(), &Config::default(), Some(" nvidia "));

    assert_eq!(report.provider_filter.as_deref(), Some("nvidia"));
    assert_eq!(report.provider_count, 1);
    assert_eq!(report.providers[0].id, "nvidia");
    assert_eq!(report.providers[0].health_state, "needs_key");
}

#[test]
fn provider_health_report_applies_live_probe_results_without_exporting_secrets() {
    let mut config = Config::default();
    let nvidia = config
        .providers
        .models
        .ensure("nvidia", "default")
        .expect("nvidia should be a known provider family");
    nvidia.api_key = Some("secret-nvidia-key".to_string());
    nvidia.model = Some("meta/llama-3.1-8b-instruct".to_string());

    let mut live_results = BTreeMap::new();
    live_results.insert(
        "nvidia".to_string(),
        ProviderLiveProbeResult::ready(
            "default",
            Some("meta/llama-3.1-8b-instruct".to_string()),
            Some(120),
            Some(42),
            LiveProbeCapabilities {
                streaming: true,
                tool_calling: true,
            },
        ),
    );

    let report = build_provider_health_report_with_live_results(
        catalog_listing(),
        &config,
        Some("nvidia"),
        live_results,
    );
    let nvidia = report
        .providers
        .iter()
        .find(|row| row.id == "nvidia")
        .expect("nvidia health row should exist");

    assert!(report.live_probe);
    assert_eq!(report.ready_provider_count, 1);
    assert_eq!(nvidia.health_state, "configured");
    assert_eq!(nvidia.live_health_state, "ready");
    assert_eq!(nvidia.checks.endpoint, "pass");
    assert_eq!(nvidia.checks.model_list, "pass");
    assert_eq!(nvidia.checks.chat_canary, "not_requested");
    assert_eq!(
        nvidia
            .probe_result
            .as_ref()
            .map(|probe| probe.live_model_count),
        Some(Some(120))
    );
    assert_eq!(
        nvidia.next_action.as_deref(),
        Some("Live probe passed for nvidia.default.")
    );

    let serialized =
        serde_json::to_string(&report).expect("health report should serialize to JSON");
    assert!(!serialized.contains("secret-nvidia-key"));
}

#[test]
fn live_provider_health_requires_provider_filter() {
    let error = validate_provider_health_request(None, true)
        .expect_err("live health probes must require an explicit provider filter");

    assert!(
        error.to_string().contains("requires --provider <provider>"),
        "{error}"
    );
}

use std::collections::BTreeMap;

use anyhow::Result;
use serde::Serialize;
use zeroclaw_config::schema::Config;
use zeroclaw_providers::provider_catalog::{
    DxProvidersCatalogLoadDiagnostic, ModelProviderCatalogEntry, ModelProviderCatalogListing,
    ModelProviderCatalogSource,
};

const PROVIDERS_LIST_SCHEMA: &str = "dx.agents.zed.providers_list.v1";
const MODELS_LIST_SCHEMA: &str = "dx.agents.zed.models_list.v1";

#[derive(Debug, Serialize)]
struct ProviderCatalogReport {
    loaded: bool,
    path: Option<String>,
    provider_count: Option<usize>,
    model_count: Option<usize>,
    error: Option<String>,
}

#[derive(Debug, Serialize)]
struct ProviderListReport {
    schema_version: &'static str,
    status: &'static str,
    provider_count: usize,
    configured_provider_count: usize,
    catalog: ProviderCatalogReport,
    providers: Vec<ProviderListRow>,
}

#[derive(Debug, Serialize)]
struct ProviderListRow {
    id: String,
    display_name: String,
    local: bool,
    source: ModelProviderCatalogSource,
    model_count: usize,
    configured: bool,
    configured_aliases: Vec<String>,
}

#[derive(Debug, Serialize)]
struct ModelListReport {
    schema_version: &'static str,
    status: &'static str,
    provider: Option<String>,
    provider_count: usize,
    model_count: usize,
    catalog: ProviderCatalogReport,
    providers: Vec<ModelListProviderRow>,
}

#[derive(Debug, Serialize)]
struct ModelListProviderRow {
    id: String,
    display_name: String,
    source: ModelProviderCatalogSource,
    model_count: usize,
    models: Vec<String>,
    catalog_error: Option<String>,
}

pub fn print_providers_json(config: &Config) -> Result<()> {
    let listing = zeroclaw_providers::provider_catalog::list_model_provider_catalog_listing();
    let configured_aliases = configured_aliases_by_family(config);
    let report = build_provider_list_report(listing, configured_aliases);

    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub fn print_models_json(_config: &Config, provider: Option<&str>) -> Result<()> {
    let listing = zeroclaw_providers::provider_catalog::list_model_provider_catalog_listing();
    let report = build_model_list_report(listing, provider);

    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

fn build_provider_list_report(
    listing: ModelProviderCatalogListing,
    configured_aliases: BTreeMap<String, Vec<String>>,
) -> ProviderListReport {
    let catalog = provider_catalog_report(listing.dx_providers_catalog);
    let providers = listing
        .entries
        .into_iter()
        .map(|entry| {
            let aliases = configured_aliases
                .get(&entry.name)
                .cloned()
                .unwrap_or_default();
            ProviderListRow {
                id: entry.name,
                display_name: entry.display_name,
                local: entry.local,
                source: entry.source,
                model_count: entry.model_count,
                configured: !aliases.is_empty(),
                configured_aliases: aliases,
            }
        })
        .collect::<Vec<_>>();
    let configured_provider_count = providers
        .iter()
        .filter(|provider| provider.configured)
        .count();

    ProviderListReport {
        schema_version: PROVIDERS_LIST_SCHEMA,
        status: report_status(&catalog, false),
        provider_count: providers.len(),
        configured_provider_count,
        catalog,
        providers,
    }
}

fn build_model_list_report(
    listing: ModelProviderCatalogListing,
    provider: Option<&str>,
) -> ModelListReport {
    let catalog = provider_catalog_report(listing.dx_providers_catalog);
    let provider = provider
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string);
    let providers = match provider.as_deref() {
        Some(provider_id) => vec![model_row_for_provider(provider_id, &catalog)],
        None => listing
            .entries
            .into_iter()
            .map(model_row_from_catalog_entry)
            .collect(),
    };
    let model_count = providers.iter().map(|row| row.model_count).sum();
    let status = report_status(
        &catalog,
        providers.iter().any(|row| row.catalog_error.is_some()),
    );

    ModelListReport {
        schema_version: MODELS_LIST_SCHEMA,
        status,
        provider,
        provider_count: providers.len(),
        model_count,
        catalog,
        providers,
    }
}

fn provider_catalog_report(
    diagnostic: Option<DxProvidersCatalogLoadDiagnostic>,
) -> ProviderCatalogReport {
    diagnostic.map_or(
        ProviderCatalogReport {
            loaded: false,
            path: None,
            provider_count: None,
            model_count: None,
            error: None,
        },
        |diagnostic| ProviderCatalogReport {
            loaded: diagnostic.loaded(),
            path: Some(diagnostic.path.display().to_string()),
            provider_count: diagnostic.provider_count,
            model_count: diagnostic.model_count,
            error: diagnostic.error,
        },
    )
}

fn report_status(catalog: &ProviderCatalogReport, has_row_error: bool) -> &'static str {
    if catalog.error.is_some() || has_row_error {
        "partial"
    } else {
        "ok"
    }
}

fn configured_aliases_by_family(config: &Config) -> BTreeMap<String, Vec<String>> {
    let mut aliases = BTreeMap::new();
    for (family, alias, _) in config.providers.models.iter_entries() {
        aliases
            .entry(family.to_string())
            .or_insert_with(Vec::new)
            .push(alias.to_string());
    }
    for family_aliases in aliases.values_mut() {
        family_aliases.sort();
    }
    aliases
}

fn model_row_for_provider(
    provider_id: &str,
    catalog: &ProviderCatalogReport,
) -> ModelListProviderRow {
    match zeroclaw_providers::provider_catalog::find_model_provider_catalog_entry(provider_id) {
        Some(entry) => {
            let models =
                zeroclaw_providers::provider_catalog::list_models_for_catalog_provider(&entry.name);
            match models {
                Ok(models) => ModelListProviderRow {
                    model_count: models.len(),
                    models,
                    catalog_error: None,
                    ..model_row_base(entry)
                },
                Err(error) => ModelListProviderRow {
                    catalog_error: Some(error.to_string()),
                    ..model_row_from_catalog_entry(entry)
                },
            }
        }
        None => ModelListProviderRow {
            id: provider_id.to_string(),
            display_name: provider_id.to_string(),
            source: ModelProviderCatalogSource::Native,
            model_count: 0,
            models: Vec::new(),
            catalog_error: Some(provider_not_found_error(provider_id, catalog)),
        },
    }
}

fn provider_not_found_error(provider_id: &str, catalog: &ProviderCatalogReport) -> String {
    match catalog.error.as_deref() {
        Some(error) => format!(
            "provider `{provider_id}` was not found because the DX providers catalog could not be loaded: {error}"
        ),
        None => "provider was not found in the provider catalog".to_string(),
    }
}

fn model_row_from_catalog_entry(entry: ModelProviderCatalogEntry) -> ModelListProviderRow {
    ModelListProviderRow {
        catalog_error: None,
        ..model_row_base(entry)
    }
}

fn model_row_base(entry: ModelProviderCatalogEntry) -> ModelListProviderRow {
    ModelListProviderRow {
        id: entry.name,
        display_name: entry.display_name,
        source: entry.source,
        model_count: entry.model_count,
        models: Vec::new(),
        catalog_error: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use zeroclaw_providers::provider_catalog::DxProvidersCatalogLoadDiagnostic;

    fn catalog_error_listing() -> ModelProviderCatalogListing {
        ModelProviderCatalogListing {
            entries: vec![ModelProviderCatalogEntry {
                name: "openai".to_string(),
                display_name: "OpenAI".to_string(),
                local: false,
                source: ModelProviderCatalogSource::Native,
                model_count: 3,
            }],
            dx_providers_catalog: Some(DxProvidersCatalogLoadDiagnostic {
                path: PathBuf::from("G:\\Dx\\providers\\data\\providers.rkyv"),
                provider_count: None,
                model_count: None,
                error: Some("provider catalog file not found".to_string()),
            }),
        }
    }

    #[test]
    fn json_schema_versions_match_zed_bridge_contract() {
        assert_eq!(PROVIDERS_LIST_SCHEMA, "dx.agents.zed.providers_list.v1");
        assert_eq!(MODELS_LIST_SCHEMA, "dx.agents.zed.models_list.v1");
    }

    #[test]
    fn provider_list_report_marks_catalog_load_errors_partial() {
        let report = build_provider_list_report(catalog_error_listing(), BTreeMap::new());

        assert_eq!(report.status, "partial");
        assert!(!report.catalog.loaded);
        assert_eq!(
            report.catalog.error.as_deref(),
            Some("provider catalog file not found")
        );
    }

    #[test]
    fn model_list_report_preserves_catalog_load_diagnostic() {
        let report = build_model_list_report(catalog_error_listing(), None);

        assert_eq!(report.status, "partial");
        assert!(!report.catalog.loaded);
        assert_eq!(
            report.catalog.error.as_deref(),
            Some("provider catalog file not found")
        );
        assert_eq!(report.provider_count, 1);
        assert_eq!(report.model_count, 3);
    }
}

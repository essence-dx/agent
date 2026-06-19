use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
};

use memmap2::Mmap;
use rkyv::{
    Archive, CheckBytes, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize,
    check_archived_root,
};
use serde::{Deserialize, Serialize};

use crate::{ModelProviderInfo, list_model_providers, provider_metadata};

const DX_PROVIDERS_CATALOG_ENV: &str = "DX_PROVIDERS_CATALOG_PATH";
const PROVIDERS_CATALOG_RELATIVE_PATH: &[&str] = &["providers", "data", "providers.rkyv"];
const MAX_DX_PROVIDERS_CATALOG_BYTES: u64 = 64 * 1024 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ModelProviderCatalogSource {
    Native,
    DxProvidersCatalog,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModelProviderCatalogEntry {
    pub name: String,
    pub display_name: String,
    pub local: bool,
    pub source: ModelProviderCatalogSource,
    pub model_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModelProviderCatalogListing {
    pub entries: Vec<ModelProviderCatalogEntry>,
    pub dx_providers_catalog: Option<DxProvidersCatalogLoadDiagnostic>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DxProvidersCatalogLoadDiagnostic {
    pub path: PathBuf,
    pub provider_count: Option<usize>,
    pub model_count: Option<usize>,
    pub error: Option<String>,
}

impl DxProvidersCatalogLoadDiagnostic {
    #[must_use]
    pub fn loaded(&self) -> bool {
        self.error.is_none()
    }
}

impl ModelProviderCatalogEntry {
    #[must_use]
    pub fn from_native(info: ModelProviderInfo) -> Self {
        Self {
            name: info.name.to_string(),
            display_name: info.display_name.to_string(),
            local: info.local,
            source: ModelProviderCatalogSource::Native,
            model_count: 0,
        }
    }

    #[must_use]
    pub fn identity(&self) -> Option<provider_metadata::ProviderIdentity> {
        provider_metadata::metadata_for_provider_id(&self.name).map(|metadata| metadata.identity())
    }

    #[must_use]
    pub fn freemium(&self) -> Option<provider_metadata::FreemiumMetadata> {
        provider_metadata::metadata_for_provider_id(&self.name).map(|metadata| metadata.freemium())
    }

    #[must_use]
    pub fn is_native(&self) -> bool {
        self.source == ModelProviderCatalogSource::Native
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DxProvidersCatalogSummary {
    pub path: PathBuf,
    pub total_providers: usize,
    pub total_models: usize,
    pub providers: Vec<DxProviderCatalogSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DxProviderCatalogSummary {
    pub id: String,
    pub name: String,
    pub model_count: usize,
    pub supports_chat: bool,
    pub supports_embedding: bool,
    pub supports_image: bool,
    pub supports_audio: bool,
    pub api_url: Option<String>,
    pub models: Vec<DxModelCatalogSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DxModelCatalogSummary {
    pub id: String,
    pub name: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ProviderCatalogError {
    #[error("provider catalog file not found at {path}")]
    NotFound { path: PathBuf },
    #[error("failed to open provider catalog {path}: {source}")]
    Open {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("failed to inspect provider catalog {path}: {source}")]
    Metadata {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("provider catalog {path} is empty")]
    Empty { path: PathBuf },
    #[error(
        "provider catalog {path} is {actual_bytes} bytes, exceeding the {max_bytes} byte limit"
    )]
    FileTooLarge {
        path: PathBuf,
        actual_bytes: u64,
        max_bytes: u64,
    },
    #[error("failed to map provider catalog {path}: {source}")]
    Map {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("failed to validate provider catalog {path}: {message}")]
    Validate { path: PathBuf, message: String },
    #[error(
        "provider catalog {path} declares {expected_providers} providers/{expected_models} models but contains {actual_providers} providers/{actual_models} models"
    )]
    CatalogCountMismatch {
        path: PathBuf,
        expected_providers: usize,
        actual_providers: usize,
        expected_models: usize,
        actual_models: usize,
    },
    #[error(
        "provider catalog {path} declares {expected_models} models for provider {provider} but contains {actual_models}"
    )]
    ProviderModelCountMismatch {
        path: PathBuf,
        provider: String,
        expected_models: usize,
        actual_models: usize,
    },
    #[error(
        "provider catalog {path} contains a blank model id for provider {provider}: {model_name}"
    )]
    BlankModelId {
        path: PathBuf,
        provider: String,
        model_name: String,
    },
    #[error("provider catalog {path} contains a blank provider id: {provider_name}")]
    BlankProviderId {
        path: PathBuf,
        provider_name: String,
    },
    #[error("provider catalog {path} contains duplicate provider id `{provider}`")]
    DuplicateProviderId { path: PathBuf, provider: String },
    #[error(
        "provider catalog {path} contains duplicate model id `{model}` for provider {provider}"
    )]
    DuplicateModelId {
        path: PathBuf,
        provider: String,
        model: String,
    },
    #[error("provider {provider} was not found in the DX providers catalog")]
    ProviderNotFound { provider: String },
}

#[must_use]
pub fn list_model_provider_catalog_entries() -> Vec<ModelProviderCatalogEntry> {
    list_model_provider_catalog_listing().entries
}

#[must_use]
pub fn list_model_provider_catalog_listing() -> ModelProviderCatalogListing {
    let native = list_model_providers();
    let (catalog, dx_providers_catalog) = load_discovered_dx_providers_catalog();

    ModelProviderCatalogListing {
        entries: merge_provider_catalog_entries(native, catalog),
        dx_providers_catalog,
    }
}

#[must_use]
pub fn discover_dx_providers_catalog() -> Option<PathBuf> {
    explicit_dx_providers_catalog_path().or_else(discover_default_dx_providers_catalog)
}

fn explicit_dx_providers_catalog_path() -> Option<PathBuf> {
    let path = std::env::var(DX_PROVIDERS_CATALOG_ENV).ok()?;
    let path = path.trim();
    (!path.is_empty()).then(|| PathBuf::from(path))
}

fn discover_default_dx_providers_catalog() -> Option<PathBuf> {
    dx_providers_catalog_candidates()
        .into_iter()
        .find(|path| path.is_file())
}

pub fn read_dx_providers_catalog(
    path: impl AsRef<Path>,
) -> Result<DxProvidersCatalogSummary, ProviderCatalogError> {
    let path = path.as_ref();
    if !path.is_file() {
        return Err(ProviderCatalogError::NotFound {
            path: path.to_path_buf(),
        });
    }

    let file = std::fs::File::open(path).map_err(|source| ProviderCatalogError::Open {
        path: path.to_path_buf(),
        source,
    })?;
    let metadata = file
        .metadata()
        .map_err(|source| ProviderCatalogError::Metadata {
            path: path.to_path_buf(),
            source,
        })?;
    let byte_len = metadata.len();
    if byte_len == 0 {
        return Err(ProviderCatalogError::Empty {
            path: path.to_path_buf(),
        });
    }
    if byte_len > MAX_DX_PROVIDERS_CATALOG_BYTES {
        return Err(ProviderCatalogError::FileTooLarge {
            path: path.to_path_buf(),
            actual_bytes: byte_len,
            max_bytes: MAX_DX_PROVIDERS_CATALOG_BYTES,
        });
    }

    let map = unsafe { Mmap::map(&file) }.map_err(|source| ProviderCatalogError::Map {
        path: path.to_path_buf(),
        source,
    })?;
    let archived = check_archived_root::<ProvidersData>(&map).map_err(|source| {
        ProviderCatalogError::Validate {
            path: path.to_path_buf(),
            message: source.to_string(),
        }
    })?;
    let catalog: ProvidersData = archived
        .deserialize(&mut rkyv::Infallible)
        .unwrap_or_else(|error| match error {});
    validate_catalog_counts(path, &catalog)?;

    Ok(DxProvidersCatalogSummary {
        path: path.to_path_buf(),
        total_providers: catalog.total_providers,
        total_models: catalog.total_models,
        providers: catalog
            .providers
            .into_iter()
            .map(|provider| DxProviderCatalogSummary {
                id: provider.id,
                name: provider.name,
                model_count: provider.model_count,
                supports_chat: provider.supports_chat,
                supports_embedding: provider.supports_embedding,
                supports_image: provider.supports_image,
                supports_audio: provider.supports_audio,
                api_url: non_empty_string(provider.api_url),
                models: provider
                    .models
                    .into_iter()
                    .filter_map(|model| {
                        let id = model.id.trim();
                        if id.is_empty() {
                            None
                        } else {
                            Some(DxModelCatalogSummary {
                                id: id.to_string(),
                                name: model.name,
                            })
                        }
                    })
                    .collect(),
            })
            .collect(),
    })
}

fn load_discovered_dx_providers_catalog() -> (
    Vec<DxProviderCatalogSummary>,
    Option<DxProvidersCatalogLoadDiagnostic>,
) {
    let Some(path) = discover_dx_providers_catalog() else {
        return (Vec::new(), None);
    };

    match read_dx_providers_catalog(&path) {
        Ok(catalog) => {
            let diagnostic = DxProvidersCatalogLoadDiagnostic {
                path: catalog.path,
                provider_count: Some(catalog.total_providers),
                model_count: Some(catalog.total_models),
                error: None,
            };
            (catalog.providers, Some(diagnostic))
        }
        Err(error) => {
            let error_message = error.to_string();
            record_dx_providers_catalog_load_failure(&path, &error_message);

            (
                Vec::new(),
                Some(DxProvidersCatalogLoadDiagnostic {
                    path,
                    provider_count: None,
                    model_count: None,
                    error: Some(error_message),
                }),
            )
        }
    }
}

fn record_dx_providers_catalog_load_failure(path: &Path, error: &str) {
    zeroclaw_log::record!(
        WARN,
        zeroclaw_log::Event::new(module_path!(), zeroclaw_log::Action::Fail)
            .with_category(zeroclaw_log::EventCategory::Provider)
            .with_outcome(zeroclaw_log::EventOutcome::Failure)
            .with_attrs(serde_json::json!({
                "catalog_path": path.display().to_string(),
                "error": error,
                "fallback": "native_model_providers",
            })),
        "DX providers catalog load failed; falling back to native providers"
    );
}

fn validate_catalog_counts(
    path: &Path,
    catalog: &ProvidersData,
) -> Result<(), ProviderCatalogError> {
    let actual_providers = catalog.providers.len();
    let actual_models = catalog
        .providers
        .iter()
        .map(|provider| provider.models.len())
        .sum::<usize>();

    if catalog.total_providers != actual_providers || catalog.total_models != actual_models {
        return Err(ProviderCatalogError::CatalogCountMismatch {
            path: path.to_path_buf(),
            expected_providers: catalog.total_providers,
            actual_providers,
            expected_models: catalog.total_models,
            actual_models,
        });
    }

    let mut provider_ids = BTreeSet::new();
    for provider in &catalog.providers {
        let provider_id = provider.id.trim();
        if provider_id.is_empty() {
            return Err(ProviderCatalogError::BlankProviderId {
                path: path.to_path_buf(),
                provider_name: provider.name.clone(),
            });
        }
        let provider_key = canonical_provider_id(provider_id);
        if !provider_ids.insert(provider_key.clone()) {
            return Err(ProviderCatalogError::DuplicateProviderId {
                path: path.to_path_buf(),
                provider: provider_key,
            });
        }

        let mut model_ids = BTreeSet::new();
        if let Some(model) = provider
            .models
            .iter()
            .find(|model| model.id.trim().is_empty())
        {
            return Err(ProviderCatalogError::BlankModelId {
                path: path.to_path_buf(),
                provider: provider.id.clone(),
                model_name: model.name.clone(),
            });
        }
        if let Some(model) = provider.models.iter().find(|model| {
            let model_id = model.id.trim();
            !model_id.is_empty()
                && !model_ids.insert(provider_scoped_model_id(&provider_key, model_id))
        }) {
            return Err(ProviderCatalogError::DuplicateModelId {
                path: path.to_path_buf(),
                provider: provider_key.clone(),
                model: provider_scoped_model_id(&provider_key, model.id.trim()),
            });
        }

        let actual_models = provider.models.len();
        if provider.model_count != actual_models {
            return Err(ProviderCatalogError::ProviderModelCountMismatch {
                path: path.to_path_buf(),
                provider: provider.id.clone(),
                expected_models: provider.model_count,
                actual_models,
            });
        }
    }

    Ok(())
}

pub fn list_models_for_catalog_provider(
    provider_id: &str,
) -> Result<Vec<String>, ProviderCatalogError> {
    let provider = find_dx_provider_catalog_summary(provider_id)?;

    let mut seen = BTreeSet::new();
    Ok(provider
        .models
        .into_iter()
        .filter_map(|model| {
            if seen.insert(model.id.clone()) {
                Some(model.id)
            } else {
                None
            }
        })
        .collect())
}

pub fn find_model_provider_catalog_entry(provider_id: &str) -> Option<ModelProviderCatalogEntry> {
    let requested_keys = provider_identity_keys(provider_id);
    list_model_provider_catalog_entries()
        .into_iter()
        .find(|entry| {
            provider_identity_keys(&entry.name)
                .iter()
                .any(|key| requested_keys.contains(key))
        })
}

pub fn find_dx_provider_catalog_summary(
    provider_id: &str,
) -> Result<DxProviderCatalogSummary, ProviderCatalogError> {
    let Some(path) = discover_dx_providers_catalog() else {
        return Err(ProviderCatalogError::NotFound {
            path: PROVIDERS_CATALOG_RELATIVE_PATH.iter().collect(),
        });
    };
    let catalog = read_dx_providers_catalog(path)?;
    let requested_keys = provider_identity_keys(provider_id);

    let Some(provider) = catalog.providers.into_iter().find(|provider| {
        catalog_provider_identity_keys(provider)
            .iter()
            .any(|key| requested_keys.contains(key))
    }) else {
        return Err(ProviderCatalogError::ProviderNotFound {
            provider: provider_id.to_string(),
        });
    };

    Ok(provider)
}

fn catalog_provider_identity_keys(provider: &DxProviderCatalogSummary) -> BTreeSet<String> {
    let mut keys = provider_identity_keys(&provider.id);
    keys.extend(provider_identity_keys(&provider.name));
    keys
}

pub fn merge_provider_catalog_entries(
    native: Vec<ModelProviderInfo>,
    catalog: Vec<DxProviderCatalogSummary>,
) -> Vec<ModelProviderCatalogEntry> {
    let mut seen = BTreeSet::new();
    let mut entries = Vec::with_capacity(native.len() + catalog.len());

    for provider in native {
        insert_provider_identity_keys(&mut seen, provider.name);
        entries.push(ModelProviderCatalogEntry::from_native(provider));
    }

    for provider in catalog {
        let id = canonical_provider_id(&provider.id);
        let display_name = provider_display_name(&provider.name, &id);
        let mut candidate_keys = provider_identity_keys(&id);
        candidate_keys.extend(provider_identity_keys(&display_name));

        if candidate_keys.iter().any(|key| seen.contains(key)) {
            continue;
        }

        seen.extend(candidate_keys);
        entries.push(ModelProviderCatalogEntry {
            name: id,
            display_name,
            local: false,
            source: ModelProviderCatalogSource::DxProvidersCatalog,
            model_count: provider.model_count,
        });
    }

    entries
}

fn dx_providers_catalog_candidates() -> Vec<PathBuf> {
    let mut candidates = Vec::new();

    if let Ok(current_dir) = std::env::current_dir() {
        push_catalog_path(&mut candidates, current_dir.clone());
        if let Some(parent) = current_dir.parent() {
            push_catalog_path(&mut candidates, parent.to_path_buf());
        }
    }

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    if let Some(dx_root) = manifest_dir
        .parent()
        .and_then(Path::parent)
        .and_then(Path::parent)
    {
        push_catalog_path(&mut candidates, dx_root.to_path_buf());
    }

    let mut seen = BTreeSet::new();
    candidates.retain(|path| seen.insert(path.clone()));
    candidates
}

fn push_catalog_path(candidates: &mut Vec<PathBuf>, root: PathBuf) {
    let path = PROVIDERS_CATALOG_RELATIVE_PATH
        .iter()
        .fold(root, |path, segment| path.join(segment));
    candidates.push(path);
}

fn insert_provider_identity_keys(seen: &mut BTreeSet<String>, id: &str) {
    seen.extend(provider_identity_keys(id));
}

fn provider_identity_keys(id: &str) -> BTreeSet<String> {
    let mut keys = BTreeSet::from([normalize_provider_id(id)]);

    if let Some(metadata) = provider_metadata::metadata_for_provider_id(id) {
        keys.insert(normalize_provider_id(metadata.canonical_id));
        if let Some(runtime_id) = metadata.runtime_id {
            keys.insert(normalize_provider_id(runtime_id));
        }
        keys.extend(
            metadata
                .aliases
                .iter()
                .map(|alias| normalize_provider_id(alias)),
        );
        keys.extend(
            metadata
                .database_ids
                .iter()
                .map(|database_id| normalize_provider_id(database_id)),
        );
    }

    keys
}

fn provider_display_name(name: &str, id: &str) -> String {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        id.to_string()
    } else {
        trimmed.to_string()
    }
}

fn canonical_provider_id(id: &str) -> String {
    let normalized = normalize_provider_id(id);
    if normalized.is_empty() {
        "provider".to_string()
    } else {
        normalized
    }
}

fn provider_scoped_model_id(provider_id: &str, model_id: &str) -> String {
    if model_id
        .split_once('/')
        .is_some_and(|(prefix, _)| prefix == provider_id)
    {
        model_id.to_string()
    } else {
        format!("{provider_id}/{model_id}")
    }
}

fn normalize_provider_id(id: &str) -> String {
    let mut normalized = String::new();
    let mut previous_dash = false;

    for ch in id.trim().chars().flat_map(char::to_lowercase) {
        if ch.is_ascii_alphanumeric() {
            normalized.push(ch);
            previous_dash = false;
        } else if !previous_dash {
            normalized.push('-');
            previous_dash = true;
        }
    }

    normalized.trim_matches('-').to_string()
}

fn non_empty_string(value: String) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

#[derive(Archive, RkyvDeserialize, RkyvSerialize, Debug, Clone)]
#[archive_attr(derive(CheckBytes))]
struct ProvidersData {
    version: String,
    generated_at: String,
    total_providers: usize,
    total_models: usize,
    providers: Vec<Provider>,
}

#[derive(Archive, RkyvDeserialize, RkyvSerialize, Debug, Clone)]
#[archive_attr(derive(CheckBytes))]
struct Provider {
    id: String,
    name: String,
    source: String,
    model_count: usize,
    supports_chat: bool,
    supports_embedding: bool,
    supports_image: bool,
    supports_audio: bool,
    api_url: String,
    docs_url: String,
    models: Vec<Model>,
}

#[derive(Archive, RkyvDeserialize, RkyvSerialize, Debug, Clone)]
#[archive_attr(derive(CheckBytes))]
struct Model {
    id: String,
    name: String,
    mode: String,
    max_tokens: u32,
    input_cost: f64,
    output_cost: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rkyv::ser::{Serializer, serializers::AllocSerializer};
    use std::sync::{Mutex, OnceLock};

    #[test]
    fn provider_catalog_merge_keeps_native_rows_ahead_of_catalog_duplicates() {
        let native = vec![
            ModelProviderInfo {
                name: "gemini",
                display_name: "Google Gemini",
                local: false,
            },
            ModelProviderInfo {
                name: "openrouter",
                display_name: "OpenRouter",
                local: false,
            },
        ];
        let catalog = vec![
            DxProviderCatalogSummary {
                id: "google".to_string(),
                name: "Google AI Studio".to_string(),
                model_count: 7,
                supports_chat: true,
                supports_embedding: false,
                supports_image: false,
                supports_audio: false,
                api_url: Some("https://generativelanguage.googleapis.com/v1beta".to_string()),
                models: Vec::new(),
            },
            DxProviderCatalogSummary {
                id: "example-labs".to_string(),
                name: "Example Labs".to_string(),
                model_count: 2,
                supports_chat: true,
                supports_embedding: false,
                supports_image: false,
                supports_audio: false,
                api_url: Some("https://api.example.test/v1".to_string()),
                models: Vec::new(),
            },
        ];

        let entries = merge_provider_catalog_entries(native, catalog);

        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].name, "gemini");
        assert_eq!(entries[1].name, "openrouter");
        assert_eq!(entries[2].name, "example-labs");
        assert!(entries[0].is_native());
        assert_eq!(
            entries[2].source,
            ModelProviderCatalogSource::DxProvidersCatalog
        );
    }

    #[test]
    fn copied_dx_providers_catalog_loads_when_available() {
        let _env_lock = env_lock();
        let Some(path) = discover_dx_providers_catalog() else {
            return;
        };

        let catalog = read_dx_providers_catalog(&path).expect("DX providers catalog should load");

        assert!(catalog.total_providers >= 100);
        assert!(catalog.total_models >= 1_000);
        assert_eq!(catalog.total_providers, catalog.providers.len());
    }

    #[test]
    fn copied_dx_providers_catalog_matches_snapshot_counts_when_available() {
        let _env_lock = env_lock();
        let Some(path) = discover_dx_providers_catalog() else {
            return;
        };

        let catalog = read_dx_providers_catalog(&path).expect("DX providers catalog should load");

        assert_eq!(catalog.total_providers, 184);
        assert_eq!(catalog.total_models, 6_245);
        assert_eq!(catalog.providers.len(), 184);
        assert_eq!(
            catalog
                .providers
                .iter()
                .map(|provider| provider.models.len())
                .sum::<usize>(),
            6_245
        );
    }

    #[test]
    fn copied_dx_providers_catalog_includes_opencode_zen_free_models_when_available() {
        let _env_lock = env_lock();
        let Some(path) = discover_dx_providers_catalog() else {
            return;
        };

        let catalog = read_dx_providers_catalog(&path).expect("DX providers catalog should load");
        let metadata = crate::provider_metadata::metadata_for_provider_id("opencode-zen")
            .expect("OpenCode Zen metadata");

        for provider_id in metadata.database_ids {
            let provider = catalog
                .providers
                .iter()
                .find(|provider| provider.id == *provider_id)
                .unwrap_or_else(|| panic!("missing DX catalog provider `{provider_id}`"));

            for model_id in metadata.free_model_ids {
                assert!(
                    provider.models.iter().any(|model| model.id == *model_id),
                    "DX catalog provider `{provider_id}` is missing OpenCode Zen free model `{model_id}`"
                );
            }
        }
    }

    #[test]
    fn merged_catalog_includes_copied_dx_providers_when_available() {
        let _env_lock = env_lock();
        if discover_dx_providers_catalog().is_none() {
            return;
        }

        let listing = list_model_provider_catalog_listing();
        let entries = listing.entries;
        let diagnostic = listing
            .dx_providers_catalog
            .expect("copied DX providers catalog should report diagnostics");

        assert!(diagnostic.loaded());
        assert_eq!(diagnostic.provider_count, Some(184));
        assert_eq!(diagnostic.model_count, Some(6_245));
        assert!(
            entries.len() >= 100,
            "merged Agent catalog should include copied DX providers"
        );
        assert!(
            entries.iter().any(|provider| provider.is_native()),
            "native providers must remain present"
        );
        assert!(
            entries
                .iter()
                .any(|provider| provider.source == ModelProviderCatalogSource::DxProvidersCatalog),
            "catalog-only DX providers must be present"
        );
    }

    #[test]
    fn catalog_listing_reports_discovered_catalog_load_errors() {
        let path = std::env::temp_dir().join(format!(
            "zeroclaw-provider-catalog-diagnostic-{}.rkyv",
            std::process::id()
        ));
        std::fs::write(&path, [0x41_u8; 256]).expect("malformed fixture should write");
        let _env_lock = env_lock();
        let _guard = EnvGuard::set(
            DX_PROVIDERS_CATALOG_ENV,
            Some(path.to_string_lossy().as_ref()),
        );

        let listing = list_model_provider_catalog_listing();
        let diagnostic = listing
            .dx_providers_catalog
            .expect("malformed DX providers catalog should report diagnostics");

        assert!(!diagnostic.loaded());
        assert_eq!(diagnostic.path, path);
        assert_eq!(diagnostic.provider_count, None);
        assert_eq!(diagnostic.model_count, None);
        assert!(
            diagnostic
                .error
                .as_deref()
                .is_some_and(|error| error.contains("failed to validate provider catalog")),
            "unexpected diagnostic: {diagnostic:?}"
        );
        assert!(
            listing.entries.iter().any(|entry| entry.is_native()),
            "native providers should remain available when the copied catalog fails"
        );

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn catalog_listing_reports_explicit_missing_catalog_path() {
        let path = missing_fixture_path("zeroclaw-provider-catalog-missing");
        let _env_lock = env_lock();
        let _guard = EnvGuard::set(
            DX_PROVIDERS_CATALOG_ENV,
            Some(path.to_string_lossy().as_ref()),
        );

        let listing = list_model_provider_catalog_listing();
        let diagnostic = listing
            .dx_providers_catalog
            .expect("explicit missing catalog path should report diagnostics");

        assert!(!diagnostic.loaded());
        assert_eq!(diagnostic.path, path);
        assert_eq!(diagnostic.provider_count, None);
        assert_eq!(diagnostic.model_count, None);
        assert!(
            diagnostic
                .error
                .as_deref()
                .is_some_and(|error| error.contains("provider catalog file not found")),
            "unexpected diagnostic: {diagnostic:?}"
        );
        assert!(
            listing.entries.iter().any(|entry| entry.is_native()),
            "native providers should remain available when an explicit catalog path is missing"
        );
    }

    #[test]
    fn explicit_missing_catalog_path_prevents_provider_lookup_fallback() {
        let path = missing_fixture_path("zeroclaw-provider-catalog-lookup-missing");
        let _env_lock = env_lock();
        let _guard = EnvGuard::set(
            DX_PROVIDERS_CATALOG_ENV,
            Some(path.to_string_lossy().as_ref()),
        );

        let error = find_dx_provider_catalog_summary("openai")
            .expect_err("explicit missing catalog path should not fall back");

        assert!(
            matches!(&error, ProviderCatalogError::NotFound { path: error_path } if error_path == &path),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn whitespace_catalog_path_env_uses_normal_discovery() {
        let _env_lock = env_lock();
        let expected = {
            let _guard = EnvGuard::set(DX_PROVIDERS_CATALOG_ENV, None);
            discover_dx_providers_catalog()
        };
        let _guard = EnvGuard::set(DX_PROVIDERS_CATALOG_ENV, Some("  \t  "));

        assert_eq!(discover_dx_providers_catalog(), expected);
    }

    #[tokio::test]
    async fn catalog_listing_logs_discovered_catalog_load_errors() {
        let path = std::env::temp_dir().join(format!(
            "zeroclaw-provider-catalog-log-diagnostic-{}.rkyv",
            std::process::id()
        ));
        std::fs::write(&path, [0x41_u8; 256]).expect("malformed fixture should write");
        let _env_lock = env_lock();
        let _guard = EnvGuard::set(
            DX_PROVIDERS_CATALOG_ENV,
            Some(path.to_string_lossy().as_ref()),
        );
        zeroclaw_log::try_install_capture_subscriber();
        let mut rx = zeroclaw_log::subscribe_or_install();
        while rx.try_recv().is_ok() {}

        let listing = list_model_provider_catalog_listing();
        let diagnostic = listing
            .dx_providers_catalog
            .expect("malformed DX providers catalog should report diagnostics");
        assert!(!diagnostic.loaded());

        let event = receive_log_message(
            &mut rx,
            "DX providers catalog load failed; falling back to native providers",
        )
        .await;
        assert_eq!(
            event.get("severity_text").and_then(|value| value.as_str()),
            Some("WARN")
        );
        assert_eq!(
            event
                .pointer("/event/category")
                .and_then(|value| value.as_str()),
            Some("provider")
        );
        assert_eq!(
            event
                .pointer("/event/outcome")
                .and_then(|value| value.as_str()),
            Some("failure")
        );
        assert_eq!(
            event
                .pointer("/attributes/fallback")
                .and_then(|value| value.as_str()),
            Some("native_model_providers")
        );
        assert_eq!(
            event
                .pointer("/attributes/catalog_path")
                .and_then(|value| value.as_str()),
            Some(path.to_string_lossy().as_ref())
        );

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn fixture_catalog_round_trips_through_rkyv_reader() {
        let path = std::env::temp_dir().join(format!(
            "zeroclaw-provider-catalog-{}.rkyv",
            std::process::id()
        ));
        write_fixture_catalog(&path);

        let catalog = read_dx_providers_catalog(&path).expect("fixture catalog should load");

        assert_eq!(catalog.total_providers, 2);
        assert_eq!(catalog.total_models, 3);
        assert_eq!(catalog.providers[0].id, "alpha");
        assert_eq!(
            catalog.providers[0].api_url.as_deref(),
            Some("https://alpha.example.test/v1")
        );
        assert!(catalog.providers[0].supports_chat);
        assert!(!catalog.providers[0].supports_image);
        assert_eq!(catalog.providers[1].model_count, 1);

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn provider_catalog_rejects_empty_archive_before_mmap() {
        let path = std::env::temp_dir().join(format!(
            "zeroclaw-provider-catalog-empty-{}.rkyv",
            std::process::id()
        ));
        std::fs::write(&path, []).expect("empty fixture should write");

        let error =
            read_dx_providers_catalog(&path).expect_err("empty catalog should fail before mmap");

        assert!(
            matches!(error, ProviderCatalogError::Empty { .. }),
            "unexpected error: {error}"
        );

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn provider_catalog_rejects_oversized_archive_before_mmap() {
        let path = std::env::temp_dir().join(format!(
            "zeroclaw-provider-catalog-oversized-{}.rkyv",
            std::process::id()
        ));
        let file = std::fs::File::create(&path).expect("oversized fixture should create");
        file.set_len(MAX_DX_PROVIDERS_CATALOG_BYTES + 1)
            .expect("oversized fixture length should set");

        let error = read_dx_providers_catalog(&path)
            .expect_err("oversized catalog should fail before mmap");

        assert!(
            matches!(error, ProviderCatalogError::FileTooLarge { .. }),
            "unexpected error: {error}"
        );

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn provider_catalog_rejects_short_malformed_archive() {
        let path = std::env::temp_dir().join(format!(
            "zeroclaw-provider-catalog-short-{}.rkyv",
            std::process::id()
        ));
        std::fs::write(&path, [0_u8]).expect("short fixture should write");

        let error =
            read_dx_providers_catalog(&path).expect_err("short malformed catalog should fail");

        assert!(
            matches!(error, ProviderCatalogError::Validate { .. }),
            "unexpected error: {error}"
        );

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn provider_catalog_rejects_random_malformed_archive() {
        let path = std::env::temp_dir().join(format!(
            "zeroclaw-provider-catalog-random-{}.rkyv",
            std::process::id()
        ));
        std::fs::write(&path, [0x41_u8; 256]).expect("random fixture should write");

        let error =
            read_dx_providers_catalog(&path).expect_err("random malformed catalog should fail");

        assert!(
            matches!(error, ProviderCatalogError::Validate { .. }),
            "unexpected error: {error}"
        );

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn provider_catalog_rejects_mismatched_declared_counts() {
        let path = std::env::temp_dir().join(format!(
            "zeroclaw-provider-catalog-mismatched-counts-{}.rkyv",
            std::process::id()
        ));
        write_fixture_catalog_with_counts(&path, 2, 4, &[2, 1]);

        let error =
            read_dx_providers_catalog(&path).expect_err("mismatched total count should fail");

        assert!(
            matches!(error, ProviderCatalogError::CatalogCountMismatch { .. }),
            "unexpected error: {error}"
        );

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn provider_catalog_rejects_mismatched_provider_model_count() {
        let path = std::env::temp_dir().join(format!(
            "zeroclaw-provider-catalog-mismatched-provider-count-{}.rkyv",
            std::process::id()
        ));
        write_fixture_catalog_with_counts(&path, 2, 3, &[3, 1]);

        let error =
            read_dx_providers_catalog(&path).expect_err("mismatched provider count should fail");

        assert!(
            matches!(
                error,
                ProviderCatalogError::ProviderModelCountMismatch { .. }
            ),
            "unexpected error: {error}"
        );

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn provider_catalog_rejects_blank_model_ids() {
        let path = std::env::temp_dir().join(format!(
            "zeroclaw-provider-catalog-blank-model-{}.rkyv",
            std::process::id()
        ));
        write_fixture_catalog_with_blank_model_id(&path);

        let error = read_dx_providers_catalog(&path).expect_err("blank model id should fail");

        assert!(
            matches!(error, ProviderCatalogError::BlankModelId { .. }),
            "unexpected error: {error}"
        );

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn provider_catalog_rejects_blank_provider_ids() {
        let path = std::env::temp_dir().join(format!(
            "zeroclaw-provider-catalog-blank-provider-{}.rkyv",
            std::process::id()
        ));
        let mut provider = fixture_provider("alpha", "Alpha AI", 1, 1);
        provider.id = "  ".to_string();
        write_fixture_catalog_from_providers(&path, vec![provider]);

        let error = read_dx_providers_catalog(&path).expect_err("blank provider id should fail");

        assert!(error.to_string().contains("blank provider id"));

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn provider_catalog_rejects_duplicate_provider_ids() {
        let path = std::env::temp_dir().join(format!(
            "zeroclaw-provider-catalog-duplicate-provider-{}.rkyv",
            std::process::id()
        ));
        write_fixture_catalog_from_providers(
            &path,
            vec![
                fixture_provider("duplicate-provider", "Duplicate Provider A", 1, 1),
                fixture_provider("duplicate-provider", "Duplicate Provider B", 1, 1),
            ],
        );

        let error =
            read_dx_providers_catalog(&path).expect_err("duplicate provider id should fail");

        assert!(
            error
                .to_string()
                .contains("duplicate provider id `duplicate-provider`"),
            "unexpected error: {error}"
        );

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn provider_catalog_rejects_duplicate_model_ids() {
        let path = std::env::temp_dir().join(format!(
            "zeroclaw-provider-catalog-duplicate-model-{}.rkyv",
            std::process::id()
        ));
        let mut provider = fixture_provider("alpha", "Alpha AI", 2, 2);
        provider.models[1].id = provider.models[0].id.clone();
        write_fixture_catalog_from_providers(&path, vec![provider]);

        let error = read_dx_providers_catalog(&path).expect_err("duplicate model id should fail");

        assert!(
            error.to_string().contains("duplicate model id"),
            "unexpected error: {error}"
        );

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn provider_catalog_rejects_duplicate_provider_scoped_model_ids() {
        let path = std::env::temp_dir().join(format!(
            "zeroclaw-provider-catalog-duplicate-provider-scoped-model-{}.rkyv",
            std::process::id()
        ));
        let mut provider = fixture_provider("openai", "OpenAI", 2, 2);
        provider.models[0].id = "sora-2".to_string();
        provider.models[1].id = "openai/sora-2".to_string();
        write_fixture_catalog_from_providers(&path, vec![provider]);

        let error =
            read_dx_providers_catalog(&path).expect_err("duplicate scoped model id should fail");

        assert!(
            error
                .to_string()
                .contains("duplicate model id `openai/sora-2`"),
            "unexpected error: {error}"
        );

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn fixture_catalog_lists_models_by_provider_id() {
        let path = std::env::temp_dir().join(format!(
            "zeroclaw-provider-catalog-models-{}.rkyv",
            std::process::id()
        ));
        write_fixture_catalog(&path);
        let _env_lock = env_lock();
        let _guard = EnvGuard::set(
            DX_PROVIDERS_CATALOG_ENV,
            Some(path.to_string_lossy().as_ref()),
        );

        let models = list_models_for_catalog_provider("alpha").expect("alpha models should load");

        assert_eq!(
            models,
            vec!["alpha/model-0".to_string(), "alpha/model-1".to_string()]
        );

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn fixture_catalog_finds_provider_summary_by_id() {
        let path = std::env::temp_dir().join(format!(
            "zeroclaw-provider-catalog-summary-{}.rkyv",
            std::process::id()
        ));
        write_fixture_catalog(&path);
        let _env_lock = env_lock();
        let _guard = EnvGuard::set(
            DX_PROVIDERS_CATALOG_ENV,
            Some(path.to_string_lossy().as_ref()),
        );

        let provider =
            find_dx_provider_catalog_summary("alpha").expect("alpha provider should load");

        assert_eq!(provider.name, "Alpha AI");
        assert_eq!(
            provider.api_url.as_deref(),
            Some("https://alpha.example.test/v1")
        );

        let _ = std::fs::remove_file(path);
    }

    async fn receive_log_message(
        rx: &mut tokio::sync::broadcast::Receiver<serde_json::Value>,
        message: &str,
    ) -> serde_json::Value {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
        loop {
            let remaining = deadline.saturating_duration_since(std::time::Instant::now());
            if remaining.is_zero() {
                panic!("timed out waiting for log message: {message}");
            }

            match tokio::time::timeout(
                remaining.min(std::time::Duration::from_millis(50)),
                rx.recv(),
            )
            .await
            {
                Ok(Ok(value)) => {
                    if value
                        .get("message")
                        .and_then(|value| value.as_str())
                        .is_some_and(|actual| actual == message)
                    {
                        return value;
                    }
                }
                Ok(Err(tokio::sync::broadcast::error::RecvError::Lagged(_))) => {}
                Ok(Err(tokio::sync::broadcast::error::RecvError::Closed)) => {
                    panic!("log broadcast closed before message arrived: {message}");
                }
                Err(_) => {}
            }
        }
    }

    fn write_fixture_catalog(path: &Path) {
        write_fixture_catalog_with_counts(path, 2, 3, &[2, 1]);
    }

    fn write_fixture_catalog_with_counts(
        path: &Path,
        total_providers: usize,
        total_models: usize,
        provider_model_counts: &[usize],
    ) {
        let catalog = ProvidersData {
            version: "test".to_string(),
            generated_at: "2026-06-05T00:00:00Z".to_string(),
            total_providers,
            total_models,
            providers: vec![
                fixture_provider("alpha", "Alpha AI", 2, provider_model_counts[0]),
                fixture_provider("beta", "Beta AI", 1, provider_model_counts[1]),
            ],
        };
        let mut serializer = AllocSerializer::<4096>::default();
        serializer
            .serialize_value(&catalog)
            .expect("fixture catalog should serialize");
        let bytes = serializer.into_serializer().into_inner();
        std::fs::write(path, bytes).expect("fixture catalog should write");
    }

    fn write_fixture_catalog_with_blank_model_id(path: &Path) {
        let mut provider = fixture_provider("alpha", "Alpha AI", 2, 2);
        provider.models[1].id = "  ".to_string();
        write_fixture_catalog_from_providers(path, vec![provider]);
    }

    fn write_fixture_catalog_from_providers(path: &Path, providers: Vec<Provider>) {
        let total_providers = providers.len();
        let total_models = providers
            .iter()
            .map(|provider| provider.models.len())
            .sum::<usize>();
        let catalog = ProvidersData {
            version: "test".to_string(),
            generated_at: "2026-06-05T00:00:00Z".to_string(),
            total_providers,
            total_models,
            providers,
        };
        let mut serializer = AllocSerializer::<4096>::default();
        serializer
            .serialize_value(&catalog)
            .expect("fixture catalog should serialize");
        let bytes = serializer.into_serializer().into_inner();
        std::fs::write(path, bytes).expect("fixture catalog should write");
    }

    fn missing_fixture_path(prefix: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "{}-{}-{}.rkyv",
            prefix,
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system clock should be after Unix epoch")
                .as_nanos()
        ))
    }

    fn fixture_provider(
        id: &str,
        name: &str,
        actual_model_count: usize,
        declared_model_count: usize,
    ) -> Provider {
        Provider {
            id: id.to_string(),
            name: name.to_string(),
            source: "test".to_string(),
            model_count: declared_model_count,
            supports_chat: true,
            supports_embedding: false,
            supports_image: false,
            supports_audio: false,
            api_url: format!("https://{id}.example.test/v1"),
            docs_url: String::new(),
            models: (0..actual_model_count)
                .map(|index| Model {
                    id: format!("{id}/model-{index}"),
                    name: format!("{name} Model {index}"),
                    mode: "chat".to_string(),
                    max_tokens: 4096,
                    input_cost: 0.0,
                    output_cost: 0.0,
                })
                .collect(),
        }
    }

    struct EnvGuard {
        key: &'static str,
        original: Option<String>,
    }

    impl EnvGuard {
        fn set(key: &'static str, value: Option<&str>) -> Self {
            let original = std::env::var(key).ok();
            match value {
                Some(value) => unsafe { std::env::set_var(key, value) },
                None => unsafe { std::env::remove_var(key) },
            }
            Self { key, original }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            match self.original.as_deref() {
                Some(value) => unsafe { std::env::set_var(self.key, value) },
                None => unsafe { std::env::remove_var(self.key) },
            }
        }
    }

    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
            .lock()
            .expect("env lock poisoned")
    }
}

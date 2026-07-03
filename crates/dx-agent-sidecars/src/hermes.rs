use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use tracing::{debug, info, warn};

/// Status returned by an update check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateStatus {
    /// Whether an update is available.
    pub available: bool,
    /// Number of commits behind (git) or semver diff indicator.
    pub behind_count: Option<i64>,
    /// Current version string (git HEAD short hash or semver).
    pub current: String,
    /// Upstream version string.
    pub upstream: Option<String>,
    /// Error message if the check failed.
    pub error: Option<String>,
}

/// Perform an update operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateResult {
    /// Whether the update succeeded.
    pub success: bool,
    /// Human-readable message.
    pub message: String,
    /// Previous version before update.
    pub previous_version: Option<String>,
    /// New version after update.
    pub new_version: Option<String>,
}

/// Cache entry for update checks (avoids hitting the network every time).
#[derive(Debug, Clone, Serialize, Deserialize)]
struct UpdateCache {
    /// Unix timestamp of the check.
    pub ts: u64,
    /// Number of commits behind.
    pub behind: Option<i64>,
    /// Embedded revision (short hash) at check time.
    pub rev: Option<String>,
}

const UPDATE_CHECK_CACHE_SECS: u64 = 21_600; // 6 hours

/// Native Rust Hermes auto-update implementation.
///
/// Replaces the previous Python sidecar approach — performs git-based
/// and ZIP-fallback updates directly without spawning an external process.
pub struct HermesUpdate {
    repo_path: PathBuf,
    cache_path: PathBuf,
}

impl HermesUpdate {
    /// Create a new updater for the repo at `repo_path`.
    ///
    /// `cache_dir` is where `.update_check` cache files are stored
    /// (typically `~/.hermes/` or equivalent).
    pub fn new(repo_path: PathBuf, cache_dir: PathBuf) -> Self {
        Self {
            repo_path,
            cache_path: cache_dir.join(".update_check"),
        }
    }

    // ── public API ──────────────────────────────────────────────────────

    /// Check whether an update is available via git rev-list.
    ///
    /// Returns `UpdateStatus` with the commit count behind `origin/main`.
    /// Results are cached for 6 hours.
    pub async fn check_update(&self) -> Result<UpdateStatus> {
        let current = self.git_head_hash()?;

        // Check cache
        if let Some(cached) = self.read_cache() {
            if cached.rev.as_deref() == Some(&current) {
                let age = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
                    .saturating_sub(cached.ts);
                if age < UPDATE_CHECK_CACHE_SECS {
                    let available = cached.behind.map(|b| b > 0).unwrap_or(false);
                    return Ok(UpdateStatus {
                        available,
                        behind_count: cached.behind,
                        current: current.clone(),
                        upstream: None,
                        error: None,
                    });
                }
            }
        }

        // Git repo check
        if !self.repo_path.join(".git").exists() {
            return Ok(UpdateStatus {
                available: false,
                behind_count: None,
                current,
                upstream: None,
                error: Some("not a git repository".into()),
            });
        }

        // Fetch origin/main
        let fetch_ok = self.git_fetch("origin", "main").await;
        if !fetch_ok {
            let err = "failed to fetch from origin — network or auth error".to_string();
            return Ok(UpdateStatus {
                available: false,
                behind_count: None,
                current,
                upstream: None,
                error: Some(err),
            });
        }

        // Count commits behind
        let behind = self.git_behind_count("origin/main");
        let upstream = behind.as_ref().ok().map(|_| self.git_upstream_hash());

        let available = behind.as_deref().copied().map(|b| b > 0).unwrap_or(false);

        // Write cache
        self.write_cache(&UpdateCache {
            ts: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            behind: behind.ok(),
            rev: Some(current.clone()),
        });

        Ok(UpdateStatus {
            available,
            behind_count: behind.ok(),
            current,
            upstream,
            error: None,
        })
    }

    /// Perform a full git-based update (fetch + pull --rebase --autostash).
    ///
    /// Falls back to ZIP download if the git pull fails on Windows.
    pub async fn perform_update(&self) -> Result<UpdateResult> {
        let current = self.git_head_hash()?;

        if !self.repo_path.join(".git").exists() {
            return self.zip_update_fallback(&current).await;
        }

        info!("Performing git update in {}", self.repo_path.display());

        // Fetch
        if !self.git_fetch("origin", "main").await {
            warn!("Git fetch failed, trying ZIP fallback");
            return self.zip_update_fallback(&current).await;
        }

        // Count behind
        let behind = self.git_behind_count("origin/main")?;
        if behind == 0 {
            return Ok(UpdateResult {
                success: true,
                message: "Already up to date!".into(),
                previous_version: Some(current.clone()),
                new_version: Some(current),
            });
        }

        // Stash local changes
        self.git_stash();

        // Pull with rebase
        let pull_ok = self.git_pull_rebase().await;
        if !pull_ok {
            self.git_stash_pop();
            warn!("Git pull failed, trying ZIP fallback");
            return self.zip_update_fallback(&current).await;
        }

        let new_hash = self.git_head_hash()?;

        // Restore stash
        self.git_stash_pop();

        info!("Update complete: {} -> {}", current, new_hash);
        Ok(UpdateResult {
            success: true,
            message: format!("Updated {} commits: {} -> {}", behind, &current[..8], &new_hash[..8]),
            previous_version: Some(current),
            new_version: Some(new_hash),
        })
    }

    /// Download the latest release ZIP from GitHub and replace the repo.
    async fn zip_update_fallback(&self, current: &str) -> Result<UpdateResult> {
        info!("Attempting ZIP-based update for {}", self.repo_path.display());

        #[cfg(feature = "native-update")]
        {
            let url = format!(
                "https://github.com/NousResearch/hermes-agent/archive/refs/heads/main.zip"
            );
            let client = reqwest::Client::builder()
                .timeout(Duration::from_secs(120))
                .build()?;
            let resp = client.get(&url).send().await?;
            if !resp.status().is_success() {
                bail!("ZIP download failed: HTTP {}", resp.status());
            }
            let bytes = resp.bytes().await?;

            let tmp = self.repo_path.with_extension("tmp_update");
            if tmp.exists() {
                std::fs::remove_dir_all(&tmp)?;
            }
            std::fs::create_dir_all(&tmp)?;

            let reader = std::io::Cursor::new(&bytes);
            let mut archive = zip::ZipArchive::new(reader)?;

            // Extract to parent, then rename into place
            let parent = self.repo_path.parent().unwrap_or(Path::new("."));
            for i in 0..archive.len() {
                let mut file = archive.by_index(i)?;
                let outpath = parent.join(file.name());
                if file.is_dir() {
                    std::fs::create_dir_all(&outpath)?;
                } else {
                    if let Some(p) = outpath.parent() {
                        std::fs::create_dir_all(p)?;
                    }
                    let mut outfile = std::fs::File::create(&outpath)?;
                    std::io::copy(&mut file, &mut outfile)?;
                }
            }

            // The ZIP contains a top-level dir like "hermes-agent-main/"
            let extracted = parent.join("hermes-agent-main");
            if extracted.exists() {
                // Remove old repo contents (keeping .git)
                let git_dir = self.repo_path.join(".git");
                for entry in std::fs::read_dir(&self.repo_path)? {
                    let entry = entry?;
                    let path = entry.path();
                    if path != git_dir {
                        if path.is_dir() {
                            std::fs::remove_dir_all(&path)?;
                        } else {
                            std::fs::remove_file(&path)?;
                        }
                    }
                }
                // Copy extracted files (but not .git)
                for entry in std::fs::read_dir(&extracted)? {
                    let entry = entry?;
                    let path = entry.path();
                    let name = path.file_name().unwrap();
                    if name != ".git" {
                        let dest = self.repo_path.join(name);
                        if path.is_dir() {
                            copy_dir(&path, &dest)?;
                        } else {
                            std::fs::copy(&path, &dest)?;
                        }
                    }
                }
                std::fs::remove_dir_all(&extracted)?;
            }

            let new_hash = self.git_head_hash().unwrap_or_else(|_| "unknown".into());
            std::fs::remove_dir_all(&tmp)?;

            info!("ZIP update complete");
            Ok(UpdateResult {
                success: true,
                message: format!("Updated via ZIP: {} -> {}", &current[..8], &new_hash[..8]),
                previous_version: Some(current.to_string()),
                new_version: Some(new_hash),
            })
        }

        #[cfg(not(feature = "native-update"))]
        {
            let _ = current;
            bail!("ZIP update requires the 'native-update' feature");
        }
    }

    /// Get the list of available update "phases" (informational).
    pub fn list_update_phases(&self) -> Vec<&'static str> {
        vec![
            "1. check git origin HEAD",
            "2. fetch origin/main",
            "3. count commits behind",
            "4. stash local changes",
            "5. git pull --rebase --autostash",
            "6. restore stash",
            "7. (fallback) download ZIP from GitHub",
            "8. (fallback) atomic replace repo contents",
        ]
    }

    // ── git helpers ──────────────────────────────────────────────────────

    fn git_head_hash(&self) -> Result<String> {
        let out = self.git_output(&["rev-parse", "--short", "HEAD"])?;
        Ok(out.trim().to_string())
    }

    fn git_upstream_hash(&self) -> String {
        self.git_output(&["rev-parse", "--short", "origin/main"])
            .unwrap_or_else(|_| "unknown".into())
            .trim()
            .to_string()
    }

    fn git_behind_count(&self, target: &str) -> Result<i64> {
        let out = self.git_output(&["rev-list", "--count", &format!("HEAD..{}", target)])?;
        out.trim()
            .parse::<i64>()
            .context("failed to parse commit count")
    }

    fn git_fetch(&self, remote: &str, refspec: &str) -> bool {
        self.git_run(&["fetch", remote, refspec])
    }

    fn git_stash(&self) {
        self.git_run(&["stash"]);
    }

    fn git_stash_pop(&self) {
        self.git_run(&["stash", "pop"]);
    }

    async fn git_pull_rebase(&self) -> bool {
        let status = tokio::process::Command::new("git")
            .args(["pull", "--rebase", "--autostash"])
            .current_dir(&self.repo_path)
            .status()
            .await;
        match status {
            Ok(s) if s.success() => true,
            _ => false,
        }
    }

    fn git_run(&self, args: &[&str]) -> bool {
        std::process::Command::new("git")
            .args(args)
            .current_dir(&self.repo_path)
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }

    fn git_output(&self, args: &[&str]) -> Result<String> {
        let out = std::process::Command::new("git")
            .args(args)
            .current_dir(&self.repo_path)
            .output()
            .context("failed to run git command")?;
        if !out.status.success() {
            let stderr = String::from_utf8_lossy(&out.stderr);
            bail!("git {} failed: {}", args.join(" "), stderr.trim());
        }
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    }

    // ── cache ─────────────────────────────────────────────────────────────

    fn read_cache(&self) -> Option<UpdateCache> {
        let data = std::fs::read_to_string(&self.cache_path).ok()?;
        serde_json::from_str(&data).ok()
    }

    fn write_cache(&self, cache: &UpdateCache) {
        if let Ok(data) = serde_json::to_string(cache) {
            let _ = std::fs::write(&self.cache_path, data);
        }
    }
}

/// Recursively copy a directory.
#[cfg(feature = "native-update")]
fn copy_dir(src: &Path, dst: &Path) -> Result<()> {
    if !dst.exists() {
        std::fs::create_dir_all(dst)?;
    }
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dest_path = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir(&entry.path(), &dest_path)?;
        } else {
            std::fs::copy(&entry.path(), &dest_path)?;
        }
    }
    Ok(())
}

/// Legacy Hermes sidecar kept for backward compatibility.
///
/// Uses the native `HermesUpdate` internally instead of spawning Python.
pub struct HermesSidecar {
    updater: Option<HermesUpdate>,
}

impl HermesSidecar {
    pub fn new() -> Self {
        Self { updater: None }
    }

    /// Initialize with the repo path where Hermes is installed.
    pub fn init(&mut self, repo_path: PathBuf, cache_dir: PathBuf) {
        self.updater = Some(HermesUpdate::new(repo_path, cache_dir));
    }

    /// Check if an update is available.
    pub async fn check_update(&self) -> Result<serde_json::Value> {
        let Some(updater) = &self.updater else {
            bail!("HermesSidecar not initialized — call init() first");
        };
        let status = updater.check_update().await?;
        Ok(serde_json::to_value(status)?)
    }

    /// Perform a full update.
    pub async fn perform_update(&self) -> Result<serde_json::Value> {
        let Some(updater) = &self.updater else {
            bail!("HermesSidecar not initialized — call init() first");
        };
        let result = updater.perform_update().await?;
        Ok(serde_json::to_value(result)?)
    }

    /// List available update phases.
    pub async fn list_update_phases(&self) -> Result<serde_json::Value> {
        let Some(updater) = &self.updater else {
            bail!("HermesSidecar not initialized — call init() first");
        };
        let phases = updater.list_update_phases();
        Ok(serde_json::to_value(phases)?)
    }

    /// No-op — kept for API compatibility.
    pub async fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Default for HermesSidecar {
    fn default() -> Self {
        Self::new()
    }
}

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::Mutex;
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
const BACKGROUND_CHECK_INTERVAL: Duration = Duration::from_secs(3600); // 1 hour

/// Native Rust Hermes auto-update implementation.
///
/// Performs git-based and ZIP-fallback updates directly without
/// spawning an external Python process.
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
    /// Results are cached on disk for 6 hours.
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

// ═══════════════════════════════════════════════════════════════════════════
// Background auto-update check loop
//
// Mirrors the Python pattern from hermes-agent/hermes_cli/banner.py:
//   prefetch_update_check() → daemon thread → check_for_updates()
//   get_update_result(timeout) → cached result
//
// Spawns a background task that periodically checks for updates and
// stores the result in an Arc<Mutex<Option<UpdateStatus>>>.
// ═══════════════════════════════════════════════════════════════════════════

/// A background auto-update checker that runs on an interval.
///
/// Call [`BackgroundUpdater::spawn`] to start the background loop,
/// then [`BackgroundUpdater::latest`] to get the most recent result.
pub struct BackgroundUpdater {
    inner: Arc<BackgroundInner>,
}

struct BackgroundInner {
    updater: HermesUpdate,
    latest: Mutex<Option<UpdateStatus>>,
    running: AtomicBool,
}

impl BackgroundUpdater {
    /// Create a new background updater backed by the given `HermesUpdate`.
    pub fn new(updater: HermesUpdate) -> Self {
        Self {
            inner: Arc::new(BackgroundInner {
                updater,
                latest: Mutex::new(None),
                running: AtomicBool::new(false),
            }),
        }
    }

    /// Spawn the background check loop.
    ///
    /// Runs immediately on spawn, then every `interval`.
    /// The task is detached (runs until dropped).
    pub fn spawn(&self, interval: Duration) {
        let inner = self.inner.clone();
        self.inner.running.store(true, Ordering::Relaxed);

        tokio::spawn(async move {
            info!("Background update checker started");

            // Immediate first check
            inner.run_check().await;

            // Periodic checks
            while inner.running.load(Ordering::Relaxed) {
                tokio::time::sleep(interval).await;
                if !inner.running.load(Ordering::Relaxed) {
                    break;
                }
                inner.run_check().await;
            }

            info!("Background update checker stopped");
        });
    }

    /// Stop the background loop (best-effort, signal-only).
    pub fn stop(&self) {
        self.inner.running.store(false, Ordering::Relaxed);
    }

    /// Get the latest cached check result, or run a fresh check if none exists.
    pub async fn latest(&self) -> Option<UpdateStatus> {
        let mut latest = self.inner.latest.lock().await;
        if latest.is_some() {
            return latest.clone();
        }
        // First-time: run a check synchronously
        let result = self.inner.updater.check_update().await.ok();
        *latest = result.clone();
        result
    }

    /// Whether the background loop is running.
    pub fn is_running(&self) -> bool {
        self.inner.running.load(Ordering::Relaxed)
    }
}

impl BackgroundInner {
    async fn run_check(&self) {
        debug!("Running background update check");
        let status = self.updater.check_update().await;
        match status {
            Ok(s) => {
                let mut latest = self.latest.lock().await;
                if s.available {
                    info!(
                        "Update available: {} commits behind (current={})",
                        s.behind_count.unwrap_or(0),
                        s.current
                    );
                }
                *latest = Some(s);
            }
            Err(e) => {
                debug!("Background update check failed: {e:#}");
            }
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

// ═══════════════════════════════════════════════════════════════════════════
// HermesSidecar — COMMENTED OUT for shipping focus.
//
// This was the original Python sidecar proxy. The native HermesUpdate +
// BackgroundUpdater above now replace it entirely. Re-enable only if a
// consumer specifically needs the old JSON-RPC response shape.
// ═══════════════════════════════════════════════════════════════════════════
// pub struct HermesSidecar { ... }

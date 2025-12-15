use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

/// Git reference type for template download
#[derive(Debug, Clone)]
pub enum GitRef {
    /// A specific commit hash
    Commit(String),
    /// The main branch (latest)
    Main,
}

/// Version to commit mapping structure
#[derive(Debug, Deserialize)]
struct VersionMapping {
    #[serde(flatten)]
    versions: HashMap<String, String>,
}

/// Resolve rmk-template version to a git reference
///
/// # Arguments
/// * `version` - Optional version string (e.g., "0.7", "0.8")
///
/// # Returns
/// * GitRef representing the template version
pub async fn resolve_template_version(version: Option<&str>) -> GitRef {
    match version {
        Some(version) => {
            // Fetch version mapping from remote config
            match fetch_version_mapping(version).await {
                Ok(commit) => {
                    println!("📌 Using rmk-template commit: {}", &commit);
                    GitRef::Commit(commit)
                }
                Err(e) => {
                    println!("⚠️  Failed to fetch version mapping: {}", e);
                    println!("⚠️  Using latest template from main branch");
                    GitRef::Main
                }
            }
        }
        None => {
            // Default to latest
            GitRef::Main
        }
    }
}

/// Fetch version to commit mapping from remote config
async fn fetch_version_mapping(version: &str) -> Result<String, Box<dyn Error>> {
    let config_url =
        "https://raw.githubusercontent.com/HaoboGu/rmk-template/main/version-mapping.json";

    let client = Client::new();
    let response = client.get(config_url).send().await?;

    if !response.status().is_success() {
        return Err(format!("Failed to fetch version mapping: {}", response.status()).into());
    }

    let mapping: VersionMapping = response.json().await?;

    mapping
        .versions
        .get(version)
        .cloned()
        .ok_or_else(|| format!("Version {} not found in mapping", version).into())
}

/// Build GitHub archive URL based on git reference
///
/// # Arguments
/// * `user` - GitHub username
/// * `repo` - Repository name
/// * `git_ref` - Git reference (commit or main branch)
///
/// # Returns
/// * GitHub archive URL
pub fn build_github_archive_url(user: &str, repo: &str, git_ref: &GitRef) -> String {
    match git_ref {
        GitRef::Commit(hash) => {
            format!("https://github.com/{}/{}/archive/{}.zip", user, repo, hash)
        }
        GitRef::Main => {
            format!(
                "https://github.com/{}/{}/archive/refs/heads/main.zip",
                user, repo
            )
        }
    }
}

use reqwest;
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct GithubPR {
    pub number: u32,
    pub title: String,
    pub html_url: String,
    pub updated_at: String,
    pub user: GithubUser,
}

#[derive(Debug, Deserialize)]
pub struct GithubUser {
    pub login: String,
}

#[derive(Debug, Deserialize)]
struct GithubError {
    message: String,
    #[serde(default)]
    documentation_url: String,
}

/// Get GitHub token from multiple sources (in order of priority):
/// 1. GITHUB_TOKEN environment variable
/// 2. .github-token file in the executable directory
/// 3. .github-token file in the user's home directory
fn get_github_token() -> Option<String> {
    // Try environment variable first
    if let Ok(token) = env::var("GITHUB_TOKEN") {
        if !token.trim().is_empty() {
            return Some(token.trim().to_string());
        }
    }
    
    // Try .github-token in executable directory
    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let token_file = exe_dir.join(".github-token");
            if let Ok(token) = fs::read_to_string(&token_file) {
                let token = token.trim().to_string();
                if !token.is_empty() {
                    return Some(token);
                }
            }
        }
    }
    
    // Try .github-token in home directory
    if let Some(home_dir) = dirs_next::home_dir() {
        let token_file = home_dir.join(".github-token");
        if let Ok(token) = fs::read_to_string(&token_file) {
            let token = token.trim().to_string();
            if !token.is_empty() {
                return Some(token);
            }
        }
    }
    
    None
}

pub async fn fetch_prs(owner: &str, repo: &str) -> Result<Vec<super::PullRequest>, Box<dyn std::error::Error>> {
    let url = format!("https://api.github.com/repos/{}/{}/pulls?state=open&sort=updated&direction=desc", owner, repo);
    
    let client = reqwest::Client::new();
    let mut request = client
        .get(&url)
        .header("User-Agent", "godot-pr-launcher")
        .header("Accept", "application/vnd.github.v3+json");
    
    // Add authorization header if token is available
    if let Some(token) = get_github_token() {
        request = request.header("Authorization", format!("token {}", token));
    }
    
    let response = request.send().await?;
    
    let status = response.status();
    
    // Check if the request was successful
    if !status.is_success() {
        // Try to parse as GitHub error response
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        
        // Try to parse as GitHub error JSON
        if let Ok(github_err) = serde_json::from_str::<GithubError>(&error_text) {
            return Err(format!("GitHub API error ({}): {}", status, github_err.message).into());
        } else {
            return Err(format!("GitHub API error ({}): {}", status, error_text).into());
        }
    }
    
    // Parse the JSON response
    let github_prs = response.json::<Vec<GithubPR>>().await
        .map_err(|e| {
            format!("Failed to parse GitHub response: {}. The repository '{}/{}' may not exist or may be private.", 
                e, owner, repo)
        })?;
    
    let prs = github_prs.into_iter().map(|pr| super::PullRequest {
        number: pr.number,
        title: pr.title,
        html_url: pr.html_url,
        updated_at: pr.updated_at,
        user: pr.user.login,
    }).collect();
    
    Ok(prs)
}

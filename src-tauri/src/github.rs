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

/// Mask token for safe logging (show first 5 and last 5 characters)
fn mask_token(token: &str) -> String {
    if token.len() <= 10 {
        return "*****".to_string();
    }
    let first = &token[..5];
    let last = &token[token.len()-5..];
    format!("{}...{}", first, last)
}

/// Get GitHub token from multiple sources (in order of priority):
/// 1. GITHUB_TOKEN environment variable
/// 2. .github-token file in the executable directory
/// 3. .github-token file in the user's home directory
fn get_github_token() -> (Option<String>, String) {
    let mut debug_info = String::new();
    
    // Try environment variable first
    if let Ok(token) = env::var("GITHUB_TOKEN") {
        if !token.trim().is_empty() {
            let token = token.trim().to_string();
            debug_info.push_str(&format!("✓ Token found in GITHUB_TOKEN env var: {}\n", mask_token(&token)));
            return (Some(token), debug_info);
        } else {
            debug_info.push_str("✗ GITHUB_TOKEN env var is empty\n");
        }
    } else {
        debug_info.push_str("✗ GITHUB_TOKEN env var not set\n");
    }
    
    // Try .github-token in executable directory
    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let token_file = exe_dir.join(".github-token");
            debug_info.push_str(&format!("Checking: {}\n", token_file.display()));
            if let Ok(token) = fs::read_to_string(&token_file) {
                let token = token.trim().to_string();
                if !token.is_empty() {
                    debug_info.push_str(&format!("✓ Token found in exe directory: {}\n", mask_token(&token)));
                    return (Some(token), debug_info);
                } else {
                    debug_info.push_str("✗ .github-token file is empty\n");
                }
            } else {
                debug_info.push_str("✗ .github-token file not found or not readable\n");
            }
        }
    }
    
    // Try .github-token in home directory
    if let Some(home_dir) = dirs_next::home_dir() {
        let token_file = home_dir.join(".github-token");
        debug_info.push_str(&format!("Checking: {}\n", token_file.display()));
        if let Ok(token) = fs::read_to_string(&token_file) {
            let token = token.trim().to_string();
            if !token.is_empty() {
                debug_info.push_str(&format!("✓ Token found in home directory: {}\n", mask_token(&token)));
                return (Some(token), debug_info);
            } else {
                debug_info.push_str("✗ .github-token file is empty\n");
            }
        } else {
            debug_info.push_str("✗ .github-token file not found or not readable\n");
        }
    } else {
        debug_info.push_str("✗ Could not determine home directory\n");
    }
    
    debug_info.push_str("\n⚠️  No GitHub token found. Using unauthenticated API (rate limited to 60 req/hour)\n");
    (None, debug_info)
}

pub async fn fetch_prs(owner: &str, repo: &str) -> Result<Vec<super::PullRequest>, Box<dyn std::error::Error>> {
    let url = format!("https://api.github.com/repos/{}/{}/pulls?state=open&sort=updated&direction=desc", owner, repo);
    
    // Get token and debug info
    let (token_opt, debug_info) = get_github_token();
    
    // Log debug info to console
    println!("\n=== GitHub API Debug Info ===");
    println!("Repository: {}/{}", owner, repo);
    println!("API URL: {}", url);
    println!("\n{}", debug_info);
    
    let client = reqwest::Client::new();
    let mut request = client
        .get(&url)
        .header("User-Agent", "godot-pr-launcher")
        .header("Accept", "application/vnd.github.v3+json");
    
    // Add authorization header if token is available
    if let Some(token) = token_opt {
        request = request.header("Authorization", format!("token {}", token));
        println!("✓ Authorization header added to request");
    } else {
        println!("✗ No authorization header (using public API)");
    }
    println!("=============================\n");
    
    let response = request.send().await?;
    
    let status = response.status();
    
    // Check if the request was successful
    if !status.is_success() {
        // Try to parse as GitHub error response
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        
        // Build detailed error message with debug info
        let mut error_msg = format!("GitHub API error ({}):\n\n", status);
        
        // Try to parse as GitHub error JSON
        if let Ok(github_err) = serde_json::from_str::<GithubError>(&error_text) {
            error_msg.push_str(&format!("{}\n\n", github_err.message));
        } else {
            error_msg.push_str(&format!("{}\n\n", error_text));
        }
        
        // Add debug info to error message
        error_msg.push_str("Debug Information:\n");
        error_msg.push_str(&debug_info);
        error_msg.push_str(&format!("\nRepository: {}/{}", owner, repo));
        
        // Add helpful hints based on status code
        match status.as_u16() {
            404 => {
                error_msg.push_str("\n\n💡 Possible causes:");
                error_msg.push_str("\n• Repository doesn't exist");
                error_msg.push_str("\n• Repository name is incorrect");
                error_msg.push_str("\n• Repository is private and token is missing/invalid");
            },
            401 => {
                error_msg.push_str("\n\n💡 Authentication failed:");
                error_msg.push_str("\n• Token is invalid or expired");
                error_msg.push_str("\n• Create new token at: https://github.com/settings/tokens");
            },
            403 => {
                error_msg.push_str("\n\n💡 Access denied:");
                error_msg.push_str("\n• Token lacks 'repo' scope");
                error_msg.push_str("\n• Rate limit exceeded (60/hour without token)");
            },
            _ => {}
        }
        
        return Err(error_msg.into());
    }
    
    // Parse the JSON response
    let github_prs = response.json::<Vec<GithubPR>>().await
        .map_err(|e| {
            format!("Failed to parse GitHub response: {}.\n\nDebug Info:\n{}", e, debug_info)
        })?;
    
    println!("✓ Successfully loaded {} pull request(s)", github_prs.len());
    
    let prs = github_prs.into_iter().map(|pr| super::PullRequest {
        number: pr.number,
        title: pr.title,
        html_url: pr.html_url,
        updated_at: pr.updated_at,
        user: pr.user.login,
    }).collect();
    
    Ok(prs)
}

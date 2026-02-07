use reqwest;
use serde::Deserialize;

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

pub async fn fetch_prs(owner: &str, repo: &str) -> Result<Vec<super::PullRequest>, Box<dyn std::error::Error>> {
    let url = format!("https://api.github.com/repos/{}/{}/pulls?state=open&sort=updated&direction=desc", owner, repo);
    
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "godot-pr-launcher")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await?;
    
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

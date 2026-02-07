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

pub async fn fetch_prs(owner: &str, repo: &str) -> Result<Vec<super::PullRequest>, Box<dyn std::error::Error>> {
    let url = format!("https://api.github.com/repos/{}/{}/pulls?state=open&sort=updated&direction=desc", owner, repo);
    
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "godot-pr-launcher")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await?
        .json::<Vec<GithubPR>>()
        .await?;
    
    let prs = response.into_iter().map(|pr| super::PullRequest {
        number: pr.number,
        title: pr.title,
        html_url: pr.html_url,
        updated_at: pr.updated_at,
        user: pr.user.login,
    }).collect();
    
    Ok(prs)
}

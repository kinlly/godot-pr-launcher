use std::process::Command;
use std::path::Path;

pub fn launch(pr_number: u32) -> Result<String, String> {
    let repo_path = "c:\\repos\\ylbtm";
    
    // First, checkout the PR branch
    println!("Fetching and checking out PR #{}", pr_number);
    
    // First checkout main to avoid issues with deleting the current branch
    let checkout_main = Command::new("git")
        .current_dir(repo_path)
        .args(&["checkout", "main"])
        .output()
        .map_err(|e| format!("Failed to checkout main: {}. Make sure git is installed and the repo path is correct.", e))?;
    
    if !checkout_main.status.success() {
        let stderr = String::from_utf8_lossy(&checkout_main.stderr);
        return Err(format!("Git checkout main failed: {}", stderr));
    }
    
    // Delete the local PR branch if it exists (to get the latest version)
    let delete_result = Command::new("git")
        .current_dir(repo_path)
        .args(&["branch", "-D", &format!("pr-{}", pr_number)])
        .output()
        .map_err(|e| format!("Failed to delete old PR branch: {}", e))?;
    
    // It's OK if the branch doesn't exist (first time checking out this PR)
    if delete_result.status.success() {
        println!("✓ Deleted old pr-{} branch", pr_number);
    }
    
    // Fetch the PR ref and create a new branch
    let fetch_result = Command::new("git")
        .current_dir(repo_path)
        .args(&["fetch", "origin", &format!("pull/{}/head:pr-{}", pr_number, pr_number)])
        .output()
        .map_err(|e| format!("Failed to fetch PR: {}. Make sure git is installed and the repo path is correct.", e))?;
    
    if !fetch_result.status.success() {
        let stderr = String::from_utf8_lossy(&fetch_result.stderr);
        return Err(format!("Git fetch failed: {}", stderr));
    }
    
    println!("✓ Fetched PR #{}", pr_number);
    
    // Checkout the PR branch
    let checkout_result = Command::new("git")
        .current_dir(repo_path)
        .args(&["checkout", &format!("pr-{}", pr_number)])
        .output()
        .map_err(|e| format!("Failed to checkout PR branch: {}", e))?;
    
    if !checkout_result.status.success() {
        let stderr = String::from_utf8_lossy(&checkout_result.stderr);
        return Err(format!("Git checkout failed: {}", stderr));
    }
    
    println!("✓ Checked out PR #{}", pr_number);
    
    // Now launch Godot with the PR branch checked out
    let output = Command::new("C:\\repos\\godot.exe")
        .args(&["--path", repo_path])
        .spawn()
        .map_err(|e| format!("Failed to launch Godot: {}. Make sure Godot is installed at C:\\repos\\godot.exe", e))?;
    
    println!("✓ Launched Godot for PR #{} (PID: {:?})", pr_number, output.id());
    
    Ok(format!("Checked out and launched PR #{}", pr_number))
}

pub fn launch_main() -> Result<String, String> {
    let repo_path = "c:\\repos\\ylbtm";
    
    println!("Checking out main branch");
    
    // Checkout main branch
    let checkout_result = Command::new("git")
        .current_dir(repo_path)
        .args(&["checkout", "main"])
        .output()
        .map_err(|e| format!("Failed to checkout main: {}. Make sure git is installed and the repo path is correct.", e))?;
    
    if !checkout_result.status.success() {
        let stderr = String::from_utf8_lossy(&checkout_result.stderr);
        return Err(format!("Git checkout failed: {}", stderr));
    }
    
    println!("✓ Checked out main branch");
    
    // Now launch Godot with main branch checked out
    let output = Command::new("C:\\repos\\godot.exe")
        .args(&["--path", repo_path])
        .spawn()
        .map_err(|e| format!("Failed to launch Godot: {}. Make sure Godot is installed at C:\\repos\\godot.exe", e))?;
    
    println!("✓ Launched Godot with main branch (PID: {:?})", output.id());
    
    Ok("Checked out and launched main branch".to_string())
}

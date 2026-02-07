use std::process::Command;

/// Helper to run a git command and return a friendly error on failure
fn run_git(repo_path: &str, args: &[&str], error_context: &str) -> Result<String, String> {
    let output = Command::new("git")
        .current_dir(repo_path)
        .args(args)
        .output()
        .map_err(|e| format!("{}: {}. Make sure git is installed and the repo path is correct.", error_context, e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if !output.status.success() {
        return Err(format!("{}: {}", error_context, stderr));
    }

    // Print any useful output
    if !stdout.trim().is_empty() {
        println!("  {}", stdout.trim());
    }

    Ok(stdout)
}

pub fn launch(pr_number: u32, branch_name: &str) -> Result<String, String> {
    let repo_path = "c:\\repos\\ylbtm";

    println!("=== Launching PR #{} (branch: {}) ===", pr_number, branch_name);

    // 1. Fetch latest refs from origin
    println!("Fetching latest from origin...");
    run_git(repo_path, &["fetch", "origin"], "Failed to fetch from origin")?;
    println!("✓ Fetched latest refs");

    // 2. Checkout the PR's source branch
    println!("Checking out branch '{}'...", branch_name);
    // Try to checkout the branch - it may already exist locally or need to be created from origin
    let checkout_result = run_git(repo_path, &["checkout", branch_name], "checkout branch");

    if checkout_result.is_err() {
        // Branch doesn't exist locally, create it tracking the remote
        println!("Branch '{}' not found locally, creating from origin...", branch_name);
        run_git(
            repo_path,
            &["checkout", "-b", branch_name, &format!("origin/{}", branch_name)],
            &format!("Failed to checkout and track branch '{}'", branch_name),
        )?;
    }
    println!("✓ Checked out branch '{}'", branch_name);

    // 3. Pull latest changes for this branch
    println!("Pulling latest changes for '{}'...", branch_name);
    run_git(
        repo_path,
        &["pull", "origin", branch_name],
        &format!("Failed to pull branch '{}'", branch_name),
    )?;
    println!("✓ Pulled latest changes for '{}'", branch_name);

    // 4. Launch Godot
    let output = Command::new("C:\\repos\\godot.exe")
        .args(&["--path", repo_path])
        .spawn()
        .map_err(|e| format!("Failed to launch Godot: {}. Make sure Godot is installed at C:\\repos\\godot.exe", e))?;

    println!("✓ Launched Godot for PR #{} on branch '{}' (PID: {:?})", pr_number, branch_name, output.id());

    Ok(format!("Checked out branch '{}' (PR #{}) with latest changes and launched Godot", branch_name, pr_number))
}

pub fn launch_main() -> Result<String, String> {
    let repo_path = "c:\\repos\\ylbtm";

    println!("=== Launching Main Branch ===");

    // 1. Checkout main branch
    println!("Checking out main branch...");
    run_git(
        repo_path,
        &["checkout", "main"],
        "Failed to checkout main",
    )?;
    println!("✓ Checked out main branch");

    // 2. Fetch latest refs from origin
    println!("Fetching latest from origin...");
    run_git(repo_path, &["fetch", "origin"], "Failed to fetch from origin")?;
    println!("✓ Fetched latest refs");

    // 3. Pull latest changes
    println!("Pulling latest changes for main...");
    run_git(
        repo_path,
        &["pull", "origin", "main"],
        "Failed to pull main",
    )?;
    println!("✓ Pulled latest changes for main");

    // 4. Launch Godot
    let output = Command::new("C:\\repos\\godot.exe")
        .args(&["--path", repo_path])
        .spawn()
        .map_err(|e| format!("Failed to launch Godot: {}. Make sure Godot is installed at C:\\repos\\godot.exe", e))?;

    println!("✓ Launched Godot with main branch (PID: {:?})", output.id());

    Ok("Checked out main branch with latest changes and launched Godot".to_string())
}

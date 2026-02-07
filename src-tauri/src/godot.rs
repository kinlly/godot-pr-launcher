use std::process::Command;

pub fn launch(pr_number: u32) -> Result<String, String> {
    // Ejecutar Godot en background
    let output = Command::new("C:\\repos\\godot.exe")
        .args(&["--path", "c:\\repos\\ylbtm"])
        .spawn()
        .map_err(|e| format!("Failed to launch Godot: {}", e))?;
    
    println!("Launched Godot for PR #{} (PID: {:?})", pr_number, output.id());
    
    Ok(format!("Godot launched for PR #{}", pr_number))
}

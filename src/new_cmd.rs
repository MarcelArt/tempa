use std::{fs::{self}, process::Command};
use crate::utils::snake_case;

fn clone_template(project_name: String) {
    let template_url = "https://github.com/MarcelArt/godot_rust_template.git";
    let output = Command::new("git")
        .arg("clone")
        .arg(template_url)
        .arg(&project_name)
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        eprintln!("Error cloning template: {}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }
}

fn rollback(project_name: String) {
    let output = Command::new("rm")
        .arg("-rf")
        .arg(&project_name)
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        eprintln!("Error rolling back: {}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }
}

fn remove_git_directory(project_name: String) {
    let output = Command::new("rm")
        .arg("-rf")
        .arg(format!("{}/.git", project_name))
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        eprintln!("Error removing .git directory: {}", String::from_utf8_lossy(&output.stderr));
        rollback(project_name.clone());
        std::process::exit(1);
    }
}

fn refactor_project_name(project_name: String, snake_case_name: String) -> Result<(), String> {
    // Refactor Cargo.toml
    let cargo_toml_path = format!("{}/Cargo.toml", snake_case_name);
    let content = fs::read_to_string(&cargo_toml_path).map_err(|e| e.to_string())?;
    let new_content = content.replace("godot_rust_template", &snake_case_name);
    fs::write(cargo_toml_path, new_content).map_err(|e| e.to_string())?;

    // Refactor project.godot
    let project_godot_path = format!("{}/godot/project.godot", snake_case_name);
    let content = fs::read_to_string(&project_godot_path).map_err(|e| e.to_string())?;
    let new_content = content.replace("godot_rust_template", &project_name);
    fs::write(project_godot_path, new_content).map_err(|e| e.to_string())?;

    Ok(())
}

fn generate_gdextension_file(snake_case_name: String) -> Result<(), String> {
    let gdextension_path = format!("{}/godot/gdrust.gdextension", snake_case_name);
    let content = format!(
        r#"[configuration]
entry_symbol = "gdext_rust_init"
compatibility_minimum = 4.1
reloadable = true

[libraries]
linux.debug.x86_64 =     "res://../target/debug/lib{project_name}.so"
linux.release.x86_64 =   "res://../target/release/lib{project_name}.so"
windows.debug.x86_64 =   "res://../target/debug/{project_name}.dll"
windows.release.x86_64 = "res://../target/release/{project_name}.dll"
macos.debug =            "res://../target/debug/lib{project_name}.dylib"
macos.release =          "res://../target/release/lib{project_name}.dylib"
macos.debug.arm64 =      "res://../target/debug/lib{project_name}.dylib"
macos.release.arm64 =    "res://../target/release/lib{project_name}.dylib"
        "#,
        project_name = snake_case_name
    );

    fs::write(gdextension_path, content).map_err(|e| e.to_string())?;

    Ok(())
}

fn git_init(project_name: String) {
    let output = Command::new("git")
        .arg("init")
        .arg(&project_name)
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        eprintln!("Error initializing git: {}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }
}

pub fn execute(project_name: String) {
    let snake_case_name = snake_case(project_name.clone());

    clone_template(snake_case_name.clone());
    remove_git_directory(snake_case_name.clone());

    if let Err(e) = refactor_project_name(project_name.clone(), snake_case_name.clone()) {
        eprintln!("Error refactoring project name: {}", e);
        rollback(snake_case_name.clone());
        std::process::exit(1);
    }

    if let Err(e) = generate_gdextension_file(snake_case_name.clone()) {
        eprintln!("Error generating GDExtension file: {}", e);
        rollback(snake_case_name.clone());
        std::process::exit(1);
    }

    git_init(snake_case_name.clone());

    println!("Project {} created successfully!", project_name);
    println!("You can now run the project with:");
    println!("1. cd {}", snake_case_name);
    println!("2. cargo build");
    println!("3. Open the /godot directory using Godot 4.1 or later");
    println!("4. Run the project");
    println!("5. Enjoy!");
}
// git-sg: A tool to open the Sourcegraph page for any local Git repository
// Enhanced URL parsing ensures proper validation and normalization of user input

use std::fs;
use std::process::Command;
use open;
use std::env;
use url::Url;
use git_sg::{get_config_path, parse_config, prompt_for_config, normalize_url};

fn main() {
    // Check for command-line arguments
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        match args[1].as_str() {
            "init" => {
                initialize_config();
                return;
            }
            "config" => {
                if args.len() > 2 {
                    match args[2].as_str() {
                        "--view" => {
                            view_config();
                            return;
                        }
                        "--edit" => {
                            edit_config();
                            return;
                        }
                        _ => {
                            println!("Unknown config subcommand: {}", args[2]);
                            println!("Usage: git-sg config [--view|--edit]");
                            return;
                        }
                    }
                } else {
                    println!("Missing config subcommand");
                    println!("Usage: git-sg config [--view|--edit]");
                    return;
                }
            }
            _ => {
                println!("Unknown command: {}", args[1]);
                println!("Usage: git-sg [init|config]");
                return;
            }
        }
    }

    // Determine the configuration file path based on the OS
    let config_path = get_config_path();

    // Check if the configuration file exists
    let (base_url, git_provider_url) = if config_path.exists() {
        // Read the configuration from the configuration file
        let config_content = fs::read_to_string(&config_path).expect("Failed to read configuration file");
        parse_config(&config_content)
    } else {
        // Prompt for initial configuration
        prompt_for_config(&config_path)
    };

    // Get the origin URL of the Git repository
    let origin_url = get_git_origin_url();

    // Parse the origin URL to extract the repository path
    let repo_path = parse_git_url(&origin_url);

    // Combine the base URL, git provider URL, and the repository path
    let mut sourcegraph_url = format!("{}/{}/{}", base_url.trim_end_matches('/'), git_provider_url.trim_end_matches('/'), repo_path);
    
    // Remove .git suffix if present
    if sourcegraph_url.ends_with(".git") {
        sourcegraph_url = sourcegraph_url.trim_end_matches(".git").to_string();
    }

    // Open the Sourcegraph URL in the default browser
    open::that(sourcegraph_url).expect("Failed to open URL in browser");
}

fn get_git_origin_url() -> String {
    let output = Command::new("git")
        .args(["config", "--get", "remote.origin.url"])
        .output()
        .expect("Failed to execute git command");

    if !output.status.success() {
        panic!("Failed to get Git origin URL");
    }

    String::from_utf8(output.stdout).expect("Git output is not valid UTF-8").trim().to_string()
}

fn parse_git_url(git_url: &str) -> String {
    // Handle different Git URL formats ssh or https
    if git_url.starts_with("git@") {
        // Handle SSH URL
        let parts: Vec<&str> = git_url.split(':').collect();
        if parts.len() == 2 {
            return parts[1].trim_start_matches('/').to_string();
        } else {
            panic!("Invalid SSH Git URL format");
        }
    } else if git_url.starts_with("http://") || git_url.starts_with("https://") {
        // Handle HTTPS URL
        let url = Url::parse(git_url).expect("Failed to parse Git URL");
        return url.path().trim_start_matches('/').to_string();
    } else {
        panic!("Unsupported Git URL format");
    }
}

fn view_config() {
    let config_path = get_config_path();
    if !config_path.exists() {
        println!("Configuration file does not exist. Run 'git-sg init' to create one.");
        return;
    }

    let config_content = fs::read_to_string(&config_path).expect("Failed to read configuration file");
    println!("Current configuration:");
    println!("--------------------");
    println!("{}", config_content.trim());
    println!("--------------------");
}

fn edit_config() {
    let config_path = get_config_path();
    if !config_path.exists() {
        println!("Configuration file does not exist. Run 'git-sg init' to create one.");
        return;
    }

    // Determine which editor to use
    let editor = if cfg!(target_os = "windows") {
        // On Windows, try to use Notepad if no editor is set
        env::var("EDITOR")
            .or_else(|_| env::var("VISUAL"))
            .unwrap_or_else(|_| String::from("notepad.exe"))
    } else {
        // On Unix-like systems, default to vim
        env::var("EDITOR")
            .or_else(|_| env::var("VISUAL"))
            .unwrap_or_else(|_| String::from("vim"))
    };

    // Open the config file in the editor
    let mut command = Command::new(&editor);
    command.arg(&config_path);

    // For Windows Notepad, ensure the path is in Windows format
    if cfg!(target_os = "windows") && editor == "notepad.exe" {
        if let Some(path_str) = config_path.to_str() {
            command = Command::new("cmd");
            command.args(["/C", "notepad", path_str]);
        }
    }

    let status = command.status().unwrap_or_else(|e| {
        eprintln!("Failed to open editor: {}. Try setting the EDITOR environment variable.", e);
        std::process::exit(1);
    });

    if !status.success() {
        eprintln!("Failed to edit configuration file");
        return;
    }

    // Validate the config after editing
    match fs::read_to_string(&config_path) {
        Ok(content) => {
            if let Err(err) = validate_config(&content) {
                eprintln!("Warning: The configuration file may be invalid: {}", err);
            }
        }
        Err(err) => {
            eprintln!("Failed to read configuration file after editing: {}", err);
        }
    }
}

fn validate_config(content: &str) -> Result<(), String> {
    let lines: Vec<&str> = content.lines().collect();
    if lines.len() < 2 {
        return Err("Configuration must contain both base URL and git provider URL".to_string());
    }

    // Validate base URL
    if let Err(err) = normalize_url(lines[0].trim(), true) {
        return Err(format!("Invalid base URL: {}", err));
    }

    // Validate git provider URL
    if let Err(err) = normalize_url(lines[1].trim(), false) {
        return Err(format!("Invalid git provider URL: {}", err));
    }

    Ok(())
}

fn initialize_config() {
    let config_path = get_config_path();
    
    // Create parent directories if they don't exist
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create configuration directory");
    }
    
    // Reprompt for configuration
    let (base_url, git_provider_url) = prompt_for_config(&config_path);
    println!("Configuration updated successfully!");
    println!("Base URL: {}", base_url);
    println!("Git Provider URL: {}", git_provider_url);
}
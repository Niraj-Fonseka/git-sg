// git-sg: A tool to open the Sourcegraph page for any local Git repository
// Enhanced URL parsing ensures proper validation and normalization of user input

use std::fs;
use std::process::Command;
use open;
use std::env;
use url::Url;
use git_sg::{get_config_path, parse_config, prompt_for_config};

fn main() {
    // Check for command-line arguments
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        match args[1].as_str() {
            "init" => {
                initialize_config();
                return;
            }
            _ => {
                println!("Unknown command: {}", args[1]);
                println!("Usage: git-sg [init]");
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
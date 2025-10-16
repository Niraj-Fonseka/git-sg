use std::fs;
use std::io::{self};
use std::path::PathBuf;
use url::Url;
use std::process::Command;
use open;
use dirs;

fn main() {
    // Determine the configuration file path based on the OS
    let config_path = get_config_path();

    // Check if the configuration file exists
    let base_url = if config_path.exists() {
        // Read the base URL from the configuration file
        fs::read_to_string(&config_path).expect("Failed to read configuration file")
    } else {
        // Prompt the user for the base URL
        println!("Enter the base URL for your Sourcegraph instance:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let base_url = input.trim().to_string();

        // Save the base URL to the configuration file
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create configuration directory");
        }
        fs::write(&config_path, &base_url).expect("Failed to write configuration file");

        base_url
    };

    // Get the origin URL of the Git repository
    let origin_url = get_git_origin_url();

    // Parse the origin URL to extract the repository path
    let repo_path = parse_git_url(&origin_url);

    // Combine the base URL with the repository path
    let sourcegraph_url = format!("{}/{}", base_url.trim_end_matches('/'), repo_path);

    // Open the Sourcegraph URL in the default browser
    open::that(sourcegraph_url).expect("Failed to open URL in browser");
}

fn get_config_path() -> PathBuf {
    let app_name = "gitsg";
    if cfg!(target_os = "macos") {
        dirs::home_dir()
            .expect("Failed to get home directory")
            .join("Library")
            .join("Application Support")
            .join(app_name)
            .join("config")
    } else if cfg!(target_os = "windows") {
        dirs::config_dir()
            .expect("Failed to get config directory")
            .join(app_name)
            .join("config")
    } else {
        dirs::config_dir()
            .expect("Failed to get config directory")
            .join(app_name)
            .join("config")
    }
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
    println!("Parsing Git URL: {}", git_url);

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

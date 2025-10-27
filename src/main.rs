use std::fs;
use std::io;
use std::path::PathBuf;
use url::Url;
use std::process::Command;
use open;
use dirs;
use std::env;

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

fn parse_config(config_content: &str) -> (String, String) {
    let lines: Vec<&str> = config_content.lines().collect();
    if lines.len() >= 2 {
        let base_url = lines[0].trim().to_string();
        let git_provider_url = lines[1].trim().to_string();
        (base_url, git_provider_url)
    } else {
        panic!("Invalid configuration file format. Expected base URL and git provider URL.");
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

fn prompt_for_config(config_path: &PathBuf) -> (String, String) {
    // Prompt the user for the base URL
    println!("Enter the base URL for your Sourcegraph instance:");
    let mut base_input = String::new();
    io::stdin().read_line(&mut base_input).expect("Failed to read input");
    let base_url = base_input.trim().to_string();

    // Prompt the user for the git provider URL
    println!("Enter the git provider URL (e.g., gitlab.com):");
    let mut provider_input = String::new();
    io::stdin().read_line(&mut provider_input).expect("Failed to read input");
    let mut git_provider_url = provider_input.trim().to_string();
    
    // ------------ Git provider URL cleaning ------------
    // Remove https:// prefix if present from the git provider URL
    if git_provider_url.starts_with("https://") {
        git_provider_url = git_provider_url.trim_start_matches("https://").to_string();
    }
    // Remove http:// prefix if present from the git provider URL
    if git_provider_url.starts_with("http://") {
        git_provider_url = git_provider_url.trim_start_matches("http://").to_string();
    }
    // Remove trailing slashes
    git_provider_url = git_provider_url.trim_end_matches('/').to_string();


    // Save the configuration to the configuration file
    let config_content = format!("{}\n{}", base_url, git_provider_url);
    fs::write(&config_path, &config_content).expect("Failed to write configuration file");

    (base_url, git_provider_url)
}

use std::fs;
use std::io;
use std::path::PathBuf;
use url::Url;
use dirs;

/// Normalize and validate a URL, ensuring it has proper format
pub fn normalize_url(input: &str, is_base_url: bool) -> Result<String, String> {
    let trimmed = input.trim();
    
    if trimmed.is_empty() {
        return Err("URL cannot be empty".to_string());
    }
    
    if is_base_url {
        // For base URLs, we want to ensure they have a scheme and are properly formatted
        let normalized = if !trimmed.starts_with("http://") && !trimmed.starts_with("https://") {
            // Default to https if no scheme is provided
            format!("https://{}", trimmed)
        } else {
            trimmed.to_string()
        };
        
        // Validate the URL by parsing it
        match Url::parse(&normalized) {
            Ok(url) => {
                // Additional validation: ensure the host part looks reasonable
                if let Some(host) = url.host_str() {
                    if !host.contains('.') || host.is_empty() {
                        return Err(format!("Invalid base URL format: '{}'. Please enter a valid URL (e.g., https://company.sourcegraph.com)", trimmed));
                    }
                } else {
                    return Err(format!("Invalid base URL format: '{}'. Please enter a valid URL (e.g., https://company.sourcegraph.com)", trimmed));
                }
                
                // Return the URL without trailing slash for consistency
                let mut result = url.to_string();
                if result.ends_with('/') {
                    result.pop();
                }
                Ok(result)
            }
            Err(_) => {
                Err(format!("Invalid base URL format: '{}'. Please enter a valid URL (e.g., https://company.sourcegraph.com)", trimmed))
            }
        }
    } else {
        // For git provider URLs, we want just the hostname without scheme or trailing slashes
        let mut normalized = trimmed.to_string();
        
        // Remove protocol if present
        if normalized.starts_with("https://") {
            normalized = normalized.trim_start_matches("https://").to_string();
        }
        if normalized.starts_with("http://") {
            normalized = normalized.trim_start_matches("http://").to_string();
        }
        
        // Remove trailing slashes
        normalized = normalized.trim_end_matches('/').to_string();
        
        // Remove any path components (keep only hostname)
        if let Some(slash_pos) = normalized.find('/') {
            normalized = normalized[..slash_pos].to_string();
        }
        
        // Basic validation - ensure it looks like a hostname
        if normalized.is_empty() || !normalized.contains('.') {
            return Err(format!("Invalid git provider URL format: '{}'. Please enter a valid hostname (e.g., gitlab.com, github.com)", trimmed));
        }
        
        Ok(normalized)
    }
}

pub fn get_config_path() -> PathBuf {
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

pub fn parse_config(config_content: &str) -> (String, String) {
    let lines: Vec<&str> = config_content.lines().collect();
    if lines.len() >= 2 {
        let base_url = match normalize_url(lines[0].trim(), true) {
            Ok(url) => url,
            Err(err) => {
                eprintln!("Error parsing base URL from config: {}", err);
                std::process::exit(1);
            }
        };
        let git_provider_url = match normalize_url(lines[1].trim(), false) {
            Ok(url) => url,
            Err(err) => {
                eprintln!("Error parsing git provider URL from config: {}", err);
                std::process::exit(1);
            }
        };
        (base_url, git_provider_url)
    } else {
        eprintln!("Invalid configuration file format. Expected base URL and git provider URL.");
        std::process::exit(1);
    }
}

pub fn prompt_for_config(config_path: &PathBuf) -> (String, String) {
    // Prompt the user for the base URL
    println!("Enter the base URL for your Sourcegraph instance (e.g., https://company.sourcegraph.com):");
    let mut base_input = String::new();
    io::stdin().read_line(&mut base_input).expect("Failed to read input");
    let base_url = loop {
        match normalize_url(base_input.trim(), true) {
            Ok(url) => break url,
            Err(err) => {
                eprintln!("Error: {}", err);
                println!("Please enter the base URL again:");
                base_input.clear();
                io::stdin().read_line(&mut base_input).expect("Failed to read input");
            }
        }
    };

    // Prompt the user for the git provider URL
    println!("Enter the git provider URL (e.g., gitlab.com):");
    let mut provider_input = String::new();
    io::stdin().read_line(&mut provider_input).expect("Failed to read input");
    let git_provider_url = loop {
        match normalize_url(provider_input.trim(), false) {
            Ok(url) => break url,
            Err(err) => {
                eprintln!("Error: {}", err);
                println!("Please enter the git provider URL again:");
                provider_input.clear();
                io::stdin().read_line(&mut provider_input).expect("Failed to read input");
            }
        }
    };

    // Save the configuration to the configuration file
    let config_content = format!("{}\n{}", base_url, git_provider_url);
    fs::write(&config_path, &config_content).expect("Failed to write configuration file");

    (base_url, git_provider_url)
}
use git_sg::normalize_url;

#[test]
fn test_normalize_base_url() {
    // Test base URL normalization
    assert_eq!(normalize_url("https://company.sourcegraph.com", true).unwrap(), "https://company.sourcegraph.com");
    assert_eq!(normalize_url("https://company.sourcegraph.com/", true).unwrap(), "https://company.sourcegraph.com");
    assert_eq!(normalize_url("company.sourcegraph.com", true).unwrap(), "https://company.sourcegraph.com");
    assert_eq!(normalize_url("http://company.sourcegraph.com", true).unwrap(), "http://company.sourcegraph.com");
    assert_eq!(normalize_url("  https://company.sourcegraph.com  ", true).unwrap(), "https://company.sourcegraph.com");
    
    // Test invalid cases
    assert!(normalize_url("", true).is_err());
    assert!(normalize_url("not-a-valid-url", true).is_err());
    assert!(normalize_url("ftp://invalid.scheme.com", true).is_err());
}

#[test]
fn test_normalize_git_provider_url() {
    // Test git provider URL normalization
    assert_eq!(normalize_url("gitlab.com", false).unwrap(), "gitlab.com");
    assert_eq!(normalize_url("https://gitlab.com", false).unwrap(), "gitlab.com");
    assert_eq!(normalize_url("https://gitlab.com/", false).unwrap(), "gitlab.com");
    assert_eq!(normalize_url("https://gitlab.com/some/path", false).unwrap(), "gitlab.com");
    assert_eq!(normalize_url("http://github.com", false).unwrap(), "github.com");
    assert_eq!(normalize_url("  github.com  ", false).unwrap(), "github.com");
    assert_eq!(normalize_url("github.enterprise.com", false).unwrap(), "github.enterprise.com");
    
    // Test invalid cases
    assert!(normalize_url("", false).is_err());
    assert!(normalize_url("invalid-url", false).is_err());
    assert!(normalize_url("localhost", false).is_err()); // No dot in hostname
}

#[test]
fn test_normalize_url_edge_cases() {
    // Test additional edge cases for base URLs
    assert_eq!(normalize_url("sourcegraph.example.com:3080", true).unwrap(), "https://sourcegraph.example.com:3080");
    assert_eq!(normalize_url("https://sourcegraph.example.com:3080/", true).unwrap(), "https://sourcegraph.example.com:3080");
    
    // Test additional edge cases for git provider URLs
    assert_eq!(normalize_url("git.company.com", false).unwrap(), "git.company.com");
    assert_eq!(normalize_url("https://git.company.com:8080/gitlab", false).unwrap(), "git.company.com:8080");
}

#[test]
fn test_normalize_url_with_subdomains() {
    // Test base URLs with various subdomains
    assert_eq!(normalize_url("my-company.sourcegraph.com", true).unwrap(), "https://my-company.sourcegraph.com");
    assert_eq!(normalize_url("dev.internal.sourcegraph.example.org", true).unwrap(), "https://dev.internal.sourcegraph.example.org");
    
    // Test git provider URLs with various subdomains
    assert_eq!(normalize_url("git.internal.company.com", false).unwrap(), "git.internal.company.com");
    assert_eq!(normalize_url("https://source.dev.company.org/some/path/", false).unwrap(), "source.dev.company.org");
}
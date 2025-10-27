## GIT SG

`git-sg` is a tool to open the Sourcegraph page for any local Git repository.

### Features
- Prompts for the Sourcegraph base URL and git provider URL on first run and saves them to a configuration file.
- Supports both HTTPS and SSH Git URLs.
- Automatically constructs the Sourcegraph URL using the base URL, git provider URL, and the repository path.
- Opens the Sourcegraph URL in the default browser.
- Automatically strips `https://`, `http://`, and trailing slashes from the git provider URL.

### Installation
- If you have Rust/Cargo installed, you can install `git-sg` using Cargo:

   ```bash
   cargo install git-sg
   ```

### Build from source

1. Clone this repository.
2. Build the binary using the following command:

   ```bash
   make build
   ```

3. Add the binary to your PATH for easy access.

### Usage

1. Navigate to any Git repository.
2. Run the following command:

   ```bash
   git-sg
   ```

3. On the first run, you will be prompted to enter:
   - The base URL for your Sourcegraph instance (e.g., `https://heb.sourcegraph.com`)
   - The git provider URL (e.g., `gitlab.com`)

   These will be saved for future use.

4. To reconfigure or update your settings anytime, run:

   ```bash
   git-sg init
   ```

### Configuration File

The configuration file is stored in the appropriate location based on your operating system:

- macOS: `~/Library/Application Support/gitsg/config`
- Linux: `~/.config/gitsg/config`
- Windows: `%APPDATA%\gitsg\config`

The configuration file contains two lines:

1. The base URL for your Sourcegraph instance
2. The git provider URL

If you need to change the configuration, you can either:

- Edit the file directly
- Run `git-sg init` to be prompted for new values

### Requirements

- Rust installed on your system.
- Git installed and accessible from the command line.

### License

This project is licensed under the MIT License.

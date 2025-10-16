## GITREMOTE

`gitsg` is a tool to open the Sourcegraph page for any local Git repository.

### Features
- Prompts for the Sourcegraph base URL on first run and saves it to a configuration file.
- Supports both HTTPS and SSH Git URLs.
- Automatically constructs the Sourcegraph URL using the base URL and the repository path.
- Opens the Sourcegraph URL in the default browser.

### Installation
1. Clone this repository.
2. Build the binary using the following command:
   ```
   make build
   ```
3. Add the binary to your PATH for easy access.

### Usage
1. Navigate to any Git repository.
2. Run the following command:
   ```
   git-sg
   ```
3. On the first run, you will be prompted to enter the base URL for your Sourcegraph instance. This will be saved for future use.

### Configuration File
The configuration file is stored in the appropriate location based on your operating system: 
- macOS: `~/Library/Application Support/gitsg/config`
- Linux: `~/.config/gitsg/config`
- Windows: `%APPDATA%\gitsg\config`

The configuration file contains the base URL for your Sourcegraph instance. If you need to change it, simply edit the file.

### Requirements
- Rust installed on your system.
- Git installed and accessible from the command line.

### License
This project is licensed under the MIT License.

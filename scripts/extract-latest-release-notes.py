#!/usr/bin/env python3
"""
Extract the latest release notes from CHANGELOG.md for GitHub releases.
This script extracts only the most recent version's release notes instead of the entire changelog.
"""

import re
import sys
from pathlib import Path


def extract_latest_release_notes(changelog_path: Path, output_path: Path):
    """Extract the latest release notes from a changelog file."""
    
    if not changelog_path.exists():
        print(f"Error: Changelog file {changelog_path} not found")
        sys.exit(1)
    
    content = changelog_path.read_text(encoding='utf-8')
    
    # Split content into lines
    lines = content.split('\n')
    
    # Find the first version header (should be the latest)
    latest_section = []
    found_first_version = False
    
    for line in lines:
        # Check if this line is a version header (starts with ## v)
        if re.match(r'^## v\d+\.\d+\.\d+', line):
            if found_first_version:
                # We've reached the next version, stop collecting
                break
            else:
                # This is the first version header we found (latest version)
                found_first_version = True
                latest_section.append(line)
        elif found_first_version:
            # We're inside the latest version section
            latest_section.append(line)
    
    if not found_first_version:
        print("Error: No version sections found in changelog")
        sys.exit(1)
    
    # Remove trailing empty lines
    while latest_section and not latest_section[-1].strip():
        latest_section.pop()
    
    # Write the extracted release notes
    output_content = '\n'.join(latest_section)
    output_path.write_text(output_content, encoding='utf-8')
    
    print(f"Extracted latest release notes to {output_path}")
    print("Release notes content:")
    print("-" * 40)
    print(output_content)
    print("-" * 40)


if __name__ == "__main__":
    changelog_file = Path("CHANGELOG.md")
    output_file = Path("LATEST_RELEASE_NOTES.md")
    
    extract_latest_release_notes(changelog_file, output_file)
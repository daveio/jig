#!/usr/bin/env python3
"""
Screenshot Renamer using Claude API
Analyzes screenshot content and renames files with descriptive names
"""

import os
import base64
import re
import sys
from pathlib import Path
from datetime import datetime
import anthropic
from typing import Optional

# Configuration
SCREENSHOTS_DIR = "/Users/dave/Library/CloudStorage/Dropbox/Screenshots"
SUPPORTED_FORMATS = {'.png', '.jpg', '.jpeg', '.gif', '.webp', '.bmp'}
MAX_FILENAME_LENGTH = 200
DRY_RUN = False  # Set to True to preview changes without renaming

def sanitize_filename(filename: str) -> str:
    """Remove invalid characters and limit filename length"""
    # Remove or replace invalid characters
    filename = re.sub(r'[<>:"/\\|?*]', '-', filename)
    filename = re.sub(r'\s+', '_', filename)  # Replace spaces with underscores
    filename = re.sub(r'[^\w\-_.]', '', filename)  # Keep only alphanumeric, dash, underscore, dot
    filename = re.sub(r'_+', '_', filename)  # Remove duplicate underscores
    filename = filename.strip('_.-')  # Remove leading/trailing special chars

    # Limit length
    if len(filename) > MAX_FILENAME_LENGTH:
        filename = filename[:MAX_FILENAME_LENGTH]

    return filename

def encode_image(image_path: Path) -> str:
    """Encode image to base64 for API"""
    with open(image_path, "rb") as image_file:
        return base64.b64encode(image_file.read()).decode('utf-8')

def get_image_description(client: anthropic.Anthropic, image_path: Path) -> Optional[str]:
    """Get description of image content from Claude"""
    try:
        # Get file extension and mime type
        ext = image_path.suffix.lower()
        mime_types = {
            '.png': 'image/png',
            '.jpg': 'image/jpeg',
            '.jpeg': 'image/jpeg',
            '.gif': 'image/gif',
            '.webp': 'image/webp',
            '.bmp': 'image/bmp'
        }
        media_type = mime_types.get(ext, 'image/jpeg')

        # Encode image
        base64_image = encode_image(image_path)

        # Call Claude API
        message = client.messages.create(
            model="claude-3-5-sonnet-20241022",
            max_tokens=150,
            messages=[{
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Please provide a brief, descriptive filename for this screenshot. Focus on the main content, application, or purpose visible. Keep it concise (max 5-7 words). Don't include 'screenshot' in the name. Reply with just the filename suggestion, no extension."
                    },
                    {
                        "type": "image",
                        "source": {
                            "type": "base64",
                            "media_type": media_type,
                            "data": base64_image
                        }
                    }
                ]
            }]
        )

        return message.content[0].text.strip()

    except Exception as e:
        print(f"Error analyzing {image_path.name}: {str(e)}")
        return None

def generate_unique_filename(directory: Path, base_name: str, extension: str) -> Path:
    """Generate unique filename if file already exists"""
    counter = 1
    new_path = directory / f"{base_name}{extension}"

    while new_path.exists():
        new_path = directory / f"{base_name}_{counter}{extension}"
        counter += 1

    return new_path

def process_screenshots():
    """Main function to process all screenshots"""
    # Check API key
    api_key = os.environ.get('ANTHROPIC_API_KEY')
    if not api_key:
        print("Error: ANTHROPIC_API_KEY environment variable not set")
        sys.exit(1)

    # Initialize Claude client
    client = anthropic.Anthropic(api_key=api_key)

    # Check directory
    screenshots_path = Path(SCREENSHOTS_DIR)
    if not screenshots_path.exists():
        print(f"Error: Directory {SCREENSHOTS_DIR} does not exist")
        sys.exit(1)

    # Get all image files
    image_files = []
    for ext in SUPPORTED_FORMATS:
        image_files.extend(screenshots_path.glob(f"*{ext}"))
        image_files.extend(screenshots_path.glob(f"*{ext.upper()}"))

    if not image_files:
        print("No image files found in the screenshots directory")
        return

    print(f"Found {len(image_files)} image files to process")
    if DRY_RUN:
        print("DRY RUN MODE - No files will be renamed\n")

    # Process each file
    renamed_count = 0
    skipped_count = 0
    error_count = 0

    for i, image_path in enumerate(image_files, 1):
        print(f"\nProcessing {i}/{len(image_files)}: {image_path.name}")

        # Skip if filename already looks descriptive (not a default screenshot name)
        if not re.match(r'^(Screenshot|LWScreenShot|Screen Shot|Capture|Image|CleanShot).*', image_path.name, re.IGNORECASE):
            print("  → Skipping: Filename already appears descriptive")
            skipped_count += 1
            continue

        # Get description from Claude
        description = get_image_description(client, image_path)
        if not description:
            print("  → Error: Could not get description")
            error_count += 1
            continue

        # Create new filename
        sanitized_name = sanitize_filename(description)
        if not sanitized_name:
            print("  → Error: Could not create valid filename from description")
            error_count += 1
            continue

        # Add timestamp to maintain uniqueness and chronology
        timestamp = datetime.fromtimestamp(image_path.stat().st_mtime).strftime('%Y%m%d_%H%M%S')
        new_base_name = f"{sanitized_name}_{timestamp}"

        # Generate unique path
        new_path = generate_unique_filename(screenshots_path, new_base_name, image_path.suffix)

        print(f"  → Description: {description}")
        print(f"  → New name: {new_path.name}")

        # Rename file
        if not DRY_RUN:
            try:
                image_path.rename(new_path)
                print("  → Renamed successfully")
                renamed_count += 1
            except Exception as e:
                print(f"  → Error renaming: {str(e)}")
                error_count += 1
        else:
            renamed_count += 1

    # Summary
    print(f"\n{'='*50}")
    print(f"Summary:")
    print(f"  Total files: {len(image_files)}")
    print(f"  Renamed: {renamed_count}")
    print(f"  Skipped: {skipped_count}")
    print(f"  Errors: {error_count}")
    if DRY_RUN:
        print("\nDRY RUN COMPLETE - No files were actually renamed")
        print("Set DRY_RUN = False to perform actual renaming")

if __name__ == "__main__":
    process_screenshots()

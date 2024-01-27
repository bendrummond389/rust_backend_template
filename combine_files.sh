#!/bin/bash

# Combined output file
OUTPUT_FILE="combined.txt"

# Directories to ignore
IGNORE_DIRS=("target") # Add directory names to ignore

# Function to check if a directory is in the ignore list
should_ignore_dir() {
    for dir in "${IGNORE_DIRS[@]}"; do
        if [[ $1 == *"$dir"* ]]; then
            return 0 # True, should ignore
        fi
    done
    return 1 # False, should not ignore
}

# Initialize or clear the output file
> "$OUTPUT_FILE"

# Iterate over all .rs files in the project
# Adjust the find command to search for different file types if needed
find . -type f -name '*.rs' | while read -r file; do
    if should_ignore_dir "$file"; then
        continue
    fi
    echo "File: $file" >> "$OUTPUT_FILE"
    cat "$file" >> "$OUTPUT_FILE"
    echo "" >> "$OUTPUT_FILE"
done

echo "Combined file created: $OUTPUT_FILE"

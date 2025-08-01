#!/bin/bash

# Directory to search (you can also pass this as an argument)
BASE_DIR="."

# Output file
OUTPUT_FILE="output.txt"

# Clear the output file first
> "$OUTPUT_FILE"

# Loop through each subdirectory
for dir in "$BASE_DIR"/*/; do
    if [ -d "$dir" ]; then
        echo "Processing: $dir" >> "$OUTPUT_FILE"
        
        # Run your command here. Replace `ls` with your actual command.
        # Example: list files in the subdirectory
        cd "$dir"
        # > "$OUTPUT_FILE"
        timeout 120 cargo run -- formal-verify >> "../$OUTPUT_FILE" 
        # 2>&1
        # ls "$dir" >> "$OUTPUT_FILE" 2>&1

        # echo "-----------------------------------" >> "$OUTPUT_FILE"
        cd ..
    fi
done

echo "Done. Output saved to $OUTPUT_FILE"

#!/bin/bash

# Directory to search
BASE_DIR="."

# Output file
OUTPUT_FILE="time_int.txt"

# Clear the output file
# > "$OUTPUT_FILE"

# for dir in "$BASE_DIR"/*/*/; do
#     echo "--------"
#     echo $dir
#     if [ -d "$dir" ]; then
#         echo "---------------"
#         dir_name=$(basename "$dir")
#         first_char=$(echo "$dir_name" | cut -c1 | tr '[:upper:]' '[:lower:]')
#         echo $dir_name

#         parent_dir=$(dirname "$dir")     # → BASE_DIR/a/b
#         dir_name=$(basename "$parent_dir")  # → b
#         echo $parent_dir
#         echo $dir_name
#         echo "---------------"
#     fi
# done

# Loop through each subdirectory
for dir in "$BASE_DIR"/*/*/; do
    if [ -d "$dir" ]; then
        dir_name=$(basename "$dir")
        first_char=$(echo "$dir_name" | cut -c1 | tr '[:upper:]' '[:lower:]')
        # echo $first_char
        # parent_dir=$(dirname "$dir")     # → BASE_DIR/a/b
        # dir_name=$(basename "$parent_dir")  # → b
        # Check the first character
        if [[ "$first_char" =~ [a-z] ]]; then
        # if [[ " ${dir_names[*]} " == *" $dir_name "* ]]; then
        # for item in "${dir_names[@]}"; do
        #     if [[ "$item" == "$dir_name" ]]; then
                echo "Processing: $dir_name" >> "$OUTPUT_FILE"
                # echo "Processing: $dir_name" >> "$OUTPUT_FILE"
                cd "$dir"
                /usr/bin/time -f "time=%e" \
                -a -o "../../$OUTPUT_FILE" \
                timeout 120 cargo run -- formal-verify >> "../../$OUTPUT_FILE"
                # 2>&1
                # timeout 120 cargo run -- formal-verify >> "../$OUTPUT_FILE"
                # /usr/bin/time -a -o "../../$OUTPUT_FILE" \
                # echo "-----------------------------------" >> "$OUTPUT_FILE"
                cd ../..
        fi
        # done
    fi
done

echo "Done. Output saved to $OUTPUT_FILE"

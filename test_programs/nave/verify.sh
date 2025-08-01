#!/bin/bash

# Default values
output_file=""
search_dir="./*/*/*"

# Parse optional arguments
while getopts "o:d:" opt; do
  case $opt in
    o) output_file="$OPTARG" ;;
    d) backend="$OPTARG" ;;
    *)
      echo "Usage: $0 [-o output_file] [-d input_directory]"
      exit 1
      ;;
  esac
done

# Run on every directory inside search_dir
for dir in $search_dir; do
  if [[ -d "$dir" ]]; then
    echo "Processing directory: $dir"

    if [[ -n "$output_file" ]]; then
      # Command output goes to file
      {
        echo "===== $dir ====="
        cd "$dir"/*
        time cargo run -- formal-verify 
        cd "../../../.."
      } >> "$output_file"
    else
      # Command output goes to console
        echo "===== $dir ====="
        cd "$dir"/*
        time cargo run -- formal-verify
        cd "../../../.."
    fi
  fi
done

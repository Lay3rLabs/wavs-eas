#!/bin/bash

# Extract schema UID from forge script output
# Usage: bash script/extract-schema-uid.sh [input_file]
#        echo "forge output" | bash script/extract-schema-uid.sh

set -e

# Function to extract schema UID from input
extract_uid() {
    # Look for the SCHEMA_REGISTRATION_RESULT line and extract the schema_uid
    grep -A 1 "SCHEMA_REGISTRATION_RESULT:" | \
    grep -o '"schema_uid":"[^"]*"' | \
    cut -d'"' -f4
}

# Check if input file provided or use stdin
if [ $# -eq 1 ]; then
    # Read from file
    cat "$1" | extract_uid
else
    # Read from stdin
    extract_uid
fi

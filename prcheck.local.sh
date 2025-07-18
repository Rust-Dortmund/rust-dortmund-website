#!/bin/bash

# Ensure we're in the right folder
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd $SCRIPT_DIR

# Initialize error flag
ERROR_FOUND=0
ERROR_REPORT=""

# Function to run cargo command and capture errors
run_cargo() {
    local cmd_name="$1"
    shift
    local cmd_output
    local cmd_status

    echo "Running $cmd_name..."
    # Run command and capture output and status
    cmd_output=$(RUSTFLAGS="-D warnings" "$@" 2>&1)
    cmd_status=$?

    if [ $cmd_status -ne 0 ]; then
        ERROR_FOUND=1
        ERROR_REPORT+="$cmd_name failed with status $cmd_status:\n"
        ERROR_REPORT+="$cmd_output\n"
        ERROR_REPORT+="----------------------------------------\n"
    else
        echo "$cmd_name completed successfully"
    fi
}

# Run cargo commands
run_cargo "cargo fmt" cargo fmt --check
run_cargo "cargo test" cargo test --workspace
run_cargo "cargo clippy" cargo clippy --workspace

# Print error report if any errors occurred
if [ $ERROR_FOUND -ne 0 ]; then
    echo -e "\n=== ERROR REPORT ==="
    echo -e "$ERROR_REPORT"
    echo "One or more cargo commands failed!"
    exit 1
else
    echo -e "\nAll cargo commands completed successfully!"
    exit 0
fi
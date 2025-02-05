###  This script 
###     - generates the Rust target
###     - runs the tests
###  Invoke 'build_and_run.sh <RAMBLE_FILE>'


# Get script directory to get the correct directory to pass to cmake, regardless of where the script is executed from
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )


# If no ramble file specified look in the local directory for a default
RAMBLE_FILE="$(realpath  ${1:-'./ramble.yaml'})"


if [ ! -f $RAMBLE_FILE ]; then
        echo "error: ramble file could not be found"
        exit 1
fi


RAMBLE_GENERATED_DIRECTORY=$SCRIPT_DIR/src/generated
RAMBLE_ROOT=$SCRIPT_DIR/../../                          # TODO:

if [[ -z "${RAMBLE_TEST_DIR}" ]]; then
    echo "assuming test directory location"
    RAMBLE_TEST_DIR=$SCRIPT_DIR
fi 

echo "Using RAMBLE_FILE=$RAMBLE_FILE"
echo "Using RAMBLE_GENERATED_DIRECTORY=$RAMBLE_GENERATED_DIRECTORY"
echo "Using RAMBLE_TEST_DIR=$RAMBLE_TEST_DIR"
echo "Using RAMBLE_ROOT=$RAMBLE_ROOT"
# clean destination folder folder
rm $RAMBLE_GENERATED_DIRECTORY/*.rs 2> /dev/null


# Generate files to src

(cd $RAMBLE_ROOT; cargo run -- generate -o $RAMBLE_GENERATED_DIRECTORY -f $RAMBLE_FILE --rust)



# Run tests

(cd $RAMBLE_TEST_DIR; cargo test)

# Cleanup generated files
# Disabled to make testing easier
# rm $RAMBLE_GENERATED_DIRECTORY/*.rs 2> /dev/null

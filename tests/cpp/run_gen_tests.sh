###  This script 
###     - generates the CPP target
###     - builds the CPP tests
###     - runs the tests
###  Invoke 'build_and_run.sh <RAMBLE_FILE>'

# Get script directory to get the correct directory to pass to cmake, regardless of where the script is executed from
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )


# If no ramble file specified look in the local directory for a default
RAMBLE_FILE="$(realpath  ${1:-'./ramble.yaml'})"

echo "$RAMBLE_FILE"
if [ ! -f $RAMBLE_FILE ]; then
        echo "error: ramble file could not be found"
        exit 1
fi


export RAMBLE_GENERATED_DIRECTORY=`mktemp -d -t tmp.rambleXXXXXX`
# verify tmp was created successfully
if [[ ! "$RAMBLE_GENERATED_DIRECTORY" || ! -d "$RAMBLE_GENERATED_DIRECTORY" ]]; then
    echo "Could not create folder for generated files"
    exit 2
fi


if [[ -z "${RAMBLE_TEST_DIR}" ]]; then
    echo "assuming test directory location"
    RAMBLE_TEST_DIR=$SCRIPT_DIR
fi 

echo "Using RAMBLE_FILE=$RAMBLE_FILE"
echo "Using RAMBLE_GENERATED_DIRECTORY=$RAMBLE_GENERATED_DIRECTORY"
echo "Using RAMBLE_TEST_DIR=$RAMBLE_TEST_DIR"

# ============== Generate C Target ================
cargo run generate -f $RAMBLE_FILE -o $RAMBLE_GENERATED_DIRECTORY --C


# ============== Build the Tests ================
cmake $RAMBLE_TEST_DIR

retVal=$?
if [ $retVal -ne 0 ]; then
    echo "Failed to configure cmake" 
    exit $retVal
fi
export CMAKE_VERBOSE_MAKEFILE=True
cmake --build  $RAMBLE_TEST_DIR

retVal=$?
if [ $retVal -ne 0 ]; then
    echo "Failed to build tests" 
    exit $retVal
fi

# ============== Run Test Binary ================

$RAMBLE_TEST_DIR/bin/tests

retVal=$?
if [ $retVal -ne 0 ]; then
    echo "Tests failed" 
    exit $retVal
fi


# Cleanup temp folder on 
function cleanup {      
  rm -rf $RAMBLE_GENERATED_DIRECTORY
  echo "Deleted temp working directory $RAMBLE_GENERATED_DIRECTORY"
}

# # register the cleanup function to be called on the EXIT signal
trap cleanup EXIT
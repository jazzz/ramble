###  This script builds and then runs the cpp tests
###  Invoke 'run_script.sh <PATH_TO_RAMBLE.HPP>'

# Get script directory to get the correct directory to pass to cmake, regardless of where the script is executed from
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

if [[ ! -z "$1" ]]; then
    # Expand passed in directory incase it was a relative path
    # RAMBLE_GENERATED_DIRECTORY is exported so it is available in CMAKE
    export RAMBLE_GENERATED_DIRECTORY="$(realpath $1)"
fi 

if [[ -z "${RAMBLE_GENERATED_DIRECTORY}" ]]; then
    echo "envvar:RAMBLE_GENERATED_DIRECTORY must be set before tests can be compiled; try passing it in."
    exit 1
fi

if [[ -z "${RAMBLE_TEST_DIR}" ]]; then
    echo "assume SCRIPT"
    RAMBLE_TEST_DIR=$SCRIPT_DIR
fi 

echo "Using RAMBLE_GENERATED_DIRECTORY=$RAMBLE_GENERATED_DIRECTORY"
echo "Using RAMBLE_TEST_DIR=$RAMBLE_TEST_DIR"


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

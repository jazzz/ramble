###  This script builds and then runs the cpp tests
###  Invoke 'run_script.sh <PATH_TO_RAMBLE.HPP>'

# Get script directory to get the correct directory to pass to cmake, regardless of where the script is executed from
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

if [[ ! -z "$1" ]]; then
    # Expand passed in directory incase it was a relative path
    export GENERATED_DIRECTORY="$(realpath $1)"
fi 

if [[ -z "${GENERATED_DIRECTORY}" ]]; then
    echo "envvar:GENERATED_DIRECTORY must be set before tests can be compiled; try passing it in."
    exit 1
fi

echo "Using GENERATED_DIRECTORY=$GENERATED_DIRECTORY"

cmake $SCRIPT_DIR

retVal=$?
if [ $retVal -ne 0 ]; then
    echo "Failed to configure cmake" 
    exit $retVal
fi
export CMAKE_VERBOSE_MAKEFILE=True
cmake --build  $SCRIPT_DIR

retVal=$?
if [ $retVal -ne 0 ]; then
    echo "Failed to build tests" 
    exit $retVal
fi

$SCRIPT_DIR/bin/tests

retVal=$?
if [ $retVal -ne 0 ]; then
    echo "Tests failed" 
    exit $retVal
fi

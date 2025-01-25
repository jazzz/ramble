###  This script builds and then runs the cpp tests
###  Invoke 'run_script.sh <PATH_TO_RAMBLE.HPP>'

if [[ ! -z "$1" ]]; then
    GENERATED_DIRECTORY=$1
fi 

if [[ -z "${GENERATED_DIRECTORY}" ]]; then
    echo "envvar:GENERATED_DIRECTORY must be set before tests can be compiled; try passing it in."
    exit 1
fi

echo "Using GENERATED_DIRECTORY=$GENERATED_DIRECTORY"

cd ./tests/cpp/


cmake .

retVal=$?
if [ $retVal -ne 0 ]; then
    echo "Failed to configure cmake" 
    exit $retVal
fi

cmake --build .

retVal=$?
if [ $retVal -ne 0 ]; then
    echo "Failed to build tests" 
    exit $retVal
fi

./bin/tests

retVal=$?
if [ $retVal -ne 0 ]; then
    echo "Tests failed" 
    exit $retVal
fi

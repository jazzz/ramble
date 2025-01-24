cd ./tests/cpp/
cmake .
cmake --build .
./bin/tests

retVal=$?
exit $retVal
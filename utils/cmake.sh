
cmake -S code -B .build &&
cmake --build .build &&
if [ $# -ne 0 ]; then
  .build/bin/$1
fi

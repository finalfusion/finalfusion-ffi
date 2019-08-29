# finalfusion FFI

## Running tests with valgrind

Tests can be run with Valgrind. First make sure that the `valgrind` binary is
in your path. Then compile `finalfusion-ffi` as usual. For example:

~~~shell
$ mkdir build
$ cd build
$ cmake ..
$ make
~~~

Then the CMake `ctest` command can be used to run the tests with Valgrind:

~~~shell
$ ctest -T memcheck
~~~

# GLang Library
This is the entire module system included with the glang installer.

**Folder** `core`:

GLang's core library.

**Folder** `std`:

GLang's standard library.

**Folder** `tests`:

Tests and runnable files to try out features in GLang.

## About
The `core` library is a set of functions and constants (e.g. `add`) for GLang. All programs use the `core` library (loaded automatically at the top of programs)

The standard library is built on top of the `core` library. It uses these fundamental functions (e.g. `add`) to create modules like `std_os` or `std_hashmap`.

The `tests` library can be used for both development or program testing. The shell script `run_tests.sh` can be used to run a set of test suites in GLang.

# GLang Library
This is the entire module system included with the glang installer.

**Folder** `fundamental`:

GLang's fundamental library

**Folder** `std`:

GLang's standard library.

**Folder** `tests`:

Tests and runnable files to try out features in GLang. _Use `cargo build --release --features benchmark` to see the elapsed time of test programs._

## About
The fundamental library is a set of functions and constants (e.g. `push`) for GLang. All programs use the fundamental library (loaded automatically at the top of programs)

The standard library is built on top of the fundamental library. It uses these fundamental functions (e.g. `push`) to create modules like `std_os` or `std_hashmap`

The tests library can be used for both development or program testing. You can run files like `test_loop.glang` to see how fast GLang is running.

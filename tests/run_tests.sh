echo "Building release binary (optimized)"
cargo build --release
echo "Running tests..."
target/release/glang tests/test_comparisons.glang
target/release/glang tests/test_constants.glang
target/release/glang tests/test_imports.glang
target/release/glang tests/test_loop.glang
target/release/glang tests/test_mutability.glang
target/release/glang tests/test_recursion.glang
target/release/glang tests/test_scope.glang
target/release/glang tests/test_try.glang

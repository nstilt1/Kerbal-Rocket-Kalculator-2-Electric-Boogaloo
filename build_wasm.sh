# Run cargo tests
cargo test -- --skip max_volume_tests

# Check if the tests were successful
if [ $? -eq 0 ]; then
    echo "No breaks found"
    
    # compile rust code to webassembly
    wasm-pack build --target web --release

    # move the files to the public folder
    mv pkg/ksp_bg.wasm ../frontend/public/ksp_bg.wasm
    mv pkg/ksp.js ../frontend/public/ksp.js
    mv pkg/package.json ../frontend/public/package.json

    echo "Build successful"
else
    echo "Tests failed. Run `cargo test` to see which tests failed."
fi
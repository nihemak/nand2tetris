# HardwareSimulator (SDL)

```bash
# test all
# https://stackoverflow.com/questions/74637159/how-to-increase-stack-size-of-threads-used-by-cargo-test
RUST_MIN_STACK=3000000 cargo test

# execute
cargo run

# execute (release build)
cargo build --release
./target/release/HardwareSimulator
```

set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

# Run benchmarks
bench pattern="":
    cd benchmark && cargo bench "{{pattern}}"

# Run tests
test pattern="":
    cargo test "{{pattern}}"

# Run binary
run binary:
    cargo run --bin "{{binary}}"

# Run & keep running a binary
watch binary:
    bacon run -- "{{binary}}"

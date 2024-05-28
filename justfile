# Build all wasm using cosmwasm/optimizer:0.15.1 docker image
build-optimize:
  echo "Compiling optimized wasm..."
  docker run --rm -t -v "$(pwd)":/code \
    --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
    --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
    cosmwasm/optimizer:0.15.1

# Run cargo fmt and clippy checks
lint:
  cargo fmt --all -- --check
  cargo clippy --all-targets --all-features -- -D warnings

# Generate JSON schema files for all contracts in the project
generate-schemas:
  echo "Generating JSON schema files..."
  cargo run --bin schema

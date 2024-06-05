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

# Generate JSON schema files for all contracts in the project in a directory named `schemas`
generate-schemas:
  mkdir -p schemas
  echo "Generating JSON schema files for ics02-client..."
  cargo run --bin ics02_schema
  cp schema/cw-ibc-lite-ics02-client.json schemas/
  echo "Generating JSON schema files for ics07-tendermint..."
  cargo run --bin ics07_schema
  cp schema/cw-ibc-lite-ics07-tendermint.json schemas/
  echo "Generating JSON schema files for ics26-router..."
  cargo run --bin ics26_schema
  cp schema/cw-ibc-lite-ics26-router.json schemas/
  echo "Generating JSON schema files for ics20-transfer..."
  cargo run --bin ics20_schema
  rm -r schema

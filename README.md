# CosmWasm IBC Lite

This workspace contains the CosmWasm IBC Lite implementation. IBC lite is a work in progress trimmed down specification of the IBC protocol. It is designed to be simpler to implement and be as secure as the full IBC protocol. Learn more about IBC lite in the [IBC Lite Spec](https://github.com/cosmos/ibc/pull/1093).

This is a implementation of the IBC Protocol in pure Rust and CosmWasm. It would work on CosmWasm chains without the need for `ibc-go`. There are no current relayer implementations for IBC Lite.

This repository is a work in progress and is not ready for production use. This repo also contains the e2e test between cw-eureka and ibc-go-lite.

## How to reproduce the e2e test

Ensure you have configured anything you need to run:

- ibc-go code
- rust code
- docker

For simplicity, we need `just` installed. So if you're using macOS run: `brew install just`.

Compile cw contracts, run: `just build-optimize`.

Finally, run the test: `just e2e-test TestWithIBCLiteTestSuite/TestCW20Transfer`.

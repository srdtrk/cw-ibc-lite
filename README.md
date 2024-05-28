# CosmWasm Workspace Example

This is an example cargo workspace for a CosmWasm project. It contains a template contract in the `contracts` directory. The goal is to show how to structure a cargo workspace with multiple contracts. I use this as a template for my own projects. If you also need to maintain packages shared between contracts, you can create a `packages` directory and add them there, you'll also need to modify `Cargo.toml` to include them.

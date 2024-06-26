# `CosmWasm` IBC Lite Router Contract

This contract is the core router for `CosmWasm` IBC Lite. It is responsible for handling:

- `SendPacket`
- `RecvPacket`
- `Acknowledgement`
- `Timeout`

It also stores the following provable state as defined in [ICS-24 host requirements](https://github.com/cosmos/ibc/blob/main/spec/core/ics-024-host-requirements/README.md):

| Store          | Path format                                                                    | Value type        | Defined in |
| -------------- | ------------------------------------------------------------------------------ | ----------------- | ---------------------- |
| provableStore  | "commitments/ports/{identifier}/channels/{identifier}/sequences/{sequence}"    | bytes             | [ICS 4](../ics-004-channel-and-packet-semantics) |
| provableStore  | "receipts/ports/{identifier}/channels/{identifier}/sequences/{sequence}"       | bytes             | [ICS 4](../ics-004-channel-and-packet-semantics) |
| provableStore  | "acks/ports/{identifier}/channels/{identifier}/sequences/{sequence}"           | bytes             | [ICS 4](../ics-004-channel-and-packet-semantics) |

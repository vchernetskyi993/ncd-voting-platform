# Elections contract

Core part of open elections. Contact for communicating with NEAR network.

## Testnet

```shell
near delete elections.vchernetskyi.testnet vchernetskyi.testnet
near create-account elections.vchernetskyi.testnet --masterAccount vchernetskyi.testnet

./build.sh
near deploy elections.vchernetskyi.testnet --wasmFile res/elections.wasm --initFunction new --initArgs '{}'
near call elections.vchernetskyi.testnet register_organization \
    --args '{"account": "org1.vchernetskyi.testnet"}' \
    --accountId elections.vchernetskyi.testnet

ELECTION_DATA=$(python scripts/generate_election.py)
near call elections.vchernetskyi.testnet create_election --accountId org1.vchernetskyi.testnet --deposit 1 --args "$ELECTION_DATA"
```


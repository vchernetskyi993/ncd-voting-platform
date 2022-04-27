# Elections contract

Core part of open elections. Contact for communicating with NEAR network.

## Local network

```shell
nearup run localnet
export NODE_ENV=local
# if network is started on different machine:
# export NEAR_NODE_URL="http://<host_ip>:3030"

near create-account vchernetskyi.node0 \
    --masterAccount node0 \
    --initialBalance 1000 \
    --keyPath ~/.near/localnet/node0/validator_key.json
```

## Generic deployment commands

```shell
export MASTER_ACCOUNT=vchernetskyi.node0
# or: export MASTER_ACCOUNT=vchernetskyi.testnet

near delete elections.$MASTER_ACCOUNT $MASTER_ACCOUNT
near create-account elections.$MASTER_ACCOUNT --masterAccount $MASTER_ACCOUNT

./build.sh
near deploy elections.$MASTER_ACCOUNT --wasmFile res/elections.wasm \
    --initFunction new --initArgs '{}'

near create-account org1.$MASTER_ACCOUNT --masterAccount $MASTER_ACCOUNT

near call elections.$MASTER_ACCOUNT register_organization \
    --args "{\"account\": \"org1.$MASTER_ACCOUNT\"}" \
    --accountId elections.$MASTER_ACCOUNT

ELECTION_DATA=$(python scripts/generate_election.py)
near call elections.$MASTER_ACCOUNT create_election \
    --accountId org1.$MASTER_ACCOUNT \
    --deposit 1 \
    --args "{\"input\": \"$ELECTION_DATA\"}"
```

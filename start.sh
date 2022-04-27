#!/bin/bash

BLUE='\033[0;34m'
NC='\033[0m'

main() {
    init_variables
    deploy_contract
    init_logging
    start_gateway
    start_front_end

    # FIXME: Ctrl+C works correctly only after this point
    echo -e "\n${BLUE}All services started. Waiting for Ctrl + C...${NC}"
    wait
}

init_variables() {
    if [[ ! -v MASTER_ACCOUNT ]]; then
        echo "MASTER_ACCOUNT variable is not set. Exiting..."
        exit 1
    fi

    CONTRACT_ACCOUNT=elections.$MASTER_ACCOUNT
    ORGANIZATION_ACCOUNT=org1.$MASTER_ACCOUNT
    REPO_DIR=$(pwd)
}

deploy_contract() {
    echo -e "\n${BLUE}[1] Deploying Elections contract${NC}\n"
    cd $REPO_DIR/contract
    
    near delete elections.$MASTER_ACCOUNT $MASTER_ACCOUNT
    near create-account $CONTRACT_ACCOUNT --masterAccount $MASTER_ACCOUNT

    ./build.sh
    near deploy $CONTRACT_ACCOUNT --wasmFile res/elections.wasm \
        --initFunction new --initArgs '{}'

    near create-account $ORGANIZATION_ACCOUNT --masterAccount $MASTER_ACCOUNT

    near call $CONTRACT_ACCOUNT register_organization \
        --args "{\"account\": \"$ORGANIZATION_ACCOUNT\"}" \
        --accountId $CONTRACT_ACCOUNT    
}

init_logging() {
    SERVICES_LOG=${REPO_DIR}/$$.log
    touch $SERVICES_LOG
    tail -f $SERVICES_LOG & 
    TAIL_PID=$!
}

start_gateway() {
    echo -e "\n${BLUE}[2] Starting REST Gateway${NC}"
    cd $REPO_DIR/gateway
    npm install
    
    ORGANIZATION_PRIVATE_KEY=$(cat $HOME/.near-credentials/testnet/$ORGANIZATION_ACCOUNT.json | jq -r ".private_key")

    NEAR_NETWORK_ID=testnet NEAR_NODE_URL=https://rpc.testnet.near.org \
    NEAR_ACCOUNT=$ORGANIZATION_ACCOUNT NEAR_CONTRACT=$CONTRACT_ACCOUNT \
    NEAR_PRIVATE_KEY=$ORGANIZATION_PRIVATE_KEY \
        npm start > $SERVICES_LOG &

    GATEWAY_PID=$!

    wait_for "Express is listening"
}

start_front_end() {
    echo -e "\n${BLUE}[3] Starting Voter front-end application${NC}"
    cd $REPO_DIR/voter-dapp
    npm install

    BROWSER=none REACT_APP_NEAR_ENV=testnet \
    REACT_APP_ELECTIONS_CONTRACT_ID=$CONTRACT_ACCOUNT \
    REACT_APP_ORGANIZATION_ID=$ORGANIZATION_ACCOUNT \
        npm start > $SERVICES_LOG &

    FRONT_END_PID=$!

    wait_for "No issues found."
}

wait_for() {
    tail -f $SERVICES_LOG | grep -q "$1"
}

shutdown() {
    echo ""
    shutdown_service "$FRONT_END_PID" "Voter Front-end App"
    shutdown_service "$GATEWAY_PID" "Gateway"
    echo "Clean up..."
    kill $TAIL_PID
    rm $SERVICES_LOG
}

shutdown_service() {
    if [[ ! -z $1 ]]; then
        echo "Shutting down ${2}..."
        kill $1
    fi
}

trap shutdown INT TERM

main

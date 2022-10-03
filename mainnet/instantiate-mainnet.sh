#!/bin/bash
# Run this after deploying and getting the code ID
# User should pass in the code ID to the contract like:
# ./instantiate-local.sh 2 juno1yhqft6d2msmzpugdjtawsgdlwvgq3samrm5wrw
if [ -z "$1" ]
then
    echo "Must provide code ID (Example ./instantiate-local.sh 19 juno1yhqft6d2msmzpugdjtawsgdlwvgq3samrm5wrw)"
    exit 1
else
    CODE_ID=$1
fi

INIT='{"allowed": ["juno1gvc0l4upc88arx673tmg7u3g7zsssnyyle5ph5"]}'
junod tx wasm instantiate "$CODE_ID" "$INIT" --label "yieldcat" --from mikereg --node https://rpc-juno.itastakers.com:443 --chain-id juno-1 --gas-prices 0.025ujuno --gas auto --gas-adjustment 1.3 --output json -b block --no-admin -y
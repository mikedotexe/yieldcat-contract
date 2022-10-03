#!/bin/bash
# Not optimized
junod tx wasm store ../output/yieldcat.wasm --from juno1yhqft6d2msmzpugdjtawsgdlwvgq3samrm5wrw --node https://juno-testnet-rpc.polkachu.com:443 --chain-id uni-5 --gas-prices 0.025ujunox --gas auto --gas-adjustment 1.3 --output json -b block -y
# Optimized
#junod tx wasm store ../target/wasm32-unknown-unknown/release/yieldcat.wasm --from mikereg --node https://juno-testnet-rpc.polkachu.com:443 --chain-id uni-5 --gas-prices 0.025ujunox --gas auto --gas-adjustment 1.3 --output json -b block -y
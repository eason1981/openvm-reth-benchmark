#!/bin/bash
set -e
cd bin/client-eth
cargo openvm build --no-transpile
mkdir -p ../host/elf
SRC="target/riscv32im-risc0-zkvm-elf/release/openvm-client-eth"
DEST="../host/elf/openvm-client-eth"

if [ ! -f "$DEST" ] || ! cmp -s "$SRC" "$DEST"; then
  cp "$SRC" "$DEST"
fi
cd ../..

mkdir -p rpc-cache
source .env
MODE=prove-agg # can be execute, prove, or prove-e2e
PROFILE="release"
BLOCK_NUMBER=17106222

arch=$(uname -m)
case $arch in
arm64 | aarch64)
  RUSTFLAGS="-Ctarget-cpu=native"
  ;;
x86_64 | amd64)
  RUSTFLAGS="-Ctarget-cpu=native -C target-feature=+avx512f"
  ;;
*)
  echo "Unsupported architecture: $arch"
  exit 1
  ;;
esac
RUSTFLAGS=$RUSTFLAGS cargo build --bin openvm-reth-benchmark --profile=$PROFILE --no-default-features --features=$FEATURES
PARAMS_DIR="params"
RUST_LOG="info,p3_=warn" OUTPUT_PATH="metrics.json" ./target/$PROFILE/openvm-reth-benchmark --kzg-params-dir $PARAMS_DIR --$MODE --block-number $BLOCK_NUMBER --chain-id 1 --cache-dir rpc-cache
# RUST_LOG="info,p3_=warn" OUTPUT_PATH="metrics.json" ./target/$PROFILE/openvm-reth-benchmark --kzg-params-dir $PARAMS_DIR --$MODE --block-number $BLOCK_NUMBER --rpc-url $RPC_1 --cache-dir rpc-cache

# reth or tendermint
# RUST_LOG="info,p3_=warn" OUTPUT_PATH="metrics.json" ./target/$PROFILE/openvm-reth-benchmark --kzg-params-dir $PARAMS_DIR --$MODE --block-number $BLOCK_NUMBER --chain-id 1 --cache-dir rpc-cache
# fib
# RUST_LOG="info,p3_=warn" OUTPUT_PATH="metrics.json" ./target/$PROFILE/openvm-reth-benchmark --kzg-params-dir $PARAMS_DIR --$MODE --block-number $BLOCK_NUMBER --chain-id 1 --cache-dir rpc-cache --fib-n 1000

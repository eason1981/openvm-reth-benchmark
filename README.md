# OpenVM Reth Benchmark

Benchmarks of running [Reth](https://github.com/paradigmxyz/reth) on the [OpenVM](https://github.com/openvm-org/openvm)
framework to generate zero-knowledge proofs of EVM block execution on Ethereum Mainnet.

> [!CAUTION]
>
> This repository is still an active work-in-progress and is not audited or meant for production usage.

## Getting Started

To run these benchmarks locally, you must first have [Rust](https://www.rust-lang.org/tools/install) installed. Then follow the rest of the instructions below.

### Installing the `cargo-openvm` CLI

Install the OpenVM command line interface by building from source via:

```bash
cargo install --git 'http://github.com/openvm-org/openvm.git' cargo-openvm
```

### RPC Node Requirement

RSP fetches block and state data from a JSON-RPC node. You must use an archive node which preserves historical intermediate trie nodes needed for fetching storage proofs. Common RPC providers such as Alchemy and QuickNode offer endpoints that provide these storage proofs.

You can pass the RPC URL into the CLI by either using the `--rpc-url` argument

```bash
cargo run --bin openvm-reth-benchmark --release -- --block-number 18884864 --rpc-url <RPC>
```

or by providing the `RPC_1` variable in the `.env` file and specifying the chain id in the CLI command like this:

```bash
cargo run --bin openvm-reth-benchmark --release -- --block-number 18884864 --chain-id 1
```

#### Using cached client input

The client input (witness) generated by executing against RPC can be cached to speed up iteration of the client program by supplying the `--cache-dir` option:

```bash
mkdir -p rpc-cache
cargo run --bin openvm-reth-benchmark --release -- --block-number 18884864 --chain-id 1 --cache-dir rpc-cache
```

Note that even when utilizing a cached input, the host still needs access to the chain ID to identify the network type, either through `--rpc-url` or `--chain-id`.

## Running Benchmarks

### Helper Script

We describe the different steps and commands needed to run the benchmark in subsequent sections, but to ease the process, we provide a helper script in [`run.sh`](./run.sh) that you can run directly. You only need to edit the `$MODE` variable in the script depending on your usage.

### Compiling the Guest Program

Before running the benchmark, you must first compile the guest program using `cargo-openvm`. Starting from the root of the repository, run:

```bash
cd bin/client-eth
cargo openvm build
mkdir -p ../host/elf
cp target/riscv32im-risc0-zkvm-elf/release/openvm-client-eth ../host/elf/
cd ../..
```

This process must currently be done manually, but will soon be automated with build scripts.

If this is your first time using `cargo-openvm`, cargo may prompt you to install the `rust-src` component for a nightly toolchain. This will look like:

```bash
rustup component add rust-src --toolchain nightly-2024-10-30-$arch-unknown-linux-gnu
```

where `$arch` is the architecture of your machine (e.g. `x86_64` or `aarch64`).

### Executing the Runtime

To execute the guest program for only the OpenVM runtime (without proving), for example to collect metrics such as cycle counts, run:

```bash
RUSTFLAGS="-Ctarget-cpu=native" RUST_LOG=info OUTPUT_PATH="metrics.json" \
cargo run --bin openvm-reth-benchmark --release -- \
--execute --block-number 21345144 --rpc-url $RPC_1 --cache-dir rpc-cache
```

By default a minimal set of metrics will be collected and output to a `metrics.json` file.

### Generating App Proofs

The overall program for executing an Ethereum block may be long depending on how many transactions on in the block. The OpenVM framework uses continuations to prove unbounded program execution by splitting the program into multiple segments and proving segments separately.

To prove all segments of the block execution program (without aggregation, see next section for that), run:

```bash
RUSTFLAGS="-Ctarget-cpu=native" RUST_LOG=info OUTPUT_PATH="metrics.json" \
cargo run --bin openvm-reth-benchmark --release -- \
--prove --block-number 21345144 --rpc-url $RPC_1 --cache-dir rpc-cache
```

This will generate proofs locally on your machine. Given how large these programs are, it might take a while for the proof to generate.

### Generating Proof for On-Chain Verification

In order to have a single proof of small size for on-chain verification in the EVM, the OpenVM framework uses proof aggregation and STARK-to-SNARK recursion to generate a final proof for verification in a smart contract.

Before running the benchmark, you will need to download a KZG trusted setup necessary for generating the Halo2 SNARK proofs:

```bash
#!/bin/bash

for k in {5..24}
do
    wget "https://axiom-crypto.s3.amazonaws.com/challenge_0085/kzg_bn254_${k}.srs"
    # for faster download, install s5cmd and use:
    # s5cmd --no-sign-request cp --concurrency 10 "s3://axiom-crypto/challenge_0085/${pkey_file}" .
done

mv *.srs params/
export PARAMS_DIR=$(pwd)/params
```

To run the full end-to-end benchmark for EVM verification, run:

```bash
RUSTFLAGS="-Ctarget-cpu=native" RUST_LOG=info OUTPUT_PATH="metrics.json" \
cargo run --bin openvm-reth-benchmark --release -- \
--prove-e2e --block-number 21345144 --rpc-url $RPC_1 --cache-dir rpc-cache
```

### Summarizing Benchmark Results

After running an [end-to-end benchmark](#generating-proof-for-on-chain-verification), there will be a `metrics.json` with collected metrics. We have a python script that will parse the JSON into a markdown summary. Run it by installing `python3` and running:

```bash
python3 ci/summarize.py metrics.json --print
```

### Advanced Configuration

The benchmark command accepts additional arguments that can be used to configure the benchmark. **These are low-level and require knowledge of the proof system.** They include:

- `--app-log-blowup`: Set the blowup factor for the App VM proofs (default: 2)
- `--agg-log-blowup`: Set the blowup factor for the leaf aggregation proofs (default: 2)
- `--internal-log-blowup`: Set the blowup factor for the internal non-leaf aggregation proofs (default: 2)
- `--root-log-blowup`: Set the blowup factor for the root STARK aggregation proof (default: 3)
- `--max-segment-length`: Set the threshold number of cycles before the execution should segment (default: `2 ** 23 - 100`)

### Github Workflow

A github actions workflow for running the benchmark via workflow dispatch is available [here](.github/workflows/reth-benchmark.yml).

### What are good testing blocks

A good small block to test on for Ethereum mainnet is: `21345144`, which has only 8 transactions.

## Acknowledgements

- The zkVM framework uses [OpenVM](https://github.com/openvm-org/openvm).
- The underlying Rust libraries make heavy use of [Reth](https://github.com/paradigmxyz/reth) and [Revm](https://github.com/bluealloy/revm/).
- This repo was forked from [RSP](https://github.com/succinctlabs/rsp/tree/main)
- The RSP repo builds on work from [Zeth](https://github.com/risc0/zeth)

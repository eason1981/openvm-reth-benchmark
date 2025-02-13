use core::mem::transmute;

use openvm::io::{println, read, reveal};
use openvm_client_executor::{io::ClientExecutorInput, ClientExecutor, EthereumVariant};
#[allow(unused_imports, clippy::single_component_path_imports)]
use {
    openvm_algebra_guest::IntMod,
    openvm_bigint_guest, // trigger extern u256 (this may be unneeded)
    openvm_ecc_guest::k256::Secp256k1Point,
    openvm_keccak256_guest, // trigger extern native-keccak256
    openvm_pairing_guest::bn254::Bn254G1Affine,
};

pub fn run() {
    println("fib starting");
    setup_all_moduli();
    setup_all_complex_extensions();


    // Read the input.
    let input: ClientExecutorInput = read();
    println("finished reading input");

    // Execute the block.
    let executor = ClientExecutor;
    let _ = executor.execute::<EthereumVariant>(input).expect("failed to execute client");
}

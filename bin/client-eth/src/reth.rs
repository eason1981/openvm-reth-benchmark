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

openvm_algebra_moduli_macros::moduli_init! {
    "0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47", // Bn254Fp Coordinate field
    "0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001", // Bn254 Scalar
    "0xFFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE FFFFFC2F", // secp256k1 Coordinate field
    "0xFFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE BAAEDCE6 AF48A03B BFD25E8C D0364141", // secp256k1 Scalar field
}
openvm_ecc_sw_macros::sw_init! {
    Bn254G1Affine,
    Secp256k1Point,
}
openvm_algebra_complex_macros::complex_init! {
    Bn254Fp2 { mod_idx = 0 },
}

pub fn run() {
    println("reth starting");
    setup_all_moduli();
    setup_all_curves();
    setup_all_complex_extensions();

    // Read the input.
    let input: ClientExecutorInput = read();
    println("finished reading input");

    // Execute the block.
    let executor = ClientExecutor;
    let header = executor.execute::<EthereumVariant>(input).expect("failed to execute client");
    let block_hash = header.hash_slow();

    // Commit the block hash.
    let block_hash = unsafe { transmute::<_, [u32; 8]>(block_hash) };

    block_hash.into_iter().enumerate().for_each(|(i, x)| reveal(x, i));
}

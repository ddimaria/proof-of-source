// use anyhow::{Context, Result};
use blake3::hash;
// use fs_extra::dir::copy;
// use fs_extra::dir::CopyOptions;

fn main() {
    let deployed_contract =
        include_bytes!("../../contract/target/wasm32-unknown-unknown/release/contract.wasm");
    let hashed = hash(deployed_contract);

    println!("{}", hashed);

    let built_contract =
        include_bytes!("../../bafybeigielvchcsuaz7lihigqwu5noajmefqyxqmnnbkawcjutynsj2jn4/contract/target/wasm32-unknown-unknown/release/contract.wasm");
    let hashed = hash(built_contract);

    println!("{}", hashed);
}

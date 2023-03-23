use blake3::{hash, Hash};
use paste::paste;
use std::process::{Command, Output};

use crate::error::{Result, RuntimeError};

const PATH: &str = "../temp";

// macro_rules! bytes_path {
//     ($cid: ident, $path: ident) => {{
//         let bytes = concat!(
//             stringify!($path),
//             "/",
//             stringify!($cid),
//             "/erc20/target/wasm32-unknown-unknown/release/erc20_wit.wasm"
//         );

//         paste! {
//            $bytes
//         }
//     }};
// }

pub fn compile(cid: &str) -> Result<Hash> {
    // first, get the source code from IPFS
    get_from_ipfs(cid)?;

    let command = format!(
        "cd {}/{}/erc20 && cross build --target wasm32-unknown-unknown --release",
        PATH, cid
    );

    let _output = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output()
        .map_err(|e| RuntimeError::CompileError(cid.into(), e.to_string()))?;

    // TODO(ddimaria): implement macro to make this dynamic
    let bytes =
        include_bytes!("../../temp/bafybeie5pre4ytk63ualikeokdc2ehdse6km5pek365g2sxtqp2czeoy24/erc20/target/wasm32-unknown-unknown/release/erc20_wit.wasm");

    Ok(hash(bytes))
}

pub fn get_from_ipfs(cid: &str) -> Result<Output> {
    // first, clean up any old directories
    clean(cid)?;

    let command = format!("cd {} && iroh get {}", PATH, cid);
    Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output()
        .map_err(|e| RuntimeError::RetrieveFromIpfsError(cid.into(), e.to_string()))
}

fn clean(cid: &str) -> Result<Output> {
    let command = format!("mkdir -p {} && cd {} && rm -rf {}", PATH, PATH, cid);
    Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output()
        .map_err(|e| RuntimeError::CleanError(cid.into(), e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    // use test_log::test;

    const CID: &str = "bafybeie5pre4ytk63ualikeokdc2ehdse6km5pek365g2sxtqp2czeoy24";

    #[test]
    fn it_compiles() {
        compile(CID).unwrap();
    }
}

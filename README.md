# Proof of Source

Linking up source code to compiled binaries is essential for formal and independent auditing of smart contract code.
This proof of concept blockchain will compile Rust WASM smart contracts in a way that other blockchains execute smart contract functions.
Give that a directory of source code can be hashed into a CID in IPFS, we can formally verify that a specified CID matches the compiled source.

## Initial PoC

To validate this concept, I wrote some scripts and a small rust program to hash and compare hashes of compiled WASM code:

```shell
 curl -fsSL https://sh.iroh.computer/install_laptop.sh | sh
iroh start store
rm -rf temp/
rsync -av --progress contract temp --exclude contract/target
URL=$(iroh add temp/contract --recursive --offline | tail -n 1)
CID=${URL##*/}
iroh get $CID
cd $CID
cross build --target wasm32-unknown-unknown --release
cd ./../proof
cargo run
```

## Blockchain

This blockchain will look similar to other blockchains, but highly tuned to compile Rust WASM.

### Repeatable Builds

To ensure that builds happen the same way for each node, we're assuming there is a `Cargo.lock` file for each source code directory.  We're also compiling with a docker container to standardize the builds.  We'll assume that any individual compiler will compile using the same method.

### Types

#### Account

Like Ethereum, accounts will just be the last 20 bytes of a 265 bit hash.  We'll be using Blake3 as the hashing algorithm.  Since there are no smart contract, all accounts are externally owned.

#### Transaction

There will only be one type of transaction in this blockchain, and it will simply compile source from a CID within IPFS and record the 256 bit hash.

```rust
struct Transaction {
    pub from: Address,
    hash: Option<H256>,
    nonce: U256,
    cid: CID,
    target: Target,
}

enum Target {
    Wasm32UnknownUnknown
}
```

The verification process will involve:

1. Pulling the source code from an IPFS node
2. Compiling the source to a binary
3. Hashing the binary

Transaction receipts will capture this information:

```rust
struct TransactionReceipt {
    transaction_hash: H256,
    cid: CID,
    compiled_hash: H256,
}
```

Transaction receipts will be stored on chain to allow for quick lookups by CID and Target.

#### Block

```rust
struct Block {
    number: U64,
    hash: Option<H256>,
    parent_hash: H256,
    transactions: Vec<Transaction>,
    transactions_root: H256,
    state_root: H256,
}
```

### Methods

The nodes will support a json-rpc interface.  Since this is a sample blockchain, we'll only implemnt a handful of methods.

#### send_raw_transaction

After creating and signing a transaction, send it to the blockchain for execution.

#### get_transaction_receipt

Once a transaction is confirmed and added to a block, allow for the retrieval of the transaction receipt.

#### get_transaction_count

In order to keep track of account nonces, support querying the blockchain for the total number of transactions for a given account.

#### get_compiled_hash

Using the `CID` and the `Target` for any confirmed transaction, return the hash of the compiled source.
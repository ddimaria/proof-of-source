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

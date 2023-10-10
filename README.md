# Get holders for an NFT collection

This is a minimal rust program allowing you to get the list of holders for an NFT collection
using the Shyft api. This could be useful for airdrops or other things.

1) Clone this repo
2) Install rust (https://www.rust-lang.org/tools/install)
3) Copy the .env.example file to .env and fill in your [Shyft](https://shyft.to) api key 
4) Set the COLLECTION variable in src/main.rs to the address of the collection
5) Run `cargo run` to get the holders in ./holder_map.json
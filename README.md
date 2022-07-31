# Blockchain in [Rust](https://www.rust-lang.org)

## Cryptocurrency Blockchains
### Two main data structures
- The blocks in blockchains
- The transactions within the blocks
### Ancillary data
- Wallets
- Addresses
- Balances
- Peers

## Generic Blockchains (with PoW support)
### Blockchain = chronological, sequential list of *blocks*
### Blocks contain this information:
- **Index**: this block's location within the list of blocks
- **Payload**: any relevant information or events that have ocurred for/in the block
- **Timestamp**: gives our blockchain a sense of time
- **Nonce**: special number used for mining (for PoW verification)
- **Previous block hash**: cryptographic fingerprint of previous block
- **Hash**: cryptographic fingerprint of all of the above data concatenated together

## What is Hashing?
### In a nutshell, a hash algorithm consists of a set of irreversible computations that can be performed on a datum to generate a (usually) unique byte sequence.
- **MD5("rustlang")** = "d37847bc2e2451e9cf3e764e673b8b26"
- **SHA-1("rustlang")** = "5bc1d865ae852c5a599c3e1b5eaa2947fbe98b32"
- **SHA-256("rustlang")** = "cb21864ba536863171f9437ac6d867281dfb29e055bdb8356539af0889822ef9"

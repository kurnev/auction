# Auction algorithm implemented in [Rust](https://www.rust-lang.org/)

## Instructions

To build locally you need to install rust and cargo from official website: https://rustup.rs/

To build and run the binary (program will terminate, because it is not programmed to wait for incoming data):
```
cargo run
```

To run tests:
```
cargo run
```

## Requirements for an algorithm:
Second-price, sealed-bid auction:

- An object is for sale with a reserve price.
- We have several potential buyers, each one being able to place one or more bids.
- The buyer winning the auction is the one with the highest bid above or equal to the reserve price.
- The winning price is the highest bid price from a non-winning buyer above the reserve price (or the reserve price if none applies)
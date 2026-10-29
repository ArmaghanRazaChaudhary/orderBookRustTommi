# orderbook

A small L2 limit order book in Rust.

It keeps bids and asks as sorted price levels and tracks every order by ID so it can be cancelled and answers the questions a trading system asks constantly such as what is the best bid, the best ask, and the spread?..etc

## What it does

- `add` an order (bid or ask). Rejects zero quantity and duplicate IDs.
- `cancel` an order by ID. Reduces its price level and removes the level when it empties.

## Run

cargo run     
cargo test   

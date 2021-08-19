pub mod auction;
pub mod bid;
mod item;
mod user;

// TODO: Open web port to communicate via REST

pub fn init() {
    let _state = auction::AuctionState::new();
    println!("Auction initalized!");
}

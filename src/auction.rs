use std::cmp::Ordering;

use crate::bid::Bid;
use crate::bid::BidResultCode;
use crate::item::Item;
use crate::user::User;
use crate::user::Winner;

pub struct AuctionState {
    pub users: Vec<User>,
    pub items: Vec<Item>,
}

impl AuctionState {
    pub fn new() -> AuctionState {
        let users: Vec<User> = Vec::new();
        let items: Vec<Item> = Vec::new();
        AuctionState { users, items }
    }

    pub fn create_new_user(&mut self) -> i32 {
        let last_user = self.users.last();
        let user_id: i32;
        match last_user {
            Some(user) => user_id = user.id,
            None => user_id = 0,
        }
        let new_user = User::new(user_id);
        self.users.push(new_user);
        self.users.last().unwrap().id
    }

    // TODO: create generic for creating stuff
    pub fn create_new_item(&mut self, reserve_price: f32) -> i32 {
        let last_item = self.items.last();
        let item_id: i32;
        match last_item {
            Some(item) => item_id = item.id,
            None => item_id = 0,
        }
        let new_item = Item::new(item_id, reserve_price);
        self.items.push(new_item);
        self.items.last().unwrap().id
    }

    pub fn register_new_bid(&mut self, user_id: i32, item_id: i32, price: f32) -> BidResultCode {
        if price == 0.0 {
            return BidResultCode::BidIsNull;
        };
        let mut iter = self.items.iter_mut();
        let item = iter.find(|x| x.id == item_id);
        match item {
            Some(item) => {
                let last_bid = item.bids.last();
                let bid_id: i32;
                // TODO: add id automatically
                match last_bid {
                    Some(bid) => bid_id = bid.id,
                    None => bid_id = 0,
                };
                let bid: Bid = Bid {
                    id: bid_id,
                    user_id,
                    price,
                };
                let lowest_bid_from_same_user = item
                    .bids
                    .iter()
                    .filter(|x| x.user_id == user_id)
                    .min_by(|a, b| {
                        // a.price.partial_cmp(&b.price).unwrap_or(Ordering::Equal)
                        a.price.partial_cmp(&b.price).unwrap_or(Ordering::Equal)
                    });
                match lowest_bid_from_same_user {
                    Some(lowest_bid) => {
                        if lowest_bid.price < price {
                            item.bids.push(bid);
                            return BidResultCode::Success;
                        } else {
                            return BidResultCode::BidLowerOrEqToPrevious;
                        }
                    }
                    None => {
                        item.bids.push(bid);
                        return BidResultCode::Success;
                    }
                }
            }
            None => {
                return BidResultCode::NoSuchItem;
            }
        }
    }

    pub fn end_auction(self, item_id: i32) -> Option<Winner> {
        let mut iter = self.items.iter();
        let item = iter.find(|&x| x.id == item_id).unwrap();
        let mut sorted_bids = item.bids.to_vec();
        sorted_bids.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap_or(Ordering::Equal));
        sorted_bids.reverse();
        let winning_bid = &sorted_bids[0];
        if winning_bid.price < item.reserve_price {
            println!("No winner!");
            None
        } else {
            let winning_user_id = winning_bid.user_id;
            let mut iter = sorted_bids.iter();
            let next_winning_bid = iter.find(|x| x.user_id != winning_user_id);

            match next_winning_bid {
                Some(bid) => {
                    println!(
                        "Auction has ended! Winner is the user #{} with price {}",
                        winning_bid.user_id, winning_bid.price
                    );
                    return Some(Winner {
                        user_id: winning_user_id,
                        winning_price: bid.price,
                    });
                }
                None => {
                    println!(
                        "Auction has ended! Winner is the user #{} with price {}",
                        winning_bid.user_id, winning_bid.price
                    );
                    return Some(Winner {
                        user_id: winning_user_id,
                        winning_price: item.reserve_price,
                    });
                }
            }
        }
    }
}

use std::cmp::Ordering;
use std::slice::Iter;

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

    pub fn create_new_user(&mut self) -> usize {
        self.users.push(User::new());
        self.users.last().unwrap().id
    }

    pub fn create_new_item(&mut self, reserve_price: f32) -> usize {
        self.items.push(Item::new(reserve_price));
        self.items.last().unwrap().id
    }

    pub fn register_new_bid(
        &mut self,
        user_id: usize,
        item_id: usize,
        price: f32,
    ) -> BidResultCode {
        if price == 0.0 {
            return BidResultCode::BidIsNull;
        };
        let mut iter = self.items.iter_mut();
        let item = iter.find(|x| x.id == item_id);
        match item {
            Some(item) => {
                let bid: Bid = Bid::new(user_id, price);
                let lowest_bid_from_same_user = item
                    .bids
                    .iter()
                    .filter(|x| x.user_id == user_id)
                    .min_by(|a, b| a.price.partial_cmp(&b.price).unwrap_or(Ordering::Equal));
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

    pub fn end_auction(&mut self, item_id: usize) -> Option<Winner> {
        let item = self.items.iter().find(|&x| x.id == item_id);
        match item {
            Some(item) => {
                let mut sorted_bids = item.bids.to_vec();
                sorted_bids
                    .sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap_or(Ordering::Equal));
                sorted_bids.reverse();

                let winning_bid = &sorted_bids.get(0);

                match winning_bid {
                    Some(winning_bid) => {
                        if winning_bid.price < item.reserve_price {
                            // Highest bid is lower than reserve price - no winner
                            AuctionState::report_no_winner()
                        } else {
                            let mut winning_user_id = winning_bid.user_id;
                            let mut iter = sorted_bids.iter();
                            let next_winning_bid = iter.find(|x| x.user_id != winning_user_id);

                            // Check if there are buyers with the same highest bid - first one is a winner
                            let first_highest_bid = iter
                                .filter(|x| x.price == winning_bid.price)
                                .min_by_key(|&x| x.id);

                            match first_highest_bid {
                                Some(bid) => {
                                    if bid.user_id != winning_user_id {
                                        winning_user_id = bid.user_id;
                                    }
                                }
                                _ => (),
                            }

                            match next_winning_bid {
                                Some(bid) => {
                                    // Check that next bid is above the reserve price
                                    if bid.price < item.reserve_price {
                                        // Next bid is below reserve price - use reserve price as winning price
                                        AuctionState::report_winner(
                                            winning_user_id,
                                            item.reserve_price,
                                        )
                                    } else {
                                        AuctionState::report_winner(winning_user_id, bid.price)
                                    }
                                }
                                None => {
                                    AuctionState::report_winner(winning_user_id, item.reserve_price)
                                }
                            }
                        }
                    }
                    None => {
                        // No bids - no winner
                        AuctionState::report_no_winner()
                    }
                }
            }
            // No such item with provided item_id - no winner
            None => AuctionState::report_no_winner(),
        }
    }

    fn report_winner(user_id: usize, winning_price: f32) -> Option<Winner> {
        println!(
            "Auction has ended! Winner is the user #{} with price {}",
            user_id, winning_price
        );
        Some(Winner {
            user_id,
            winning_price,
        })
    }

    fn report_no_winner() -> Option<Winner> {
        println!("No winner!");
        None
    }
}

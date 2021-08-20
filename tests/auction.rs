extern crate auction;

#[cfg(test)]
mod tests {

    use auction::{auction::AuctionState, bid::BidResultCode};

    #[test]
    fn simple_5_buyers_auction_with_a_winner() {
        let mut auction = AuctionState::new();
        let user_id1 = auction.create_new_user();
        let _user_id2 = auction.create_new_user();
        let user_id3 = auction.create_new_user();
        let user_id4 = auction.create_new_user();
        let user_id5 = auction.create_new_user();
        let item_id = auction.create_new_item(100.0);

        auction.register_new_bid(user_id1, item_id, 110.0);
        auction.register_new_bid(user_id1, item_id, 130.0);
        auction.register_new_bid(user_id3, item_id, 125.0);
        auction.register_new_bid(user_id4, item_id, 105.0);
        auction.register_new_bid(user_id4, item_id, 115.0);
        auction.register_new_bid(user_id4, item_id, 90.0);
        auction.register_new_bid(user_id5, item_id, 132.0);
        auction.register_new_bid(user_id5, item_id, 135.0);
        auction.register_new_bid(user_id5, item_id, 140.0);

        let winner = auction.end_auction(item_id).unwrap();
        assert!(winner.user_id == user_id5);
        assert!(winner.winning_price == 130.0);
    }

    #[test]
    fn no_winner() {
        let mut auction = AuctionState::new();
        let user_id1 = auction.create_new_user();
        let user_id2 = auction.create_new_user();
        let item_id = auction.create_new_item(100.0);

        let mut result = auction.register_new_bid(user_id1, item_id, 79.0);
        matches!(result, BidResultCode::Success);
        result = auction.register_new_bid(user_id1, item_id, 81.0);
        matches!(result, BidResultCode::Success);
        result = auction.register_new_bid(user_id2, item_id, 99.0);
        matches!(result, BidResultCode::Success);

        let winner = auction.end_auction(item_id);
        assert!(winner.is_none());
    }

    #[test]
    fn one_bidder_with_bid_lower_than_reserve_price() {
        let mut auction = AuctionState::new();
        let user_id = auction.create_new_user();
        let item_id = auction.create_new_item(1000.0);

        let result = auction.register_new_bid(user_id, item_id, 0.01);
        matches!(result, BidResultCode::Success);

        let winner = auction.end_auction(item_id);
        assert!(winner.is_none());
    }

    #[test]
    fn one_bidder_with_bid_higher_than_reserve_price() {
        let mut auction = AuctionState::new();
        let user_id = auction.create_new_user();
        let reserve_price = 112.11;
        let item_id = auction.create_new_item(reserve_price);

        let result = auction.register_new_bid(user_id, item_id, 444479.11);
        matches!(result, BidResultCode::Success);

        let winner = auction.end_auction(item_id).unwrap();
        assert!(winner.user_id == user_id);
        assert!(winner.winning_price == reserve_price);
    }

    #[test]
    fn one_bidder_with_bid_equal_to_reserve_price() {
        let mut auction = AuctionState::new();
        let user_id = auction.create_new_user();
        let reserve_price = 112.11;
        let item_id = auction.create_new_item(reserve_price);

        let result = auction.register_new_bid(user_id, item_id, reserve_price);
        matches!(result, BidResultCode::Success);

        let winner = auction.end_auction(item_id).unwrap();
        assert!(winner.user_id == user_id);
        assert!(winner.winning_price == reserve_price);
    }

    #[test]
    fn bid_cant_be_0() {
        let mut auction = AuctionState::new();
        let user_id = auction.create_new_user();
        let item_id = auction.create_new_item(199.0);

        let result = auction.register_new_bid(user_id, item_id, 0.0);
        matches!(result, BidResultCode::BidIsNull);

        let winner = auction.end_auction(item_id);
        assert!(winner.is_none());
    }

    #[test]
    fn bid_cant_be_lower_or_eq_to_previous_from_same_buyer() {
        let mut auction = AuctionState::new();
        let user_id = auction.create_new_user();
        let item_id = auction.create_new_item(199.0);

        let mut result = auction.register_new_bid(user_id, item_id, 100.0);
        matches!(result, BidResultCode::Success);
        result = auction.register_new_bid(user_id, item_id, 20.0);
        matches!(result, BidResultCode::BidLowerOrEqToPrevious);

        let winner = auction.end_auction(item_id);
        assert!(winner.is_none());
    }

    #[test]
    fn bid_can_be_lower_or_eq_to_previous_from_different_buyer() {
        let mut auction = AuctionState::new();
        let user_id1 = auction.create_new_user();
        let user_id2 = auction.create_new_user();
        let item_id = auction.create_new_item(199.0);

        let mut result = auction.register_new_bid(user_id1, item_id, 100.0);
        matches!(result, BidResultCode::Success);
        result = auction.register_new_bid(user_id2, item_id, 20.0);
        matches!(result, BidResultCode::Success);
        result = auction.register_new_bid(user_id2, item_id, 200.0);
        matches!(result, BidResultCode::Success);

        let winner = auction.end_auction(item_id).unwrap();
        assert!(winner.user_id == user_id2);
        assert!(winner.winning_price == 199.0);
    }

    #[test]
    fn no_bids_no_winner() {
        let mut auction = AuctionState::new();
        let _user_id1 = auction.create_new_user();
        let item_id = auction.create_new_item(199.0);

        let winner = auction.end_auction(item_id);
        assert!(winner.is_none());
    }

    #[test]
    fn several_bids_with_the_same_price() {
        let mut auction = AuctionState::new();
        let user_id1 = auction.create_new_user();
        let user_id2 = auction.create_new_user();
        let user_id3 = auction.create_new_user();
        let item_id = auction.create_new_item(0.5);

        let mut result = auction.register_new_bid(user_id1, item_id, 50.0);
        matches!(result, BidResultCode::Success);
        result = auction.register_new_bid(user_id2, item_id, 50.0);
        matches!(result, BidResultCode::Success);
        result = auction.register_new_bid(user_id3, item_id, 50.0);
        matches!(result, BidResultCode::Success);

        let winner = auction.end_auction(item_id).unwrap();
        assert!(winner.user_id == user_id1);
        assert!(winner.winning_price == 50.0);
    }

    #[test]
    fn several_bids_for_several_different_items() {
        let mut auction = AuctionState::new();
        let user_id1 = auction.create_new_user();
        let user_id2 = auction.create_new_user();
        let user_id3 = auction.create_new_user();
        let item_id1 = auction.create_new_item(0.5);
        let item_id2 = auction.create_new_item(555.10);
        let item_id3 = auction.create_new_item(50.0);

        let mut result = auction.register_new_bid(user_id1, item_id1, 50.0);
        matches!(result, BidResultCode::Success);
        result = auction.register_new_bid(user_id2, item_id2, 50.0);
        matches!(result, BidResultCode::Success);
        result = auction.register_new_bid(user_id3, item_id3, 100.0);
        matches!(result, BidResultCode::Success);
        result = auction.register_new_bid(user_id3, item_id1, 200.0);
        matches!(result, BidResultCode::Success);
        result = auction.register_new_bid(user_id2, item_id1, 300.0);
        matches!(result, BidResultCode::Success);

        let winner1 = auction.end_auction(item_id1).unwrap();
        assert!(winner1.user_id == user_id2);
        assert!(winner1.winning_price == 200.0);

        let winner2 = auction.end_auction(item_id2);
        assert!(winner2.is_none());

        let winner3 = auction.end_auction(item_id3).unwrap();
        assert!(winner3.user_id == user_id3);
        assert!(winner3.winning_price == 50.0);
    }
}

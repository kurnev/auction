use std::cmp::Ordering; 

struct User {
    id: i32,
}

struct Item {
    id: i32,
    reserve_price: f32,
    bids: Vec<Bid>,
}

#[derive(Clone, Debug)]
struct Bid {
    id: i32,
    user_id: i32,
    price: f32,
}

#[derive(Debug)]
struct Winner {
    user_id: i32,
    winning_price: f32,
}

struct AuctionState {
    users: Vec<User>,
    items: Vec<Item>,
}

impl User {
    pub fn new(id: i32) -> User {
      User {
          id,
      }
    }
}

impl Item {
    pub fn new(id: i32, reserve_price: f32) -> Item {
      let bids: Vec<Bid> = Vec::new();
      Item {
          id,
          reserve_price,
          bids,
      }
    }
}

impl AuctionState {
    pub fn new() -> AuctionState {
      let users: Vec<User> = Vec::new();
      let items: Vec<Item> = Vec::new();
      AuctionState {
          users,
          items,
      }
    }

    fn create_new_user(&mut self) -> i32 {
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
    fn create_new_item(&mut self, reserve_price: f32) -> i32 {
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
    
    fn register_new_bid(&mut self, user_id: i32, item_id: i32, price: f32) {
       let mut iter = self.items.iter_mut();
       let item = iter.find(|x| x.id == item_id).unwrap();
       let last_item = item.bids.last();
       let bid_id: i32;
       match last_item {
           Some(item) => bid_id = item.id,
           None => bid_id = 0,
       }
       let bid: Bid = Bid {id: bid_id, user_id, price};
       item.bids.push(bid);
    }

    fn end_auction(self, item_id: i32) -> Option<Winner> {
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
           //   Some(Winner {user_id: winning_bid.user_id, winning_price: winning_bid.price})
           let winning_user = winning_bid.user_id;
           // Find next bidder 
           let mut iter = sorted_bids.iter();
           let next_winning_price = iter.find(|x| x.user_id != winning_user).unwrap().price;
             
           println!("Auction has ended! Winner is the user #{} with price {}", winning_bid.user_id, winning_bid.price);
           Some(Winner {user_id: 3, winning_price: 130.0})
       }
    }
}

// TODO: Open web port to communicate via REST

fn main() {
    let state = AuctionState::new();
    println!("Auction initalized!");
}



#[cfg(test)]
mod tests {
    use crate::AuctionState;

    #[test]
    fn basic_test() {
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
        assert!(winner.user_id == 3);
        assert!(winner.winning_price == 130.0);
    }

    #[test]
    fn lower_than_reserve_price() {
        let mut auction = AuctionState::new();
        let user_id1 = auction.create_new_user();
        let user_id2 = auction.create_new_user();
        let item_id = auction.create_new_item(100.0);
        
        auction.register_new_bid(user_id1, item_id, 79.0);
        auction.register_new_bid(user_id1, item_id, 81.0);
        auction.register_new_bid(user_id2, item_id, 99.0);

        let winner = auction.end_auction(item_id); 
        assert!(winner.is_none());
    }
}
struct User {
    id: i32,
}

struct Item {
    id: i32,
    reserve_price: f32,
    bids: Bid,
}

struct Bid {
    id: i32,
    user_id: i32,
    value: f32,
}

struct AuctionState {
    users: Vec<User>,
    items: Vec<Item>,
    bids: Vec<Bid>,
}

impl AuctionState {
    pub fn new() -> AuctionState {
      let users: Vec<User> = Vec::new();
      let items: Vec<Item> = Vec::new();
      let bids: Vec<Bid> = Vec::new();
      AuctionState {
          users,
          items,
          bids
      }
    }

    fn make_bid(&mut self, user_id: i32) {

    }
}

fn main() {
    let state = AuctionState::new();
    println!("Auction initalized!");
}


fn end_auction() {
    
}

fn create_new_user(users: &Vec<User>) -> User {
    let user = User {id: 1};
    println!("Created new user {}", user.id);
    user
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic_test() {
        assert_eq!(2 + 2, 4);
    }
}
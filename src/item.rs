use crate::bid::Bid;
pub struct Item {
    pub id: i32,
    pub reserve_price: f32,
    pub bids: Vec<Bid>,
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

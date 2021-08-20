use crate::bid::Bid;
use std::cell::Cell;

thread_local!(static ITEM_ID: Cell<usize> = Cell::new(0));

pub struct Item {
    pub id: usize,
    pub reserve_price: f32,
    pub bids: Vec<Bid>,
}

impl Item {
    pub fn new(reserve_price: f32) -> Item {
        ITEM_ID.with(|thread_id| {
            let id = thread_id.get();
            thread_id.set(id + 1);
            Item {
                id,
                reserve_price,
                bids: Vec::new(),
            }
        })
    }
}

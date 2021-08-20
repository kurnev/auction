use std::cell::Cell;

thread_local!(static BID_ID: Cell<usize> = Cell::new(0));

#[derive(Clone, Debug)]
pub struct Bid {
    pub id: usize,
    pub user_id: usize,
    pub price: f32,
}

impl Bid {
    pub fn new(user_id: usize, price: f32) -> Bid {
        BID_ID.with(|thread_id| {
            let id = thread_id.get();
            thread_id.set(id + 1);
            Bid { id, user_id, price }
        })
    }
}

pub enum BidResultCode {
    Success,
    BidLowerOrEqToPrevious,
    BidIsNull,
    NoSuchItem,
}

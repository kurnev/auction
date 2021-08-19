#[derive(Clone, Debug)]
pub struct Bid {
    pub id: i32,
    pub user_id: i32,
    pub price: f32,
}

pub enum BidResultCode {
    Success,
    BidLowerOrEqToPrevious,
    BidIsNull,
    NoSuchItem,
}

pub struct User {
    pub id: i32,
}

impl User {
    pub fn new(id: i32) -> User {
        User { id }
    }
}

#[derive(Debug)]
pub struct Winner {
    pub user_id: i32,
    pub winning_price: f32,
}

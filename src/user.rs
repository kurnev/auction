use std::cell::Cell;

pub struct User {
    pub id: usize,
}

thread_local!(static USER_ID: Cell<usize> = Cell::new(0));

impl User {
    pub fn new() -> User {
        USER_ID.with(|thread_id| {
            let id = thread_id.get();
            thread_id.set(id + 1);
            User { id }
        })
    }
}

#[derive(Debug)]
pub struct Winner {
    pub user_id: usize,
    pub winning_price: f32,
}

use std::net::SocketAddr;
use common::manager::{Id, Item, Manager};

#[derive(Debug, Clone)]
pub struct User {
    pub id: Id,
    addr: SocketAddr,
}

impl Item<SocketAddr> for User {
    fn new(id: &Id, addr: &SocketAddr) -> Self {
        User {
            id: *id,
            addr: addr.clone(),
        }
    }
}

pub type UserManager = Manager<User, SocketAddr>;

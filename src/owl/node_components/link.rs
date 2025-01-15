use crate::owl::node::NULL_IDX;

pub struct Link{
    pub next:u16,
    pub prev:u16
}

impl Default for Link {
    fn default() -> Self {
        Self { next: NULL_IDX, prev: NULL_IDX }
    }
}
#[derive(Clone)]
pub struct Item {
    pub id: ItemId,
    pub count: u64,
}

impl Item {
    pub fn new(id: ItemId, count: u64) -> Self {
        Self {
            id, count
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum ItemId {
    Drill(u8),
    Conveyor(u8),
    CoalOre,
    Coal,
}
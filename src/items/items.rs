pub struct Item {
    id: ItemId,
    count: u64,
}

pub enum ItemId {
    Drill(u8),
    Conveyor(u8),
    CoalOre,
}
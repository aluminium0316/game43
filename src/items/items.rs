use std::{fmt::Debug, ops::Mul};

use crate::blocks::{block::{Error, MultiBlock}, conveyor::Conveyor, drill::Drill};

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

impl Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}({})", self.id, self.count))
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ItemId {
    Drill,
    Conveyor,
    CoalOre,
    Coal,
}

impl ItemId {
    pub fn to_block(&self) -> Option<Box<dyn MultiBlock>> {
        match self {
            ItemId::Drill => Some(Box::new(Drill::default())),
            ItemId::Conveyor => Some(Box::new(Conveyor::default())),
            _ => None
        }
    }
}
use crate::items::{filter::Filter, items::Item};

use super::block::Side;

#[derive(Clone)]
pub enum Event {
    Craft { input: Vec<Item>, output: Vec<Item> },
    Pull { count: u64, filter: Filter, side: Side },
}
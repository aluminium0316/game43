use crate::items::{filter::Filter, items::Item};

pub enum Event {
    None,
    Craft { input: Vec<Item>, output: Vec<Item> },
    Pull { count: u64, filter: Filter },
}
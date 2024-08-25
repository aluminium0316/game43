use crate::blocks::block::Side;

use super::{filter::Filter, items::{Item, ItemId}};

#[derive(Default)]
pub struct Inventory {
    pub input: Vec<Item>,
    pub output: Vec<Item>,
}

pub trait Storage {
    fn insert(&mut self, items: Vec<Item>);
    fn extract(&mut self, count: u64, filter: Filter, side: Side) -> Vec<Item>;
    fn craft(&mut self, items: Vec<Item>, output: Vec<Item>);
}

impl Inventory {
    pub fn insert(&mut self, items: Vec<Item>) {
        for item in &items {
            if let Some((item1, _)) = Inventory::find(&mut self.input, item.id.clone()) {
                item1.count += item.count;
            }
            else {
                self.input.push(item.clone());
            }
        }
    }
    pub fn extract(&mut self, mut count: u64, filter: Filter) -> Vec<Item> {
        let mut filtered = filter.filter(self.output.clone());
        for item in &mut filtered {
            if let Some((output_item, _)) = Inventory::find(&mut self.output, item.id.clone()) {
                if output_item.count < count {
                    count -= output_item.count;
                    item.count = output_item.count; 
                    output_item.count = 0;
                }
                else {
                    output_item.count -= count;
                    item.count = count;
                    count = 0;
                }
            }
        }
        self.output = self.output.iter().filter(|x| x.count != 0).cloned().collect();
        filtered
    }
    pub fn craft(&mut self, items: Vec<Item>, output: Vec<Item>) {
        for item in &items {
            if let Some((item1, index)) = Inventory::find(&mut self.input, item.id.clone()) {
                item1.count -= item.count;
                if item1.count == 0 {
                    self.input.remove(index);
                }
            }
        }
        for item in &output {
            if let Some((item1, _)) = Inventory::find(&mut self.output, item.id.clone()) {
                item1.count += item.count;
            }
            else {
                self.output.push(item.clone());
            }
        }
    }
    pub fn view(&self) -> String {
        format!("input: {:?}\noutput: {:?}", self.input, self.output)
    }
    pub fn find<'a>(io: &'a mut Vec<Item>, id: ItemId) -> Option<(&'a mut Item, usize)> {
        let mut i = 0;
        for item in io.iter_mut() {
            if item.id == id {
                return Some((item, i));
            }
            i += 1;
        }
        None
    }
}
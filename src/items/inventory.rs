use super::{filter::Filter, items::Item};

#[derive(Default)]
pub struct Inventory {
    input: Vec<Item>,
    output: Vec<Item>,
    pulling: Vec<Item>
}

pub trait Storage {
    fn insert(&mut self, items: Vec<Item>);
    fn extract(&mut self, count: u64, filter: Filter) -> Vec<Item>;
}

impl Inventory {
    pub fn insert(&mut self, mut items: Vec<Item>) {
        self.input.append(&mut items);
    }
    pub fn extract(&mut self, mut count: u64, filter: Filter) -> Vec<Item> {
        let mut filtered = filter.filter(self.output.clone());
        for item in &mut filtered {
            for output_item in &mut self.output {
                if output_item.id == item.id {
                    if output_item.count < count {
                        count -= output_item.count;
                        item.count = output_item.count; 
                        output_item.count = 0;
                    }
                    else {
                        output_item.count -= count;
                        count = 0;
                        item.count = 0;
                    }
                    break;
                }
            }
        }
        self.output = self.output.iter().filter(|x| x.count != 0).cloned().collect();
        filtered
    }
}
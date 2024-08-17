use super::items::Item;

struct Inventory {
    items: Vec<Item>,
}

trait Storage {
    fn insert(items: Vec<Item>);
    fn extract(count: u64) -> Vec<Item>;
}
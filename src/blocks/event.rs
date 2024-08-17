use crate::items::items::Item;

enum Event {
    Craft,
    Push(Vec<Item>),
    Pull(Vec<Item>)
}
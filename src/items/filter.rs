use super::items::Item;

#[derive(Clone)]
pub struct Filter {
    f: Vec<Vec<String>>,
}

impl Filter {
    pub fn new(filter: &str) -> Self {
        let or_filter = filter.split('|');
        let and_filter = or_filter
            .map(|x| x
                .trim()
                .split('&')
                .map(|x| x.to_owned())
                .collect())
            .collect();
        Self {
            f: and_filter,
        }
    }

    pub fn filter(&self, items: Vec<Item>) -> Vec<Item> {
        items
    }
}
use std::collections::HashMap;

pub struct Counter<T>{
    items: Vec<T>,

}

impl<T:std::cmp::PartialEq + std::cmp::Eq + std::hash::Hash> Counter<T>{
    /// Create a new counter
    ///
    pub fn new() -> Counter<T>{
        //! Create a new counter
        //! ```
        //! use nano_collection::Counter;
        //! let counter = Counter::new();
        //! assert_eq!(counter.count(), 0);
        //! ```
        Counter{
            items: Vec::new(),
        }
    }
    pub fn add(&mut self, item: T){
        //! Add an item to the counter
        //! ```
        //! use nano_collection::Counter;
        //! let mut counter = Counter::new();
        //! counter.add(1);
        //! assert_eq!(counter.count(), 1);
        //! ```
        self.items.push(item);
    }
    pub fn remove_item(&mut self, item: T){
        //! Remove an item from the counter
        //! ```
        //! use nano_collection::Counter;
        //! let mut counter = Counter::new();
        //! counter.add(1);
        //! counter.add(2);
        //! counter.remove(1);
        //! assert_eq!(counter.count(), 1);
        //! ```
        let index = self.items.iter().position(|x| *x == item);
        match index {
            Some(i) => {
                self.items.remove(i);
            },
            None => {},
        }
    }

    pub fn count(&self) -> usize{
        //! Count the number of items in the counter
        //! ```
        //! use nano_collection::Counter;
        //! let mut counter = Counter::new();
        //! counter.add(1);
        //! counter.add(2);
        //! assert_eq!(counter.count(), 2);
        //! ```
        self.items.len()
    }
    pub fn count_item(&self, item: T) -> usize{
        //! Count the number of a specific item in the counter
        //! ```
        //! use nano_collection::Counter;
        //! let mut counter = Counter::new();
        //! counter.add(1);
        //! counter.add(2);
        //! assert_eq!(counter.count_item(1), 1);
        //! ```
        self.items.iter().filter(|x| **x == item).count()
    }
    pub fn count_most_common(&self) -> Option<&T>{
        //! Count the most common item in the counter
        //! ```
        //! use nano_collection::Counter;
        //! let mut counter = Counter::new();
        //! counter.add(1);
        //! counter.add(2);
        //! counter.add(2);
        //! assert_eq!(counter.count_most_common(), Some(&2));
        let mut map = HashMap::new();
        for item in &self.items{
            let count = map.entry(item).or_insert(0);
            *count += 1;
        }
        let mut max = 0;
        let mut max_item = None;
        for (item, count) in map{
            if count > max{
                max = count;
                max_item = Some(item);
            }
        }
        max_item
    }

}

#[macro_export]
macro_rules! counter {
    ($($x:expr),*) => {
        {
            let mut temp_counter = Counter::new();
            $(
                temp_counter.add($x);
            )*
            temp_counter
        }
    };
}
#[test]
fn test_macro(){
    let counter = counter![1,2,3,3];
    assert_eq!(counter.count(), 4);
}
#[test]
fn test_macro2(){
    let counter = counter![1,2,3,3];
    assert_eq!(counter.count_item(3), 2);
}

#[test]
fn test_count_no_item(){
    let counter = counter![1,2,3,3];
    assert_eq!(counter.count_item(4), 0);
}

#[test]
fn test_create_counter(){
    let mut counter = Counter::new();
    counter.add(1);
    counter.add(2);
    counter.add(3);
    assert_eq!(counter.count(), 3);
}
#[test]
fn test_remove_item(){
    let mut counter = Counter::new();
    counter.add(1);
    counter.add(2);
    counter.add(3);
    counter.remove_item(2);
    assert_eq!(counter.count(), 2);
}
#[test]
fn test_count_item(){
    let mut counter = Counter::new();
    counter.add(1);
    counter.add(2);
    counter.add(3);
    counter.add(3);
    assert_eq!(counter.count_item(3), 2);
}

#[test]
fn test_count_most_common(){
    let mut counter = Counter::new();
    counter.add(1);
    counter.add(2);
    counter.add(3);
    counter.add(3);
    assert_eq!(counter.count_most_common(), Some(&3));
}

#[test]
#[should_panic]
fn test_remove_item_panic(){
    let mut counter = Counter::new();
    counter.add(1);
    counter.add(2);
    counter.add(3);
    counter.remove_item(4);
    assert_eq!(counter.count(), 2);
}

#[test]
#[should_panic]
fn test_count_item_panic(){
    let mut counter = Counter::new();
    counter.add(1);
    counter.add(2);
    counter.add(3);
    counter.add(3);
    assert_eq!(counter.count_item(4), 2);
}
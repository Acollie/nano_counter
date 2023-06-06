use std::collections::HashMap;

pub struct Counter<T>{
    items: Vec<T>,

}

impl<T:std::cmp::PartialEq + std::cmp::Eq + std::hash::Hash> Counter<T>{
    fn new() -> Counter<T>{
        Counter{
            items: Vec::new(),
        }
    }
    fn add(&mut self, item: T){
        self.items.push(item);
    }
    fn remove_item(&mut self, item: T){
        let index = self.items.iter().position(|x| *x == item);
        match index {
            Some(i) => {
                self.items.remove(i);
            },
            None => {},
        }
    }

    fn count(&self) -> usize{
        self.items.len()
    }
    fn count_item(&self, item: T) -> usize{
        self.items.iter().filter(|x| **x == item).count()
    }
    fn count_most_common(&self) -> Option<&T>{
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
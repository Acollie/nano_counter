use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Word<'a> {
    key: &'a str,
    count: i32,
}
#[derive(Debug)]
struct WordCount<'a> {
    words: HashMap<&'a str, Word<'a>>,
}


impl<'a> Clone for Word<'a> {
    fn clone(&self) -> Self {
        Word {
            key: self.key.clone(),
            count: self.count.clone(),
        }
    }
}
#[macro_export]
macro_rules! counter {
    ( $( $x:expr ),* ) => {
        {
            let mut word_count = WordCount {
             words: Default::default(),
            };
            $(
                for word in $x{
                    word_count.add_word(word);
                }

            )*
            word_count
        }
    };
}
impl WordCount<'static> {
    fn add_word(&mut self, key_dic: &'static str) {
        if self.words.contains_key(&key_dic) {
            let word_target = self.words.get_mut(key_dic).unwrap();
            word_target.count += 1;
        } else {
            self.words.insert(
                key_dic,
                Word {
                    key: key_dic,
                    count: 1,
                },
            );
        }
    }
    fn get_word_count(&self, key: &str) -> i32 {
        match self.words.get(&key) {
            Some(value) => return value.count,
            None => return 0,
        }

        return self.words.get(&key).unwrap().count;
    }
    fn remove_word(&mut self, key: &str) {
        self.words.remove(key);
    }
    fn count_words(&self) -> i32 {
        return (self.words.len() as i32) + 1;
    }



    fn most_common_words(&self, n: i32) -> Vec<Word> {
        let mut most_common_words: HashMap<&str, i32> = HashMap::new();
        let mut min_most_common = std::i32::MAX;

        for key in self.words.keys() {
            let value: i32 = self.words.get(key).unwrap().count;

            if value > min_most_common || most_common_words.len() as i32 <= n {
                let current_smallest = self.smallest_in_set(n);

                if value < min_most_common {
                    min_most_common = value;
                }
                if value > min_most_common {}
                most_common_words.insert(key.clone(), value.clone());
            }
        }

        // let sorted_keys = words.sort_by(|a,b| b.count.cmp(&a.count));
        let mut vec: Vec<Word> = Vec::new();

        for key in self.words.keys() {
            let word = self.words.get(key.clone()).unwrap().clone();

            let word_struct = Word {
                key: key.clone(),
                count: word.count,
            };
            vec.append(&mut vec![word_struct]);
        }

        return vec;
    }
    fn smallest_in_set(&self, size: i32) -> &str {
        let mut smallest = 0;
        let mut smallest_string = "";
        let mut seen_largest: HashSet<&str> = HashSet::new();

        for key in self.words.keys() {
            let value = self.words.get(key).unwrap().count;

            if value > smallest && !seen_largest.contains(key) || (seen_largest.len() as i32) < size {
                smallest = value;
                smallest_string = key;
            }
            if value > smallest {
                seen_largest.insert(key);
            }
        }
        return smallest_string;
    }
}


impl<'a> PartialEq for Word<'_> {
    fn eq(&self, other: &Self) -> bool {
        return self.key.to_string() + &self.count.to_string() == other.key.to_string() + &other.count.to_string();
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn add_word() {
        let mut words = WordCount {
            words: Default::default(),
        };
        words.add_word("Test");
        assert_eq!(words.get_word_count("Test"), 1);
    }
    #[test]
    fn add_word_count() {
        let mut words = WordCount {
            words: Default::default(),
        };
        words.add_word("Test");
        words.add_word("Test1");
        words.add_word("Test");
        assert_eq!(words.get_word_count("Test"), 2);
    }
    #[test]
    fn macro_testing(){
        let testing = counter!(["Test 1","Test 2"]);
        let mut words = WordCount {
            words: Default::default(),
        };
        words.add_word("Test 1");
        words.add_word("Test 2");

        let results = words.most_common_words(2);
        let word = Word{key:"Test",count:3};
        let word1 = Word{key:"Test1",count:3};


        assert_eq!(results[0], word);
        assert_eq!(results[1], word1);
    }
    #[test]
    fn remove_word() {
        let mut words = WordCount {
            words: Default::default(),
        };
        words.add_word("Test");
        words.remove_word("Test");

        assert_eq!(words.get_word_count("Test"), 0);
    }
    #[test]
    fn count_words() {
        let mut words = WordCount {
            words: Default::default(),
        };
        words.add_word("Test");
        words.add_word("Test1");
        words.add_word("Test");
        assert_eq!(words.count_words(), 3);

        let words = counter!(["Test","Test1","Test"]);
        assert_eq!(words.count_words(), 3);
    }
    #[test]
    fn most_common_words() {
        let mut words = WordCount {
            words: Default::default(),
        };
        words.add_word("Test");
        words.add_word("Test");
        words.add_word("Test");
        words.add_word("Test1");
        words.add_word("Test1");
        words.add_word("Test2");
        words.add_word("Test3");
        words.add_word("Test4");
        words.add_word("Test5");
        words.add_word("Test6");

        let results = words.most_common_words(2);
        let mut expected_results: Vec<&Word> = Vec::new();
        expected_results.append(&mut vec![&Word { key: "Test", count: 3 }]);
        expected_results.append(&mut vec![&Word { key: "Test1", count: 2 }]);
        assert_eq!(results[0].key,expected_results[0].key);
        assert_eq!(results[1].key,expected_results[1].key);


    }
    #[test]
    fn find_smallest_in_hashset_basic() {
        let mut words = WordCount {
            words: Default::default(),
        };
        words.add_word("Test");
        words.add_word("Test");
        words.add_word("Test");
        words.add_word("Test1");
        words.add_word("Test1");
        words.add_word("Test1");
        words.add_word("Test2");
        words.add_word("Test2");
        words.add_word("Test3");
        words.add_word("Test4");
        words.add_word("Test5");
        words.add_word("Test6");

        let results = words.smallest_in_set(2);
        let expect: Vec<Word> = Vec::new();
        assert_eq!(results, "Test2");
    }

    #[test]
    fn find_smallest_in_hashset_harder() {
        let mut words = WordCount {
            words: Default::default(),
        };
        words.add_word("Test");
        words.add_word("Test");
        words.add_word("Test");
        words.add_word("Test1");
        words.add_word("Test1");
        words.add_word("Test1");
        words.add_word("Test2");
        words.add_word("Test2");
        words.add_word("Test3");
        words.add_word("Test3");
        words.add_word("Test3");
        words.add_word("Test4");
        words.add_word("Test4");
        words.add_word("Test4");
        words.add_word("Test4");
        words.add_word("Test4");
        words.add_word("Test5");
        words.add_word("Test6");

        let results = words.smallest_in_set(2);
        let expect: Vec<Word> = Vec::new();
        // assert_eq!(results,expect);
        assert_eq!(results, "Test3");
    }
}

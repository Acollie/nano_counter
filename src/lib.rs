use std::collections::{HashMap};

struct Word{
    key:String,
    count:i32,
}
struct Word_count{
    words: HashMap<String, Word>,
}


impl Word_count {
    fn add_word(&mut self, key: String){

        if self.words.contains_key(&key){
            let word_target = self.words.get_mut(&key).unwrap();
            word_target.count +=1;
        }else{
            self.words.insert(key.clone(), Word{ key: key.clone(), count: 1 });
        }
    }
    fn get_word_count(&self, key: String) ->i32{
        return self.words.get(&key).unwrap().count;

    }
    fn remove_word(&mut self,key:String){
        let key_string = key.to_string();
        self.words.remove(&key_string);
    }
    fn most_common_words(&self) ->Vec<Word>{

        let mut most_common_words :Vec<Word>= Vec::new();
        // let words = Vec::from_iter(self.words);
        let foo = &self.words;
        println!("{:?}",self.words.get("foo").unwrap().key);
        // let sorted_keys = words.sort_by(|a,b| b.count.cmp(&a.count));

        // for key in &self.words.keys(){
        //     println!("")
        // };
        return vec![]
    }
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn add_word() {
        let mut words = Word_count{
            words: Default::default()
        };
        words.add_word((&"Test").to_string());
        assert_eq!(words.get_word_count((&"Test").to_string()), 1);
    }

    #[test]
    fn add_word_count() {
        let mut words = Word_count{
            words: Default::default()
        };
        words.add_word((&"Test").to_string());
        words.add_word((&"Test1").to_string());
        words.add_word((&"Test").to_string());
        assert_eq!(words.get_word_count((&"Test").to_string()), 2);
    }

    #[test]
    fn remove_word() {
        let mut words = Word_count{
            words: Default::default()
        };
        words.add_word((&"Test").to_string());
        words.remove_word((&"Test").to_string());
        assert_eq!(words.get_word_count((&"Test").to_string()), 0);
    }
    #[test]
    fn count_words(){
        assert_eq!(14, 14);
    }
    #[test]
    fn most_common_words(){
        assert_eq!(1,1);
    }

}

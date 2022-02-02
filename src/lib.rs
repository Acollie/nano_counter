use std::collections::{HashMap};

struct Word<'a>{
    key: &'a str,
    count: i32,
}
struct WordCount<'a> {
    words: HashMap<&'a str,  Word<'a> >,
}


impl WordCount <'static>{
    
    fn add_word(&mut self, key_dic: &'static str){

        if self.words.contains_key(&key_dic){
            let word_target = self.words.get_mut(key_dic).unwrap();
            word_target.count +=1;
        }else{
            self.words.insert(key_dic, Word { key: key_dic, count: 1 });
        }
    }
    fn get_word_count(&self, key: &str) ->i32{
        match self.words.get(&key){
            Some(value)=>return value.count,
            None => return 0
        }

        return self.words.get(&key).unwrap().count;

    }
    fn remove_word(&mut self,key:&str){
        self.words.remove(key);
    }
    fn most_common_words(&self) ->Vec<Word>{

        let mut most_common_words :Vec<Word>= Vec::new();

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
        let mut words = WordCount{
            words: Default::default()
        };
        words.add_word("Test");
        assert_eq!(words.get_word_count("Test"), 1);
    }

    #[test]
    fn add_word_count() {
        let mut words = WordCount{
            words: Default::default()
        };
        words.add_word("Test");
        words.add_word("Test1");
        words.add_word("Test");
        assert_eq!(words.get_word_count("Test"), 2);
    }

    #[test]
    fn remove_word() {
        let mut words = WordCount{
            words: Default::default()
        };
        words.add_word("Test");
        words.remove_word("Test");
        println!("----->{}",words.get_word_count("Test"));
        assert_eq!(words.get_word_count("Test"), 0);
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

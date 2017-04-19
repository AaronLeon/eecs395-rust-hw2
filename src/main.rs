use std::env;
use std::fs::File;
use std::io::{stdin, Read};
use std::collections::HashMap;

mod trie;

fn main() {
    let alphabet:Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    let words = process_input(read_train_file());
    let t = build_trie(count_words(words));

    let user_input = read_input(stdin());

    for s in &user_input {
        spell_check(&s.to_lowercase(), &t)
    }

    println!("user input: {:?}", user_input);
}

fn spell_check(input:&str, trie:&trie::Trie<char, String>) {
    let mut input_:Vec<char> = input.chars().collect();

    if let Some(result) = trie.fetch(&input_) {
        println!("{}", input);
    }

    for i in 0..input_.len() {
        /*
         *  call check_* on index i. this will return intermediate string 'temp' with one edit. check if 
         *  temp is in trie. if it is, then print it :) otherwise loop again over all indices of temp and 
         *  call check_* again. check again if the results are in trie.
         */
    }
}

fn check_insert(input:&str, trie:&trie::Trie<char, String>, index:usize) -> String {
    "not implemented yet".to_string()
}

fn check_delete(input:&str, trie:&trie::Trie<char, String>, index:usize) -> String {
    "not implemented yet".to_string()
}

fn check_replace(input:&str, trie:&trie::Trie<char, String>, index:usize) -> String {
    "not implemented yet".to_string()
}

fn check_swap(input:&str, trie:&trie::Trie<char, String>, index:usize) -> String {
    "not implemented yet".to_string()
}

fn read_input<R: Read>(mut reader: R) -> Vec<String> {
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).ok();
    
    let result:Vec<String> = buffer.split_whitespace()
        .map(|s| s.to_string())
        .collect();

    result
}

fn read_train_file() -> String {
    let file_name = env::args().nth(2).unwrap();
    let mut f = File::open(file_name).unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).ok();
    
    buffer
}

fn process_input(input: String) -> Vec<String> {
    input.split_whitespace()
        .filter(|word| !word.is_empty())
        .map(|word| word.to_lowercase())
        .collect()
}

fn count_words(words: Vec<String>) -> HashMap<String, usize> {
    let mut counts:HashMap<String, usize> = HashMap::new();
    for word in words.iter() {
        if counts.contains_key(word) {
            *counts.get_mut(word).unwrap() += 1;
        } else {
            counts.insert(word.to_string(), 1);
        }
    }

    counts
}

fn build_trie(dictionary:HashMap<String, usize>) -> trie::Trie<char, String> {
    let mut t:trie::Trie<char, String> = trie::Trie::new();
    
    for (k, v) in dictionary {
        t.insert(k.chars().collect(), k) 
    }

    t
}

fn print_results(results:HashMap<String, usize>) {
    let mut temp: Vec<_> = results.iter().collect();

    temp.sort_by(|a, b| a.1.cmp(b.1));
    for pair in temp.iter() {
        println!("{}: {}", pair.0, pair.1);
    }
}

//#[cfg(test)]
//mod mean_and_sum_tests {
    //use super::{sum, mean, calculate_results};

    //#[test]
    //fn sum_empty_is_0() {
        //assert_eq!(0., sum(&[]));
    //}
//}


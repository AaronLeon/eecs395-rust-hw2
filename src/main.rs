use std::env;
use std::fs::File;
use std::io::{stdin, Read};
use std::collections::{HashMap, HashSet};

//mod trie;

const ALPHABET: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
fn main() {
    let words = process_input(read_train_file());
    let dictionary = count_words(words);

    let user_input = read_input(stdin());

    for s in &user_input {
        spell_check(&s.to_lowercase(), &dictionary)
    }

    // println!("user input: {:?}", user_input);
}

fn spell_check(input:&str, dictionary:&HashMap<String, usize>) {

    if let Some(result) = dictionary.get(input) {
        return println!("These words were found in the dictionary, {}", input);
        // return;
    }

    let mut candidate_words:HashSet<String> = HashSet::new();
    candidate_words.insert(input.to_string());

    check_edits(&mut candidate_words);
    check_edits(&mut candidate_words);

    let correct_word:String = find_probable_match(dictionary, candidate_words);

    if correct_word.is_empty() {
        println!("{}, -", input);
    } else {
        println!("Your spelling: {}, correct spelling: {}", input, correct_word);
    }
}

fn find_probable_match(dictionary: &HashMap<String, usize>, candidate_words:HashSet<String>) -> String {
    let mut best_word:String = String::new();
    let mut best_count:usize = 0;
    for word in candidate_words {
        if let Some(count) = dictionary.get(&word) {
            if *count > best_count {
                best_count = *count;
                best_word= word;
            }
        }
    }

    best_word
}

fn check_edits(words:&mut HashSet<String>){
    let words_so_far:HashSet<String> = words.clone();

    for word in words_so_far {
        for i in 0..word.len() {
            check_insert(&word, words,i);
            check_delete(&word, words,i);
            check_replace(&word, words,i);
            check_transpose(&word, words,i);
        }
    }
}

fn check_insert(input:&str, words:&mut HashSet<String>, index:usize){
    // let mut res:HashSet<String> = words.clone();

    for letter in ALPHABET.into_iter() {
        let prefix = &input[..index];
        let suffix = &input[index..];

        let word:String = format!("{}{}{}", prefix, letter, suffix);

        words.insert(word);
    }
}

fn check_delete(input:&str, words:&mut HashSet<String>, index:usize) {
    let mut word:String = "".to_string();
    let input_string: String = input.to_string();
    // let left: String = (input[..index]).to_owned();
    for (pos, c) in input_string.chars().enumerate() {
        if pos == index{
            continue;
        }
        else{ 
            word.push(c)
        }
    }
    words.insert(word.to_owned());
    // let word:String = left + &input[index+1..];
    // words.insert(word);
}

fn check_replace(input:&str, words:&mut HashSet<String>, index:usize) {
    // let mut word = "";
    let mut temp_word:String = "".to_owned();
    let input_string: String = input.to_string();
    for letter in ALPHABET.into_iter() {
        if input.len() < 1 {
            // word = letter.to_string();
            continue;
        }
        else {
            for (pos, c) in input_string.chars().enumerate() {
                if pos == index{
                    // let repl: char = *letter;
                    // let temp_word:String = format!("{}{}", word, repl);
                    // word = temp_word.as_str();
                    temp_word.push(*letter);
                }
                else{ 
                    // let temp_word:String = format!("{}{}", word,c);
                    // word = temp_word.as_str();
                    temp_word.push(c);
                }
            }
        }

         if temp_word.len() > 0{
            words.insert(temp_word.clone());
        }     
    }
}

fn check_transpose(input:&str, words:&mut HashSet<String>, index:usize) {
	// let mut word = "";
    let mut temp_word:String = "".to_owned();
    let input_string:String = input.to_string();
    if input_string.len() <= 1{
        words.insert(input_string);    
    }
    else{
        for (pos, c) in input_string.chars().enumerate() {
            if pos == index {
                if index+1 < input_string.len(){
                    let next: char = input_string.chars().nth(index+1).unwrap();
                    // temp_word = format!("{}{}{}", word, next, c);
                    // temp_word.push_str(word);
                    temp_word.push(next);
                    temp_word.push(c);
                    
                }
            }
            else{ 
                // temp_word.push_str(word);
                temp_word.push(c);    
                // let temp_word:String = word.to_string();
                // temp_word.push(c);
                // let temp_word:String = format!("{}{}", word,c);
                // word = temp_word.as_str();
            }
        }
    }
    if temp_word.len() > 0{
            words.insert(temp_word.clone());
    }  
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
    let file_name = env::args().nth(1).unwrap();
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

    // counts.insert("checkcheckcheck".to_string(), 1);
    counts
}

//#[cfg(test)]
//mod mean_and_sum_tests {
    //use super::{sum, mean, calculate_results};

    //#[test]
    //fn sum_empty_is_0() {
        //assert_eq!(0., sum(&[]));
    //}
//}


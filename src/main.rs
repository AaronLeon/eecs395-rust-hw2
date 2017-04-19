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

    println!("user input: {:?}", user_input);
}

fn spell_check(input:&str, dictionary:&HashMap<String, usize>) {

    if let Some(result) = dictionary.get(input) {
        return println!("{}", input);
    }

    let mut candidate_words:HashSet<String> = HashSet::new();
    candidate_words.insert(input.to_string());

    check_edits(&mut candidate_words);
    check_edits(&mut candidate_words);

    let correct_word:String = find_probable_match(dictionary, candidate_words);

    if correct_word.is_empty() {
        println!("{}, -", input);
    } else {
        println!("{}, {}", input, correct_word);
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
    let mut words_so_far:HashSet<String> = words.clone();

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
        let mut prefix = &input[..index];
        let suffix = &input[index..];

        let word:String = format!("{}{}{}", prefix, letter, suffix);

        words.insert(word);
    }
}

fn check_delete(input:&str, words:&mut HashSet<String>, index:usize) {
	// let input_str: String = input.to_string();
    let left: String = (input[..index]).to_owned();

    let word:String = left + &input[index+1..];
    words.insert(word);
}

fn check_replace(input:&str, words:&mut HashSet<String>, index:usize) {
    let mut word:String = String::new();

    for letter in ALPHABET.into_iter() {
        if input.len() < 1 {
            word = letter.to_string();
            continue;
        }
        else {
            let mut prefix = &input[..index];
            let suffix = &input[index-1..];

            word = format!("{}{}{}", prefix, letter, suffix);
        }

        words.insert(word.to_owned());
    }
}

fn check_transpose(input:&str, words:&mut HashSet<String>, index:usize) {
	// if index < input.to_owned().len(){
        
    // }
    let pre_i:String = (input[..index]).to_owned();
    let post_i1: String = (input[index..]).to_owned();
	let mut swapped_string: String =  pre_i;
    swapped_string+= &(input.to_owned().chars().nth(index+1).unwrap().to_string());
	swapped_string+= &(input.to_owned().chars().nth(index).unwrap().to_string());
	swapped_string+= &(post_i1);
	words.insert(swapped_string);

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


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

    let candidate_words:HashSet<String> = HashSet::new();
    candidate_words.insert(input.to_string());

    candidate_words = check_edits(candidate_words, dictionary);
    candidate_words = check_edits(candidate_words, dictionary);

    let correct_word:String = find_probable_match(dictionary, candidate_words);

    if correct_word.is_empty() {
        println!("{}, -", input);
    } else {
        println!("{}, {}", input, correct_word);
    }
}

fn find_probable_match(dictionary: &HashMap<String, usize>, candidate_words:HashSet<String>) -> String {
    let best_word:String = String::new();
    let best_count:usize = 0;
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

fn check_edits(words:HashSet<String>, dictionary:&HashMap<String, usize>) -> HashSet<String> {
    let mut words_so_far:HashSet<String> = HashSet::new();

    for word in words {
        for i in 0..word.len() {
            words_so_far = check_insert(&word, &words_so_far, dictionary, i);
            words_so_far = check_delete(&word, &words_so_far, dictionary, i);
            words_so_far = check_replace(&word, &words_so_far, dictionary, i);
            words_so_far = check_transpose(&word, &words_so_far, dictionary, i);
        }
    }

    words_so_far
}

fn check_insert(input:&str, words:&HashSet<String>, dictionary:&HashMap<String, usize>, index:usize) -> HashSet<String> {
    let mut res:HashSet<String> = *words;

    for letter in ALPHABET.into_iter() {
        let mut prefix = &input[..index];
        let suffix = &input[index..];

        let word:String = format!("{}{}{}", prefix, letter, suffix);

        res.insert(word);
    }

    res
}

fn check_delete(input:&str, words:&HashSet<String>, dictionary:&HashMap<String, usize>, index:usize) -> HashSet<String> {
	let word:String = input[0..index] + input[index+1..];
    words.insert(word);

	words
}

fn check_replace(input:&str, words:&HashSet<String>, dictionary:&HashMap<String, usize>, index:usize) -> HashSet<String> {
    let mut res:HashSet<String> = *words;
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

        res.insert(word.to_owned());
    }

    res
}

//fn check_transpose(input:&str, words:&HashSet<String>, dictionary:&HashMap<String, usize>, index:usize) -> HashSet<String> {
	//let transpoed:String = 
	//let swapped_string: String =  ((input.to_owned())[0..index]).to_string() +
		//(input.to_owned()).chars().nth(index+1).unwrap().to_string()+
		//(input.to_owned()).chars().nth(index).unwrap().to_string() + 
		//((input.to_owned())[index+2..]).to_string();
	//words.insert(swapped_string);

    //words
//}

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

//#[cfg(test)]
//mod mean_and_sum_tests {
    //use super::{sum, mean, calculate_results};

    //#[test]
    //fn sum_empty_is_0() {
        //assert_eq!(0., sum(&[]));
    //}
//}


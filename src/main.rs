use std::env;
use std::fs::File;
use std::io::{stdin, Read, stdout, Write};
use std::collections::{HashMap, HashSet};

const ALPHABET: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
fn main() {
    let words = process_input(read_train_file());
    let dictionary = count_words(words);

    let user_input = read_input(stdin());

    for s in &user_input {
        spell_check(stdout(), &s.to_lowercase(), &dictionary, 2)
    }
}


fn spell_check<W:Write>(mut writer:W, input:&str, dictionary:&HashMap<String, usize>, edit_distance:usize) {
    if dictionary.contains_key(input) {
        return write!(writer, "{}", input).unwrap();
    }

    let mut candidate_words:HashSet<String> = HashSet::new();
    candidate_words.insert(input.to_string());

    for _ in 0..edit_distance {
        check_edits(&mut candidate_words);
    }

    let correct_word:String = find_probable_match(dictionary, candidate_words);
    // if input is incorrectly spelt
    if correct_word.is_empty() {
        write!(writer, "{}, -", input).unwrap();
    } else {
        write!(writer, "{}, {}", input, correct_word).unwrap();
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
    
    for (pos, c) in input_string.chars().enumerate() {
        if pos == index{
            continue;
        }
        else{ 
            word.push(c)
        }
    }
    words.insert(word.to_owned());
}

fn check_replace(input:&str, words:&mut HashSet<String>, index:usize) {
    let prefix = input[..index].to_string();
    let suffix = input[index + 1..].to_string();
    for letter in ALPHABET.into_iter() {
        let temp = format!("{}{}{}", prefix, letter.to_string(), suffix);
        words.insert(temp);
    }
}

fn check_transpose(input:&str, words:&mut HashSet<String>, index:usize) {
    if index >= input.len()-1 {
        return 
    }

    let prefix = input[..index].to_string();
    let suffix = input[index+2..].to_string();

    let curr:char = input.chars().nth(index).unwrap();
    let next:char = input.chars().nth(index+1).unwrap();
    let temp = format!("{}{}{}{}", prefix, next, curr, suffix);

    words.insert(temp);
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
    input.split(|c:char| !c.is_alphanumeric())
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

#[cfg(test)]
mod spell_check_tests {
    use super::{spell_check, check_insert, check_delete, check_replace, check_transpose};
    use std::io::Cursor;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn generates_correct_insertions() {
        let mut words:HashSet<String> = HashSet::new();
        check_insert("alice", &mut words, 1);
        assert!(words.contains("ablice"));
        assert!(words.contains("azlice"));
    }

    #[test]
    fn generates_correct_deletions() {
        let mut words:HashSet<String> = HashSet::new();
        check_delete("alice", &mut words, 1);
        assert!(words.contains("aice"));
    }

    #[test]
    fn generates_correct_deletions_on_edge() {
        let mut words:HashSet<String> = HashSet::new();
        check_delete("alice", &mut words, 0);
        assert!(words.contains("lice"));
        check_delete("alice", &mut words, 4);
        assert!(words.contains("alic"));
    }

    #[test]
    fn generates_correct_replacement() {
        let mut words:HashSet<String> = HashSet::new();
        check_replace("alice", &mut words, 1);
        assert!(words.contains("ajice"));
    }

    #[test]
    fn generates_correct_replacement_on_edge() {
        let mut words:HashSet<String> = HashSet::new();
        check_replace("alice", &mut words, 0);
        assert!(words.contains("dlice"));
        check_replace("alice", &mut words, 4);
        assert!(words.contains("alicx"));
    }

    #[test]
    fn generates_correct_transpositions() {
        let mut words:HashSet<String> = HashSet::new();
        check_transpose("alice", &mut words, 1);
        assert!(words.contains("ailce"));
    }

    #[test]
    fn generates_correct_transpositions_on_edge() {
        let mut words:HashSet<String> = HashSet::new();
        check_transpose("alice", &mut words, 0);
        assert!(words.contains("laice"));
        check_transpose("alice", &mut words, 3);
        assert!(words.contains("aliec"));
        check_transpose("alice", &mut words, 4);
        assert!(!words.contains("alice"));
    }

    //#[test]
    //fn picks_most_probable_word() {
        //let mut dict:HashMap<String, usize> = HashMap::new();
        //let mut words:HashSet<String> = HashSet::new();


        //check_transpose("alice", &mut words, 1);
        //for word in &words {
            //println!("{}", word);
        //}
        //assert!(words.contains("ailce"));
    //}

    #[test]
    fn corrects_typo_with_edit_distance_1() {
        let mut dict:HashMap<String, usize> = HashMap::new();
        dict.insert("banana".to_string(), 2);
        dict.insert("alice".to_string(), 4);
        dict.insert("blimps".to_string(), 3);
        dict.insert("word".to_string(), 6);

        assert_write("blice, alice", "blice", &dict);
    }

    #[test]
    fn corrects_typo_with_edit_distance_2() {
        let mut dict:HashMap<String, usize> = HashMap::new();
        dict.insert("banana".to_string(), 2);
        dict.insert("alice".to_string(), 4);
        dict.insert("blimps".to_string(), 3);
        dict.insert("word".to_string(), 6);
        assert_write( "blce, alice", "blce", &dict);
    }

    #[test]
    fn no_correction_found() {
        let mut dict:HashMap<String, usize> = HashMap::new();
        dict.insert("banana".to_string(), 2);
        dict.insert("alice".to_string(), 4);
        dict.insert("blimps".to_string(), 3);
        dict.insert("word".to_string(), 6);
        assert_write( "zzkjcbjbfkasbjasbajkda, -", "zzkjcbjbfkasbjasbajkda", &dict);
    }

    #[test]
    fn input_is_correct() {
        let mut dict:HashMap<String, usize> = HashMap::new();
        dict.insert("banana".to_string(), 2);
        dict.insert("alice".to_string(), 4);
        dict.insert("blimps".to_string(), 3);
        dict.insert("word".to_string(), 6);
        assert_write( "alice", "alice", &dict);
    }

    fn assert_write(expected: &str, input: &str, dictionary:&HashMap<String, usize>) {
        let mut writer = Cursor::new(vec![]);
        spell_check(&mut writer, input, dictionary, 2);
        assert_eq!(expected.as_bytes(), &*writer.into_inner())
    }
}

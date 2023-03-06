use spelling_corrector::corrector;
use trie_rs::{Trie, TrieBuilder};

pub fn make_builder(contents: String) -> TrieBuilder<u8> {
    let mut builder = TrieBuilder::new();

    for word in contents.split("\n") {
        builder.push(word);
    }

    builder
}

pub fn find_longest_word(words: Vec<String>) -> String {
    let mut word = "".to_string();
    for item in words {
        if item > word.clone() {
            word = item;
        }
    }

    word
}

pub fn results(trie: &Trie<u8>, input_string: &String) -> Vec<String> {
    let results_in_u8s: Vec<Vec<u8>> = trie.common_prefix_search(&input_string);
    let results_in_str: Vec<String> = results_in_u8s
        .iter()
        .map(|u8s| String::from_utf8(u8s.to_vec()).unwrap())
        .collect();

    results_in_str
}

fn remove_first(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();

    chars.as_str()
}

//fn correct_words(matches: Vec<String>, wrong_index: usize) {
//    let correct = corrector::SimpleCorrector::default();
//
//    let mut word = format!("{}{}", matches[wrong_index], matches[wrong_index + 1]);
//    println!("{:?}", word);
//    let mut word_correct: bool = false;
//    let mut next = 1;
//    while !word_correct {
//        next += 1;
//        word = format!("{}{}", word, matches[wrong_index + next]);
//
//    }
//    let corrected = correct.correct(&word);
//
//    match corrected {
//        None => {
//            println!("no word found");
//        }
//        Some(x) => {
//            println!("found {}", x);
//        }
//    }
//}

pub fn get_words(mut input_string: String, trie: Trie<u8>) -> Vec<String> {
    let mut matches: Vec<String> = Vec::new();

    //let mut wrong_index = 0;
    while input_string.len() > 0 {
        let results_in_str = results(&trie, &input_string);

        if results_in_str.len() == 0 {
            input_string = remove_first(&input_string).to_string();
        } else {
            let word = find_longest_word(results_in_str);
            input_string = input_string.replace(&word, "");
            matches.push(word.clone());
        }
    }

    //correct_words(matches, wrong_index);

    matches
}

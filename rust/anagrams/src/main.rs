// https://www.codewars.com/kata/523a86aa4230ebb5420001e1/train/rust

use std::collections::HashMap;

fn anagrams(word: &str, words: &[String]) -> Vec<String> {
    let mut vres: Vec<String> = vec![];

    let mut letter_map: HashMap<char, i32> = HashMap::new();

    for w in word.chars() {
        let mut current_value = *letter_map.get(&w).unwrap_or(&0);
        current_value += 1;
        letter_map.insert(w, current_value);
    }

    for wo in words {
        let mut letter_map_check: HashMap<char, i32> = HashMap::new();
        for rr in wo.chars() {
            let mut current_value_check = *letter_map_check.get(&rr).unwrap_or(&0);
            current_value_check += 1;
            letter_map_check.insert(rr, current_value_check);
        }
        println!("{:?}", letter_map_check);
        if letter_map_check == letter_map {
            vres.push(wo.to_string());
        }
    }
    vres
}

fn main() {
    let test: Vec<String> = ["aabb", "abcd", "bbaa", "dada"]
        .iter()
        .map(|w| w.to_string())
        .collect();
    anagrams("abba", &test);
    println!("Hello, world!");
}

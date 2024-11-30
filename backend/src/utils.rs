use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashSet,
    time::{SystemTime, UNIX_EPOCH},
};

lazy_static! {
    static ref NEW_LINE_PATTERN: Regex = Regex::new(r"\r?\n|\s+").unwrap();
}

pub fn parse_words(file_content: &str) -> Vec<String> {
    NEW_LINE_PATTERN
        .split(&file_content)
        .filter_map(|word| {
            let word = word.trim().to_lowercase().to_string();
            if word.len() < 2 || !word.is_ascii() {
                return None;
            }
            Some(word)
        })
        .collect::<HashSet<String>>()
        .into_iter()
        .collect()
}

pub fn current_time_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards.")
        .as_millis()
}

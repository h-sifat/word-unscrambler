use color_eyre::eyre;
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashSet, env, fs};
use word_unscrambler::trie::Trie;

lazy_static! {
    static ref NEW_LINE_PATTERN: Regex = Regex::new(r"\r\n|\r|\n").unwrap();
}

fn parse_words(file_content: &str) -> Vec<String> {
    NEW_LINE_PATTERN
        .split(&file_content)
        .filter_map(|word| {
            let word = word.trim().to_lowercase().to_string();
            if word.is_empty() || !word.is_ascii() {
                return None;
            }
            Some(word)
        })
        .collect::<HashSet<String>>()
        .into_iter()
        .collect()
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let trie = Trie::new();
    trie.insert_many(&["cat", "car", "cart", "call"]);

    println!("{:#?}", trie.search_words("tcra"));

    // let words_file_path = {
    //     let mut src_dir = env::current_dir().expect("Failed to get current directory!");
    //     src_dir.push("data/words.txt");
    //     src_dir.to_str().unwrap().to_owned()
    // };
    //
    // println!("{:?}", words_file_path);
    //
    // let file_content = fs::read_to_string(&words_file_path)
    //     .expect(&format!("Could not read file: '{}'", &words_file_path));
    //
    // println!("{:?}", parse_words(file_content.get(0..200).unwrap()));

    Ok(())
}

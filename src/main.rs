use color_eyre::eyre;
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashSet, env, fs};

lazy_static! {
    static ref NEW_LINE_PATTERN: Regex = Regex::new(r"\r\n|\r|\n").unwrap();
}

fn parse_words(file_content: &str) -> Vec<String> {
    NEW_LINE_PATTERN
        .split(&file_content)
        .filter_map(|word| {
            let word = word.trim().to_string();
            return if word.is_empty() { None } else { Some(word) };
        })
        .collect::<HashSet<String>>()
        .into_iter()
        .collect()
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let words_file_path = {
        let mut src_dir = env::current_dir().expect("Failed to get current directory!");
        src_dir.push("data/words.txt");
        src_dir.to_str().unwrap().to_owned()
    };

    println!("{:?}", words_file_path);

    let file_content = fs::read_to_string(&words_file_path)
        .expect(&format!("Could not read file: '{}'", &words_file_path));

    println!("{:?}", parse_words(file_content.get(0..200).unwrap()));

    Ok(())
}

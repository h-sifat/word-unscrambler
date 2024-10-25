use crate::{trie::Trie, utils::parse_words};
use actix_web::{get, web, HttpResponse, Responder};
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use serde_json::json;
use std::fs;

// -------- Constants --------------

lazy_static! {
    static ref VALID_WORD_PATTERN: Regex = Regex::new("^[a-zA-Z-]+$").unwrap();
}

// ----------- Struct --------------

pub struct RouteState {
    pub trie: Trie,
    pub max_word_len: usize,
}

#[derive(Clone)]
pub struct WordsState {
    pub words: Vec<String>,
    pub max_len: usize,
}

// ---------- Routes ------------
#[derive(Deserialize)]
struct UnscrambleQuery {
    word: Option<String>,
}

#[get("/unscramble")]
pub async fn unscramble(
    state: web::Data<RouteState>,
    queries: web::Query<UnscrambleQuery>,
) -> impl Responder {
    let word = match validate_query(queries.word.clone(), state.max_word_len) {
        Ok(word) => word,
        Err(message) => {
            return HttpResponse::BadRequest().json(json!({"message": message }));
        }
    };

    println!("request -> unscramble: '{}'", word);

    HttpResponse::Ok().json(json!({"words": state.trie.search_words(&word)}))
}

// ---------- Utils ------------
pub fn get_words_state(words_file_path: &str) -> WordsState {
    let words = {
        let file_content = fs::read_to_string(words_file_path).unwrap();
        parse_words(&file_content)
    };

    let max_len = words.iter().map(|word| word.len()).max().unwrap_or(0);

    WordsState { words, max_len }
}

fn validate_query(query: Option<String>, max_query_len: usize) -> Result<String, String> {
    if query.is_none() {
        return Err(r#"Missing query "word". e.g., "/unscramble?word=test""#.to_string());
    }

    let query = query.unwrap();

    if query.len() < 2 {
        return Err("Query must be at least 2 characters long.".to_string());
    }

    if query.len() > max_query_len {
        return Err(
            format!("The query cannot be longer than {max_query_len} characters!").to_string(),
        );
    }

    if !VALID_WORD_PATTERN.is_match(&query) {
        return Err("The query must only contain alpha (a-z) and '-' characters.".to_string());
    }

    Ok(query)
}

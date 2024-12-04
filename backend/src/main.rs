use actix_files::{self as fs, Files};
use actix_web::{web, App, HttpServer};
use api::{get_words_state, unscramble, RouteState};
use color_eyre::eyre;
use config::Config;
use std::sync::{Arc, Mutex};
use trie::Trie;

pub mod api;
pub mod config;
pub mod trie;
pub mod utils;

#[actix_web::main]
async fn main() -> eyre::Result<(), std::io::Error> {
    let config = Config::new();
    let words_state = Arc::new(Mutex::new(get_words_state(&config.words_file_path)));

    println!(
        "App running on port: {}. Is prod: {}",
        config.port, config.is_prod
    );

    HttpServer::new(move || {
        let local_state = words_state.lock().unwrap();

        print!("Building trie...");
        let trie = Trie::from_words(&local_state.words);
        println!(" Done!");

        App::new()
            .app_data(web::Data::new(RouteState {
                trie,
                max_word_len: local_state.max_len,
            }))
            .service(unscramble)
            .service(make_static_files_route(
                &config.static_directory,
                &config.index_file_name,
            ))
    })
    .workers(config.num_workers)
    .bind(("0.0.0.0", config.port))?
    .run()
    .await
}

fn make_static_files_route(dir_path: &str, index_file_name: &str) -> Files {
    fs::Files::new("/", dir_path).index_file(index_file_name)
}

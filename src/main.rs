use std::sync::{Arc, Mutex};

use actix_web::{web, App, HttpServer};
use api::{get_words_state, unscramble, RouteState};
use color_eyre::eyre;
use trie::Trie;

pub mod api;
pub mod trie;
pub mod utils;

#[actix_web::main]
async fn main() -> eyre::Result<(), std::io::Error> {
    let words_state = Arc::new(Mutex::new(get_words_state()));

    let port = 8080;

    println!("App running on port: {}", port);

    HttpServer::new(move || {
        let local_state = words_state.lock().unwrap();

        let trie = Trie::from_words(&local_state.words);
        println!("Building trie.");

        App::new()
            .app_data(web::Data::new(RouteState {
                trie,
                max_word_len: local_state.max_len,
            }))
            .service(unscramble)
    })
    .workers(1)
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

use std::env;

#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub is_prod: bool,
    pub num_workers: usize,
    pub words_file_path: String,
    pub index_file_name: String,
    pub static_directory: String,
}

impl Config {
    pub fn new() -> Self {
        let [words_file_path, static_directory] =
            [("WORDS_FILE", "data/words.txt"), ("STATIC_DIR", "static")].map(
                |(var_name, default)| {
                    let value = env::var(var_name).unwrap_or(default.to_string());

                    env::current_dir()
                        .expect("Couldn't get the current working directory")
                        .join(value)
                        .to_str()
                        .unwrap()
                        .to_string()
                },
            );

        let is_prod = env::var("MODE").map_or_else(|_| false, |v| &v == "production");

        Self {
            is_prod,
            words_file_path,
            static_directory,
            index_file_name: "index.html".to_string(),
            num_workers: if is_prod { num_cpus::get() } else { 1 },
            port: env::var("PORT")
                .unwrap_or("8080".to_string())
                .parse::<u16>()
                .expect("Invalid port number."),
        }
    }
}

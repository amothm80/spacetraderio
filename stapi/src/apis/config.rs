pub struct Config {
    pub base_path: String,
    pub bearer_token: String,
    pub client: reqwest::Client,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            base_path: String::new(),
            bearer_token: String::new(),
            client: reqwest::Client::new(),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Config {
            base_path: "https://api.spacetraders.io/v2".to_owned(),
            bearer_token: String::new(),
            client: reqwest::Client::new(),
        }
    }
}

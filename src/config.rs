pub struct Config {
    pub server_addr: String,
    pub server_port: u16,
    pub database_url: String,
    pub jwt_secret: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server_addr: "127.0.0.1".to_string(),
            server_port: 8000,
            database_url: "sqlite:game.db".to_string(),
            jwt_secret: "your-secret-key".to_string(),
        }
    }
}

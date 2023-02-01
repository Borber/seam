use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::util::bin_dir;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub video: VideoConfig,
    pub danmu: DanmuConfig,
}

#[derive(Deserialize, Debug)]
pub struct VideoConfig {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct DanmuConfig {
    pub name: String,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let config = std::fs::read_to_string(format!("{}config.json", bin_dir(),)).unwrap();
    let config = serde_json::from_str::<Config>(&config).unwrap();
    config
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        println!("{:?}", CONFIG);
    }
}

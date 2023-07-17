use std::collections::HashMap;

use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::util::bin_dir;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub video: Video,
    pub danmu: Danmu,
    pub cookie: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug)]
pub struct Video {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Danmu {
    pub name: String,
}

/// 配置文件
pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let config = std::fs::read_to_string(format!("{}config.toml", bin_dir(),)).unwrap();
    basic_toml::from_str::<Config>(&config).unwrap()
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        // 初始化 CONFIG
        let _ = CONFIG.video.name.clone();
        println!("{:#?}", CONFIG);
    }
}

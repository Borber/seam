use std::collections::HashMap;

use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::util::bin_dir;

#[derive(Deserialize, Debug)]
pub struct ConfigFile {
    pub video: Option<Video>,
    pub danmu: Option<Danmu>,
    pub cookie: Option<HashMap<String, HashMap<String, String>>>,
}

#[derive(Debug)]
pub struct Config {
    pub video: Video,
    pub danmu: Danmu,
    pub cookie: HashMap<String, HashMap<String, String>>,
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
    let config_file = basic_toml::from_str::<ConfigFile>(&config).unwrap();
    Config {
        video: config_file.video.unwrap(),
        danmu: config_file.danmu.unwrap(),
        cookie: config_file.cookie.unwrap(),
    }
});

// TODO 默认配置文件

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        // 初始化 CONFIG
        let _ = CONFIG.video;
        println!("{:#?}", CONFIG);
    }
}

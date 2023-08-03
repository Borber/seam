use std::collections::HashMap;

use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::util::bin_dir;

// TODO 可能需要区分全局配置和平台单独配置

#[derive(Deserialize, Debug)]
pub struct ConfigOption {
    pub file_name: Option<FileNameOption>,
    pub cookie: Option<HashMap<String, HashMap<String, String>>>,
}

#[derive(Debug)]
pub struct Config {
    pub file_name: FileNameConfig,
    pub cookie: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug)]
pub struct FileNameConfig {
    pub video: String,
    pub danmu: String,
}

#[derive(Deserialize, Debug)]
pub struct FileNameOption {
    pub video: Option<String>,
    pub danmu: Option<String>,
}

/// 配置文件
pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let config =
        std::fs::read_to_string(format!("{}config.toml", bin_dir(),)).unwrap_or("".to_owned());
    let config_file = basic_toml::from_str::<ConfigOption>(&config).unwrap();
    Config {
        file_name: {
            let FileNameOption { video, danmu } = config_file.file_name.unwrap_or(FileNameOption {
                video: None,
                danmu: None,
            });
            let video = video.unwrap_or("[rid]-[title]-[date]-[time]".to_string());
            let danmu = danmu.unwrap_or("[rid]-[title]-[date]-[time]".to_string());
            FileNameConfig { video, danmu }
        },
        cookie: config_file.cookie.unwrap_or_default(),
    }
});

// TODO 默认配置文件

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        // 初始化 CONFIG
        let _ = CONFIG.file_name.video;
        println!("{:#?}", CONFIG);
    }
}

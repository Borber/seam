use std::collections::HashMap;

use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::util::bin_dir;

#[derive(Deserialize, Debug)]
pub struct ConfigOption {
    pub headers: Option<HashMap<String, HashMap<String, String>>>,
}

#[derive(Debug)]
pub struct Config {
    pub headers: HashMap<String, HashMap<String, String>>,
}

/// 配置文件
pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let config =
        std::fs::read_to_string(format!("{}config.toml", bin_dir(),)).unwrap_or("".to_owned());
    let config_file = basic_toml::from_str::<ConfigOption>(&config).unwrap();
    Config {
        headers: config_file.headers.unwrap_or_default(),
    }
});

pub fn headers(live: &str) -> HashMap<String, String> {
    let global = CONFIG
        .headers
        .get("global")
        .unwrap_or(&HashMap::new())
        .clone();
    let mut live = CONFIG.headers.get(live).unwrap_or(&HashMap::new()).clone();
    live.extend(global);
    live
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        // 初始化 CONFIG
        let _ = CONFIG.headers.get("bili").unwrap_or(&HashMap::new());
        println!("{:#?}", CONFIG);
    }
}

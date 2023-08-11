use std::collections::HashMap;

use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::util::bin_dir;

#[derive(Deserialize, Debug)]
pub struct ConfigOption {
    pub play: Option<PlayOption>,
    pub headers: Option<HashMap<String, HashMap<String, String>>>,
}

#[derive(Debug)]
pub struct Config {
    pub play: Play,
    pub headers: HashMap<String, HashMap<String, String>>,
}

#[derive(Deserialize, Debug)]
pub struct PlayOption {
    pub bin: Option<String>,
    pub args: Option<Vec<String>>,
}

#[derive(Debug, Default)]
pub struct Play {
    pub bin: String,
    pub args: Vec<String>,
}

/// 配置文件
pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let config =
        std::fs::read_to_string(format!("{}config.toml", bin_dir(),)).unwrap_or("".to_owned());
    let config_file = basic_toml::from_str::<ConfigOption>(&config).unwrap();

    let bin = config_file
        .play
        .as_ref()
        .and_then(|play| play.bin.clone())
        .unwrap_or("".to_owned());

    let args = config_file
        .play
        .as_ref()
        .and_then(|play| play.args.clone())
        .unwrap_or(vec![]);

    let play = Play { bin, args };

    let headers = config_file.headers.unwrap_or_default();

    Config { play, headers }
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

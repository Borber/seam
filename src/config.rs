use anyhow::Result;
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

pub static CONFIG: tokio::sync::OnceCell<Config> = tokio::sync::OnceCell::const_new();

pub async fn get_config() -> Result<&'static Config> {
    CONFIG
        .get_or_try_init(|| async {
            let config = std::fs::read_to_string(format!("{}config.json", bin_dir(),)).unwrap();
            let config = serde_json::from_str::<Config>(&config).unwrap();
            Ok(config)
        })
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        println!("{:?}", CONFIG);
    }
}

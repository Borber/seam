use crate::{
    common::CLIENT,
    default_danmu_client,
    model::{Detail, ShowType},
    util::parse_url,
};
use anyhow::{Ok, Result};

default_danmu_client!(Inke);

const URL: &str = "https://webapi.busi.inke.cn/web/live_share_pc?uid=";

pub async fn get(rid: &str) -> Result<ShowType> {
    let json: serde_json::Value = CLIENT
        .get(format!("{URL}{rid}"))
        .send()
        .await?
        .json()
        .await?;

    match &json["data"]["status"].as_i64() {
        Some(1) => {
            let title = json["data"]["live_name"]
                .as_str()
                .unwrap_or("inke")
                .to_string();
            let mut nodes = vec![];
            for s in ["stream_addr", "hls_stream_addr", "rtmp_stream_addr"] {
                if !json["data"]["live_addr"][0][s].is_null() {
                    nodes.push(parse_url(
                        json["data"]["live_addr"][0][s]
                            .as_str()
                            .unwrap()
                            .to_string(),
                    ));
                }
            }

            Ok(ShowType::On(Detail::new(title, nodes)))
        }
        _ => Ok(ShowType::Off),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_inke() {
        println!("{}", get("713935849").await.unwrap());
    }
}

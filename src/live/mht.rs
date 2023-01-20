use crate::{
    common::CLIENT,
    default_danmu_client,
    model::{Detail, ShowType},
    util::parse_url,
};

use anyhow::{Ok, Result};
use serde_json::Value;

const URL: &str = "https://www.2cq.com/proxy/room/room/info";

default_danmu_client!(Mht);

/// 棉花糖直播
///
/// https://www.2cq.com/
pub async fn get(rid: &str) -> Result<ShowType> {
    let resp: serde_json::Value = CLIENT
        .get(URL)
        .query(&[("roomId", rid), ("appId", "1004")])
        .send()
        .await?
        .json()
        .await?;
    match &resp["errorMsg"] {
        Value::Null => {
            // 不报错的情况必然有结果返回 直接提取
            let result = &resp["result"];
            match result["liveState"].to_string().parse::<usize>()? {
                // 开播状态
                1 => {
                    let nodes = vec![parse_url(result["pullUrl"].to_string())];
                    Ok(ShowType::On(Detail::new("mht".to_owned(), nodes)))
                }
                _ => Ok(ShowType::Off),
            }
        }
        // 房间不存在或其他错误
        msg => Ok(ShowType::Error(msg.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_url() {
        println!("{}", get("932055").await.unwrap());
    }
}

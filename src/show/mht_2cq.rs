use crate::modle::ShowType;

use anyhow::{Ok, Result};
use reqwest::Client;

const URL: &str = "https://www.2cq.com/proxy/room/room/info";

/// 棉花糖直播
///
/// https://www.2cq.com/
async fn get(rid: &str, client: &Client) -> Result<ShowType> {
    let resp: serde_json::Value = client
        .get(URL)
        .query(&[("roomId", rid), ("appId", "1004")])
        .send()
        .await?
        .json()
        .await?;
    match resp.get("errorMsg") {
        // 房间不存在或其他错误
        Some(msg) => Ok(ShowType::Error(msg.to_string())),
        None => {
            // 不报错的情况必然有结果返回 直接提取
            let result = resp.get("result").unwrap();
            match result.get("liveState") {
                // 开播状态
                Some(state) => match state.to_string().as_str() {
                    "1" => Ok(ShowType::On(vec![result
                        .get("pullUrl")
                        .unwrap()
                        .to_string()])),
                    _ => Ok(ShowType::Off),
                },
                None => Ok(ShowType::Off),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::show::mht_2cq::get;
    #[tokio::test]
    async fn test_get_url() {
        let client = reqwest::Client::new();
        println!("{:#?}", get("935443", &client).await.unwrap());
    }
}

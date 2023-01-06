use crate::{
    common::CLIENT,
    modle::{Node, ShowType},
};

use anyhow::{Ok, Result};
use serde_json::Value;

const URL: &str = "https://www.2cq.com/proxy/room/room/info";

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
                1 => Ok(ShowType::On(vec![Node {
                    rate: "清晰度".to_string(),
                    url: result["pullUrl"].to_string(),
                }])),
                _ => Ok(ShowType::Off),
            }
        }
        // 房间不存在或其他错误
        msg => Ok(ShowType::Error(msg.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use crate::live::mht::get;
    use crate::util::match_show_type;

    #[tokio::test]
    async fn test_get_url() {
        match_show_type(get("932055").await.unwrap());
    }
}

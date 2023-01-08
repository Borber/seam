use crate::{
    common::{CLIENT, USER_AGENT},
    modle::{Node, ShowType},
};

use anyhow::{Ok, Result};

const INIT_URL: &str = "https://api.live.bilibili.com/room/v1/Room/room_init";
const URL: &str = "https://api.live.bilibili.com/xlive/web-room/v2/index/getRoomPlayInfo";

/// bilibili直播
///
/// https://live.bilibili.com/
pub async fn get(rid: &str) -> Result<ShowType> {
    let resp: serde_json::Value = CLIENT
        .get(INIT_URL)
        .header("User-Agent", USER_AGENT)
        .query(&[("id", rid)])
        .send()
        .await?
        .json()
        .await?;
    match resp["code"].to_string().parse::<usize>()? {
        0 => match resp["data"]["live_status"].to_string().parse::<usize>()? {
            1 => {
                let room_id = resp["data"]["room_id"].to_string();
                let mut stream_info = get_stream_info(&room_id, 10000).await?;
                let max = stream_info
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|data| {
                        data["format"][0]["codec"][0]["accept_qn"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|item| item.as_u64().unwrap())
                            .max()
                            .unwrap()
                    })
                    .max()
                    .unwrap();
                if max != 10000 {
                    stream_info = get_stream_info(&room_id, max).await?;
                }
                let mut stream_urls = vec![];
                for obj in stream_info.as_array().unwrap() {
                    for format in obj["format"].as_array().unwrap() {
                        for codec in format["codec"].as_array().unwrap() {
                            let base_url = codec["base_url"].to_string();
                            for url_info in codec["url_info"].as_array().unwrap() {
                                let host = url_info["host"].to_string();
                                let extra = url_info["extra"].to_string();
                                stream_urls.push(Node {
                                    rate: "清晰度".to_string(),
                                    url: format!("{}{}{}", host, base_url.clone(), extra)
                                        .replace('"', ""),
                                });
                            }
                        }
                    }
                }
                Ok(ShowType::On(stream_urls))
            }
            _ => Ok(ShowType::Off),
        },
        _ => Ok(ShowType::Error(resp["msg"].to_string())),
    }
}

/// 通过真实房间号获取直播源信息
async fn get_stream_info(room_id: &str, qn: u64) -> Result<serde_json::Value> {
    Ok(CLIENT
        .get(URL)
        .header("User-Agent", USER_AGENT)
        .query(&[
            ("room_id", room_id),
            ("protocol", "0,1"),
            ("format", "0,1,2"),
            ("codec", "0,1"),
            ("qn", qn.to_string().as_str()),
            ("platform", "h5"),
            ("ptype", "8"),
        ])
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?["data"]["playurl_info"]["playurl"]["stream"]
        .to_owned())
}

#[cfg(test)]
mod tests {
    use crate::live::bilibili::get;
    use crate::util::match_show_type;

    #[tokio::test]
    async fn test_get_url() {
        match_show_type(get("1785182").await.unwrap());
    }
}

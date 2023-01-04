use crate::modle::ShowType;

use anyhow::{Ok, Result};
use reqwest::Client;

const INIT_URL: &str = "https://api.live.bilibili.com/room/v1/Room/room_init";
const URL: &str = "https://api.live.bilibili.com/xlive/web-room/v2/index/getRoomPlayInfo";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36 Edg/108.0.1462.54";

async fn get_stream_info(room_id: &str, client: &Client, qn: u64) -> Result<serde_json::Value> {
    Ok(client
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
        .await?
        .get("data")
        .unwrap()
        .get("playurl_info")
        .unwrap()
        .get("playurl")
        .unwrap()
        .get("stream")
        .unwrap()
        .to_owned())
}

async fn get(rid: &str, client: &Client) -> Result<ShowType> {
    let resp: serde_json::Value = client
        .get(INIT_URL)
        .header("User-Agent", USER_AGENT)
        .query(&[("id", rid)])
        .send()
        .await?
        .json()
        .await?;
    match resp.get("code").unwrap().to_string().parse::<usize>()? {
        0 => {
            match resp
                .get("data")
                .unwrap()
                .get("live_status")
                .unwrap()
                .to_string()
                .parse::<usize>()?
            {
                1 => {
                    let room_id = resp
                        .get("data")
                        .unwrap()
                        .get("room_id")
                        .unwrap()
                        .to_string();
                    let mut stream_info = get_stream_info(&room_id, client, 10000).await?;
                    let max = stream_info
                        .as_array()
                        .unwrap()
                        .into_iter()
                        .map(|data| {
                            data.get("format").unwrap().as_array().unwrap()[0]
                                .get("codec")
                                .unwrap()
                                .as_array()
                                .unwrap()[0]
                                .get("accept_qn")
                                .unwrap()
                                .as_array()
                                .unwrap()
                                .into_iter()
                                .map(|item| item.as_u64().unwrap())
                                .max()
                                .unwrap()
                        })
                        .max()
                        .unwrap();
                    if max != 10000 {
                        stream_info = get_stream_info(&room_id, client, max).await?;
                    }
                    let mut stream_urls = vec![];
                    for obj in stream_info.as_array().unwrap() {
                        for format in obj.get("format").unwrap().as_array().unwrap() {
                            for codec in format.get("codec").unwrap().as_array().unwrap() {
                                let base_url = codec.get("base_url").unwrap().to_string();
                                for url_info in codec.get("url_info").unwrap().as_array().unwrap() {
                                    let host = url_info.get("host").unwrap().to_string();
                                    let extra = url_info.get("extra").unwrap().to_string();
                                    stream_urls.push(
                                        format!("{}{}{}", host, base_url.clone(), extra)
                                            .replace("\"", ""),
                                    );
                                }
                            }
                        }
                    }
                    Ok(ShowType::On(stream_urls))
                }
                _ => Ok(ShowType::Off),
            }
        }
        _ => Ok(ShowType::Error(resp.get("msg").unwrap().to_string())),
    }
}

#[cfg(test)]
mod tests {
    use crate::show::bilibili::get;
    #[tokio::test]
    async fn test_get_url() {
        let client = reqwest::Client::new();
        println!("{:#?}", get("26748360", &client).await.unwrap());
    }
}

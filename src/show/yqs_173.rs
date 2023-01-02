use crate::modle::ShowType;

use anyhow::{Ok, Result};
use reqwest::Client;
use std::collections::HashMap;


const URL: &str = "https://www.173.com/room/getVieoUrl";

/// 艺气山直播
///
/// https://www.173.com/
async fn get(rid: &str, client: &Client) -> Result<ShowType> {
    let mut params = HashMap::new();
    params.insert("roomId", rid);
    let resp: serde_json::Value = client
        .post(URL)
        .form(&params)
        .send()
        .await?
        .json()
        .await?;
    let data = resp.get("data").unwrap();
    match data.get("status") {
        Some(_) => match data.get("status").unwrap().to_string().as_str() {
            "2" => Ok(ShowType::On(vec![data.get("url").unwrap().to_string()])),
            _ => Ok(ShowType::Off),
        },
        None => Ok(ShowType::Off),
    }
}

#[cfg(test)]
mod tests {
    use crate::show::yqs_173::get;
    #[tokio::test]
    async fn test_get_url() {
        let client = reqwest::Client::new();
        println!("{:#?}", get("8", &client).await.unwrap());
    }
}

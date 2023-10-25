use std::collections::HashMap;

use async_trait::async_trait;
use reqwest::header::HeaderMap;
use serde_json::Value;

use super::{Live, Node, Url};
use crate::common::{CLIENT, USER_AGENT};
use crate::error::{Result, SeamError};
use crate::util::hash2header;

/// twitch直播
///
/// https://www.twitch.tv
pub struct Client;

#[async_trait]
impl Live for Client {
    async fn get(&self, rid: &str, headers: Option<HashMap<String, String>>) -> Result<Node> {
        let rid = rid.to_ascii_lowercase();

        let mut headers = hash2header(headers);
        headers.insert("Referer", "https://www.twitch.tv".parse()?);
        headers.insert("Origin", "https://www.twitch.tv".parse()?);
        headers.insert("Client-ID", "kimne78kx3ncx6brgo4mv6wki5h1ko".parse()?);

        if !headers.contains_key("User-Agent") {
            headers.insert("User-Agent", USER_AGENT.parse()?);
        }

        let metadata = self.get_channel_metadata(&rid, headers.clone()).await?;

        if metadata[1]["data"]["user"]["stream"].is_null() {
            Err(SeamError::None)
        } else {
            let (signature, token) = self.get_access_token(&rid, headers.clone()).await?;
            let urls = self
                .get_live_streams(&rid, &signature, &token, headers.clone())
                .await?
                .into_iter()
                .map(|url| Url {
                    format: super::Format::M3U,
                    url,
                })
                .collect();

            let get_string = |v: &Value| v.as_str().map(|s| s.to_string()).unwrap_or_default();

            Ok(Node {
                rid: rid.to_string(),
                title: get_string(&metadata[1]["data"]["user"]["lastBroadcast"]["title"]),
                cover: format!(
                    "https://static-cdn.jtvnw.net/previews-ttv/live_user_{rid}-320x180.jpg"
                ),
                anchor: get_string(&metadata[0]["data"]["userOrError"]["displayName"]),
                head: get_string(&metadata[1]["data"]["user"]["profileImageURL"]),
                urls,
            })
        }
    }
}

impl Client {
    async fn get_access_token(
        &self,
        rid: &str,
        mut headers: HeaderMap,
    ) -> Result<(String, String)> {
        let data = r#"{"operationName":"PlaybackAccessToken_Template","query":"query PlaybackAccessToken_Template($login: String!, $isLive: Boolean!, $vodID: ID!, $isVod: Boolean!, $playerType: String!) {  streamPlaybackAccessToken(channelName: $login, params: {platform: \"web\", playerBackend: \"mediaplayer\", playerType: $playerType}) @include(if: $isLive) {    value    signature   authorization { isForbidden forbiddenReasonCode }   __typename  }  videoPlaybackAccessToken(id: $vodID, params: {platform: \"web\", playerBackend: \"mediaplayer\", playerType: $playerType}) @include(if: $isVod) {    value    signature   __typename  }}","variables":{"isLive":true,"login":"___rid___","isVod":false,"vodID":"","playerType":"site"}}"#;
        let data = data.replace("___rid___", rid);

        headers.insert("Content-Type", "application/json".parse()?);
        let json: Value = CLIENT
            .post("https://gql.twitch.tv/gql")
            .headers(headers.clone())
            .body(data)
            .send()
            .await?
            .json()
            .await?;
        let signature = json["data"]["streamPlaybackAccessToken"]["signature"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| SeamError::NeedFix("twitch signature"))?;
        let token = json["data"]["streamPlaybackAccessToken"]["value"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| SeamError::NeedFix("twitch token"))?;
        Ok((signature, token))
    }

    async fn get_live_streams(
        &self,
        rid: &str,
        signature: &str,
        token: &str,
        headers: HeaderMap,
    ) -> Result<Vec<String>> {
        let query = [
            ("cdm", "wv"),
            ("allow_source", "true"),
            ("fast_bread", "true"),
            ("player_backend", "mediaplayer"),
            ("player_version", "1.23.0"),
            ("playlist_include_framerate", "true"),
            ("reassignments_supported", "true"),
            ("sig", signature),
            ("supported_codecs", "avc1"),
            ("token", token),
        ];

        let req = CLIENT
            .get(format!(
                "https://usher.ttvnw.net/api/channel/hls/{rid}.m3u8"
            ))
            .headers(headers)
            .query(&query);

        let urls = req
            .send()
            .await?
            .text()
            .await?
            .lines()
            .filter(|l| l.starts_with("https://"))
            .map(|l| l.to_string())
            .collect();

        Ok(urls)
    }

    async fn get_channel_metadata(&self, rid: &str, headers: HeaderMap) -> Result<Value> {
        let query = r#"
        [
            {
                "operationName": "ChannelShell",
                "variables": {
                    "login": "___rid___"
                },
                "extensions": {
                    "persistedQuery": {
                        "version": 1,
                        "sha256Hash": "580ab410bcd0c1ad194224957ae2241e5d252b2c5173d8e0cce9d32d5bb14efe"
                    }
                }
            },
            {
                "operationName": "StreamMetadata",
                "variables": {
                    "channelLogin": "___rid___"
                },
                "extensions": {
                    "persistedQuery": {
                        "version": 1,
                        "sha256Hash": "252a46e3f5b1ddc431b396e688331d8d020daec27079893ac7d4e6db759a7402"
                    }
                }
            }
        ]
        "#
        .replace("___rid___", rid);
        let json: Value = serde_json::from_str(&query)?;

        let rsp = CLIENT
            .post("https://gql.twitch.tv/gql")
            .headers(headers)
            .json(&json)
            .send()
            .await?
            .json()
            .await?;

        Ok(rsp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_twitch() {
        let c = Client;
        match c.get("nl_Kripp", None).await {
            Ok(r) => println!("{r:?}"),
            Err(SeamError::None) => {}
            Err(e) => {
                println!("{e}");
                assert!(false);
            }
        }
    }
}

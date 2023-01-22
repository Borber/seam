use crate::{
    common::{CLIENT, USER_AGENT},
    danmu::{websocket_danmu_work_flow, Danmu, DanmuRecorder},
    model::{Detail, ShowType},
    util::parse_url,
};

use std::collections::HashMap;

use anyhow::{anyhow, Ok, Result};
use async_trait::async_trait;
use miniz_oxide::inflate::decompress_to_vec_zlib;
use rand::Rng;
use regex::Regex;
use serde_json::json;

const INIT_URL: &str = "https://live.bilibili.com/";
const URL: &str = "https://api.live.bilibili.com/xlive/web-room/v2/index/getRoomPlayInfo";
const WSS_URL: &str = "wss://broadcastlv.chat.bilibili.com/sub";
const HEART_BEAT: &str = "\x00\x00\x00\x1f\x00\x10\x00\x01\x00\x00\x00\x02\x00\x00\x00\x01\x5b\x6f\x62\x6a\x65\x63\x74\x20\x4f\x62\x6a\x65\x63\x74\x5d ";
const HEART_BEAT_INTERVAL: u64 = 60;

/// bilibili直播
///
/// https://live.bilibili.com/
pub async fn get(rid: &str) -> Result<ShowType> {
    match get_real_room_info(rid).await {
        Some((live_status, room_id, title)) => match live_status {
            1 => {
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
                let mut nodes = vec![];
                for obj in stream_info.as_array().unwrap() {
                    for format in obj["format"].as_array().unwrap() {
                        for codec in format["codec"].as_array().unwrap() {
                            let base_url = codec["base_url"].as_str().unwrap();
                            for url_info in codec["url_info"].as_array().unwrap() {
                                let host = url_info["host"].as_str().unwrap();
                                let extra = url_info["extra"].as_str().unwrap();
                                nodes.push(parse_url(format!("{}{}{}", host, base_url, extra)));
                            }
                        }
                    }
                }
                Ok(ShowType::On(Detail::new(title, nodes)))
            }
            _ => Ok(ShowType::Off),
        },
        None => Ok(ShowType::Error("直播间不存在".to_owned())),
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
    use super::*;

    #[tokio::test]
    async fn test_get_url() {
        println!("{}", get("7777").await.unwrap());
    }
}

/// 获取直播间信息
/// 返回值：( 直播状态,真实房间号, 直播间标题)
pub async fn get_real_room_info(rid: &str) -> Option<(i32, String, String)> {
    let resp = CLIENT
        .get(format!("{INIT_URL}{rid}"))
        .header("User-Agent", USER_AGENT)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let re = Regex::new(r#"<script>window.__NEPTUNE_IS_MY_WAIFU__=([\s\S]*?)</script>"#).unwrap();
    let json = match re.captures(&resp) {
        Some(cap) => cap.get(1).unwrap().as_str(),
        None => return None,
    };
    let json: serde_json::Value = serde_json::from_str(json).unwrap();
    let json = &json["roomInfoRes"]["data"]["room_info"];
    let live_status = json["live_status"].to_string().parse::<i32>().unwrap();
    let room_id = json["room_id"].as_i64().unwrap().to_string();
    let title = json["title"].as_str().unwrap().to_owned();
    Some((live_status, room_id, title))
}

pub struct BiliDanmuClient {
    room_id: String,
}

impl BiliDanmuClient {
    pub async fn try_new(rid: &str) -> Result<Self> {
        if let Some((_, room_id, _)) = get_real_room_info(rid).await {
            Ok(Self { room_id })
        } else {
            Err(anyhow!("直播间不存在"))
        }
    }

    fn init_msg_generator(room_id: &str) -> Vec<Vec<u8>> {
        let mut reg_data = vec![];

        let room_id = room_id.parse::<i64>().unwrap();
        let random_uid: u64 =
            rand::thread_rng().gen_range(100_000_000_000_000..300_000_000_000_000);
        let data = json!({
            "roomid": room_id,
            "uid": random_uid,
            "protover": 1
        })
        .to_string();
        let data = vec![
            (data.len() as i32 + 16).to_be_bytes().to_vec(),
            vec![0x00, 0x10, 0x00, 0x01],
            7i32.to_be_bytes().to_vec(),
            1i32.to_be_bytes().to_vec(),
            data.as_bytes().to_vec(),
        ];
        reg_data.push(data.concat());
        reg_data
    }

    fn decode_and_record_danmu(data: &[u8], recorder: &DanmuRecorder) -> Result<()> {
        if data.len() < 16 {
            return Ok(());
        }

        let mut dm_list_compressed = vec![];
        let mut dm_list = vec![];
        let mut ops = vec![];
        let mut msgs = vec![];

        let mut sptr = 0;
        loop {
            let packet_len = u32::from_be_bytes(data[sptr..sptr + 4].try_into().unwrap());
            let ver = u16::from_be_bytes(data[sptr + 6..sptr + 8].try_into().unwrap());
            let op = u32::from_be_bytes(data[sptr + 8..sptr + 12].try_into().unwrap());

            if data[sptr..].len() < packet_len as usize {
                break;
            }

            if ver == 1 || ver == 0 {
                ops.push(op);
                dm_list.push(&data[sptr + 16..sptr + packet_len as usize]);
            } else if ver == 2 {
                dm_list_compressed.push(&data[sptr + 16..sptr + packet_len as usize]);
            }

            if data[sptr..].len() == packet_len as usize {
                break;
            } else {
                sptr += packet_len as usize;
            }
        }

        let dm_list_decompressed = dm_list_compressed
            .iter()
            .map(|dm| decompress_to_vec_zlib(dm).unwrap())
            .collect::<Vec<_>>();
        for decompressed in dm_list_decompressed.iter() {
            let mut sptr = 0;
            loop {
                let packet_len =
                    u32::from_be_bytes(decompressed[sptr..sptr + 4].try_into().unwrap());
                let op = u32::from_be_bytes(decompressed[sptr + 8..sptr + 12].try_into().unwrap());

                if decompressed[sptr..].len() < packet_len as usize {
                    break;
                }

                ops.push(op);
                dm_list.push(&decompressed[sptr + 16..sptr + packet_len as usize]);

                if decompressed[sptr..].len() == packet_len as usize {
                    break;
                } else {
                    sptr += packet_len as usize;
                }
            }
        }

        let mut msg_type_map = HashMap::new();
        msg_type_map.insert("SEND_GIFT", "gift");
        msg_type_map.insert("DANMU_MSG", "danmu");
        msg_type_map.insert("WELCOME", "enter");
        msg_type_map.insert("NOTICE_MSG", "broadcast");
        msg_type_map.insert("LIVE_INTERACTIVE_GAME", "interactive_danmaku");

        for (idx, &dm) in dm_list.iter().enumerate() {
            let mut msg = HashMap::new();
            if ops[idx] == 5 {
                let j: serde_json::Value = serde_json::from_slice(dm).unwrap();
                let msg_type: &str = j.get("cmd").unwrap().as_str().unwrap();
                let mapped_msg_type = *msg_type_map.get(msg_type).unwrap_or(&"other");
                if mapped_msg_type == "other" && msg_type.starts_with("DANMU_MSG") {
                    msg.insert("msg_type".to_owned(), "danmu".to_owned());
                } else {
                    msg.insert("msg_type".to_owned(), mapped_msg_type.to_owned());
                }

                if msg.get("msg_type").unwrap() == "danmu" {
                    // TODO: 可能panic，需要处理第二种情况: j.get("data").unwrap().get("uname").unwrap().as_str().unwrap().to_string();
                    let name = j["info"][2][1].as_str().unwrap().trim().to_string();
                    let content = j["info"][1].as_str().unwrap().trim().to_string();
                    msg.insert("name".to_owned(), name);
                    msg.insert("content".to_owned(), content);
                } else if msg.get("msg_type").unwrap() == "interactive_danmaku" {
                    let name = j["data"]["uname"].as_str().unwrap().to_string();
                    let content = j["data"]["msg"].as_str().unwrap().to_string();
                    msg.insert("name".to_owned(), name);
                    msg.insert("content".to_owned(), content);
                } else {
                    msg.insert("content".to_owned(), j.to_string());
                }
            } else {
                msg.insert("msg_type".to_owned(), "other".to_owned());
                msg.insert(
                    "content".to_owned(),
                    String::from_utf8_lossy(dm).to_string(),
                );
                msg.insert("name".to_owned(), "".to_owned());
            }
            msgs.push(msg);
        }

        if recorder == &DanmuRecorder::Terminal {
            for m in msgs.iter() {
                if m.get("msg_type") == Some(&"danmu".to_string()) {
                    println!(
                        "name: {:?}, content: {:?}",
                        m.get("name").unwrap(),
                        m.get("content").unwrap()
                    );
                }
            }
        }

        Ok(())
    }
}

async fn is_closed_room(rid: &str) -> Option<bool> {
    get_real_room_info(rid).await.map(|info| info.0 != 1)
}

#[async_trait]
impl Danmu for BiliDanmuClient {
    async fn start(&mut self, recorder: DanmuRecorder) -> Result<()> {
        let recorder_checker = |recorder: &DanmuRecorder| {
            if recorder != &DanmuRecorder::Terminal {
                return Err(anyhow!("Bilibili弹幕服务目前仅支持终端输出。"));
            }
            Ok(())
        };

        let heart_beat_msg_generator = || HEART_BEAT.as_bytes().to_vec();
        let heart_beat_interval = HEART_BEAT_INTERVAL;
        
        let room_id = &self.room_id.clone();
        let is_closed_room_closure = move || async {
            is_closed_room(room_id).await
        };

        websocket_danmu_work_flow(
            &self.room_id,
            WSS_URL,
            recorder,
            recorder_checker,
            Self::init_msg_generator,
            is_closed_room_closure,
            heart_beat_msg_generator,
            heart_beat_interval,
            Self::decode_and_record_danmu,
        )
        .await?;

        Ok(())
    }
}

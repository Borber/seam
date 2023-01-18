use crate::{
    common::{CLIENT, USER_AGENT},
    model::ShowType,
    util::parse_url,
    danmu::{Danmu, DanmuRecorder}
};

use std::pin::Pin;
use std::collections::HashMap;

use anyhow::{anyhow, Ok, Result};
use async_trait::async_trait;
use futures_util::StreamExt;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, tungstenite::protocol::Message};
use futures_sink::Sink;
use serde_json::json;
use rand::Rng;
use miniz_oxide::inflate::decompress_to_vec_zlib;

const INIT_URL: &str = "https://api.live.bilibili.com/room/v1/Room/room_init";
const URL: &str = "https://api.live.bilibili.com/xlive/web-room/v2/index/getRoomPlayInfo";
const WSS_URL: &str = "wss://broadcastlv.chat.bilibili.com/sub";
const HEART_BEAT: &str = "\x00\x00\x00\x1f\x00\x10\x00\x01\x00\x00\x00\x02\x00\x00\x00\x01\x5b\x6f\x62\x6a\x65\x63\x74\x20\x4f\x62\x6a\x65\x63\x74\x5d ";
const HEART_BEAT_INTERVAL: u64 = 60;

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
                            let base_url = codec["base_url"].as_str().unwrap();
                            for url_info in codec["url_info"].as_array().unwrap() {
                                let host = url_info["host"].as_str().unwrap();
                                let extra = url_info["extra"].as_str().unwrap();
                                stream_urls
                                    .push(parse_url(format!("{}{}{}", host, base_url, extra)));
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
    use super::*;

    #[tokio::test]
    async fn test_get_url() {
        println!("{}", get("23356199").await.unwrap());
    }
}


/// 获取真实房间号
pub async fn get_real_room_id(rid: &str) -> Result<String> {
    let resp: serde_json::Value = CLIENT
        .get(INIT_URL)
        .header("User-Agent", USER_AGENT)
        .query(&[("id", rid)])
        .send()
        .await?
        .json()
        .await?;
    match resp["code"].to_string().parse::<usize>()? {
        0 => Ok(resp["data"]["room_id"].to_string()),
        _ => Err(anyhow!(resp["msg"].to_string()))
    }
}

/// 获取直播弹幕服务
pub struct BilibiliDanmuClient<'a> {
    room_id: &'a str,
    websocket: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl<'a> BilibiliDanmuClient<'a> {
    /// 简单初始化的构造函数
    pub fn new(room_id: &'a str) -> Self {
        Self {
            room_id,
            websocket: None
        }
    }

    /// 生成websocket info
    fn get_ws_info(room_id: &str) -> Vec<Vec<u8>> {
        let mut reg_data = vec![];

        let room_id = room_id.parse::<i64>().unwrap();
        let random_uid: u64 = rand::thread_rng().gen_range(100_000_000_000_000..300_000_000_000_000);
        let data = json!({
            "roomid": room_id,
            "uid": random_uid,
            "protover": 1
        }).to_string();
        let data = vec![(data.len() as i32 + 16).to_be_bytes().to_vec(), vec![0x00, 0x10, 0x00, 0x01], 7i32.to_be_bytes().to_vec(), 1i32.to_be_bytes().to_vec(), data.as_bytes().to_vec()];
        reg_data.push(data.concat());
        reg_data
    }

    /// 初始化websocket
    async fn init_ws(&mut self) {
        let reg_datas = Self::get_ws_info(self.room_id);
        let (mut ws, _)= tokio_tungstenite::connect_async(WSS_URL).await.unwrap();
        for data in reg_datas {
            Pin::new(&mut ws).start_send(Message::Binary(data)).unwrap();
        }
        self.websocket = Some(ws);
    }

    /// 心跳机制
    async fn heart_beat(&mut self) {
        if let Some(ws) = &mut self.websocket {
            Pin::new(ws).start_send(Message::Binary(HEART_BEAT.as_bytes().to_vec())).unwrap();
        } else {
            self.init_ws().await;
        }
    }

    /// 获取弹幕。
    /// 
    /// TODO: 考虑通过接收一个函数或传入DanmuRecorder来处理弹幕。
    async fn fetch_danmu(&mut self) {
        if let Some(ws) = &mut self.websocket {
            let ws_to_stdout = {
                ws.for_each(|message| async {
                    let data = message.unwrap().into_data();
                    for m in Self::decode_msg(&data).iter() {
                        if m.get("msg_type") == Some(&"danmu".to_string()) {
                            println!("name: {:?}, content: {:?}", m.get("name").unwrap(), m.get("content").unwrap());
                        }
                    }
                })
            };
            ws_to_stdout.await;
        } else {
            self.init_ws().await;
        }
    }

    /// 解析websocket传回的弹幕信息。
    fn decode_msg(data: &[u8]) -> Vec<HashMap<String, String>> {
        if data.len() < 16 {
            return vec![];
        }

        let mut dm_list_compressed = vec![];
        let mut dm_list = vec![];
        let mut ops = vec![];
        let mut msgs = vec![];
        
        let mut sptr = 0;
        loop {
            let packet_len = u32::from_be_bytes(data[sptr..sptr + 4].try_into().unwrap());
            let ver = u16::from_be_bytes(data[sptr + 6..sptr + 8].try_into().unwrap());
            let op = u32::from_be_bytes(data[sptr + 8.. sptr + 12].try_into().unwrap());
    
            if data[sptr..].len() < packet_len as usize {
                break;
            }
    
            if ver == 1 ||  ver == 0 {
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
    
        let dm_list_decompressed = dm_list_compressed.iter().map(|dm| decompress_to_vec_zlib(dm).unwrap()).collect::<Vec<_>>();
        for decompressed in dm_list_decompressed.iter() {
            let mut sptr = 0;
            loop {
                let packet_len = u32::from_be_bytes(decompressed[sptr..sptr + 4].try_into().unwrap());
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
                    let name = j.get("info").unwrap().get(2).unwrap().get(1).unwrap().as_str().unwrap().trim().to_string();
                    let content = j.get("info").unwrap().get(1).unwrap().as_str().unwrap().trim().to_string();
                    msg.insert("name".to_owned(), name);
                    msg.insert("content".to_owned(), content);
                } else if msg.get("msg_type").unwrap() == "interactive_danmaku" {
                    let name = j.get("data").unwrap().get("uname").unwrap().as_str().unwrap().to_string();
                    let content = j.get("data").unwrap().get("msg").unwrap().as_str().unwrap().to_string();
                    msg.insert("name".to_owned(), name);
                    msg.insert("content".to_owned(), content);
                } else {
                    msg.insert("content".to_owned(), j.to_string());
                }
            } else {
                msg.insert("msg_type".to_owned(), "other".to_owned());
                msg.insert("content".to_owned(), String::from_utf8_lossy(dm).to_string());
                msg.insert("name".to_owned(), "".to_owned());
            }
            msgs.push(msg);
        }
    
        msgs
    }
}

#[async_trait]
impl Danmu for BilibiliDanmuClient<'_> {
    // TODO: 实现其他弹幕记录方式
    async fn start(&mut self, recorder: DanmuRecorder) -> Result<()> {
        if recorder != DanmuRecorder::Terminal {
            return Err(anyhow!("Bilibili弹幕服务目前仅支持终端输出。"));
        }

        self.init_ws().await;
        let mut last_heart_beat_timestamp = std::time::Instant::now();
        // 逻辑较简单，因此将逻辑放在一个loop中，避免两个&mut self引用导致不得不使用Arc<Mutex>，导致性能下降
        loop {
            if std::time::Instant::now().duration_since(last_heart_beat_timestamp).as_secs() >= HEART_BEAT_INTERVAL {
                self.heart_beat().await;
                last_heart_beat_timestamp = std::time::Instant::now();
            } else {
                let sleep = tokio::time::sleep(tokio::time::Duration::from_millis(10));
                tokio::pin!(sleep);
                tokio::select! {
                    _ = &mut sleep => {},
                    _ = self.fetch_danmu() => {}
                }
            }
        }
    }
}
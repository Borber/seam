use crate::{
    common::{CLIENT, USER_AGENT},
    danmu::{websocket_danmu_work_flow, Danmu, DanmuBody, DanmuRecorder},
    model::{Detail, ShowType},
    util::parse_url,
};

use anyhow::{anyhow, Ok, Result};
use async_trait::async_trait;
use miniz_oxide::inflate::decompress_to_vec_zlib;
use rand::Rng;
use serde_json::json;

const INIT_URL: &str = "https://api.live.bilibili.com/room/v1/Room/room_init";
const INFO_URL: &str = "https://api.live.bilibili.com/room/v1/Room/get_info";
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
                                nodes.push(parse_url(format!("{host}{base_url}{extra}")));
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
        println!("{}", get("6").await.unwrap());
    }
}

/// 获取直播间信息
/// 返回值：( 直播状态,真实房间号, 直播间标题)
pub async fn get_real_room_info(rid: &str) -> Option<(i64, String, String)> {
    let resp: serde_json::Value = CLIENT
        .get(INIT_URL)
        .query(&[("id", rid)])
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let (rid, live_status) = match resp["data"]["room_id"] {
        serde_json::Value::Null => return None,
        _ => (
            resp["data"]["room_id"].as_i64().unwrap(),
            resp["data"]["live_status"].as_i64().unwrap(),
        ),
    };

    let json: serde_json::Value = CLIENT
        .get(INFO_URL)
        .query(&[("room_id", rid)])
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let title = json["data"]["title"].as_str().unwrap().to_owned();

    Some((live_status, rid.to_string(), title))
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

    fn decode_and_record_danmu(data: &[u8]) -> Result<Vec<DanmuBody>> {
        if data.len() < 16 {
            return Ok(vec![]);
        }

        let mut msgs = vec![];

        let data_to_danmu_body = |sliced_data: &[u8]| -> Option<DanmuBody> {
            let j: serde_json::Value = serde_json::from_slice(sliced_data).unwrap();
            let msg_type = j.get("cmd").unwrap().as_str().unwrap();
            if msg_type == "DANMU_MSG" {
                let user = j["info"][2][1].as_str().unwrap().trim().to_string();
                let content = j["info"][1].as_str().unwrap().trim().to_string();
                Some(DanmuBody::new(user, content))
            } else {
                None
            }
        };

        let decompress_data_to_danmu_body = |compressed_data: &[u8]| -> Vec<DanmuBody> {
            let decompressed = decompress_to_vec_zlib(compressed_data).unwrap();
            let mut sptr = 0;
            let mut danmu_bodies = vec![];

            loop {
                let packet_len =
                    u32::from_be_bytes(decompressed[sptr..sptr + 4].try_into().unwrap());
                let op = u32::from_be_bytes(decompressed[sptr + 8..sptr + 12].try_into().unwrap());

                if decompressed[sptr..].len() < packet_len as usize {
                    break;
                }

                if op == 5 {
                    if let Some(danmu_body) =
                        data_to_danmu_body(&decompressed[sptr + 16..sptr + packet_len as usize])
                    {
                        danmu_bodies.push(danmu_body);
                    }
                }

                if decompressed[sptr..].len() == packet_len as usize {
                    break;
                } else {
                    sptr += packet_len as usize;
                }
            }

            danmu_bodies
        };

        let mut sptr = 0;
        loop {
            let packet_len = u32::from_be_bytes(data[sptr..sptr + 4].try_into().unwrap());
            let ver = u16::from_be_bytes(data[sptr + 6..sptr + 8].try_into().unwrap());
            let op = u32::from_be_bytes(data[sptr + 8..sptr + 12].try_into().unwrap());

            if data[sptr..].len() < packet_len as usize {
                break;
            }

            if (ver == 1 || ver == 0) && (op == 5) {
                if let Some(danmu_body) =
                    data_to_danmu_body(&data[sptr + 16..sptr + packet_len as usize])
                {
                    msgs.push(danmu_body);
                }
            } else if ver == 2 {
                msgs.extend(decompress_data_to_danmu_body(
                    &data[sptr + 16..sptr + packet_len as usize],
                ));
            }

            if data[sptr..].len() == packet_len as usize {
                break;
            } else {
                sptr += packet_len as usize;
            }
        }

        Ok(msgs)
    }
}

async fn is_closed_room(rid: &str) -> Option<bool> {
    get_real_room_info(rid).await.map(|info| info.0 != 1)
}

#[async_trait]
impl Danmu for BiliDanmuClient {
    async fn start(&mut self, recorder: Vec<&dyn DanmuRecorder>) -> Result<()> {
        let heart_beat_msg_generator = || HEART_BEAT.as_bytes().to_vec();
        let heart_beat_interval = HEART_BEAT_INTERVAL;

        let room_id = &self.room_id.clone();
        let is_closed_room_closure = move || async { is_closed_room(room_id).await };

        websocket_danmu_work_flow(
            &self.room_id,
            WSS_URL,
            recorder,
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

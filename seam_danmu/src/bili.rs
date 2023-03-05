use async_trait::async_trait;
use miniz_oxide::inflate::decompress_to_vec_zlib;
use rand::Rng;
use seam_status::{bili::BiliStatusClient, Client};
use serde_json::json;

use crate::error::Result;

use super::{websocket_danmu_work_flow, Danmu, DanmuBody, DanmuRecorder};

const WSS_URL: &str = "wss://broadcastlv.chat.bilibili.com/sub";
const HEART_BEAT: &str = "\x00\x00\x00\x1f\x00\x10\x00\x01\x00\x00\x00\x02\x00\x00\x00\x01\x5b\x6f\x62\x6a\x65\x63\x74\x20\x4f\x62\x6a\x65\x63\x74\x5d ";
const HEART_BEAT_INTERVAL: u64 = 60;

fn init_msg_generator(rid: &str) -> Vec<Vec<u8>> {
    let mut reg_data = vec![];

    let room_id = rid.parse::<i64>().unwrap();
    let random_uid: u64 = rand::thread_rng().gen_range(100_000_000_000_000..300_000_000_000_000);
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
            let packet_len = u32::from_be_bytes(decompressed[sptr..sptr + 4].try_into().unwrap());
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

pub struct BiliDanmuClient;

#[async_trait]
impl Danmu for BiliDanmuClient {
    async fn start(rid: &str, recorder: Vec<&dyn DanmuRecorder>) -> Result<()> {
        let heart_beat_msg_generator = || HEART_BEAT.as_bytes().to_vec();
        let heart_beat_interval = HEART_BEAT_INTERVAL;

        let is_closed_room = || async { BiliStatusClient::status(rid).await.unwrap() };

        websocket_danmu_work_flow(
            rid,
            WSS_URL,
            recorder,
            init_msg_generator,
            is_closed_room,
            heart_beat_msg_generator,
            heart_beat_interval,
            decode_and_record_danmu,
        )
        .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Terminal;

    use super::*;

    #[tokio::test]
    async fn test_danmu_terminal() {
        BiliDanmuClient::start("6", vec![&Terminal::try_new(None).unwrap()])
            .await
            .unwrap();
    }
}

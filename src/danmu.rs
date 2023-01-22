//! 弹幕相关模块。
//!
//! 本模块提供了标准化的弹幕记录的async trait 以及
//! 标准化的弹幕记录方式enum。
//!
//! 本模块提供了基于websocket的标准弹幕工作流。
//! 如无定制需求，可以直接使用本模块提供的工作流。

use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;

use anyhow::Result;
use async_trait::async_trait;
use futures_sink::Sink;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream};

/// 标准化弹幕记录异步接口。
#[async_trait]
pub trait Danmu {
    /// 运行弹幕记录服务。
    ///
    /// 本函数通常将运行websocket长连接，并按指定方式记录弹幕。
    /// 由于websockt的机制，本函数需要`&mut self`作为参数。
    ///
    /// # Errors
    ///
    /// 发生不可继续运行的错误的情况下，返回错误。
    async fn start(&mut self, recorder: DanmuRecorder) -> Result<()>;
}

/// 弹幕记录方式: 文件, 终端, 文件+终端, 不记录。
// TODO: 未来可能会添加其他记录方式，比如sqlite，xml等。
// TODO: 为了方便，目前只支持终端输出，后期需要添加文件输出。所以暂时allow dead_code。
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DanmuRecorder {
    File(PathBuf),
    Terminal,
    None,
}

/// 标准弹幕格式
// TODO: 时间戳
pub struct DanmuBody {
    pub user: String,
    pub content: String,
}

impl DanmuBody {
    pub fn new(user: String, content: String) -> Self {
        Self { user, content }
    }
}

/// 基于websocket的标准弹幕工作流。
///
/// 本函数将会运行websocket长连接，并按指定方式记录弹幕。
///
//
// # 本函数接管的工作流
//
// 1. 连接websocket服务器。
// 2. 发送初始化消息。
// 3. 维持心跳/接收websocket返回的消息。
//
// # 本函数未接管的工作
//
// 1. 检查recorder选项是否支持。
// 2. 生成websocket使用的初始化消息。
// 3. 生成心跳消息。
// 4. 解码并按照recorder的要求记录弹幕。
//
// # 本函数的参数设计
//
// - recorder_checker: 检查recorder选项是否支持，不支持请返回错误。
//
// - init_msg_generator: 生成初始化消息，返回一个Vec<Vec<u8>>，每个Vec<u8>为一条消息。
//   生成的消息将逐条发送给服务器以供初始化websocket。
//
// - heart_beat_msg_generator: 生成心跳消息，返回一个Vec<u8>，为一条消息。
//   生成的消息将按照`heart_beat_interval`的间隔发送给服务器以保持websocket长连接。
//
// - heart_beat_interval: 心跳间隔，单位为秒。
//
// - decode_and_record_danmu: 解码并按照recorder的要求记录弹幕。
//
// - 特别说明：heart_beat与decode_and_record_danmu将在同一线程异步并行。
pub async fn websocket_danmu_work_flow<B>(
    room_id: &str,
    url: &str,
    recorder: DanmuRecorder,
    recorder_checker: fn(&DanmuRecorder) -> Result<()>,
    init_msg_generator: fn(&str) -> Vec<Vec<u8>>,
    is_closed_room: impl Fn() -> B,
    heart_beat_msg_generator: fn() -> Vec<u8>,
    heart_beat_interval: u64,
    decode_and_record_danmu: fn(&[u8], &DanmuRecorder) -> Result<()>,
) -> Result<()>
where
    B: Future<Output = Option<bool>>,
{
    // 检查recorder是否支持
    recorder_checker(&recorder)?;

    // 初始化websocket连接
    let reg_datas = init_msg_generator(room_id);
    let (mut ws, _) = tokio_tungstenite::connect_async(url).await?;
    for data in reg_datas {
        Pin::new(&mut ws).start_send(Message::Binary(data))?;
    }

    // 分开websocket的读写
    let (mut write, mut read) = ws.split();

    // 异步执行心跳机制和弹幕获取
    // 需要检测直播间是否关闭，如果关闭则停止心跳机制和弹幕获取
    tokio::select! {
        _ = closed_room_checker(is_closed_room) => { }
        _ = heart_beat(&mut write, heart_beat_msg_generator, heart_beat_interval) => { println!("websocket已关闭"); }
        _ = fetch_danmu(&mut read, decode_and_record_danmu, &recorder) => { println!("websocket已关闭"); }
    }
    
    Ok(())
}

// 检测直播间是否关闭
async fn closed_room_checker<B>(is_closed_room: impl Fn() -> B)
    where B: Future<Output = Option<bool>>
{
    loop {
        if let Some(if_room_closed) = is_closed_room().await {
            if if_room_closed {
                println!("直播间已关闭");
                break;
            } else {
                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            }
        } else {
            println!("直播间不存在");
        }
    }
}

// 心跳机制
async fn heart_beat(ws_write: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>, heart_beat_msg_generator: fn() -> Vec<u8>, heart_beat_interval: u64) {
    loop {
        let msg = heart_beat_msg_generator();
        if Pin::new(&mut *ws_write).send(Message::Binary(msg)).await.is_ok() {
            tokio::time::sleep(tokio::time::Duration::from_secs(heart_beat_interval)).await;
        } else {
            let short_rebeat_interval = if heart_beat_interval / 10 > 3 {
                heart_beat_interval / 10
            } else {
                3
            };
            tokio::time::sleep(tokio::time::Duration::from_secs(short_rebeat_interval)).await;
        }
    }
}

// 解码并记录弹幕
async fn fetch_danmu(ws_read: &mut SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>, decode_and_record_danmu: fn(&[u8], &DanmuRecorder) -> Result<()>, recorder: &DanmuRecorder) {
    let ws_to_stdout = {
        ws_read.for_each(|message| async {
            let data = message.unwrap().into_data();
            let msgs = decode_and_record_danmu(&data).unwrap();
            match recorder {
                DanmuRecorder::File(_) => {}
                DanmuRecorder::Terminal => {
                    msgs.iter().for_each(|DanmuBody{user, content}| {
                        println!("user: {user}, content: {content}");
                    });
                }
                DanmuRecorder::None => {}
            };
        })
    };
    ws_to_stdout.await;
}

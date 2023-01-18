//! 弹幕相关模块。
//!
//! 本模块提供了标准化的弹幕记录的async trait 以及
//! 标准化的弹幕记录方式enum。
//!
//! TODO: 考虑标准化websocket链接的async trait。

use std::path::PathBuf;

use anyhow::Result;
use async_trait::async_trait;

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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DanmuRecorder {
    File(PathBuf),
    Terminal,
    None,
}

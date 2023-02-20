//! 直播状态检测相关模块。
//!
//! 本模块提供了标准化的直播状态检测的 async trait 以及
//! 标准化的直播源信息和直播状态enum

pub mod bili;
pub mod cc;
mod common;
pub mod error;

use async_trait::async_trait;
use error::Result;

/// 直播状态检测模块
pub struct StatusClient {}

#[async_trait]
pub trait Client {
    // 获取直播间状态
    // rid: 直播间号
    // ext: 是否返回附加信息
    async fn status(rid: &str) -> Result<bool>;
}

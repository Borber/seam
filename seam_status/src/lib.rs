//! 直播状态检测相关模块。
//!
//! 本模块提供了标准化的直播状态检测的 async trait 以及
//! 标准化的直播源信息和直播状态enum

mod common;
pub mod error;
pub mod live;

use async_trait::async_trait;
use error::Result;

/// 直播状态检测模块
pub struct StatusClient {}

#[async_trait]
pub trait Client {
    // 获取直播间状态
    // rid: 直播间号
    async fn status(rid: &str) -> Result<bool>;
}

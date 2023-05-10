//! 直播状态检测相关模块。
//!
//! 本模块提供了标准化的直播状态检测的 async trait

use async_trait::async_trait;

use error::Result;

mod common;
pub mod error;
pub mod status;

#[async_trait]
pub trait StatusTrait {
    // 获取直播间状态
    // rid: 直播间号
    async fn status(rid: &str) -> Result<bool>;
}

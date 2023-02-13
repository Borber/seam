use async_trait::async_trait;

use crate::model::Status;

pub mod bili;

/// 获取直播信息
#[async_trait]
pub trait Live {
    // 获取直播源
    // rid: 直播间号
    async fn get(self) -> Self;
    // 获取直播间状态
    // rid: 直播间号
    // ext: 附加信息或附加操作
    async fn status(rid: &str, ext: bool) -> Status;
}

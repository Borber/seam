use crate::model::{Node, Status};
use anyhow::Result;

pub mod bili;

/// 获取直播信息
pub trait Live {
    // 获取直播源
    // rid: 直播间号
    async fn get(self) -> Result<Node>;
    // 获取直播间状态
    // rid: 直播间号
    // ext: 附加信息或附加操作
    async fn status(self, ext: bool) -> Result<Status>;
}

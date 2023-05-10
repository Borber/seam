mod declarative;

use crate::declarative::{get_source_url, Commands};
use anyhow::Result;
use clap::Parser;

/// 获取直播源
#[derive(Parser)]
#[command(name = "seam")]
#[command(about ="
________ _______ _______ _______
|     __|    ___|   _   |   |   |
|__     |    ___|       |       |
|_______|_______|___|___|__|_|__|", long_about = None)]
struct Cli {
    // 子命令
    #[command(subcommand)]
    command: Commands,
    /// 直接录播功能
    #[arg(short = 'r', global = true)]
    record: bool,
    /// 自动监控录播功能
    #[arg(short = 'R', global = true)]
    auto_record: bool,
    /// 输出到终端的弹幕功能
    #[arg(short = 'd', global = true)]
    danmu: bool,
    /// 根据参数指定的文件地址输出弹幕
    #[arg(short = 'D', global = true)]
    config_danmu: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    get_source_url().await?;
    Ok(())
}

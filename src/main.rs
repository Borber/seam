#[macro_use]
mod declarative;

mod common;
mod live;
mod model;
mod util;

use crate::declarative::Commands;
use anyhow::{Ok, Result};
use clap::Parser;
use paste::paste;

/// 获取直播源
#[derive(Debug, Parser)]
#[command(name = "seam")]
#[command(about ="
________ _______ _______ _______
|     __|    ___|   _   |   |   |
|__     |    ___|       |       |
|_______|_______|___|___|__|_|__|

获取直播源地址, 目前支持 B站, 斗鱼, 抖音, 虎牙, 快手, 网易CC, 花椒, 艺气山, 棉花糖", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() -> Result<()> {
    get_resource_impl!(Cli::parse().command; Bili, Douyu, Douyin, Huya, Kuaishou, Cc, Huajiao, Yqs, Mht);
    Ok(())
}

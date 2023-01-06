mod live;
mod modle;
mod util;
mod common;

use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};
use reqwest::Client;

/// 获取直播源
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "seam")]
#[command(about ="
________ _______ _______ _______
|     __|    ___|   _   |   |   |
|__     |    ___|       |       |
|_______|_______|___|___|__|_|__|

获取直播源地址, 目前支持 bilibili, 抖音, 艺气山, 棉花糖", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// 获取B站直播源地址
    Bili {
        /// 房间号
        rid: String,
    },
    /// 获取抖音直播源地址
    Douyin {
        /// 房间号
        rid: String,
    },
    /// 获取艺气山直播源地址
    Yqs {
        /// 房间号
        rid: String,
    },
    /// 获取棉花糖直播源地址
    Mht {
        /// 房间号
        rid: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::Bili { rid } => util::match_show_type(live::bilibili::get(&rid).await?),
        Commands::Douyin { rid } => util::match_show_type(live::douyin::get(&rid).await?),
        Commands::Yqs { rid } => util::match_show_type(live::yqs::get(&rid).await?),
        Commands::Mht { rid } => util::match_show_type(live::mht::get(&rid).await?),
    }
    Ok(())
}

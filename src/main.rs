mod common;
mod live;
mod modle;
mod util;

use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};

/// 获取直播源
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "seam")]
#[command(about ="
________ _______ _______ _______
|     __|    ___|   _   |   |   |
|__     |    ___|       |       |
|_______|_______|___|___|__|_|__|

获取直播源地址, 目前支持 B站, 斗鱼, 抖音, 虎牙, 艺气山, 棉花糖", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// B站
    Bili {
        /// 房间号
        rid: String,
    },
    /// 斗鱼
    Douyu {
        /// 房间号
        rid: String,
    },
    /// 抖音
    Douyin {
        /// 房间号
        rid: String,
    },
    /// 虎牙
    Huya {
        /// 房间号
        rid: String,
    },
    /// 花椒
    Huajiao {
        /// 房间号
        rid: String,
    },
    /// 艺气山
    Yqs {
        /// 房间号
        rid: String,
    },
    /// 棉花糖
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
        Commands::Douyu { rid } => util::match_show_type(live::douyu::get(&rid).await?),
        Commands::Douyin { rid } => util::match_show_type(live::douyin::get(&rid).await?),
        Commands::Huya { rid } => util::match_show_type(live::huya::get(&rid).await?),
        Commands::Huajiao { rid } => util::match_show_type(live::huajiao::get(&rid).await?),
        Commands::Yqs { rid } => util::match_show_type(live::yqs::get(&rid).await?),
        Commands::Mht { rid } => util::match_show_type(live::mht::get(&rid).await?),
    }
    Ok(())
}

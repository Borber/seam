mod live;
mod modle;
mod util;

use anyhow::{Ok, Result};
use clap::{Args, Parser, Subcommand, ValueEnum};
use reqwest::Client;

/// 获取直播源
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "seam")]
#[command(about = "获取直播源地址, 目前支持 bilibili, 艺气山, 棉花糖", long_about = None)]
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
    /// 获取艺气山直播源地址
    Yqs { rid: String },
    /// 获取棉花糖直播源地址
    Mht { rid: String },
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();
    let args = Cli::parse();
    match args.command {
        Commands::Bili { rid } => util::match_show_type(live::bilibili::get(&rid, &client).await?),
        Commands::Yqs { rid } => util::match_show_type(live::yqs_173::get(&rid, &client).await?),
        Commands::Mht { rid } => util::match_show_type(live::mht_2cq::get(&rid, &client).await?),
    }
    Ok(())
}

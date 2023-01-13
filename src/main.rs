mod common;
mod live;
mod modle;
mod util;

use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};
use paste::paste;

/// 获取直播源
#[derive(Debug, Parser)] // requires `derive` feature
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
    /// 快手
    Kuaishou {
        /// 房间号
        rid: String,
    },
    /// 网易CC
    Cc {
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
    get_resource_impl!(Cli::parse().command; Bili, Douyu, Douyin, Huya, Kuaishou, Cc, Huajiao, Yqs, Mht);
    Ok(())
}

// 适配不同直播平台，获取直播源地址
// 直白的实现方式，fmt以及具体放置位置有待reviewer决定以及优化
// 使用方法：get_resource_impl!(args.command; Bili, Douyu, Douyin, Huya, Kuaishou, Cc, Huajiao, Yqs, Mht)
#[macro_export]
macro_rules! get_resource_impl {
    ($args: expr; $($name: ident),*) => {
        paste! {
            match $args {
                $(
                    Commands::$name { rid } => {
                        util::match_show_type(live::[<$name: lower>]::get(&rid).await?)
                    }
                )*
            }
        }
    }
}

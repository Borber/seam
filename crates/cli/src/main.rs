mod common;
mod config;
mod util;

use crate::{common::GLOBAL_CLIENT, config::CONFIG};
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
    /// 平台名称
    #[arg(short = 'l', global = true)]
    live: String,
    /// 直播间号
    #[arg(short = 'i', global = true)]
    id: String,
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

// 获取直播源的实现
pub async fn cli() -> Result<()> {
    let args = Cli::parse();
    let live = args.live;
    let rid = args.id;

    let node = GLOBAL_CLIENT
        .get(&live)
        .unwrap()
        .get(&rid, &CONFIG.cookie.get(&live))
        .await;

    let node = match node {
        Ok(node) => node,
        Err(e) => {
            println!("{}", e.to_string());
            return Ok(());
        }
    };

    let record = args.record;

    // 无参数情况下，直接输出直播源信息
    if !(args.danmu || args.config_danmu || record || args.auto_record) {
        println!("{}", &node.json());
        return Ok(());
    }

    // 收集不同参数功能的异步线程handler
    let mut thread_handlers = vec![];

    // 处理参数-d，直接输出弹幕。
    // 由于该函数为cli层，所以出错可以直接panic。
    if args.danmu {
        let h = tokio::spawn(async move {
            // args.command
            //     .danmu(vec![&Terminal::try_new(None).unwrap()])
            //     .await
            //     .unwrap();
            println!("弹幕功能正在重构中，敬请期待") // TODO
        });
        thread_handlers.push(h);
    }
    tokio::select! {
        _ = async {
            for h in thread_handlers {
                h.await.unwrap();
            }
        } => {}
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    cli().await?;
    Ok(())
}

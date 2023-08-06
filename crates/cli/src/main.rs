mod common;
mod config;
mod util;

use crate::{common::GLOBAL_CLIENT, config::CONFIG};
use anyhow::{anyhow, Result};
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
    #[arg(short = 'l')]
    live: String,
    /// 直播间号
    #[arg(short = 'i')]
    rid: Option<String>,
    /// 直接录播功能
    #[arg(short = 'r')]
    record: bool,
    /// 自动监控录播功能
    #[arg(short = 'R')]
    auto_record: bool,
    /// 输出到终端的弹幕功能
    #[arg(short = 'd')]
    danmu: bool,
    /// 根据参数指定的文件地址输出弹幕
    #[arg(short = 'D')]
    config_danmu: bool,
}

// 获取直播源的实现
pub async fn cli() -> Result<()> {
    let Cli {
        live,
        record,
        rid,
        danmu,
        config_danmu,
        auto_record,
    } = Cli::parse();

    if live == "list" {
        println!(
            "可用平台：{}",
            GLOBAL_CLIENT.keys().cloned().collect::<Vec<_>>().join(", ")
        );
        return Ok(());
    }

    let node = match (GLOBAL_CLIENT.get(&live), &rid) {
        (Some(client), Some(rid)) => client.get(rid, &CONFIG.cookie.get(&live)).await,
        (Some(_), None) => return Err(anyhow!("请传递 -i 参数")),
        (None, _) => {
            return Err(anyhow!(
                "请检查 {} 是否为可用平台, 或前往 https://github.com/Borber/seam/issues 申请支持",
                live
            ))
        }
    };

    // 无参数情况下，直接输出直播源信息
    if !(danmu || config_danmu || record || auto_record) {
        println!("{}", node?.json());
        return Ok(());
    }

    // 收集不同参数功能的异步线程 handler
    let mut thread_handlers = vec![];

    // 处理参数-d，直接输出弹幕。
    // 由于该函数为cli层，所以出错可以直接panic。
    if danmu {
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

use crate::Cli;

use anyhow::{Ok, Result};
use clap::Parser;

use seam_cli_marcos::LivesToCommands;
use seam_danmu::{DanmuRecorder, DanmuTrait, Terminal};

#[derive(LivesToCommands)]
#[allow(dead_code)]
pub enum Lives {
    /// B站
    Bili,
    /// 斗鱼
    Douyu,
    /// 虎牙
    Huya,
    /// 抖音
    Douyin,
    /// 快手
    Ks,
    /// 花椒
    Huajiao,
    /// 网易CC
    Cc,
    /// 映客
    Inke,
    /// NOW
    Now,
    /// 棉花糖
    Mht,
    /// 艺气山
    Yqs,
    /// kk
    Kk,
    /// 千帆
    Qf,
    /// winktv
    Wink,
    /// panda
    Panda,
    /// flextv
    Flex,
    /// afreecatv
    Afreeca,
}

// 获取直播源的实现
pub async fn cli() -> Result<()> {
    let args = Cli::parse();
    println!("{:#?}", args.command.to_string());

    let node = args.command.get().await?;
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
            args.command
                .danmu(vec![&Terminal::try_new(None).unwrap()])
                .await
                .unwrap();
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

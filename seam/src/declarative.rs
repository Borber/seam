use crate::{common::GLOBAL_CLIENT, Cli};

use anyhow::Result;
use clap::Parser;

// 获取直播源的实现
pub async fn cli() -> Result<()> {
    let args = Cli::parse();
    let live = args.live;
    let rid = args.id;
    let node = GLOBAL_CLIENT.get(&live).unwrap().get(&rid, None).await;

    let node = match node {
        Ok(node) => node,
        Err(e) => panic!("直播源获取失败，可能是直播间号错误或者平台不支持该直播间号。直播间号：{}，平台：{}，错误信息：{}", rid, live, e),
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

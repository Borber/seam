use std::path::PathBuf;

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};

use seam_core::live::bili::Bili;
use seam_core::live::Live;
use seam_core::{config::CONFIG, recorder, util::get_datetime};
use seam_danmu::bili::BiliDanmuClient;
use seam_danmu::{Csv, Danmu, DanmuRecorder, Terminal};
use seam_status::bili::BiliStatusClient;
use seam_status::Client;

use crate::Cli;

// 声明宏：获取直播源的command的实现
#[derive(Debug, Subcommand)]
pub enum Commands {
    Bili {
        /// 房间号
        rid: String,
        // 输出到终端的弹幕功能
        #[arg(short = 'd')]
        danmu: bool,
        // 根据参数指定的文件地址输出弹幕
        #[arg(short = 'D')]
        config_danmu: bool,
        // 直接录播功能
        #[arg(short = 'r')]
        record: bool,
        // 自动监控录播功能
        #[arg(short = 'R')]
        auto_record: bool,
    },
}

// 获取直播源的实现
pub async fn get_source_url() -> Result<()> {
    match Cli::parse().command {
        Commands::Bili {
            rid,
            danmu,
            config_danmu,
            mut record,
            auto_record,
        } => {
            let node = Bili::get(&rid).await?;

            // 无参数情况下，直接输出直播源信息
            if !(danmu || config_danmu || record || auto_record) {
                println!("{}", &node.json());
                return Ok(());
            }

            // 排斥参数检查
            // 1. 自动监控录播当检查到当前在直播时，应当自动开启下载任务，所以record选项此时是多余的
            if auto_record {
                record = false;
            }

            // 收集不同参数功能的异步线程handler
            let mut thread_handlers = vec![];

            // 处理参数-d，直接输出弹幕。
            // 由于该函数为cli层，所以出错可以直接panic。
            if danmu {
                let rid = node.rid.clone();
                let h = tokio::spawn(async move {
                    BiliDanmuClient::start(&rid, vec![&Terminal::try_new(None).unwrap()])
                        .await
                        .unwrap();
                });
                thread_handlers.push(h);
            }

            // 处理参数-D，输出弹幕到指定文件。
            if config_danmu {
                let node_clone = node.clone();
                let h = tokio::spawn(async move {
                    let file_name = CONFIG
                        .danmu
                        .name
                        .replace("rid", &node_clone.rid)
                        .replace("time", &get_datetime())
                        .replace("title", &node_clone.title);
                    println!("弹幕文件地址：{}", file_name);
                    let cwd = std::env::current_exe().unwrap();
                    let path =
                        PathBuf::from(cwd.parent().ok_or(anyhow!("错误的弹幕记录地址。")).unwrap())
                            .join(file_name);
                    BiliDanmuClient::start(
                        &node_clone.rid,
                        vec![&Csv::try_new(Some(path)).unwrap()],
                    )
                    .await
                    .unwrap();
                });
                thread_handlers.push(h);
            }

            // 处理参数-r，录制直播源。
            if record {
                let node_clone = node.clone();
                let h = tokio::spawn(async move {
                    let file_name = CONFIG
                        .video
                        .name
                        .replace("rid", &node_clone.rid)
                        .replace("time", &get_datetime())
                        .replace("title", &node_clone.title);
                    let file_name = format!("{file_name}.mp4");
                    if let Some(url) = node_clone.urls.iter().find_map(|url| url.is_m3u8().ok()) {
                        recorder::record(&url, &file_name).await;
                    } else {
                        return;
                    }
                });
                thread_handlers.push(h);
            }

            // 处理参数-R，自动监控录制直播。
            //
            // 5秒检查一次是否开启直播。 TODO 支持自定义检查间隔
            if auto_record {
                let node_clone = node.clone();
                let h = tokio::spawn(async move {
                    let node = node_clone;
                    loop {
                        match BiliStatusClient::status(&node.rid).await {
                            Ok(true) => {
                                let file_name = CONFIG
                                    .video
                                    .name
                                    .replace("rid", &node.rid)
                                    .replace("time", &get_datetime())
                                    .replace("title", &node.title);
                                let file_name = format!("{file_name}.mp4");
                                if let Some(url) =
                                    node.urls.iter().find_map(|url| url.is_m3u8().ok())
                                {
                                    recorder::record(&url, &file_name).await;
                                } else {
                                    return;
                                }
                            }
                            _ => tokio::time::sleep(tokio::time::Duration::from_secs(5)).await,
                        }
                    }
                });
                thread_handlers.push(h);
            }

            if auto_record {
                tokio::select! {
                    _ = async {
                        for h in thread_handlers {
                            h.await.unwrap();
                        }
                    } => {}
                }
            } else {
                let rid = node.clone().rid;
                // 检查直播间是否开播
                let on_air_checker = async {
                    loop {
                        // TODO 是否可优化
                        let rid = rid.clone();
                        match BiliStatusClient::status(&rid).await {
                            Ok(true) => {
                                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                            }
                            _ => break,
                        }
                    }
                };

                // select
                tokio::select! {
                    _ = on_air_checker => {
                        println!("主线程退出。");
                        println!("直播间已关闭。");
                    },
                    _ = async {
                        for h in thread_handlers {
                            h.await.unwrap();
                        }
                    } => {}
                }
            }
            Ok(())
        }
    }
}

// 为没有实现弹幕功能的直播平台添加默认空白实现
#[macro_export]
macro_rules! default_danmu_client {
    ($name: ident) => {
        use paste::paste;

        paste! {
            use async_trait::async_trait;
            use $crate::danmu::{Danmu, DanmuRecorder};

            pub struct [<$name DanmuClient>] {}

            impl [<$name DanmuClient>] {
                pub async fn try_new(_room_id: &str) -> Result<Self> {
                    Ok(Self {})
                }
            }

            #[async_trait]
            impl Danmu for [<$name DanmuClient>] {
                async fn start(&mut self, _recorder: Vec<&dyn DanmuRecorder>) -> Result<()> {
                    println!("该直播平台暂未实现弹幕功能。");
                    Ok(())
                }
            }
        }
    };
}

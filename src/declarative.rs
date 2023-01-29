use crate::{
    config,
    danmu::{Csv, Danmu, DanmuRecorder, Terminal},
    live, model, recorder,
    util::get_datetime,
    Cli,
};

use std::path::PathBuf;

use anyhow::{anyhow, Ok, Result};
use clap::{Parser, Subcommand};
use paste::paste;
use rand::prelude::*;

// 声明宏：获取直播源的command的实现
macro_rules! get_source_url_command {
    ($($name: ident), *) => {

        #[derive(Debug, Subcommand)]
        pub enum Commands {
            $(
                $name {
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
            )*
        }

        // 获取直播源的实现
        pub async fn get_source_url() -> Result<()> {
            paste! {
                match Cli::parse().command {
                    $(
                        Commands::$name { rid, danmu, config_danmu, mut record, auto_record } => {
                            // 无参数情况下，直接输出直播源信息
                            if !(danmu || config_danmu || record || auto_record) {
                                println!("{}", live::[<$name: lower>]::get(&rid).await?);
                                return Ok(());
                            }

                            // 检查房间参数是否正确
                            let live_info = live::[<$name: lower>]::get(&rid).await?;
                            if live_info.is_bad_rid() {
                                println!("{live_info}");
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
                                let rid_clone = rid.clone();
                                let h = tokio::spawn(async move {
                                    let mut danmu_client = live::[<$name: lower>]::[<$name DanmuClient>]::try_new(&rid_clone).await.unwrap();
                                    danmu_client.start(vec![&Terminal::try_new(None).unwrap()]).await.unwrap();
                                });
                                thread_handlers.push(h);
                            }

                            // 处理参数-D，输出弹幕到指定文件。
                            if config_danmu {
                                let rid_clone = rid.clone();
                                let h = tokio::spawn(async move {
                                    let mut danmu_client = live::[<$name: lower>]::[<$name DanmuClient>]::try_new(&rid_clone).await.unwrap();
                                    let cwd = std::env::current_exe().unwrap(); // 对于MACOS，CWD可执行文件目录，所以需要使用current_exe
                                    let room_info = live::[<$name: lower>]::get(&rid_clone).await.unwrap();
                                    let room_title = room_info.get_room_title().or(Some("未知直播标题")).unwrap();
                                    let file_name = config::get_config().await.unwrap().danmu.name.replace("rid", &rid_clone).replace("time", &get_datetime()).replace("title", room_title);
                                    println!("弹幕文件地址：{}", file_name);
                                    let path = PathBuf::from(cwd.parent().ok_or(anyhow!("错误的弹幕记录地址。")).unwrap()).join(file_name);
                                    danmu_client.start(vec![&Csv::try_new(Some(path)).unwrap()]).await.unwrap();
                                });
                                thread_handlers.push(h);
                            }

                            // 处理参数-r，录制直播源。
                            if record {
                                let rid_clone = rid.clone();
                                let h = tokio::spawn(async move {
                                    let room_info = live::[<$name: lower>]::get(&rid_clone).await.unwrap();
                                    let room_title = room_info.get_room_title().or(Some("未知直播标题")).unwrap();
                                    let file_name = config::get_config().await.unwrap().video.name.replace("rid", &rid_clone).replace("time", &get_datetime()).replace("title", room_title);
                                    let file_name = format!("{file_name}.mp4");
                                    match room_info {
                                        model::ShowType::On(detail) => {
                                            if let Some(url) = detail.iter().find_map(|node| node.is_m3u8().ok()) {
                                                recorder::record(&url, &file_name).await;
                                            } else {
                                                return;
                                            }
                                        },
                                        _ => { return; }
                                    };
                                });
                                thread_handlers.push(h);
                            }

                            // 处理参数-R，自动监控录制直播。
                            //
                            // 3~9分钟检查一次是否开启直播。
                            if auto_record {
                                let rid_clone = rid.clone();
                                let h = tokio::spawn(async move {
                                    loop {
                                        if let std::result::Result::Ok(live_info) = live::[<$name: lower>]::get(&rid_clone).await {
                                            if !live_info.is_on() {
                                                continue;
                                            } else {
                                                let room_info = live::[<$name: lower>]::get(&rid_clone).await.unwrap();
                                                let room_title = room_info.get_room_title().or(Some("未知直播标题")).unwrap();
                                                let file_name = format!("{}-{}-{}.mp4", &rid_clone, get_datetime(), room_title);
                                                match room_info {
                                                    model::ShowType::On(detail) => {
                                                        if let Some(url) = detail.iter().find_map(|node| node.is_m3u8().ok()) {
                                                            recorder::record(&url, &file_name).await;
                                                        } else {
                                                            return;
                                                        }
                                                    },
                                                    _ => { return; }
                                                };
                                            }
                                        }
                                        let waiting_time = rand::thread_rng().gen_range(180..600);
                                        tokio::time::sleep(tokio::time::Duration::from_secs(waiting_time)).await;
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
                                // 检查直播间是否开播
                                let on_air_checker = async {
                                    loop {
                                        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                                        if let std::result::Result::Ok(live_info) = live::[<$name: lower>]::get(&rid).await {
                                            if !live_info.is_on() {
                                                break;
                                            }
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
                        }
                    )*
                }
            }
            Ok(())
        }

    }
}

// 展开宏命令
// 添加新的直播平台可以在括号末尾添加，并在live文件夹里添加对应的文件
get_source_url_command!(
    Bili, Douyu, Douyin, Huya, Kuaishou, Cc, Huajiao, Kk, Qf, Yqs, Mht, Now, Afreeca, Panda, Flex,
    Wink
);

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

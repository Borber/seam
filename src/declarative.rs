use crate::{
    danmu::{self, Danmu},
    live, Cli,
};
use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};
use paste::paste;
use std::path::PathBuf;

// 声明宏：获取直播源的command的实现
macro_rules! get_source_url_command {
    ($($name: ident), *) => {

        #[derive(Debug, Subcommand)]
        pub enum Commands {
            $(
                $name {
                    /// 房间号
                    rid: String,
                    // 弹幕功能
                    #[arg(short = 'd')]
                    danmu: Option<PathBuf>,
                },
            )*
        }

        // 获取直播源的实现
        pub async fn get_source_url() -> Result<()> {
            paste! {
                match Cli::parse().command {
                    $(
                        Commands::$name { rid, danmu } => {
                            println!("{}", live::[<$name: lower>]::get(&rid).await?);
                            // TODO: 目前无视输出参数，直接输出到终端，后期需要保存到参数指定的文件地址
                            if let Some(_danmu_path) = danmu {
                                let mut danmu_client = live::[<$name: lower>]::[<$name DanmuClient>]::new(&rid).await;
                                danmu_client.start(danmu::DanmuRecorder::Terminal).await?;
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
    Bili, Douyu, Douyin, Huya, Kuaishou, Cc, Huajiao, Kk, Qf, Yqs, Mht, Now, Afreeca, Panda, Flex, Wink
);

// 为没有实现弹幕功能的直播平台添加默认空白实现
#[macro_export]
macro_rules! default_danmu_client {
    ($name: ident) => {
        use paste::paste;

        paste! {
            use async_trait::async_trait;
            use crate::danmu::{Danmu, DanmuRecorder};

            pub struct [<$name DanmuClient>] {}

            impl [<$name DanmuClient>] {
                pub async fn new(_room_id: &str) -> Self {
                    Self {}
                }
            }

            #[async_trait]
            impl Danmu for [<$name DanmuClient>] {
                async fn start(&mut self, _recorder: DanmuRecorder) -> Result<()> {
                    println!("该直播平台暂未实现弹幕功能。");
                    Ok(())
                }
            }
        }
    };
}

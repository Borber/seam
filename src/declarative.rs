use crate::{
    danmu::{self, Danmu},
    live, Cli,
};
use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};
use paste::paste;

// 声明宏：获取直播源的command的实现
macro_rules! get_source_url_command {
    ($($name: ident), *) => {

        #[derive(Debug, Subcommand)]
        pub enum Commands {
            /// 获取B站弹幕
            Bilidanmu {
                /// 房间号
                rid: String
            },
            $(
                $name {
                    /// 房间号
                    rid: String,
                },
            )*
        }

        // 获取直播源的实现
        pub async fn get_source_url() -> Result<()> {
            paste! {
                match Cli::parse().command {
                    Commands::Bilidanmu { rid } => {
                        let room_id = live::bili::get_real_room_id(&rid).await.unwrap();
                        let mut danmu_client = live::bili::BilibiliDanmuClient::new(&room_id);
                        danmu_client.start(danmu::DanmuRecorder::Terminal).await?;
                    },
                    $(
                        Commands::$name { rid } => {
                            println!("{}", live::[<$name: lower>]::get(&rid).await?);
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
    Bili, Douyu, Douyin, Huya, Kuaishou, Cc, Huajiao, Yqs, Mht, Now, Afreeca, Panda, Flex, Wink
);

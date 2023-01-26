use crate::{
    danmu::{Csv, Danmu, DanmuRecorder, Terminal},
    live,
    util::get_datetime,
    Cli,
};
use anyhow::{anyhow, Ok, Result};
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
                    // 输出到终端的弹幕功能
                    #[arg(short = 'd')]
                    danmu: bool,
                    // 根据参数指定的文件地址输出弹幕
                    #[arg(short = 'D')]
                    config_danmu: bool
                },
            )*
        }

        // 获取直播源的实现
        pub async fn get_source_url() -> Result<()> {
            paste! {
                match Cli::parse().command {
                    $(
                        Commands::$name { rid, danmu, config_danmu } => {
                            // 参数D为最高优先级
                            // 参数d为次高优先级
                            // 两个参数都没有时，直接输出直播源信息
                            if config_danmu {
                                let mut danmu_client = live::[<$name: lower>]::[<$name DanmuClient>]::try_new(&rid).await?;
                                let cwd = std::env::current_exe()?; // 对于MACOS，CWD可执行文件目录，所以需要使用current_exe
                                let room_info = live::[<$name: lower>]::get(&rid).await?;
                                let room_title = room_info.get_room_title().or(Some("未知直播标题")).unwrap();
                                let file_name = format!("{}-{}-{}", rid, get_datetime(), room_title);
                                let path = PathBuf::from(cwd.parent().ok_or(anyhow!("错误的弹幕记录地址。"))?).join(file_name);
                                danmu_client.start(vec![&Csv::try_new(Some(path))?]).await?;
                            } else if danmu {
                                let mut danmu_client = live::[<$name: lower>]::[<$name DanmuClient>]::try_new(&rid).await?;
                                danmu_client.start(vec![&Terminal::try_new(None)?]).await?;
                            } else {
                                println!("{}", live::[<$name: lower>]::get(&rid).await?);
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

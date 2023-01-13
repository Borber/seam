use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
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

// 适配不同直播平台，获取直播源地址
// 直白的实现方式，fmt以及具体放置位置有待reviewer决定以及优化
// 使用方法：get_resource_impl!(args.command; Bili, Douyu, Douyin, Huya, Kuaishou, Cc, Huajiao, Yqs, Mht)
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

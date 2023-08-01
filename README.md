```
 _______ _______ _______ _______
|     __|    ___|   _   |   |   |
|__     |    ___|       |       |
|_______|_______|___|___|__|_|__|
```

[![Github]][Repo] [![License]][Repo] [![Downloads]][Release]

[Repo]: https://github.com/Borber/seam
[Github]: https://img.shields.io/badge/github-Borber/seam-8da0cb.svg?style=for-the-badge&logo=github
[Downloads]: https://img.shields.io/github/downloads/Borber/seam/total.svg?style=for-the-badge&color=82E0AA&logo=github
[Release]: https://github.com/Borber/seam/releases/latest
[License]: https://img.shields.io/github/license/borber/seam?color=%2398cbed&logo=rust&style=for-the-badge

原 `SBtream` 项目, 经历 python 不成熟的模仿, Java 重构烂尾, 目前使用 rust 进行重构开发

多平台直播源地址获取

-   目前直播录制需要自行放置 ffmpeg 到 `seam` 可执行文件所在目录下, 未正式支持, 后续将优化此模块

# 当前聚焦 - GUI

![GUI](assets/gui.png)

目前提供了简单的 GUI 界面, 进一步降低使用门槛

基于 tauri 开发. 目前仅支持 Windows 平台 ,只是单纯的还没写 linux, macos 的 打包脚本, 有能力的小伙伴可以 `clone` 项目后自行打包.

如果你是 win11, 或 win10 以下但安装过 webview2 可以直接使用, 否则你应该安装它, 下载链接: [WebView2](https://developer.microsoft.com/zh-cn/microsoft-edge/webview2/#download-section)

**注意事项: 目前抖音和快手因为 cookie 模块的加入进行了较大修改, 所以目前请不要使用 GUI 获取这两个平台的直播源, 以及 目前仅 B 站和斗鱼支持直播间标题获取, 其他平台还需要后续更新提供支持**

当前 GUI 界面 仅为早期版本, 后期会进行较大修改, 主播头像，直播封面，主播名称，全平台订阅，开播通知，自动录播 都会有的.

使用中出现任何问题都可以提 issue, 或加入 TG 群进行反馈: [Telegram](https://t.me/seam_rust)

下载链接: [Releases · seam GUI 预览版 v0.1.0](https://github.com/Borber/seam/releases/tag/pre-gui-v0.1.0)

# 使用样例

```bash
❯ .\seam.exe douyu 88080
[  {
    "rate": "超清1",
    "url": "http://url1"
  },
  {
    "rate": "超清2",
    "url": "http://url2"
  }
]
```

> 因为数据具有时效性, 所以具体链接使用 `url` 进行替换

# 使用

-   [Releases · seam](https://github.com/Borber/seam/releases) 下载你对应平台的压缩包

-   解压后

|                 **平台**                  | **子命令** |                             **`<RID>` 位置**                             | **弹幕** | **房间名获取** |
| :---------------------------------------: | :--------: | :----------------------------------------------------------------------: | :------: | :------------: |
|    [B 站](https://live.bilibili.com/)     |    bili    |                    `https://live.bilibili.com/<RID>`                     |    ✅    |       ✅       |
|      [斗鱼](https://www.douyu.com/)       |   douyu    | `https://www.douyu.com/<RID>` 或 `https://www.douyu.com/xx/xx?rid=<RID>` |          |       ✅       |
|     [抖音](https://live.douyin.com/)      |   douyin   |                     `https://live.douyin.com/<RID>`                      |          |                |
|         [虎牙](https://huya.com/)         |    huya    |                         `https://huya.com/<RID>`                         |          |                |
|    [快手](https://live.kuaishou.com/)     |  kuaishou  |                   `https://live.kuaishou.com/u/<RID>`                    |          |                |
|         [CC](https://cc.163.com/)         |     cc     |                        `https://cc.163.com/<RID>`                        |          |                |
|     [花椒](https://www.huajiao.com/)      |  huajiao   |                    `https://www.huajiao.com/l/<RID>`                     |          |                |
|      [艺气山](https://www.173.com/)       |    yqs     |                       `https://www.173.com/<RID>`                        |          |                |
|      [棉花糖](https://www.2cq.com/)       |    mht     |                       `https://www.2cq.com/<RID>`                        |          |                |
|       [kk](https://www.kktv5.com/)        |     kk     |                    `https://www.kktv5.com/show/<RID>`                    |          |                |
|      [千帆直播](https://qf.56.com/)       |     qf     |                        `https://qf.56.com/<RID>`                         |          |       ✅       |
|      [Now 直播](https://now.qq.com/)      |    now     |            `https://now.qq.com/pcweb/story.html?roomid=<RID>`            |          |                |
|       [映客](https://www.inke.cn/)        |    inke    |           `https://www.inke.cn/liveroom/index.html?uid=<RID>`            |          |       ✅       |
|     [afreeca](https://afreecatv.com/)     |  afreeca   |                     `https://bj.afreecatv.com/<RID>`                     |          |                |
| [pandalive](https://www.pandalive.co.kr/) |   panda    |               `https://www.pandalive.co.kr/channel/<RID>`                |          |                |
|     [flex](https://www.flextv.co.kr/)     |    flex    |                `https://www.flextv.co.kr/channels/<RID>`                 |          |                |
|     [wink](https://www.winktv.co.kr/)     |    wink    |                 `https://www.winktv.co.kr/channel/<RID>`                 |          |                |

# 设置

`config.toml` 放置在 `seam` 可执行文件所在目录下

```toml
# [rid]: 房间号
# [title]: 标题
# [time]: 时间戳
# [date]: 日期

[file_name]
# 录制文件标题
video = "[rid]-[title]-[date]-[time]"
# danmu文件标题
danmu = "[rid]-[title]-[date]-[time]"

# 各平台cookie
[cookie.bili]
cookie = "xxxx"
user-agent = "xxxx"

[cookie.huya]
cookie = "xxxx"
user-agent = "xxxx"
```

# 路线

[seam](https://github.com/users/Borber/projects/4/views/1)

# 相关项目

-   [seamui](https://github.com/kirito41dd/seamui) 由 [kirito41dd](https://github.com/kirito41dd) 开发的`seam`图形化界面

## 贡献者

[![GitHub Contributors](https://contrib.rocks/image?repo=Borber/seam)](https://github.com/Borber/seam/graphs/contributors)

# 感谢

-   [wbt5/real-url](https://github.com/wbt5/real-url/)
-   [banner](https://textkool.com/en/ascii-art-generator?hl=default&vl=default&font=Chunky&text=SEAM)
-   [手把手教你破解斗鱼 sign 算法](https://zhuanlan.zhihu.com/p/107330805)

## Star History

<a href="https://github.com/Borber/seam/stargazers">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=Borber/seam&type=Date&theme=dark" />
    <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=Borber/seam&type=Date" />
    <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=Borber/seam&type=Date" />
  </picture>
</a>

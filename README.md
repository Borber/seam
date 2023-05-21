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

|                   平台                    |  子命令  |                               `<RID>` 说明                               |                                         备注                                          |
| :---------------------------------------: | :------: | :----------------------------------------------------------------------: | :-----------------------------------------------------------------------------------: |
|    [B 站](https://live.bilibili.com/)     |   bili   |                    `https://live.bilibili.com/<RID>`                     |                                                                                       |
|      [斗鱼](https://www.douyu.com/)       |  douyu   | `https://www.douyu.com/<RID>` 或 `https://www.douyu.com/xx/xx?rid=<RID>` | 需要 [Jin](https://github.com/Borber/Jin/releases/latest) 运行时 放置于 seam 同级目录 |
|     [抖音](https://live.douyin.com/)      |  douyin  |                     `https://live.douyin.com/<RID>`                      |                                                                                       |
|         [虎牙](https://huya.com/)         |   huya   |                         `https://huya.com/<RID>`                         |                                                                                       |
|    [快手](https://live.kuaishou.com/)     | kuaishou |                   `https://live.kuaishou.com/u/<RID>`                    |                                                                                       |
|         [CC](https://cc.163.com/)         |    cc    |                        `https://cc.163.com/<RID>`                        |                                                                                       |
|     [花椒](https://www.huajiao.com/)      | huajiao  |                    `https://www.huajiao.com/l/<RID>`                     |                                                                                       |
|      [艺气山](https://www.173.com/)       |   yqs    |                       `https://www.173.com/<RID>`                        |                                                                                       |
|      [棉花糖](https://www.2cq.com/)       |   mht    |                       `https://www.2cq.com/<RID>`                        |                                                                                       |
|       [kk](https://www.kktv5.com/)        |    kk    |                    `https://www.kktv5.com/show/<RID>`                    |                                                                                       |
|      [千帆直播](https://qf.56.com/)       |    qf    |                        `https://qf.56.com/<RID>`                         |                                                                                       |
|      [Now 直播](https://now.qq.com/)      |   now    |            `https://now.qq.com/pcweb/story.html?roomid=<RID>`            |                                                                                       |
|       [映客](https://www.inke.cn/)        |   inke   |           `https://www.inke.cn/liveroom/index.html?uid=<RID>`            |                                                                                       |
|     [afreeca](https://afreecatv.com/)     | afreeca  |                     `https://bj.afreecatv.com/<RID>`                     |                                 主播名字而非直播间号                                  |
| [pandalive](https://www.pandalive.co.kr/) |  panda   |               `https://www.pandalive.co.kr/channel/<RID>`                |                                 主播名字而非直播间号                                  |
|     [flex](https://www.flextv.co.kr/)     |   flex   |                `https://www.flextv.co.kr/channels/<RID>`                 |                                 主播名字而非直播间号                                  |
|     [wink](https://www.winktv.co.kr/)     |   wink   |                 `https://www.winktv.co.kr/channel/<RID>`                 |                                 主播名字而非直播间号                                  |

# 设置

`config.json` 放置在 `seam` 可执行文件所在目录下

```json
{
    "video": {
        "name": "rid-title-time"
    },
    "danmu": {
        "name": "rid-title-time"
    }
}
```

-   video: 视频文件名设置
-   danmu: 弹幕文件名设置
-   rid: 直播间号
-   title: 直播间标题
-   time: 当前时间

# 路线

[seam](https://github.com/users/Borber/projects/4/views/1)

# 感谢

1. [wbt5/real-url](https://github.com/wbt5/real-url/)
2. [banner](https://textkool.com/en/ascii-art-generator?hl=default&vl=default&font=Chunky&text=SEAM)
3. [手把手教你破解斗鱼 sign 算法](https://zhuanlan.zhihu.com/p/107330805)

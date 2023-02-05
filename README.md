```
 _______ _______ _______ _______
|     __|    ___|   _   |   |   |
|__     |    ___|       |       |
|_______|_______|___|___|__|_|__|
```

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

## 设置

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

# 使用

|                   平台                    |                                房间号位置                                |
| :---------------------------------------: | :----------------------------------------------------------------------: |
|    [B 站](https://live.bilibili.com/)     |                    `https://live.bilibili.com/<RID>`                     |
|      [斗鱼](https://www.douyu.com/)       | `https://www.douyu.com/<RID>` 或 `https://www.douyu.com/xx/xx?rid=<RID>` |
|     [抖音](https://live.douyin.com/)      |                     `https://live.douyin.com/<RID>`                      |
|         [虎牙](https://huya.com/)         |                         `https://huya.com/<RID>`                         |
|    [快手](https://live.kuaishou.com/)     |                   `https://live.kuaishou.com/u/<RID>`                    |
|         [CC](https://cc.163.com/)         |                        `https://cc.163.com/<RID>`                        |
|     [花椒](https://www.huajiao.com/)      |                    `https://www.huajiao.com/l/<RID>`                     |
|      [艺气山](https://www.173.com/)       |                       `https://www.173.com/<RID>`                        |
|      [棉花糖](https://www.2cq.com/)       |                       `https://www.2cq.com/<RID>`                        |
|       [kk](https://www.kktv5.com/)        |                    `https://www.kktv5.com/show/<RID>`                    |
|      [千帆直播](https://qf.56.com/)       |                        `https://qf.56.com/<RID>`                         |
|      [Now 直播](https://now.qq.com/)      |            `https://now.qq.com/pcweb/story.html?roomid=<RID>`            |
|     [afreeca](https://afreecatv.com/)     |          `https://bj.afreecatv.com/<RID>` 主播名字而非直播间号           |
| [pandalive](https://www.pandalive.co.kr/) |     `https://www.pandalive.co.kr/channel/<RID>` 主播名字而非直播间号     |
|     [flex](https://www.flextv.co.kr/)     |      `https://www.flextv.co.kr/channels/<RID>` 主播名字而非直播间号      |
|     [wink](https://www.winktv.co.kr/)     |      `https://www.winktv.co.kr/channel/<RID>` 主播名字而非直播间号       |

# 路线

[seam](https://github.com/users/Borber/projects/4/views/1)

# 说明

1. 斗鱼平台动态 js 加密需要 js 运行时, 目前从在线 js 运行转换问 boa_engine, 减少了网络请求的时间.
2. 抖音平台连接中包含`stage` 时需要后缀参数且标签如`_or4`无法删除, 因为签名和清晰度是绑定的, 工具默认获取的链接就是能获取到的最高清的, 请直接使用完整链接, 包含`third` 时, 自动删除后缀参数和清晰度标签以获取最高清资源.

# 感谢

1. [wbt5/real-url](https://github.com/wbt5/real-url/)
2. [banner](https://textkool.com/en/ascii-art-generator?hl=default&vl=default&font=Chunky&text=SEAM)
3. [手把手教你破解斗鱼 sign 算法](https://zhuanlan.zhihu.com/p/107330805)

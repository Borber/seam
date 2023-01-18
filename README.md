```
 _______ _______ _______ _______
|     __|    ___|   _   |   |   |
|__     |    ___|       |       |
|_______|_______|___|___|__|_|__|
```

原 `SBtream` 项目, 经历 python 不成熟的模仿, Java 重构烂尾, 目前使用 rust 进行重构开发

多平台直播源地址获取

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
|平台|房间号位置|
|:-:|:-:|
|[B站](https://live.bilibili.com/)|`https://live.bilibili.com/<RID>`|
|[斗鱼](https://www.douyu.com/)|`https://www.douyu.com/<RID>` 或 `https://www.douyu.com/xx/xx?rid=<RID>`|
|[抖音](https://live.douyin.com/)|`https://live.douyin.com/<RID>`|
|[虎牙](https://huya.com/)|`https://huya.com/<RID>`|
|[快手](https://live.kuaishou.com/)|`https://live.kuaishou.com/u/<RID>`|
|[CC](https://cc.163.com/)|`https://cc.163.com/<RID>`|
|[花椒](https://www.huajiao.com/)|`https://www.huajiao.com/l/<RID>`|
|[艺气山](https://www.173.com/)|`https://www.173.com/<RID>`|
|[棉花糖](https://www.2cq.com/)|`https://www.2cq.com/<RID>`|
|[Now直播](https://now.qq.com/)|`https://now.qq.com/pcweb/story.html?roomid=<RID>`|
|[afreeca](https://afreecatv.com/)|`https://bj.afreecatv.com/<RID>` 主播名字而非直播间号|
|[pandalive](https://www.pandalive.co.kr/)|`https://www.pandalive.co.kr/channel/<RID>` 主播名字而非直播间号|
|[flex](https://www.flextv.co.kr/)|`https://www.flextv.co.kr/channels/<RID>` 主播名字而非直播间号|
|[wink](https://www.winktv.co.kr/)|`https://www.winktv.co.kr/channel/<RID>` 主播名字而非直播间号|

# 路线

[seam](https://github.com/users/Borber/projects/4/views/1)

# 说明

1. 斗鱼平台动态js加密需要js运行时, 但相关依赖中quickjs计算出错, deno v8编译失败, 所以我自己写了个在线运行js的eval接口, 地址为 js.borber.top 部署在vercel,稍后会开源出来, 方便自部署. 可以查看源码, 并无窃取信息等恶意行为. 故而斗鱼链接的获取会稍慢一点


# 感谢

1. [wbt5/real-url](https://github.com/wbt5/real-url/)
2. [banner](https://textkool.com/en/ascii-art-generator?hl=default&vl=default&font=Chunky&text=SEAM)
3. [手把手教你破解斗鱼sign算法](https://zhuanlan.zhihu.com/p/107330805)
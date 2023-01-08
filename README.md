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
|[艺气山](https://www.173.com/)|`https://www.173.com/<RID>`|
|[棉花糖](https://www.2cq.com/)|`https://www.2cq.com/<RID>`|

# 路线
- [ ] 直播平台源地址获取
  - [x] [B站](https://live.bilibili.com/)
  - [x] [斗鱼](https://www.douyu.com/)
  - [x] [抖音](https://live.douyin.com/)
  - [x] [虎牙](https://huya.com/)
  - [x] [艺气山](https://www.173.com/)
  - [x] [棉花糖](https://www.2cq.com/)
- [x] cli
- [ ] CI 自动编译
- [ ] 根据画质排序
- [ ] gui
- [ ] mpv播放



# 感谢

1. [wbt5/real-url](https://github.com/wbt5/real-url/)
2. [banner](https://textkool.com/en/ascii-art-generator?hl=default&vl=default&font=Chunky&text=SEAM)
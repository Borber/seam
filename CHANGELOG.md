# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## Added

- 弹幕 CSV 支持
- 开始支持设置
  - 弹幕, 视频 目前支持 rid title time 字段替换

## Fixed

- 修复抖音直播源获取
- B站弹幕解压情况下的顺序问题

## [0.1.15]

### Added
- 虎牙直播间标题字段支持

### Fixed
- 删除虎牙多余信息输出

## [0.1.14]

### Added

- 添加 bili 直播间 标题获取, 标题字段初步支持
- 支持 抖音, cc直播标题获取

### Fixd

- 修复抖音直播源获取

### Changed

- 抖音去除画质标签

### Changed

- 弹幕功能调整


## [0.1.13]

### Added

- 添加 kk 直播源获取
- 添加 千帆直播源获取
- bili 直播弹幕获取-预览版

## [0.1.12]

### Added

- 添加 now 直播源获取

### Changed

- Format 添加 rtmp 格式
- 删除斗鱼, 虎牙多余打印信息

### Fixed

- 修复斗鱼, 虎牙平台直播源获取


## [0.1.11]

### Added

- 添加 winktv 直播源获取

### Changed

- 修改Node结构
- 规范化 format 判定, 规范化输出方法
- 简化代码


## [0.1.10]

### Added

- 添加 flex 直播源获取

## [0.1.9]

### Added

- 添加 pandalive 直播源获取

## [0.1.8]

### Added

- 添加 afreeca 直播源获取

### Changed

- 引入宏简化代码 感谢 [@eweca-d](https://github.com/eweca-d)
- 删除部分注释及说明信息

### Fixed

- model 拼写错误修正



## [0.1.7]

### Added

- 支持网易CC直播源获取

### Changed

- 使用 `super` 替代绝对位置

## [0.1.6]

### Added

- 支持快手直播源获取

## [0.1.5] - 2023-01-11

### Fixed

- 修复斗鱼直播源获取


## [0.1.4] - 2023-01-11

### Added

- 添加全平台自动编译发布工作流
- 支持花椒直播源获取

### Changed

- 后续tag将采用 vX.X.X 的格式

### Fixed

- 修改代码格式


## [0.1.3] - 2023-01-9

### Added

- 原创虎牙直播源获取

### Changed

- 同时输出阿里 腾讯 华为CDN和 flv hls 两种直播源

### Fixed

- 从预览接口转换为标准接口, 修复部分直播间无法获取链接而显示未开播的问题
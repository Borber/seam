package com.borber.sbtream.plugin.model;

import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.NoArgsConstructor;

import java.io.Serializable;

/**
 * @ClassName PlatformBean
 * @Description 直播平台插件的相关信息
 * @Author Borber
 * @Date 2021/7/15 上午11:14
 * @Version 0.0.1
 **/
@Data
@AllArgsConstructor
@NoArgsConstructor
public class PlatformDTO implements Serializable {
    /**
     * 平台代号
     * 1. 尽量使用官网地址的简写
     * 2. 长度避免过长或过短
     */
    private String pid;
    /**
     * 直播平台的简称 尽量不带 “直播” 字样
     */
    private String name;
    /**
     * 直播平台的官网 直接 url/{rid} 作为直播间地址
     */
    private String url;
    /**
     * 直播平台图标
     */
    private String icon;
    /**
     * 是否需要 Cookies 来获取最高清的资源
     */
    private Boolean needCookie;
    /**
     * 维护人的名字
     */
    private String mMan;
    /**
     * 维护人的邮箱 方便反馈 可以为空
     */
    private String mManEmail;
    /**
     * 维护人的公告信息
     */
    private String msg;
    /**
     * 维护更新地址
     */
    private String updateUrl;
    /**
     * 当前版本
     */
    private String version;

    /**
     * 如果平台支持弹幕, 请返回弹幕限制, 格式为 20-200 下限-上限
     */
    private String danmakuLimit;
}

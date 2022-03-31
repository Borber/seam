package com.borber.sbtream.plugin.model;

import com.borber.sbtream.plugin.constans.StatusType;
import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.NoArgsConstructor;

import java.io.Serializable;

/**
 * @ClassName UrlBean
 * @Description 返回对应rid的的 直播源地址  以及 弹幕地址
 * @Author Borber
 * @Date 2021/7/15 下午10:47
 * @Version 0.0.1
 **/
@Data
@AllArgsConstructor
@NoArgsConstructor
public class DataDTO implements Serializable {
    /**
     * 是否开播 0, 开播 -1 未开播
     */
    private Integer code;
    /**
     * 直播间标题
     */
    private String title;
    /**
     * 直播间封面
     */
    private String cover;

    /**
     * 主播头像
     */
    private String headImg;

    /**
     * 主播
     */
    private String name;
    /**
     * 官网地址
     */
    private String wurl;
    /**
     * 直播源地址
     */
    private String rurl;
    /**
     * 弹幕地址
     */
    private String durl;
}

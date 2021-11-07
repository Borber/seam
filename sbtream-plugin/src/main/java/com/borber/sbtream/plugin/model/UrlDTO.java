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
public class UrlDTO implements Serializable {
    private String name;

    private String headImg;

    private String picture;
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

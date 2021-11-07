package com.borber.sbtream.server.model.dto;

import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.NoArgsConstructor;

import java.io.Serializable;
import java.util.List;

/**
 * @ClassName PlatformDTO
 * @Description 用于储存对应平台的所有关注主播
 * @Author Borber
 * @Date 2021/7/15 下午4:19
 * @Version 0.0.1
 **/
@Data
public class PlatformServiceDTO implements Serializable {
    /**
     * 给平台提供支持的插件id号
     */
    private String pluginId;

    /**
     * 平台名称
     */
    private String name;

    /**
     * 平台图标
     */
    private String icon;

    /**
     * 平台代号
     */
    private String code;

    /**
     * 平台官网
     */
    private String site;

    /**
     * 平台api基础链接
     */
    private String apiLink;

    /**
     * 是否需要 cookies 来获取最高清资源：默认为0不需要，1需要
     */
    private Byte gethdCookiesRequired;

    /**
     * 是否提供官方录播
     */
    private Byte officialRecordProvided;

    /**
     * 官方录播地址 (可能需要从插件中获取) 待定
     */
    private String officialRecordUrl;
}

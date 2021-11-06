package com.borber.sbtream.server.model.entity;

import com.baomidou.mybatisplus.annotation.IdType;
import com.baomidou.mybatisplus.annotation.TableField;
import com.baomidou.mybatisplus.annotation.TableId;
import com.baomidou.mybatisplus.annotation.TableName;
import lombok.Data;

import java.io.Serializable;
import java.util.Date;

/**
 * 平台表
 * @TableName platform
 */
@TableName(value ="platform")
@Data
public class PlatformDO implements Serializable {
    /**
     * 平台id
     */
    @TableId(type= IdType.ASSIGN_UUID)
    private String id;

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

    /**
     * 创建时间
     */
    private Date createTime;

    /**
     * 更新时间
     */
    private Date updateTime;

    /**
     * 
     */
    private Byte disable;

    @TableField(exist = false)
    private static final long serialVersionUID = -49616122763188379L;
}
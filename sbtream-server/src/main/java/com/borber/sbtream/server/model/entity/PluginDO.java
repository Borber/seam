package com.borber.sbtream.server.model.entity;

import com.baomidou.mybatisplus.annotation.IdType;
import com.baomidou.mybatisplus.annotation.TableField;
import com.baomidou.mybatisplus.annotation.TableId;
import com.baomidou.mybatisplus.annotation.TableName;
import lombok.Data;

import java.io.Serializable;
import java.util.Date;

/**
 * 插件表
 * @TableName plugin
 */
@TableName(value ="plugin")
@Data
public class PluginDO implements Serializable {
    /**
     * ID
     */
    @TableId(type= IdType.ASSIGN_UUID)
    private String id;

    /**
     * 插件官网
     */
    private String officialWebsite;

    /**
     * 插件更新地址
     */
    private String url;

    /**
     * 维护人名字
     */
    private String mName;

    /**
     * 维护人邮箱
     */
    private String mMail;

    /**
     * 公告信息
     */
    private String aInfo;

    /**
     * 当前版本
     */
    private String version;

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
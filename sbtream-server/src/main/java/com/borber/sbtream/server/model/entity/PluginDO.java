package com.borber.sbtream.server.model.entity;

import com.baomidou.mybatisplus.annotation.IdType;
import com.baomidou.mybatisplus.annotation.TableField;
import com.baomidou.mybatisplus.annotation.TableId;
import com.baomidou.mybatisplus.annotation.TableName;
import java.io.Serializable;
import java.util.Date;
import lombok.Data;
import lombok.EqualsAndHashCode;

/**
 * 插件表
 * @TableName plugin
 */
@EqualsAndHashCode(callSuper = true)
@TableName(value ="plugin")
@Data
public class PluginDO extends BaseDO {
    /**
     * 自增
     */
    @TableId
    private String id;

    /**
     * 插件官网
     */
    private String officialWebsite;

    /**
     * 插件更新地址
     */
    private String updateUrl;

    /**
     * 维护人名字
     */
    private String maintainerName;

    /**
     * 维护人邮箱
     */
    private String maintainerEmail;

    /**
     * 公告信息
     */
    private String announcementInformation;

    /**
     * 当前版本
     */
    private String version;

    @TableField(exist = false)
    private static final long serialVersionUID = -49616122763188379L;
}
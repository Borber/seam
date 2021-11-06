package com.borber.sbtream.server.model.entity;

import com.baomidou.mybatisplus.annotation.IdType;
import com.baomidou.mybatisplus.annotation.TableField;
import com.baomidou.mybatisplus.annotation.TableId;
import com.baomidou.mybatisplus.annotation.TableName;
import lombok.Data;

import java.io.Serializable;
import java.util.Date;

/**
 * cookies表
 * @TableName cookie
 */
@TableName(value ="cookie")
@Data
public class CookieDO implements Serializable {
    /**
     * 自增
     */
    @TableId(type= IdType.ASSIGN_UUID)
    private String id;

    /**
     * 用户id
     */
    private String userId;

    /**
     * 平台id
     */
    private String platformId;

    /**
     * cookie值
     */
    private String cookieValue;

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
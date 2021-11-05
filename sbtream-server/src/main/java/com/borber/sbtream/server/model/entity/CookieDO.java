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
 * cookies表
 * @TableName cookie
 */
@EqualsAndHashCode(callSuper = true)
@TableName(value ="cookie")
@Data
public class CookieDO extends BaseDO {
    /**
     * 自增
     */
    @TableId
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

    @TableField(exist = false)
    private static final long serialVersionUID = -49616122763188379L;
}
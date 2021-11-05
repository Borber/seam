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
 * 管理员表
 * @TableName admin
 */
@EqualsAndHashCode(callSuper = true)
@TableName(value ="admin")
@Data
public class AdminDO extends BaseDO {
    /**
     * 管理员id
     */
    @TableId
    private String id;

    /**
     * 管理员用户名
     */
    private String uname;

    /**
     * 管理员密码
     */
    private String passwd;

    /**
     * 管理员邮箱
     */
    private String mailbox;

    @TableField(exist = false)
    private static final long serialVersionUID = -49616122763188379L;
}
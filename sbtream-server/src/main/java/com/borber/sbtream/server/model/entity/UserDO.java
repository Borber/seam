package com.borber.sbtream.server.model.entity;

import com.baomidou.mybatisplus.annotation.*;
import lombok.Data;
import lombok.EqualsAndHashCode;

/**
 * 用户及设置表
 * @author BORBER
 * @TableName user
 */
@EqualsAndHashCode(callSuper = true)
@TableName(value ="user")
@Data
public class UserDO extends BaseDO {
    /**
     * 用户id
     */
    @TableId(type= IdType.ASSIGN_UUID)
    private String id;

    /**
     * 用户名
     */
    private String name;

    /**
     * 用户密码
     */
    private String passwd;

    /**
     * 用户邮箱
     */
    private String mail;

    @TableField(exist = false)
    private static final long serialVersionUID = -49616122763188379L;
}
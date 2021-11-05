package com.borber.sbtream.server.model.entity;

import com.baomidou.mybatisplus.annotation.*;

import java.io.Serializable;
import java.util.Date;
import lombok.Data;
import lombok.EqualsAndHashCode;

/**
 * 用户及设置表
 * @TableName user
 */

@TableName(value ="user")
@Data
public class UserDO implements Serializable {
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

    /**
     * Creation time
     */
    @TableField(fill = FieldFill.INSERT)
    private Date createTime;
    /**
     * Update time
     */
    @TableField(fill = FieldFill.INSERT_UPDATE)
    private Date modifyTime;
    /**
     * 逻辑删除
     */
    @TableLogic
    @TableField(fill = FieldFill.INSERT)
    private Boolean disabled;

    @TableField(exist = false)
    private static final long serialVersionUID = -49616122763188379L;
}
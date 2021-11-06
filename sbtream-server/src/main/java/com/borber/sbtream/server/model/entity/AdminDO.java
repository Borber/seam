package com.borber.sbtream.server.model.entity;

import com.baomidou.mybatisplus.annotation.IdType;
import com.baomidou.mybatisplus.annotation.TableField;
import com.baomidou.mybatisplus.annotation.TableId;
import com.baomidou.mybatisplus.annotation.TableName;
import lombok.Data;

import java.io.Serializable;
import java.util.Date;

/**
 * 管理员表
 * @author BORBER
 * @TableName admin
 */

@TableName(value ="admin")
@Data
public class AdminDO implements Serializable {
    /**
     * 管理员id
     */
    @TableId(type= IdType.ASSIGN_UUID)
    private String id;

    /**
     * 管理员用户名
     */
    private String name;

    /**
     * 管理员密码
     */
    private String passwd;

    /**
     * 管理员邮箱
     */
    private String mail;

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
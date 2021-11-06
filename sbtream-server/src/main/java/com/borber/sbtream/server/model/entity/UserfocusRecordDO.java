package com.borber.sbtream.server.model.entity;

import com.baomidou.mybatisplus.annotation.IdType;
import com.baomidou.mybatisplus.annotation.TableField;
import com.baomidou.mybatisplus.annotation.TableId;
import com.baomidou.mybatisplus.annotation.TableName;
import lombok.Data;

import java.io.Serializable;
import java.util.Date;

/**
 * 用户关注及录播表
 * @TableName userfocus_record
 */
@TableName(value ="userfocus_record")
@Data
public class UserfocusRecordDO implements Serializable {
    /**
     * 
     */
    @TableId(type= IdType.ASSIGN_UUID)
    private String id;

    /**
     * 用户id
     */
    private String userId;

    /**
     * 主播id
     */
    private String anchorId;

    /**
     * 是否自动录播,默认0为否
     */
    private Byte autoRecordAble;

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
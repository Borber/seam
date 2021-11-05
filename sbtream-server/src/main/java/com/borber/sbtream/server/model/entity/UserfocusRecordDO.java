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
 * 用户关注及录播表
 * @TableName userfocus_record
 */
@EqualsAndHashCode(callSuper = true)
@TableName(value ="userfocus_record")
@Data
public class UserfocusRecordDO extends BaseDO {
    /**
     * 
     */
    @TableId
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
    private Boolean autoRecordAble;

    @TableField(exist = false)
    private static final long serialVersionUID = -49616122763188379L;
}
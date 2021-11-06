package com.borber.sbtream.server.model.entity;

import com.baomidou.mybatisplus.annotation.IdType;
import com.baomidou.mybatisplus.annotation.TableField;
import com.baomidou.mybatisplus.annotation.TableId;
import com.baomidou.mybatisplus.annotation.TableName;
import lombok.Data;

import java.io.Serializable;
import java.util.Date;

/**
 * 观看记录表
 * @TableName watch_history
 */
@TableName(value ="watch_history")
@Data
public class WatchHistoryDO implements Serializable {
    /**
     * 自增
     */
    @TableId(type= IdType.ASSIGN_UUID)
    private String id;

    /**
     * 主播id
     */
    private String anchorId;

    /**
     * 直播标题
     */
    private String livestreamTitle;

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
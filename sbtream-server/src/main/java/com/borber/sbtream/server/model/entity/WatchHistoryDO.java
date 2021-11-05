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
 * 观看记录表
 * @TableName watch_history
 */
@EqualsAndHashCode(callSuper = true)
@TableName(value ="watch_history")
@Data
public class WatchHistoryDO extends BaseDO {
    /**
     * 自增
     */
    @TableId
    private String id;

    /**
     * 主播id
     */
    private String anchorId;

    /**
     * 直播标题
     */
    private String livestreamTitle;

    @TableField(exist = false)
    private static final long serialVersionUID = -49616122763188379L;
}
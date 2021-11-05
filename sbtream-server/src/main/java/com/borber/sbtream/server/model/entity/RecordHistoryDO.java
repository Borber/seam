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
 * 录播历史表
 * @TableName record_history
 */
@EqualsAndHashCode(callSuper = true)
@TableName(value ="record_history")
@Data
public class RecordHistoryDO extends BaseDO {
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
     * 历史记录回放地址
     */
    private String playbackhistoryUrl;

    /**
     * 弹幕文件存放地址
     */
    private String barragefileUrl;

    /**
     * 直播标题
     */
    private String livestreamTitle;

    /**
     * 是否观看过，0表示没看过，1表示看过，默认值为0
     */
    private Boolean viewed;


    @TableField(exist = false)
    private static final long serialVersionUID = -49616122763188379L;
}
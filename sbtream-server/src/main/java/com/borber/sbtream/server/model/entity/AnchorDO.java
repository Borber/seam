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
 * 主播表
 * @TableName anchor
 */
@EqualsAndHashCode(callSuper = true)
@TableName(value ="anchor")
@Data
public class AnchorDO extends BaseDO {
    /**
     * 主播id
     */
    @TableId
    private String id;

    /**
     * 所属平台id
     */
    private String pfId;

    /**
     * 主播名称
     */
    private String name;

    /**
     * 主播头像url, 或文件地址
     */
    private String iconUrl;

    /**
     * 主播房间号
     */
    private String roomcode;

    /**
     * 直播间名称
     */
    private String roomname;

    /**
     * 直播类型 (可以类型, 也可以是具体的游戏, 项目)
     */
    private String liveType;

    /**
     * 名称备注 (优先显示备注)
     */
    private String nameComment;

    /**
     * 详情备注
     */
    private String commentDetails;

    /**
     * 是否自动录播：默认0为否，1为是
     */
    private Boolean autoRecordAble;

    @TableField(exist = false)
    private static final long serialVersionUID = -49616122763188379L;
}
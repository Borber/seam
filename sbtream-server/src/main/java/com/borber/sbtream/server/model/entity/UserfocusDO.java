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
 * 用户关注表
 * @TableName userfocus
 */
@EqualsAndHashCode(callSuper = true)
@TableName(value ="userfocus")
@Data
public class UserfocusDO extends BaseDO {
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
     * 插件id
     */
    private String pluginId;

    /**
     * 主播房间号
     */
    private String rid;


    @TableField(exist = false)
    private static final long serialVersionUID = 1L;
}
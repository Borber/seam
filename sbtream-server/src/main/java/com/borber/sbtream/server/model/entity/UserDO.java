package com.borber.sbtream.server.model.entity;

import com.baomidou.mybatisplus.annotation.IdType;
import com.baomidou.mybatisplus.annotation.TableField;
import com.baomidou.mybatisplus.annotation.TableId;
import com.baomidou.mybatisplus.annotation.TableName;
import java.io.Serializable;
import java.util.Date;
import lombok.Data;

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
    @TableId
    private String userId;

    /**
     * 用户名
     */
    private String userUname;

    /**
     * 用户密码
     */
    private String userPasswd;

    /**
     * 用户邮箱
     */
    private String userMailbox;

    /**
     * 是否开启观看时自动录播，默认为0关闭，1开启
     */
    private Boolean autoRecordAble;

    /**
     * 是否开启关注主播通知，默认为1开启，0关闭
     */
    private Boolean anchorNotificationAble;

    /**
     * 创建时间
     */
    private Date userCreateTime;

    /**
     * 修改时间
     */
    private Date userModifyTime;

    /**
     * 创建人id
     */
    private String userFounderId;

    /**
     * 修改人id
     */
    private String userModifierId;

    /**
     * 逻辑删除：默认为0，删除操作时修改为1
     */
    private Boolean userDisabled;

    @TableField(exist = false)
    private static final long serialVersionUID = 1L;

    @Override
    public boolean equals(Object that) {
        if (this == that) {
            return true;
        }
        if (that == null) {
            return false;
        }
        if (getClass() != that.getClass()) {
            return false;
        }
        UserDO other = (UserDO) that;
        return (this.getUserId() == null ? other.getUserId() == null : this.getUserId().equals(other.getUserId()))
            && (this.getUserUname() == null ? other.getUserUname() == null : this.getUserUname().equals(other.getUserUname()))
            && (this.getUserPasswd() == null ? other.getUserPasswd() == null : this.getUserPasswd().equals(other.getUserPasswd()))
            && (this.getUserMailbox() == null ? other.getUserMailbox() == null : this.getUserMailbox().equals(other.getUserMailbox()))
            && (this.getAutoRecordAble() == null ? other.getAutoRecordAble() == null : this.getAutoRecordAble().equals(other.getAutoRecordAble()))
            && (this.getAnchorNotificationAble() == null ? other.getAnchorNotificationAble() == null : this.getAnchorNotificationAble().equals(other.getAnchorNotificationAble()))
            && (this.getUserCreateTime() == null ? other.getUserCreateTime() == null : this.getUserCreateTime().equals(other.getUserCreateTime()))
            && (this.getUserModifyTime() == null ? other.getUserModifyTime() == null : this.getUserModifyTime().equals(other.getUserModifyTime()))
            && (this.getUserFounderId() == null ? other.getUserFounderId() == null : this.getUserFounderId().equals(other.getUserFounderId()))
            && (this.getUserModifierId() == null ? other.getUserModifierId() == null : this.getUserModifierId().equals(other.getUserModifierId()))
            && (this.getUserDisabled() == null ? other.getUserDisabled() == null : this.getUserDisabled().equals(other.getUserDisabled()));
    }

    @Override
    public int hashCode() {
        final int prime = 31;
        int result = 1;
        result = prime * result + ((getUserId() == null) ? 0 : getUserId().hashCode());
        result = prime * result + ((getUserUname() == null) ? 0 : getUserUname().hashCode());
        result = prime * result + ((getUserPasswd() == null) ? 0 : getUserPasswd().hashCode());
        result = prime * result + ((getUserMailbox() == null) ? 0 : getUserMailbox().hashCode());
        result = prime * result + ((getAutoRecordAble() == null) ? 0 : getAutoRecordAble().hashCode());
        result = prime * result + ((getAnchorNotificationAble() == null) ? 0 : getAnchorNotificationAble().hashCode());
        result = prime * result + ((getUserCreateTime() == null) ? 0 : getUserCreateTime().hashCode());
        result = prime * result + ((getUserModifyTime() == null) ? 0 : getUserModifyTime().hashCode());
        result = prime * result + ((getUserFounderId() == null) ? 0 : getUserFounderId().hashCode());
        result = prime * result + ((getUserModifierId() == null) ? 0 : getUserModifierId().hashCode());
        result = prime * result + ((getUserDisabled() == null) ? 0 : getUserDisabled().hashCode());
        return result;
    }

    @Override
    public String toString() {
        StringBuilder sb = new StringBuilder();
        sb.append(getClass().getSimpleName());
        sb.append(" [");
        sb.append("Hash = ").append(hashCode());
        sb.append(", userId=").append(userId);
        sb.append(", userUname=").append(userUname);
        sb.append(", userPasswd=").append(userPasswd);
        sb.append(", userMailbox=").append(userMailbox);
        sb.append(", autoRecordAble=").append(autoRecordAble);
        sb.append(", anchorNotificationAble=").append(anchorNotificationAble);
        sb.append(", userCreateTime=").append(userCreateTime);
        sb.append(", userModifyTime=").append(userModifyTime);
        sb.append(", userFounderId=").append(userFounderId);
        sb.append(", userModifierId=").append(userModifierId);
        sb.append(", userDisabled=").append(userDisabled);
        sb.append(", serialVersionUID=").append(serialVersionUID);
        sb.append("]");
        return sb.toString();
    }
}
package com.borber.sbtream.server.model.entity;

import com.baomidou.mybatisplus.annotation.IdType;
import com.baomidou.mybatisplus.annotation.TableField;
import com.baomidou.mybatisplus.annotation.TableId;
import com.baomidou.mybatisplus.annotation.TableName;
import java.io.Serializable;
import java.util.Date;
import lombok.Data;

/**
 * cookies表
 * @TableName cookie
 */
@TableName(value ="cookie")
@Data
public class CookieDO implements Serializable {
    /**
     * 自增
     */
    @TableId
    private String id;

    /**
     * 用户id
     */
    private String userId;

    /**
     * 平台id
     */
    private String platformId;

    /**
     * cookie值
     */
    private String cookieValue;

    /**
     * 创建时间
     */
    private Date createTime;

    /**
     * 修改时间
     */
    private Date modificationTime;

    /**
     * 创建人id
     */
    private String founderId;

    /**
     * 修改人id
     */
    private String modifierId;

    /**
     * 逻辑删除 默认值为0
     */
    private Boolean disabled;

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
        CookieDO other = (CookieDO) that;
        return (this.getId() == null ? other.getId() == null : this.getId().equals(other.getId()))
            && (this.getUserId() == null ? other.getUserId() == null : this.getUserId().equals(other.getUserId()))
            && (this.getPlatformId() == null ? other.getPlatformId() == null : this.getPlatformId().equals(other.getPlatformId()))
            && (this.getCookieValue() == null ? other.getCookieValue() == null : this.getCookieValue().equals(other.getCookieValue()))
            && (this.getCreateTime() == null ? other.getCreateTime() == null : this.getCreateTime().equals(other.getCreateTime()))
            && (this.getModificationTime() == null ? other.getModificationTime() == null : this.getModificationTime().equals(other.getModificationTime()))
            && (this.getFounderId() == null ? other.getFounderId() == null : this.getFounderId().equals(other.getFounderId()))
            && (this.getModifierId() == null ? other.getModifierId() == null : this.getModifierId().equals(other.getModifierId()))
            && (this.getDisabled() == null ? other.getDisabled() == null : this.getDisabled().equals(other.getDisabled()));
    }

    @Override
    public int hashCode() {
        final int prime = 31;
        int result = 1;
        result = prime * result + ((getId() == null) ? 0 : getId().hashCode());
        result = prime * result + ((getUserId() == null) ? 0 : getUserId().hashCode());
        result = prime * result + ((getPlatformId() == null) ? 0 : getPlatformId().hashCode());
        result = prime * result + ((getCookieValue() == null) ? 0 : getCookieValue().hashCode());
        result = prime * result + ((getCreateTime() == null) ? 0 : getCreateTime().hashCode());
        result = prime * result + ((getModificationTime() == null) ? 0 : getModificationTime().hashCode());
        result = prime * result + ((getFounderId() == null) ? 0 : getFounderId().hashCode());
        result = prime * result + ((getModifierId() == null) ? 0 : getModifierId().hashCode());
        result = prime * result + ((getDisabled() == null) ? 0 : getDisabled().hashCode());
        return result;
    }

    @Override
    public String toString() {
        StringBuilder sb = new StringBuilder();
        sb.append(getClass().getSimpleName());
        sb.append(" [");
        sb.append("Hash = ").append(hashCode());
        sb.append(", id=").append(id);
        sb.append(", userId=").append(userId);
        sb.append(", platformId=").append(platformId);
        sb.append(", cookieValue=").append(cookieValue);
        sb.append(", createTime=").append(createTime);
        sb.append(", modificationTime=").append(modificationTime);
        sb.append(", founderId=").append(founderId);
        sb.append(", modifierId=").append(modifierId);
        sb.append(", disabled=").append(disabled);
        sb.append(", serialVersionUID=").append(serialVersionUID);
        sb.append("]");
        return sb.toString();
    }
}
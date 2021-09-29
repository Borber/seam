package com.borber.sbtream.server.model.entity;

import com.baomidou.mybatisplus.annotation.IdType;
import com.baomidou.mybatisplus.annotation.TableField;
import com.baomidou.mybatisplus.annotation.TableId;
import com.baomidou.mybatisplus.annotation.TableName;
import java.io.Serializable;
import java.util.Date;
import lombok.Data;

/**
 * 管理员表
 * @TableName admin
 */
@TableName(value ="admin")
@Data
public class AdminDO implements Serializable {
    /**
     * 管理员id
     */
    @TableId
    private String adminId;

    /**
     * 管理员用户名
     */
    private String adminUname;

    /**
     * 管理员密码
     */
    private String adminPasswd;

    /**
     * 管理员邮箱
     */
    private String adminMailbox;

    /**
     * 创建时间
     */
    private Date adminCreateTime;

    /**
     * 修改时间
     */
    private Date adminModifyTime;

    /**
     * 创建人id
     */
    private String adminFounderId;

    /**
     * 修改人id
     */
    private String adminModifierId;

    /**
     * 逻辑删除：默认为0，删除操作则修改为1
     */
    private Boolean adminDisabled;

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
        AdminDO other = (AdminDO) that;
        return (this.getAdminId() == null ? other.getAdminId() == null : this.getAdminId().equals(other.getAdminId()))
            && (this.getAdminUname() == null ? other.getAdminUname() == null : this.getAdminUname().equals(other.getAdminUname()))
            && (this.getAdminPasswd() == null ? other.getAdminPasswd() == null : this.getAdminPasswd().equals(other.getAdminPasswd()))
            && (this.getAdminMailbox() == null ? other.getAdminMailbox() == null : this.getAdminMailbox().equals(other.getAdminMailbox()))
            && (this.getAdminCreateTime() == null ? other.getAdminCreateTime() == null : this.getAdminCreateTime().equals(other.getAdminCreateTime()))
            && (this.getAdminModifyTime() == null ? other.getAdminModifyTime() == null : this.getAdminModifyTime().equals(other.getAdminModifyTime()))
            && (this.getAdminFounderId() == null ? other.getAdminFounderId() == null : this.getAdminFounderId().equals(other.getAdminFounderId()))
            && (this.getAdminModifierId() == null ? other.getAdminModifierId() == null : this.getAdminModifierId().equals(other.getAdminModifierId()))
            && (this.getAdminDisabled() == null ? other.getAdminDisabled() == null : this.getAdminDisabled().equals(other.getAdminDisabled()));
    }

    @Override
    public int hashCode() {
        final int prime = 31;
        int result = 1;
        result = prime * result + ((getAdminId() == null) ? 0 : getAdminId().hashCode());
        result = prime * result + ((getAdminUname() == null) ? 0 : getAdminUname().hashCode());
        result = prime * result + ((getAdminPasswd() == null) ? 0 : getAdminPasswd().hashCode());
        result = prime * result + ((getAdminMailbox() == null) ? 0 : getAdminMailbox().hashCode());
        result = prime * result + ((getAdminCreateTime() == null) ? 0 : getAdminCreateTime().hashCode());
        result = prime * result + ((getAdminModifyTime() == null) ? 0 : getAdminModifyTime().hashCode());
        result = prime * result + ((getAdminFounderId() == null) ? 0 : getAdminFounderId().hashCode());
        result = prime * result + ((getAdminModifierId() == null) ? 0 : getAdminModifierId().hashCode());
        result = prime * result + ((getAdminDisabled() == null) ? 0 : getAdminDisabled().hashCode());
        return result;
    }

    @Override
    public String toString() {
        StringBuilder sb = new StringBuilder();
        sb.append(getClass().getSimpleName());
        sb.append(" [");
        sb.append("Hash = ").append(hashCode());
        sb.append(", adminId=").append(adminId);
        sb.append(", adminUname=").append(adminUname);
        sb.append(", adminPasswd=").append(adminPasswd);
        sb.append(", adminMailbox=").append(adminMailbox);
        sb.append(", adminCreateTime=").append(adminCreateTime);
        sb.append(", adminModifyTime=").append(adminModifyTime);
        sb.append(", adminFounderId=").append(adminFounderId);
        sb.append(", adminModifierId=").append(adminModifierId);
        sb.append(", adminDisabled=").append(adminDisabled);
        sb.append(", serialVersionUID=").append(serialVersionUID);
        sb.append("]");
        return sb.toString();
    }
}
package com.borber.sbtream.server.model.entity;

import com.baomidou.mybatisplus.annotation.IdType;
import com.baomidou.mybatisplus.annotation.TableField;
import com.baomidou.mybatisplus.annotation.TableId;
import com.baomidou.mybatisplus.annotation.TableName;
import java.io.Serializable;
import java.util.Date;
import lombok.Data;

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
    @TableId
    private String frId;

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

    /**
     * 创建时间
     */
    private Date frCreateTime;

    /**
     * 修改时间
     */
    private Date frModifyTime;

    /**
     * 创建人id
     */
    private String frFounderId;

    /**
     * 修改人id
     */
    private String frModifierId;

    /**
     * 逻辑删除:默认为0，删除操作修改为1
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
        UserfocusRecordDO other = (UserfocusRecordDO) that;
        return (this.getFrId() == null ? other.getFrId() == null : this.getFrId().equals(other.getFrId()))
            && (this.getUserId() == null ? other.getUserId() == null : this.getUserId().equals(other.getUserId()))
            && (this.getAnchorId() == null ? other.getAnchorId() == null : this.getAnchorId().equals(other.getAnchorId()))
            && (this.getAutoRecordAble() == null ? other.getAutoRecordAble() == null : this.getAutoRecordAble().equals(other.getAutoRecordAble()))
            && (this.getFrCreateTime() == null ? other.getFrCreateTime() == null : this.getFrCreateTime().equals(other.getFrCreateTime()))
            && (this.getFrModifyTime() == null ? other.getFrModifyTime() == null : this.getFrModifyTime().equals(other.getFrModifyTime()))
            && (this.getFrFounderId() == null ? other.getFrFounderId() == null : this.getFrFounderId().equals(other.getFrFounderId()))
            && (this.getFrModifierId() == null ? other.getFrModifierId() == null : this.getFrModifierId().equals(other.getFrModifierId()))
            && (this.getDisabled() == null ? other.getDisabled() == null : this.getDisabled().equals(other.getDisabled()));
    }

    @Override
    public int hashCode() {
        final int prime = 31;
        int result = 1;
        result = prime * result + ((getFrId() == null) ? 0 : getFrId().hashCode());
        result = prime * result + ((getUserId() == null) ? 0 : getUserId().hashCode());
        result = prime * result + ((getAnchorId() == null) ? 0 : getAnchorId().hashCode());
        result = prime * result + ((getAutoRecordAble() == null) ? 0 : getAutoRecordAble().hashCode());
        result = prime * result + ((getFrCreateTime() == null) ? 0 : getFrCreateTime().hashCode());
        result = prime * result + ((getFrModifyTime() == null) ? 0 : getFrModifyTime().hashCode());
        result = prime * result + ((getFrFounderId() == null) ? 0 : getFrFounderId().hashCode());
        result = prime * result + ((getFrModifierId() == null) ? 0 : getFrModifierId().hashCode());
        result = prime * result + ((getDisabled() == null) ? 0 : getDisabled().hashCode());
        return result;
    }

    @Override
    public String toString() {
        StringBuilder sb = new StringBuilder();
        sb.append(getClass().getSimpleName());
        sb.append(" [");
        sb.append("Hash = ").append(hashCode());
        sb.append(", frId=").append(frId);
        sb.append(", userId=").append(userId);
        sb.append(", anchorId=").append(anchorId);
        sb.append(", autoRecordAble=").append(autoRecordAble);
        sb.append(", frCreateTime=").append(frCreateTime);
        sb.append(", frModifyTime=").append(frModifyTime);
        sb.append(", frFounderId=").append(frFounderId);
        sb.append(", frModifierId=").append(frModifierId);
        sb.append(", disabled=").append(disabled);
        sb.append(", serialVersionUID=").append(serialVersionUID);
        sb.append("]");
        return sb.toString();
    }
}
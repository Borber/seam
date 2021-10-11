package com.borber.sbtream.server.model.entity;

import com.baomidou.mybatisplus.annotation.IdType;
import com.baomidou.mybatisplus.annotation.TableField;
import com.baomidou.mybatisplus.annotation.TableId;
import com.baomidou.mybatisplus.annotation.TableName;
import java.io.Serializable;
import java.util.Date;
import lombok.Data;

/**
 * 主播表
 * @TableName anchor
 */
@TableName(value ="anchor")
@Data
public class AnchorDO implements Serializable {
    /**
     * 主播id
     */
    @TableId
    private String anchorId;

    /**
     * 所属平台id
     */
    private String pfId;

    /**
     * 主播名称
     */
    private String anchorName;

    /**
     * 主播头像url, 或文件地址
     */
    private String anchorIconUrl;

    /**
     * 直播封面
     */

    /**
     * 主播房间号
     */
    private String anchorRoomCode;

    /**
     * 直播间名称
     */
    private String anchorRoomName;

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

    /**
     * 创建时间
     */
    private Date anchorCreateTime;

    /**
     * 修改时间
     */
    private Date anchorModifyTime;

    /**
     * 创建人id
     */
    private String anchorFounderId;

    /**
     * 修改人id
     */
    private String anchorModifierId;

    /**
     * 逻辑删除:默认为0，删除后修改为1
     */
    private Boolean anchorDisabled;

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
        AnchorDO other = (AnchorDO) that;
        return (this.getAnchorId() == null ? other.getAnchorId() == null : this.getAnchorId().equals(other.getAnchorId()))
            && (this.getPfId() == null ? other.getPfId() == null : this.getPfId().equals(other.getPfId()))
            && (this.getAnchorName() == null ? other.getAnchorName() == null : this.getAnchorName().equals(other.getAnchorName()))
            && (this.getAnchorIconUrl() == null ? other.getAnchorIconUrl() == null : this.getAnchorIconUrl().equals(other.getAnchorIconUrl()))
            && (this.getAnchorRoomCode() == null ? other.getAnchorRoomCode() == null : this.getAnchorRoomCode().equals(other.getAnchorRoomCode()))
            && (this.getAnchorRoomName() == null ? other.getAnchorRoomName() == null : this.getAnchorRoomName().equals(other.getAnchorRoomName()))
            && (this.getLiveType() == null ? other.getLiveType() == null : this.getLiveType().equals(other.getLiveType()))
            && (this.getNameComment() == null ? other.getNameComment() == null : this.getNameComment().equals(other.getNameComment()))
            && (this.getCommentDetails() == null ? other.getCommentDetails() == null : this.getCommentDetails().equals(other.getCommentDetails()))
            && (this.getAutoRecordAble() == null ? other.getAutoRecordAble() == null : this.getAutoRecordAble().equals(other.getAutoRecordAble()))
            && (this.getAnchorCreateTime() == null ? other.getAnchorCreateTime() == null : this.getAnchorCreateTime().equals(other.getAnchorCreateTime()))
            && (this.getAnchorModifyTime() == null ? other.getAnchorModifyTime() == null : this.getAnchorModifyTime().equals(other.getAnchorModifyTime()))
            && (this.getAnchorFounderId() == null ? other.getAnchorFounderId() == null : this.getAnchorFounderId().equals(other.getAnchorFounderId()))
            && (this.getAnchorModifierId() == null ? other.getAnchorModifierId() == null : this.getAnchorModifierId().equals(other.getAnchorModifierId()))
            && (this.getAnchorDisabled() == null ? other.getAnchorDisabled() == null : this.getAnchorDisabled().equals(other.getAnchorDisabled()));
    }

    @Override
    public int hashCode() {
        final int prime = 31;
        int result = 1;
        result = prime * result + ((getAnchorId() == null) ? 0 : getAnchorId().hashCode());
        result = prime * result + ((getPfId() == null) ? 0 : getPfId().hashCode());
        result = prime * result + ((getAnchorName() == null) ? 0 : getAnchorName().hashCode());
        result = prime * result + ((getAnchorIconUrl() == null) ? 0 : getAnchorIconUrl().hashCode());
        result = prime * result + ((getAnchorRoomCode() == null) ? 0 : getAnchorRoomCode().hashCode());
        result = prime * result + ((getAnchorRoomName() == null) ? 0 : getAnchorRoomName().hashCode());
        result = prime * result + ((getLiveType() == null) ? 0 : getLiveType().hashCode());
        result = prime * result + ((getNameComment() == null) ? 0 : getNameComment().hashCode());
        result = prime * result + ((getCommentDetails() == null) ? 0 : getCommentDetails().hashCode());
        result = prime * result + ((getAutoRecordAble() == null) ? 0 : getAutoRecordAble().hashCode());
        result = prime * result + ((getAnchorCreateTime() == null) ? 0 : getAnchorCreateTime().hashCode());
        result = prime * result + ((getAnchorModifyTime() == null) ? 0 : getAnchorModifyTime().hashCode());
        result = prime * result + ((getAnchorFounderId() == null) ? 0 : getAnchorFounderId().hashCode());
        result = prime * result + ((getAnchorModifierId() == null) ? 0 : getAnchorModifierId().hashCode());
        result = prime * result + ((getAnchorDisabled() == null) ? 0 : getAnchorDisabled().hashCode());
        return result;
    }

    @Override
    public String toString() {
        StringBuilder sb = new StringBuilder();
        sb.append(getClass().getSimpleName());
        sb.append(" [");
        sb.append("Hash = ").append(hashCode());
        sb.append(", anchorId=").append(anchorId);
        sb.append(", pfId=").append(pfId);
        sb.append(", anchorName=").append(anchorName);
        sb.append(", anchorIconUrl=").append(anchorIconUrl);
        sb.append(", anchorRoomcode=").append(anchorRoomCode);
        sb.append(", anchorRoomname=").append(anchorRoomName);
        sb.append(", liveType=").append(liveType);
        sb.append(", nameComment=").append(nameComment);
        sb.append(", commentDetails=").append(commentDetails);
        sb.append(", autoRecordAble=").append(autoRecordAble);
        sb.append(", anchorCreateTime=").append(anchorCreateTime);
        sb.append(", anchorModifyTime=").append(anchorModifyTime);
        sb.append(", anchorFounderId=").append(anchorFounderId);
        sb.append(", anchorModifierId=").append(anchorModifierId);
        sb.append(", anchorDisabled=").append(anchorDisabled);
        sb.append(", serialVersionUID=").append(serialVersionUID);
        sb.append("]");
        return sb.toString();
    }
}
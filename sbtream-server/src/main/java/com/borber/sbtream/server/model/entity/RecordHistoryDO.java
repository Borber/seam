package com.borber.sbtream.server.model.entity;

import com.baomidou.mybatisplus.annotation.IdType;
import com.baomidou.mybatisplus.annotation.TableField;
import com.baomidou.mybatisplus.annotation.TableId;
import com.baomidou.mybatisplus.annotation.TableName;
import java.io.Serializable;
import java.util.Date;
import lombok.Data;

/**
 * 录播历史表
 * @TableName record_history
 */
@TableName(value ="record_history")
@Data
public class RecordHistoryDO implements Serializable {
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
        RecordHistoryDO other = (RecordHistoryDO) that;
        return (this.getId() == null ? other.getId() == null : this.getId().equals(other.getId()))
            && (this.getAnchorId() == null ? other.getAnchorId() == null : this.getAnchorId().equals(other.getAnchorId()))
            && (this.getPlaybackhistoryUrl() == null ? other.getPlaybackhistoryUrl() == null : this.getPlaybackhistoryUrl().equals(other.getPlaybackhistoryUrl()))
            && (this.getBarragefileUrl() == null ? other.getBarragefileUrl() == null : this.getBarragefileUrl().equals(other.getBarragefileUrl()))
            && (this.getLivestreamTitle() == null ? other.getLivestreamTitle() == null : this.getLivestreamTitle().equals(other.getLivestreamTitle()))
            && (this.getViewed() == null ? other.getViewed() == null : this.getViewed().equals(other.getViewed()))
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
        result = prime * result + ((getAnchorId() == null) ? 0 : getAnchorId().hashCode());
        result = prime * result + ((getPlaybackhistoryUrl() == null) ? 0 : getPlaybackhistoryUrl().hashCode());
        result = prime * result + ((getBarragefileUrl() == null) ? 0 : getBarragefileUrl().hashCode());
        result = prime * result + ((getLivestreamTitle() == null) ? 0 : getLivestreamTitle().hashCode());
        result = prime * result + ((getViewed() == null) ? 0 : getViewed().hashCode());
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
        sb.append(", anchorId=").append(anchorId);
        sb.append(", playbackhistoryUrl=").append(playbackhistoryUrl);
        sb.append(", barragefileUrl=").append(barragefileUrl);
        sb.append(", livestreamTitle=").append(livestreamTitle);
        sb.append(", viewed=").append(viewed);
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
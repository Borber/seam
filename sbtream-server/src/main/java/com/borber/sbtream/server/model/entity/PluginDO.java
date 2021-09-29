package com.borber.sbtream.server.model.entity;

import com.baomidou.mybatisplus.annotation.IdType;
import com.baomidou.mybatisplus.annotation.TableField;
import com.baomidou.mybatisplus.annotation.TableId;
import com.baomidou.mybatisplus.annotation.TableName;
import java.io.Serializable;
import java.util.Date;
import lombok.Data;

/**
 * 插件表
 * @TableName plugin
 */
@TableName(value ="plugin")
@Data
public class PluginDO implements Serializable {
    /**
     * 自增
     */
    @TableId
    private String id;

    /**
     * 插件官网
     */
    private String pluginOfficialWebsite;

    /**
     * 插件更新地址
     */
    private String pluginUpdateUrl;

    /**
     * 维护人名字
     */
    private String maintainerName;

    /**
     * 维护人邮箱
     */
    private String maintainerEmail;

    /**
     * 公告信息
     */
    private String announcementInformation;

    /**
     * 当前版本
     */
    private String version;

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
        PluginDO other = (PluginDO) that;
        return (this.getId() == null ? other.getId() == null : this.getId().equals(other.getId()))
            && (this.getPluginOfficialWebsite() == null ? other.getPluginOfficialWebsite() == null : this.getPluginOfficialWebsite().equals(other.getPluginOfficialWebsite()))
            && (this.getPluginUpdateUrl() == null ? other.getPluginUpdateUrl() == null : this.getPluginUpdateUrl().equals(other.getPluginUpdateUrl()))
            && (this.getMaintainerName() == null ? other.getMaintainerName() == null : this.getMaintainerName().equals(other.getMaintainerName()))
            && (this.getMaintainerEmail() == null ? other.getMaintainerEmail() == null : this.getMaintainerEmail().equals(other.getMaintainerEmail()))
            && (this.getAnnouncementInformation() == null ? other.getAnnouncementInformation() == null : this.getAnnouncementInformation().equals(other.getAnnouncementInformation()))
            && (this.getVersion() == null ? other.getVersion() == null : this.getVersion().equals(other.getVersion()))
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
        result = prime * result + ((getPluginOfficialWebsite() == null) ? 0 : getPluginOfficialWebsite().hashCode());
        result = prime * result + ((getPluginUpdateUrl() == null) ? 0 : getPluginUpdateUrl().hashCode());
        result = prime * result + ((getMaintainerName() == null) ? 0 : getMaintainerName().hashCode());
        result = prime * result + ((getMaintainerEmail() == null) ? 0 : getMaintainerEmail().hashCode());
        result = prime * result + ((getAnnouncementInformation() == null) ? 0 : getAnnouncementInformation().hashCode());
        result = prime * result + ((getVersion() == null) ? 0 : getVersion().hashCode());
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
        sb.append(", pluginOfficialWebsite=").append(pluginOfficialWebsite);
        sb.append(", pluginUpdateUrl=").append(pluginUpdateUrl);
        sb.append(", maintainerName=").append(maintainerName);
        sb.append(", maintainerEmail=").append(maintainerEmail);
        sb.append(", announcementInformation=").append(announcementInformation);
        sb.append(", version=").append(version);
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
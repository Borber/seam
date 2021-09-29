package com.borber.sbtream.server.model.entity;

import com.baomidou.mybatisplus.annotation.IdType;
import com.baomidou.mybatisplus.annotation.TableField;
import com.baomidou.mybatisplus.annotation.TableId;
import com.baomidou.mybatisplus.annotation.TableName;
import java.io.Serializable;
import java.util.Date;
import lombok.Data;

/**
 * 平台表
 * @TableName platform
 */
@TableName(value ="platform")
@Data
public class PlatformDO implements Serializable {
    /**
     * 平台id
     */
    @TableId
    private String platformId;

    /**
     * 给平台提供支持的插件id号
     */
    private String pluginId;

    /**
     * 平台名称
     */
    private String platformName;

    /**
     * 平台图标
     */
    private String platformIcon;

    /**
     * 平台代号
     */
    private String platformCode;

    /**
     * 平台官网
     */
    private String platformSite;

    /**
     * 平台api基础链接
     */
    private String platformApiLink;

    /**
     * 是否需要 cookies 来获取最高清资源：默认为0不需要，1需要
     */
    private Boolean gethdCookiesRequired;

    /**
     * 是否提供官方录播
     */
    private Boolean officialRecordProvided;

    /**
     * 官方录播地址 (可能需要从插件中获取) 待定
     */
    private String officialRecordUrl;

    /**
     * 创建时间
     */
    private Date platformCreateTime;

    /**
     * 修改时间
     */
    private Date platformModifyTime;

    /**
     * 创建人id
     */
    private String platformFounderId;

    /**
     * 修改人id
     */
    private String platformModifierId;

    /**
     * 逻辑删除：默认为0，删除操作时修改为1
     */
    private Boolean platformDisabled;

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
        PlatformDO other = (PlatformDO) that;
        return (this.getPlatformId() == null ? other.getPlatformId() == null : this.getPlatformId().equals(other.getPlatformId()))
            && (this.getPluginId() == null ? other.getPluginId() == null : this.getPluginId().equals(other.getPluginId()))
            && (this.getPlatformName() == null ? other.getPlatformName() == null : this.getPlatformName().equals(other.getPlatformName()))
            && (this.getPlatformIcon() == null ? other.getPlatformIcon() == null : this.getPlatformIcon().equals(other.getPlatformIcon()))
            && (this.getPlatformCode() == null ? other.getPlatformCode() == null : this.getPlatformCode().equals(other.getPlatformCode()))
            && (this.getPlatformSite() == null ? other.getPlatformSite() == null : this.getPlatformSite().equals(other.getPlatformSite()))
            && (this.getPlatformApiLink() == null ? other.getPlatformApiLink() == null : this.getPlatformApiLink().equals(other.getPlatformApiLink()))
            && (this.getGethdCookiesRequired() == null ? other.getGethdCookiesRequired() == null : this.getGethdCookiesRequired().equals(other.getGethdCookiesRequired()))
            && (this.getOfficialRecordProvided() == null ? other.getOfficialRecordProvided() == null : this.getOfficialRecordProvided().equals(other.getOfficialRecordProvided()))
            && (this.getOfficialRecordUrl() == null ? other.getOfficialRecordUrl() == null : this.getOfficialRecordUrl().equals(other.getOfficialRecordUrl()))
            && (this.getPlatformCreateTime() == null ? other.getPlatformCreateTime() == null : this.getPlatformCreateTime().equals(other.getPlatformCreateTime()))
            && (this.getPlatformModifyTime() == null ? other.getPlatformModifyTime() == null : this.getPlatformModifyTime().equals(other.getPlatformModifyTime()))
            && (this.getPlatformFounderId() == null ? other.getPlatformFounderId() == null : this.getPlatformFounderId().equals(other.getPlatformFounderId()))
            && (this.getPlatformModifierId() == null ? other.getPlatformModifierId() == null : this.getPlatformModifierId().equals(other.getPlatformModifierId()))
            && (this.getPlatformDisabled() == null ? other.getPlatformDisabled() == null : this.getPlatformDisabled().equals(other.getPlatformDisabled()));
    }

    @Override
    public int hashCode() {
        final int prime = 31;
        int result = 1;
        result = prime * result + ((getPlatformId() == null) ? 0 : getPlatformId().hashCode());
        result = prime * result + ((getPluginId() == null) ? 0 : getPluginId().hashCode());
        result = prime * result + ((getPlatformName() == null) ? 0 : getPlatformName().hashCode());
        result = prime * result + ((getPlatformIcon() == null) ? 0 : getPlatformIcon().hashCode());
        result = prime * result + ((getPlatformCode() == null) ? 0 : getPlatformCode().hashCode());
        result = prime * result + ((getPlatformSite() == null) ? 0 : getPlatformSite().hashCode());
        result = prime * result + ((getPlatformApiLink() == null) ? 0 : getPlatformApiLink().hashCode());
        result = prime * result + ((getGethdCookiesRequired() == null) ? 0 : getGethdCookiesRequired().hashCode());
        result = prime * result + ((getOfficialRecordProvided() == null) ? 0 : getOfficialRecordProvided().hashCode());
        result = prime * result + ((getOfficialRecordUrl() == null) ? 0 : getOfficialRecordUrl().hashCode());
        result = prime * result + ((getPlatformCreateTime() == null) ? 0 : getPlatformCreateTime().hashCode());
        result = prime * result + ((getPlatformModifyTime() == null) ? 0 : getPlatformModifyTime().hashCode());
        result = prime * result + ((getPlatformFounderId() == null) ? 0 : getPlatformFounderId().hashCode());
        result = prime * result + ((getPlatformModifierId() == null) ? 0 : getPlatformModifierId().hashCode());
        result = prime * result + ((getPlatformDisabled() == null) ? 0 : getPlatformDisabled().hashCode());
        return result;
    }

    @Override
    public String toString() {
        StringBuilder sb = new StringBuilder();
        sb.append(getClass().getSimpleName());
        sb.append(" [");
        sb.append("Hash = ").append(hashCode());
        sb.append(", platformId=").append(platformId);
        sb.append(", pluginId=").append(pluginId);
        sb.append(", platformName=").append(platformName);
        sb.append(", platformIcon=").append(platformIcon);
        sb.append(", platformCode=").append(platformCode);
        sb.append(", platformSite=").append(platformSite);
        sb.append(", platformApiLink=").append(platformApiLink);
        sb.append(", gethdCookiesRequired=").append(gethdCookiesRequired);
        sb.append(", officialRecordProvided=").append(officialRecordProvided);
        sb.append(", officialRecordUrl=").append(officialRecordUrl);
        sb.append(", platformCreateTime=").append(platformCreateTime);
        sb.append(", platformModifyTime=").append(platformModifyTime);
        sb.append(", platformFounderId=").append(platformFounderId);
        sb.append(", platformModifierId=").append(platformModifierId);
        sb.append(", platformDisabled=").append(platformDisabled);
        sb.append(", serialVersionUID=").append(serialVersionUID);
        sb.append("]");
        return sb.toString();
    }
}
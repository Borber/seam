package com.borber.sbtream.server.model.vo;

import lombok.Data;

import javax.validation.constraints.NotBlank;

@Data
public class UrlVO {
    /**
     * 插件ID
     */
    @NotBlank
    private String pluginId;

    /**
     * cookies
     */

    private String cookies;
}

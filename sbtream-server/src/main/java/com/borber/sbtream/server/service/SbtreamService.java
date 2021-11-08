package com.borber.sbtream.server.service;

import com.borber.sbtream.plugin.model.DataDTO;

/**
 * @author x
 * @description 用户操作插件返回响应的数据
 * @createDate 2021-11-06 14:58:04
 */
public interface SbtreamService {
    /**
     * 获取直播源地址
     * @param pid 直播平台代号
     * @param rid 主播房间代号
     * @param pluginId 插件ID
     * @return 直播源地址
     */
    DataDTO getUrl(String pid, String rid, String pluginId, String cookies);
}

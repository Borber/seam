package com.borber.sbtream.server.service;

import com.baomidou.mybatisplus.extension.service.IService;
import com.borber.sbtream.common.response.BaseResponse;
import com.borber.sbtream.plugin.model.MetaDataDTO;
import com.borber.sbtream.server.model.entity.PluginDO;

import java.util.List;

/**
* @author x
* @description 针对表【plugin(插件表)】的数据库操作Service
* @createDate 2021-11-06 14:58:04
*/
public interface PluginService extends IService<PluginDO> {
    /**
     * @param pluginDO 添加插件
     */

    void addPlugin(PluginDO pluginDO);

    List<MetaDataDTO> listPlugin();

    List<PluginDO> refreshPlugin();

    void testPlugin();
}

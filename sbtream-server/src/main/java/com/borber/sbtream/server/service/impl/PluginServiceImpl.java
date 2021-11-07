package com.borber.sbtream.server.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.borber.sbtream.plugin.api.Api;
import com.borber.sbtream.plugin.model.MetaDataDTO;
import com.borber.sbtream.server.constans.SBtreamConfigConstans;
import com.borber.sbtream.server.mapper.PluginMapper;
import com.borber.sbtream.server.model.entity.PluginDO;
import com.borber.sbtream.server.service.PluginService;
import com.borber.sbtream.server.util.PluginUtil;
import org.springframework.stereotype.Service;

import javax.annotation.Resource;
import java.io.File;
import java.util.ArrayList;
import java.util.List;

/**
* @author x
* @description 针对表【plugin(插件表)】的数据库操作Service实现
* @createDate 2021-11-06 14:58:04
*/
@Service
public class PluginServiceImpl extends ServiceImpl<PluginMapper, PluginDO>
    implements PluginService{
    @Resource
    PluginMapper pluginMapper;

    @Resource
    PluginUtil pluginUtil;

    @Override
    public void addPlugin(PluginDO pluginDO) {
        pluginMapper.insert(pluginDO);
    }

    @Override
    public List<MetaDataDTO> listPlugin(){

        ArrayList<Api> plugins = new ArrayList<>();
        try {
                plugins = pluginUtil.getPlugins();
        }catch (Exception e){
            e.printStackTrace();
        }
        ArrayList<MetaDataDTO> metas = new ArrayList<>();
        for (Api p: plugins){
            metas.add(p.getMetaData());
        }
        return metas;
    }

    @Override
    public List<PluginDO> refreshPlugin() {
        return null;
    }

    @Override
    public void testPlugin() {

    }
}





package com.borber.sbtream.server.service.impl;

import com.borber.sbtream.plugin.api.Api;
import com.borber.sbtream.plugin.model.MetaDataDTO;
import com.borber.sbtream.plugin.model.DataDTO;
import com.borber.sbtream.server.service.SbtreamService;
import com.borber.sbtream.server.util.PluginUtil;
import org.springframework.stereotype.Service;

import javax.annotation.Resource;
import java.util.ArrayList;

@Service
public class SbtreamServiceImpl implements SbtreamService {

    ArrayList<Api> plugins;

    @Override
    public DataDTO getUrl(String pid, String rid, String pluginId, String cookies) {
        MetaDataDTO meta;
        try {
            plugins = PluginUtil.getPlugins();
            System.out.println(plugins);
        } catch (Exception e) {
            e.printStackTrace();
        }
        for (Api p : plugins){
            meta = p.getMetaData();
            if(meta.getId().equals(pluginId) && meta.getPid().equals(pid)){
                // TODO 添加cookies支持
                return p.getUrl(rid,cookies);
            }
        }
        return new DataDTO();
    }
}

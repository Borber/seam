package com.borber.sbtream.server.service.impl;

import com.baomidou.mybatisplus.core.conditions.query.LambdaQueryWrapper;
import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.borber.sbtream.plugin.api.Api;
import com.borber.sbtream.plugin.model.MetaDataDTO;
import com.borber.sbtream.server.constans.SBtreamConfigConstans;
import com.borber.sbtream.server.mapper.PluginMapper;
import com.borber.sbtream.server.model.entity.PluginDO;
import com.borber.sbtream.server.service.PluginService;
import com.borber.sbtream.server.util.PluginUtil;
import org.springframework.stereotype.Service;
import org.springframework.web.multipart.MultipartFile;

import javax.annotation.Resource;
import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.List;

import static com.borber.sbtream.server.constans.SBtreamConfigConstans.PLUGINS_PATH;

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

    @Override
    public void addPlugin(PluginDO pluginDO) {
        pluginMapper.insert(pluginDO);
    }

    @Override
    public Boolean upload(MultipartFile file) {
        File pluginFolder = new File(PLUGINS_PATH);
        if (!pluginFolder.exists()){
            pluginFolder.mkdir();
        }

        String fileName = file.getOriginalFilename();
        String filePath = PLUGINS_PATH + "/" + fileName;

        File dest = new File(filePath);
        try {
            Files.copy(file.getInputStream(), dest.toPath());
        } catch (IOException e) {
            e.printStackTrace();
            return false;
        }
        //TODO 返回插件的meta而非单纯的bool;
        return true;
    }

    @Override
    public List<MetaDataDTO> listPlugin(){

        ArrayList<Api> plugins = new ArrayList<>();
        try {
                plugins = PluginUtil.getPlugins();
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
        ArrayList<Api> plugins = new ArrayList<>();
        try {
            plugins = PluginUtil.getPlugins();
        }catch (Exception e){
            e.printStackTrace();
        }
        ArrayList<MetaDataDTO> metas = new ArrayList<>();
        for (Api p: plugins){
            metas.add(p.getMetaData());
        }
        LambdaQueryWrapper<PluginDO> queryWrapper = new LambdaQueryWrapper<>();
        for (MetaDataDTO m : metas){

        }
        return null;
    }

    @Override
    public void testPlugin() {

    }
}





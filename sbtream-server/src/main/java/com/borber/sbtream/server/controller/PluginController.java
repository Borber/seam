package com.borber.sbtream.server.controller;

import com.borber.sbtream.common.response.BaseResponse;
import com.borber.sbtream.plugin.model.MetaDataDTO;
import com.borber.sbtream.server.model.entity.PluginDO;
import com.borber.sbtream.server.service.PluginService;
import com.borber.sbtream.server.util.PluginUtil;
import org.springframework.web.bind.annotation.*;
import org.springframework.web.multipart.MultipartFile;

import javax.annotation.Resource;

import java.util.ArrayList;
import java.util.List;

import static com.borber.sbtream.server.constans.SBtreamConfigConstans.PLUGIN_API_URL;

@RestController
@RequestMapping(PLUGIN_API_URL)
public class PluginController {
    @Resource
    PluginService pluginService;

    @PostMapping("list")
    public BaseResponse list(){
        return BaseResponse.success(pluginService.listPlugin());
    }

    @PostMapping("upload")
    public BaseResponse upload(@RequestParam("pluginJar") MultipartFile file){
        System.out.println(file);
        return pluginService.upload(file)?BaseResponse.success("上传成功"):BaseResponse.fail("-1","上传失败");
    }
    @PostMapping("test")
    public BaseResponse test(){
        pluginService.testPlugin();
        return BaseResponse.success("success");
    }
}

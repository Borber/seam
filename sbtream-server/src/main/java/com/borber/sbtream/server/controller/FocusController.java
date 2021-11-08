package com.borber.sbtream.server.controller;

import com.borber.sbtream.common.response.BaseResponse;
import com.borber.sbtream.plugin.model.DataDTO;
import com.borber.sbtream.server.service.SbtreamService;
import com.borber.sbtream.server.service.UserfocusService;
import org.springframework.web.bind.annotation.*;

import javax.annotation.Resource;

import java.util.ArrayList;
import java.util.List;

import static com.borber.sbtream.server.constans.SBtreamConfigConstans.Focus_API_URL;

@RestController
@RequestMapping(Focus_API_URL)
public class FocusController {
    @Resource
    UserfocusService userfocusService;

    @Resource
    SbtreamService sbtreamService;

    @PostMapping("{pid}/{rid}")
    public BaseResponse addFocus(@RequestHeader("token") String token, @RequestParam("pluginId") String pluginId, @PathVariable String rid){
        return userfocusService.add(token,pluginId,rid)? BaseResponse.success(true):BaseResponse.fail("-1","关注失败, 请检查你的输入");
    }

    @PostMapping("{pid}")
    public BaseResponse listFocus(@RequestHeader("token") String token, @RequestParam("pluginId") String pluginId, @PathVariable String pid){
        List<String> rids = userfocusService.listFocus(token, pluginId, pid);
        ArrayList<DataDTO> dataList = new ArrayList<>();
        for (String s : rids){
            dataList.add(sbtreamService.getUrl(pid,s,pluginId,""));
        }
        return BaseResponse.success(dataList);
    }
}

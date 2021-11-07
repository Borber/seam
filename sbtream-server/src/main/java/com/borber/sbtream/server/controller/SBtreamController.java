package com.borber.sbtream.server.controller;


import com.borber.sbtream.common.response.BaseResponse;
import com.borber.sbtream.plugin.model.UrlDTO;
import com.borber.sbtream.server.model.vo.UrlVO;
import com.borber.sbtream.server.service.SbtreamService;
import org.springframework.validation.annotation.Validated;
import org.springframework.web.bind.annotation.*;

import javax.annotation.Resource;

import static com.borber.sbtream.server.constans.SBtreamConfigConstans.URL_API_TRL;

/**
 * @author BORBER
 */
@RestController
@RequestMapping(URL_API_TRL)
public class SBtreamController {
    @Resource
    SbtreamService service;
    /**
     * #TODO 获取指定平台指定房间号的直播状态/地址
     */
    @PostMapping("/{pid}/{rid}")
    public BaseResponse getRealUrl(@PathVariable String pid, @PathVariable String rid, @RequestBody @Validated UrlVO vo){
        UrlDTO urlDTO = service.getUrl(pid, rid, vo.getPluginId(), vo.getCookies());
        if (urlDTO==null){
            return BaseResponse.fail("-1","失败");
        }
        return BaseResponse.success(urlDTO);
    }

    /**
     * #TODO 返回平台信息
     * @param pid 平台代号
     * @return json
     */
    @GetMapping("/{pid}")
    public BaseResponse getPlatform(@PathVariable String pid){
        return null;
    }

    /**
     * #TODO 返回主播信息
     * @param pid 平台代号
     * @param rid 房间号
     * @return json
     */
    @GetMapping("/{pid}/{rid}")
    public BaseResponse getAnchor(@PathVariable String pid, @PathVariable String rid){
        return null;
    }

}

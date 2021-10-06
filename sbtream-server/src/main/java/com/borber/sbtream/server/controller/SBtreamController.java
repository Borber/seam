package com.borber.sbtream.server.controller;

import com.borber.sbtream.api.plugin.response.BaseResponse;
import org.springframework.web.bind.annotation.*;

/**
 * @author BORBER
 */
@RestController
@RequestMapping("/api/v1")
public class SBtreamController {
    /**
     * #TODO 获取指定平台指定房间号的直播状态/地址
     */
    @PostMapping("/{pid}/{rid}")
    public BaseResponse getRealUrl(@PathVariable String pid, @PathVariable String rid){
        return null;
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

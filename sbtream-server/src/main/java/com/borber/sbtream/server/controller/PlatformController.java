package com.borber.sbtream.server.controller;

import com.borber.sbtream.common.response.BaseResponse;
import com.borber.sbtream.server.service.PlatformService;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import javax.annotation.Resource;

import static com.borber.sbtream.server.constans.SBtreamConfigConstans.PLATFORM_API_URL;

/**
 * @author BORBER
 */
@RestController
@RequestMapping(PLATFORM_API_URL)
public class PlatformController {
    @Resource
    PlatformService platformService;

    @PostMapping("list")
    public BaseResponse list(){
        return BaseResponse.success("");
    }

    @PostMapping("{pid}")
    public BaseResponse info(@PathVariable String pid){
        return BaseResponse.success(platformService.getById(pid));
    }
}

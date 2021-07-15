package com.borber.sbtream.api.plugin.api;

import com.borber.sbtream.api.plugin.model.PlatformBean;
import com.borber.sbtream.api.plugin.response.BaseResponse;

/**
 * @ClassName GetSomeThing
 * @Description 返回对应的数据
 * @Author Borber
 * @Date 2021/7/15 上午10:47
 * @Version 0.0.1
 **/
public interface GetSomeThing {
    /**
     * 获取直播平台元数据
     * @Author Borber
     * @Date 2021/7/15 上午11:28
     * @Param []
     * @return com.borber.sbtream.api.plugin.response.BaseResponse<com.borber.sbtream.api.plugin.model.PlatformBean,java.lang.Object>
     **/
    BaseResponse<PlatformBean,Object> getMetaData();
    
    /**
     * 通过房间号获取真实直播地址
     * @Author Borber
     * @Date 2021/7/15 上午11:30
     * @Param [rid]
     * @return com.borber.sbtream.api.plugin.response.BaseResponse<java.lang.String,java.lang.Object>
     **/
    BaseResponse<String,Object> getRealUrl(String rid);

}

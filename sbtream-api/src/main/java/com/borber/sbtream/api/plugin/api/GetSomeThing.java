package com.borber.sbtream.api.plugin.api;

import com.borber.sbtream.api.plugin.model.PlatformBean;
import com.borber.sbtream.api.plugin.model.UrlBean;
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
     * @Date 2021/7/15 下午10:57
     * @Param [rid]
     * @return com.borber.sbtream.api.plugin.response.BaseResponse<com.borber.sbtream.api.plugin.model.UrlBean,java.lang.Object>
     **/
    BaseResponse<UrlBean,Object> getRealUrl(String rid);

    /**
     * 通过房间号获取真实直播地址  如果是需要 Cookies 来获取最高清的的方法 请实现这个方法
     * @Author Borber
     * @Date 2021/7/15 下午10:54
     * @Param [rid, cookies]
     * @return com.borber.sbtream.api.plugin.response.BaseResponse<com.borber.sbtream.api.plugin.model.UrlBean,java.lang.Object>
     **/
    BaseResponse<UrlBean,Object> getRealUrl(String rid, String cookies);

}

package com.borber.sbtream.api.plugin.api;


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
     * @return String 但是必须是 BaseResponse<PlatformDTO, Object> 的 序列化字符串
     **/
    String getMetaData();
    
    /**
     * 通过房间号获取真实直播地址
     * @Author Borber
     * @Date 2021/7/15 下午10:57
     * @Param [rid]
     * @return String 但是必须是 BaseResponse<UrlDTO, Object> 的 序列化字符串
     **/
    String getRealUrl(String rid);

    /**
     * 通过房间号获取真实直播地址  如果是需要 Cookies 来获取最高清的的方法 请实现这个方法
     * @Author Borber
     * @Date 2021/7/15 下午10:54
     * @Param [rid, cookies]
     * @return String 但是必须是 BaseResponse<UrlDTO, Object> 的 序列化字符串
     **/
    String getRealUrl(String rid, String cookies);

}

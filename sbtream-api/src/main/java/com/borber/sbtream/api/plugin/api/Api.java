package com.borber.sbtream.api.plugin.api;


/**
 * @ClassName GetSomeThing
 * @Description 返回对应的数据
 * @Author Borber
 * @Date 2021/7/15 上午10:47
 * @Version 0.0.1
 **/
public interface Api {
    /**
     * 获取直播平台元数据
     * @Author Borber
     * @Date 2021/7/15 上午11:28
     * @Param []
     * @return String 但是必须是 BaseResponse<PlatformDTO, Object> 的 序列化字符串
     **/
    String getMetaData();

    /**
     * 通过房间号获取真实直播地址  如果是需要 Cookies 来获取即使用, 如果不需要则忽略 cookies 变量
     * @Author Borber
     * @Date 2021/7/15 下午10:54
     * @Param [rid, cookies]
     * @return String 但是必须是 BaseResponse<UrlDTO, Object> 的 序列化字符串
     **/
    String getRealUrl(String rid, String cookies);

    /**
     *
     * 通过cookies发送弹幕, 返回是否成功
     * @Author Borber
     * @Date  2021/09/08 下午03:50
     * @Param
     * @return String 但是必须是 BaseResponse<Boolean, Object> 的 序列化字符串
     */
    String sendDanmaku(String str, String cookies);

}

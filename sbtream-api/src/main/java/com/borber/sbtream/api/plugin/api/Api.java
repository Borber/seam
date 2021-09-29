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
     * @author Borber
     * @date 2021/7/15 上午11:28
     * @param pid 平台代号
     * @return String 但是必须是 BaseResponse<PlatformDTO, Object> 的 序列化字符串
     **/
    String getMetaData(String pid);

    /**
     * 通过房间号获取真实直播地址  如果是需要 Cookies 来获取即使用, 如果不需要则忽略 cookies 变量
     * @author Borber
     * @date 2021/7/15 下午10:54
     * @param pid 平台代号
     * @param rid 房间号
     * @param cookies 用户认证
     * @return String 但是必须是 BaseResponse<UrlDTO, Object> 的 序列化字符串
     **/
    String getRealUrl(String pid, String rid, String cookies);

    /**
     *
     * 通过cookies发送弹幕, 返回是否成功
     * @author Borber
     * @date  2021/09/08 下午03:50
     * @param pid 平台代号
     * @param rid 主播房间号
     * @param str 弹幕内容
     * @param cookies 用户认证
     * @return String 但是必须是 BaseResponse<Boolean, Object> 的 序列化字符串
     */
    String sendDanmaku(String pid, String rid, String str, String cookies);

}

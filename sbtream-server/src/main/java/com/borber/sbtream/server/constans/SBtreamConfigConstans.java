package com.borber.sbtream.server.constans;

import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Component;

/**
 * @ClassName SBtreamConfigConstans
 * @Description TODO
 * @Author Borber
 * @Date 2021/7/15 下午5:34
 * @Version 0.0.1
 **/
public class SBtreamConfigConstans {
    public static final String PLUGINS_PATH = "plugin";
    public static final String SETTING_PATH = "settings";
    public static final String BASE_API_URL = "/api/v1";
    public static final String USER_API_URL = BASE_API_URL + "/user";
    public static final String PLATFORM_API_URL = BASE_API_URL + "/platform";
    public static final String PLUGIN_API_URL = BASE_API_URL + "/plugin";
    public static final String URL_API_TRL = BASE_API_URL + "/url";
}

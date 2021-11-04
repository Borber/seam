package com.borber.sbtream.server.constans;

import lombok.Getter;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Component;

/**
 * @ClassName SBtreamConfigConstans
 * @Description TODO
 * @Author Borber
 * @Date 2021/7/15 下午5:34
 * @Version 0.0.1
 **/
@Component
public class SBtreamConfigConstans {
    @Value("${sbtream.plugin.path}")
    public static String PLUGINS_PATH;
    @Value("${sbtream.data.json.setting}")
    public static String SETTING_PATH;

    public static final String BASE_API_URL = "/api/v1";
    public static final String USER_API_URL = BASE_API_URL + "/user";
}

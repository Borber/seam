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
@Getter
public class SBtreamConfigConstans {
    @Value("${sbtream.plugin.path}")
    private String PLUGINS_PATH;
    @Value("${sbtream.data.json.setting}")
    private String SETTING_PATH;
}

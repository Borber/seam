package com.borber.sbtream.server.service;

import com.borber.sbtream.server.model.entity.UserfocusDO;
import com.baomidou.mybatisplus.extension.service.IService;

import java.util.List;

/**
 *
 */
public interface UserfocusService extends IService<UserfocusDO> {
    Boolean add(String token, String pluginId, String rid);
    List<String> listFocus(String token, String pluginId, String pid);
}

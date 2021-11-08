package com.borber.sbtream.server.service.impl;

import cn.hutool.jwt.JWT;
import com.baomidou.mybatisplus.core.conditions.query.LambdaQueryWrapper;
import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.borber.sbtream.server.model.entity.UserfocusDO;
import com.borber.sbtream.server.service.UserfocusService;
import com.borber.sbtream.server.mapper.UserfocusMapper;
import org.springframework.stereotype.Service;

import javax.annotation.Resource;
import java.util.ArrayList;
import java.util.List;

/**
 *
 */
@Service
public class UserfocusServiceImpl extends ServiceImpl<UserfocusMapper, UserfocusDO>
    implements UserfocusService{
    @Resource
    UserfocusMapper userfocusMapper;

    @Override
    public Boolean add(String token, String pluginId, String rid) {
        String id = (String) JWT.of(token).getPayload("id");
        LambdaQueryWrapper<UserfocusDO> lambdaQueryWrapper = new LambdaQueryWrapper<>();
        lambdaQueryWrapper.eq(UserfocusDO::getUserId, id)
                .eq(UserfocusDO::getPluginId, pluginId)
                .eq(UserfocusDO::getRid, rid);
        List<UserfocusDO> list = userfocusMapper.selectList(lambdaQueryWrapper);
        if (list.size() > 0){
            return false;
        }
        UserfocusDO userfocusDO = new UserfocusDO();
        userfocusDO.setUserId(id);
        userfocusDO.setPluginId(pluginId);
        userfocusDO.setRid(rid);
        userfocusMapper.insert(userfocusDO);
        return true;
    }

    @Override
    public List<String> listFocus(String token, String pluginId, String pid) {
        String id = (String) JWT.of(token).getPayload("id");
        LambdaQueryWrapper<UserfocusDO> lambdaQueryWrapper = new LambdaQueryWrapper<>();
        lambdaQueryWrapper.eq(UserfocusDO::getUserId, id);
        List<UserfocusDO> list = userfocusMapper.selectList(lambdaQueryWrapper);
        ArrayList<String> rids = new ArrayList<>();
        for (UserfocusDO userfocusDO : list) {
            rids.add(userfocusDO.getRid());
        }
        return rids;
    }
}





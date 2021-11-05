package com.borber.sbtream.server.service.impl;

import cn.hutool.jwt.JWT;
import com.baomidou.mybatisplus.core.conditions.query.LambdaQueryWrapper;
import com.baomidou.mybatisplus.core.conditions.query.QueryWrapper;
import com.baomidou.mybatisplus.core.toolkit.Wrappers;
import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.borber.sbtream.server.model.dto.LoginDTO;
import com.borber.sbtream.server.model.vo.UserVO;
import com.borber.sbtream.server.model.entity.UserDO;
import com.borber.sbtream.server.service.UserService;
import com.borber.sbtream.server.mapper.UserMapper;
import org.springframework.beans.BeanUtils;
import org.springframework.stereotype.Service;

import javax.annotation.Resource;
import java.util.List;

/**
 *
 */
@Service
public class UserServiceImpl extends ServiceImpl<UserMapper, UserDO>
    implements UserService{

    @Resource
    UserMapper userMapper;

    @Override
    public void addUser(UserVO vo) {
        UserDO user = new UserDO();
        BeanUtils.copyProperties(vo,user);
        userMapper.insert(user);
    }

    @Override
    public UserDO getUserByID(String id) {
        return userMapper.selectById(id);
    }



    //生成签名
    @Override
    public LoginDTO login(UserVO vo) {
        LambdaQueryWrapper<UserDO> queryWrapper = new LambdaQueryWrapper<>();
        queryWrapper.eq(UserDO::getName, "BORBER");
        UserDO user = userMapper.selectOne(queryWrapper);
//        UserDO user = userMapper.getUserByNAP(vo.getName(), vo.getPasswd());
        System.out.println(user);
        byte[] key = vo.getPasswd().getBytes();
        String token = JWT.create()
                .setPayload("id", user.getId())
                .setKey(key)
                .sign();
        LoginDTO loginDTO = new LoginDTO();
        loginDTO.setToken(token);
        return loginDTO;
    }
}





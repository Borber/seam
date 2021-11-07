package com.borber.sbtream.server.service.impl;

import cn.hutool.jwt.JWT;
import com.baomidou.mybatisplus.core.conditions.query.LambdaQueryWrapper;
import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.borber.sbtream.server.mapper.UserMapper;
import com.borber.sbtream.server.model.dto.LoginDTO;
import com.borber.sbtream.server.model.entity.UserDO;
import com.borber.sbtream.server.model.vo.UserVO;
import com.borber.sbtream.server.service.UserService;
import org.springframework.beans.BeanUtils;
import org.springframework.stereotype.Service;

import javax.annotation.Resource;

/**
* @author x
* @description 针对表【user(用户及设置表)】的数据库操作Service实现
* @createDate 2021-11-06 14:58:04
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
        System.out.println(user);
        userMapper.insert(user);
    }

    @Override
    public UserDO getUserById(String id) {
        return userMapper.selectById(id);
    }


    @Override
    public LoginDTO login(UserVO vo) {
        LambdaQueryWrapper<UserDO> queryWrapper = new LambdaQueryWrapper<>();
        queryWrapper.eq(UserDO::getName, vo.getName())
                .eq(UserDO::getPasswd, vo.getPasswd());
        UserDO user = userMapper.selectOne(queryWrapper);
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





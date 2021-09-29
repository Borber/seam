package com.borber.sbtream.server.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.borber.sbtream.server.model.entity.UserDO;
import com.borber.sbtream.server.service.UserService;
import com.borber.sbtream.server.mapper.UserMapper;
import org.springframework.stereotype.Service;

/**
 *
 */
@Service
public class UserServiceImpl extends ServiceImpl<UserMapper, UserDO>
    implements UserService{

}





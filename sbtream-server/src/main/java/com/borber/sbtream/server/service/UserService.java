package com.borber.sbtream.server.service;

import com.borber.sbtream.server.model.dto.LoginDTO;
import com.borber.sbtream.server.model.vo.UserVO;
import com.borber.sbtream.server.model.entity.UserDO;
import com.baomidou.mybatisplus.extension.service.IService;

/**
 *
 */
public interface UserService extends IService<UserDO> {
    void addUser(UserVO user);
    UserDO getUserByID(String id);
    LoginDTO login(UserVO vo);
}

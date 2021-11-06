package com.borber.sbtream.server.service;

import com.baomidou.mybatisplus.extension.service.IService;
import com.borber.sbtream.server.model.dto.LoginDTO;
import com.borber.sbtream.server.model.entity.UserDO;
import com.borber.sbtream.server.model.vo.UserVO;

/**
* @author x
* @description 针对表【user(用户及设置表)】的数据库操作Service
* @createDate 2021-11-06 14:58:04
*/

public interface UserService extends IService<UserDO> {
    /**
     * @param user 添加用户
     */
    void addUser(UserVO user);

    /**
     * @param id 用户ID
     * @return 用户信息
     */
    UserDO getUserById(String id);

    /**
     * @param vo 请求穿参
     * @return 登录token
     */
    LoginDTO login(UserVO vo);
}
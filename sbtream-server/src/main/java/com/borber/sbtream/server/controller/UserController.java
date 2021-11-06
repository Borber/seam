package com.borber.sbtream.server.controller;



import com.borber.sbtream.common.response.BaseResponse;
import com.borber.sbtream.server.conf.annotation.PassToken;
import com.borber.sbtream.server.model.vo.UserVO;
import com.borber.sbtream.server.service.UserService;
import org.springframework.validation.annotation.Validated;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import javax.annotation.Resource;

import static com.borber.sbtream.server.constans.SBtreamConfigConstans.USER_API_URL;

@RestController
@RequestMapping(USER_API_URL)
public class UserController {
    @Resource
    UserService userService;
    // 登陆接口

    @PassToken
    @PostMapping("login")
    public BaseResponse login(@RequestBody @Validated UserVO vo){
        return BaseResponse.success(userService.login(vo));
    }

    // 注册接口

    @PassToken
    @PostMapping("register")
    public BaseResponse register(@RequestBody @Validated UserVO vo){
        userService.addUser(vo);
        return BaseResponse.success("注册成功");
    }

}

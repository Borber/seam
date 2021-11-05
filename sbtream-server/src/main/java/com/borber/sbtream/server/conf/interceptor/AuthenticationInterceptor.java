package com.borber.sbtream.server.conf.interceptor;

import cn.hutool.jwt.JWT;
import com.borber.sbtream.server.conf.annotation.PassToken;
import com.borber.sbtream.server.model.entity.UserDO;
import com.borber.sbtream.server.service.UserService;
import org.springframework.web.method.HandlerMethod;
import org.springframework.web.servlet.HandlerInterceptor;

import javax.annotation.Resource;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import java.lang.reflect.Method;

public class AuthenticationInterceptor implements HandlerInterceptor {
    @Resource
    UserService userService;

    @Override
    public boolean preHandle(HttpServletRequest httpServletRequest, HttpServletResponse httpServletResponse, Object object) throws Exception {
        String token = httpServletRequest.getHeader("token");// 从 http 请求头中取出 token
        // 如果不是映射到方法直接通过
        if (!(object instanceof HandlerMethod)) {
            return true;
        }
        HandlerMethod handlerMethod = (HandlerMethod) object;
        Method method = handlerMethod.getMethod();
        //检查是否有passtoken注释，有则跳过认证
        if (method.isAnnotationPresent(PassToken.class)) {
            PassToken passToken = method.getAnnotation(PassToken.class);
            System.out.println("走PassToken");
            if (passToken.required()) {
                System.out.println("PassToken返回Ture了");
                return true;
            }
        }
        System.out.println("继续了");
        //检查有没有需要用户权限的注解
        // 执行认证
        if (token == null) {
            throw new RuntimeException("无token，请重新登录");
        }
        // 获取 token 中的 user id

        String id = (String) JWT.of(token).getPayload("id");
        UserDO user = userService.getUserByID(id);
        if (user == null) {
            throw new RuntimeException("用户不存在，请重新登录");
        }
        if (!JWT.of(token).setKey(token.getBytes()).verify()){
            throw new RuntimeException("401");
        }

        return true;
    }

}
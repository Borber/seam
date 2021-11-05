package com.borber.sbtream.server.model.dto;

import lombok.Data;

//登陆返回Token
@Data
public class LoginDTO {
    // jwt token
    private String token;
}

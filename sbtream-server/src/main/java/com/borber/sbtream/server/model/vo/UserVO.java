package com.borber.sbtream.server.model.vo;

import lombok.Data;

import javax.validation.constraints.NotBlank;

@Data
public class UserVO {
    @NotBlank
    private String name;
    // TODO 前后端对密码进行加密
    @NotBlank
    private String passwd;

    private String mail;
}

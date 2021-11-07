package com.borber.sbtream.plugin.constans;

import lombok.AllArgsConstructor;
import lombok.Getter;

@Getter
@AllArgsConstructor
public enum StatusType {
    /**
     * 开播啦
     */
    _200("200"),
    /**
     * 未开播
     */
    _404("404");

    /**
     * 状态码
     */
    private String code;
}

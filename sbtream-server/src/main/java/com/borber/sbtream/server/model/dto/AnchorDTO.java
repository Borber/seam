package com.borber.sbtream.server.model.dto;

import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.NoArgsConstructor;

import java.io.Serializable;

/**
 * @ClassName AnchorDTO
 * @Description 储存主播相关信息的实体类
 * @Author Borber
 * @Date 2021/7/15 下午4:17
 * @Version 0.0.1
 **/
@Data
@AllArgsConstructor
@NoArgsConstructor
public class AnchorDTO implements Serializable {
    private String rid;
    private String name;
    private Boolean star;
}

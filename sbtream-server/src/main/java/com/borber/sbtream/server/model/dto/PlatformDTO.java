package com.borber.sbtream.server.model.dto;

import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.NoArgsConstructor;

import java.io.Serializable;
import java.util.List;

/**
 * @ClassName PlatformDTO
 * @Description 用于储存对应平台的所有关注主播
 * @Author Borber
 * @Date 2021/7/15 下午4:19
 * @Version 0.0.1
 **/
@Data
@AllArgsConstructor
@NoArgsConstructor
public class PlatformDTO implements Serializable {
    private String pid;
    private List<AnchorDTO> anchorDTO;
}

package com.borber.sbtream.server.model.dto;

import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.NoArgsConstructor;

import java.io.Serializable;
import java.util.List;

/**
 * @ClassName FavoriteDTO
 * @Description 储存所有平台关注主播
 * @Author Borber
 * @Date 2021/7/15 下午4:19
 * @Version 0.0.1
 **/
@Data
@AllArgsConstructor
@NoArgsConstructor
public class FavoriteDTO implements Serializable {
    List<PlatformServiceDTO> platformDTO;
}

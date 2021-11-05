package com.borber.sbtream.server.mapper;

import com.borber.sbtream.server.model.entity.UserDO;
import com.baomidou.mybatisplus.core.mapper.BaseMapper;
import org.apache.ibatis.annotations.Param;
import org.apache.ibatis.annotations.Select;

import java.util.List;

/**
 * @Entity com.borber.sbtream.server.model.entity.UserDO
 */
public interface UserMapper extends BaseMapper<UserDO> {
    @Select("<script>" +
            "SELECT * from user " +
            " where name=#{name} AND passwd=#{passwd} limit 1" +
            "</script>")
    UserDO getUserByNAP(@Param("name") String name, @Param("passwd") String passwd);


}





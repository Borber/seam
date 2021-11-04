package com.borber.sbtream.server.conf;

import org.mybatis.spring.annotation.MapperScan;
import org.springframework.context.annotation.Configuration;

@MapperScan("com.borber.sbtream.server.mapper")
@Configuration
public class MybatisConfiguration {
}

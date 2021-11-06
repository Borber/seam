package com.borber.sbtream.server.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.borber.sbtream.server.mapper.AdminMapper;
import com.borber.sbtream.server.model.entity.AdminDO;
import com.borber.sbtream.server.service.AdminService;
import org.springframework.stereotype.Service;

/**
* @author x
* @description 针对表【admin(管理员表)】的数据库操作Service实现
* @createDate 2021-11-06 14:58:04
*/
@Service
public class AdminServiceImpl extends ServiceImpl<AdminMapper, AdminDO>
    implements AdminService{

}





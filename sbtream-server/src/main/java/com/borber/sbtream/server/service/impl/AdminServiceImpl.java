package com.borber.sbtream.server.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.borber.sbtream.server.model.entity.AdminDO;
import com.borber.sbtream.server.service.AdminService;
import com.borber.sbtream.server.mapper.AdminMapper;
import org.springframework.stereotype.Service;

/**
 *
 */
@Service
public class AdminServiceImpl extends ServiceImpl<AdminMapper, AdminDO>
    implements AdminService{

}





package com.borber.sbtream.server.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.borber.sbtream.server.mapper.WatchHistoryMapper;
import com.borber.sbtream.server.model.entity.WatchHistoryDO;
import com.borber.sbtream.server.service.WatchHistoryService;
import org.springframework.stereotype.Service;

/**
* @author x
* @description 针对表【watch_history(观看记录表)】的数据库操作Service实现
* @createDate 2021-11-06 14:58:04
*/
@Service
public class WatchHistoryServiceImpl extends ServiceImpl<WatchHistoryMapper, WatchHistoryDO>
    implements WatchHistoryService{

}





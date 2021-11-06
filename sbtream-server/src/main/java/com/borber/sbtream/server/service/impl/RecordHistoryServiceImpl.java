package com.borber.sbtream.server.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.borber.sbtream.server.mapper.RecordHistoryMapper;
import com.borber.sbtream.server.model.entity.RecordHistoryDO;
import com.borber.sbtream.server.service.RecordHistoryService;
import org.springframework.stereotype.Service;

/**
* @author x
* @description 针对表【record_history(录播历史表)】的数据库操作Service实现
* @createDate 2021-11-06 14:58:04
*/
@Service
public class RecordHistoryServiceImpl extends ServiceImpl<RecordHistoryMapper, RecordHistoryDO>
    implements RecordHistoryService{

}





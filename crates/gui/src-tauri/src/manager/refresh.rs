use crate::{clients, config::headers, service};

use std::collections::HashMap;

use anyhow::Result;
use tauri::{App, Manager};

// TODO 声明返回类型, 指明所属平台
// 刷新单个订阅
pub async fn refresh(app: &App, live: String, rid: String) -> Result<()> {
    let clients = clients!();
    let node = clients
        .get(&live)
        .unwrap()
        .get(&rid, Some(headers(&live)))
        .await?;
    
    app.emit_all("refresh", node)?;
    Ok(())
}

/// 刷新所有订阅的直播源
pub async fn refresh_all(app: &App) -> Result<()> {
    let lives = service::subscribe::all().await?;
    let mut lists = HashMap::new();
    for live in lives {
        let entry = lists.entry(live.live).or_insert_with(Vec::new);
        entry.push(live.rid);
    }

    loop {
        if lists.is_empty() {
            // TODO 发送更新完毕事件
            break;
        }

        let once = lists
            .iter_mut()
            .map(|(live, rids)| rids.pop().map(|rid| (live.clone(), rid)))
            .collect::<Vec<_>>();

        for (live, rid) in once.into_iter().flatten() {
            refresh(app, live, rid).await?;
        }

        // 去除所需获取主播为空的平台
        lists.retain(|_, rids| !rids.is_empty());

        // 等待间隔
        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    }
    Ok(())
}

// TODO 手动更新
// 1. 全部获取完毕发送更新完毕通知给前端
//    - 开播
//        - 存在卡片, 不做动作
//        - 不存在卡片, 新增直播卡片
//    - 未开播
//        - 存在卡片, 删除
//        - 不存在卡片, 不做动作

// TODO 定时更新直播
// 界面启动时,调用后端命令,然后获取App句柄,随后进行循环命令

// TODO 设置增加 每次自动刷新的间隔时间

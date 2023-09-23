use anyhow::Result;
use sea_orm::*;

use crate::{database::subscribe, pool};

/// 获取所有订阅
///
/// Get all subscribe
pub async fn all() -> Result<Vec<subscribe::Model>> {
    Ok(subscribe::Entity::find().all(pool!()).await?)
}

/// 添加订阅
///
/// Add subscribe
pub async fn add(live: String, rid: String) -> Result<bool> {
    subscribe::Entity::insert(subscribe::ActiveModel {
        live: Set(live),
        rid: Set(rid),
    })
    .exec(pool!())
    .await?;
    Ok(true)
}

/// 删除订阅
///
/// Delete subscribe
pub async fn remove(live: String, rid: String) -> Result<bool> {
    subscribe::Entity::delete_many()
        .filter(subscribe::Column::Live.eq(live))
        .filter(subscribe::Column::Rid.eq(rid))
        .exec(pool!())
        .await?;
    Ok(true)
}

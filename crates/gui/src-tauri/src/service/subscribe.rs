use anyhow::Result;
use sea_orm::*;

use crate::{database::subscribe, pool};

pub async fn add(live: String, rid: String) -> Result<bool> {
    subscribe::Entity::insert(subscribe::ActiveModel {
        live: Set(live),
        rid: Set(rid),
    })
    .exec(pool!())
    .await?;
    Ok(true)
}

pub async fn all() -> Result<Vec<subscribe::Model>> {
    Ok(subscribe::Entity::find().all(pool!()).await?)
}

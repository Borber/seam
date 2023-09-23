pub mod subscribe;

use anyhow::Result;
use sea_orm::*;

/// 初始化数据库
pub async fn init(db: &DatabaseConnection) -> Result<ExecResult, DbErr> {
    let backend = DbBackend::Sqlite;
    let schema = Schema::new(backend);
    let stmt = backend.build(&schema.create_table_from_entity(subscribe::Entity));
    db.execute(stmt).await
}

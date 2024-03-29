use std::{collections::HashMap, path::Path, sync::Arc};

use sea_orm::Database;
use sea_orm::DatabaseConnection;
use seam_core::live::{self, Live};
use tokio::sync::OnceCell;

use crate::database;
use crate::util::bin_dir;

#[macro_export]
macro_rules! pool {
    () => {
        &$crate::common::CONTEXT.get().unwrap().pool
    };
}

#[macro_export]
macro_rules! clients {
    () => {
        &$crate::common::CONTEXT.get().unwrap().clients
    };
}

pub static CONTEXT: OnceCell<Context> = OnceCell::const_new();

pub struct Context {
    pub pool: DatabaseConnection,
    pub clients: HashMap<String, Arc<dyn Live>>,
}

pub async fn load() -> Context {
    let path = Path::new(&bin_dir()).join("data.db");
    let flag = path.exists();
    if !flag {
        std::fs::File::create(&path).unwrap();
    }

    // TODO 后续需要优化
    let path = format!("sqlite://{}", path.to_str().unwrap());

    let pool = Database::connect(path)
        .await
        .expect("Connect database failed");
    if !flag {
        // 初始化数据库
        // TODO 打印日志
        match database::init(&pool).await {
            Ok(_) => {
                println!("初始化数据库成功");
            }
            Err(e) => {
                panic!("{}", e.to_string());
            }
        };
    }
    let clients = live::all();
    Context { pool, clients }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test() {
        println!(
            "{:#?}",
            &super::CONTEXT
                .get()
                .unwrap()
                .clients
                .get("bili")
                .unwrap()
                .get("6", None)
                .await
                .unwrap()
        );
    }
}

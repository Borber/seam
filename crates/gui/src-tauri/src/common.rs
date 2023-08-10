use std::{collections::HashMap, sync::Arc};

use once_cell::sync::Lazy;
use seam_core::live::{self, Live};

pub static GLOBAL_CLIENT: Lazy<HashMap<String, Arc<dyn Live>>> = Lazy::new(live::all);

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test() {
        println!(
            "{:#?}",
            super::GLOBAL_CLIENT
                .get("bili")
                .unwrap()
                .get("6", None)
                .await
                .unwrap()
        );
    }
}

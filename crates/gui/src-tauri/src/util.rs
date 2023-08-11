use anyhow::Result;
use std::process::Command;

use crate::config::CONFIG;

#[cfg(windows)]
const SEPARATOR: &str = "\\";

#[cfg(not(windows))]
const SEPARATOR: &str = "/";

pub fn bin_dir() -> String {
    let p = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    format!("{p}{SEPARATOR}")
}

pub fn play(url: &str) -> Result<bool> {
    if CONFIG.play.bin.is_empty() {
        return Result::Err(anyhow::anyhow!("please set play bin"));
    }
    let _ = match Command::new(&CONFIG.play.bin)
        .args(&CONFIG.play.args)
        .arg(url)
        .spawn()
    {
        Ok(child) => child,
        Err(_) => return Result::Err(anyhow::anyhow!("play error")),
    };
    Ok(true)
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_play() {
        super::play("https://cn-gddg-cu-01-02.bilivideo.com/live-bvc/217714/live_7734200_bs_1348183_bluray.flv?expires=1691721645&pt=h5&deadline=1691721645&len=0&oi=989332308&platform=h5&qn=10000&trid=1000241c8c242c7547639410b015865e52c8&uipk=100&uipv=100&nbs=1&uparams=cdn,deadline,len,oi,platform,qn,trid,uipk,uipv,nbs&cdn=cn-gotcha01&upsig=6750be48b70e8858b347738ba6e8404c&sk=15a1692aea632e29feeea2a18b1d9063&p2p_type=0&sl=10&free_type=0&mid=0&sid=cn-gddg-cu-01-02&chash=0&sche=ban&score=18&pp=rtmp&source=onetier&trace=8a0&site=21ff47c9de1e6b98250021b0a0e34eb5&order=1").unwrap();
    }
}

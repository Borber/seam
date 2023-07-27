use once_cell::sync::Lazy;
use reqwest::Client;

// TODO 这玩意也应该额外传入
pub const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36 Edg/108.0.1462.54";

// TODO 这玩意应该从外边传进来, 因为有可能需要代理的 Client
pub static CLIENT: Lazy<Client> = Lazy::new(Client::new);

// TODO 全局 Client 对象库

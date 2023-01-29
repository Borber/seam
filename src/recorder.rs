use tokio::process::Command;

pub async fn record(url: &str, output: &str) {
    let exe = std::env::current_exe().unwrap();
    let cwd = exe.parent().unwrap();
    Command::new("ffmpeg")
        .args([
            "-hide_banner",
            "-loglevel",
            "error",
            "-i",
            url,
            "-c",
            "copy",
            output,
        ])
        .current_dir(cwd)
        .spawn()
        .unwrap()
        .wait()
        .await
        .unwrap();
}

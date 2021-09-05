use std::process::Command;

pub async fn start_checker() {
    let arg = if cfg!(target_os = "windows") {
        "-n"
    } else {
        "-c"
    };

    let _output = Command::new("ping")
        .arg(arg)
        .arg("1")
        .arg("8.8.8.8")
        .output()
        .unwrap();
}

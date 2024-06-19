#[cfg(feature = "windows-specific")]
extern crate winapi;

use std::io;
use std::process::Command;
use std::path::Path;
use std::env;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::thread::sleep;


#[cfg(feature = "windows-specific")]
use winapi::um::winbase::SetThreadExecutionState;
#[cfg(feature = "windows-specific")]
use winapi::um::winnt::{ES_CONTINUOUS, ES_SYSTEM_REQUIRED};

fn main() {
    prevent_sleep();
    println!("电脑将不会自动睡眠直到程序结束...");

    // 获取用户输入的目录
    println!("请输入文件夹地址:");
    let mut dir = String::new();
    io::stdin().read_line(&mut dir).expect("读取输入失败");
    let dir = dir.trim(); // 移除可能的换行符
    println!("请输入执行提交的Unix时间戳（秒）:");
    let mut timestamp = String::new();
    io::stdin().read_line(&mut timestamp).expect("读取时间失败");
    let timestamp: u64 = timestamp.trim().parse().expect("时间格式错误");
    let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("时间读取失败").as_secs();
    // if timestamp > now {
    //     let wait_time = timestamp - now;
    //     println!("等待 {} 秒...", wait_time);
    //     sleep(Duration::from_secs(wait_time));
    // }
    if timestamp > now {
        let mut wait_time = timestamp - now;
        while wait_time > 0 {
            println!("等待 {} 秒...", wait_time);
            sleep(Duration::from_secs(1));
            wait_time -= 1;
        }
    }



    // 切换到指定的目录
    if env::set_current_dir(Path::new(dir)).is_err() {
        println!("错误：无法切换到指定目录");
        allow_sleep();
        return;
    }

    // 执行 Git 命令
    // if run_git_command("git add .").is_err()
    //     || run_git_command("git commit -m \"fix bug\"").is_err()
    //     || run_git_command("git push").is_err() {
    //     println!("Git 操作执行失败");
    // } else {
    //     println!("Git 操作成功执行");
    // }
    // if let Err(e) = run_git_command("git add .") {
    //     println!("Git add 失败: {}", e);
    // }
    // if let Err(e) = run_git_command("git commit -m \"fix bug\"") {
    //     println!("Git commit 失败: {}", e);
    // }
    // if let Err(e) = run_git_command("git push") {
    //     println!("Git push 失败: {}", e);
    // } else {
    //     println!("Git 操作成功执行");
    // }
    if let Err(e) = run_git_command(&["git", "add", "."]) {
        println!("Git add 失败: {}", e);
    }
    if let Err(e) = run_git_command(&["git", "commit", "-m", "fix bug"]) {
        println!("Git commit 失败: {}", e);
    }
    if let Err(e) = run_git_command(&["git", "push"]) {
        println!("Git push 失败: {  }", e);
    } else {
        println!("Git 操作成功执行");
    }
    allow_sleep();
    println!("电脑现在可以正常睡眠。");
}

fn run_git_command(args: &[&str]) -> Result<(), String> {
    let output = Command::new(args[0])
    .args(&args[1..])
    .output()
    .map_err(|e| format!("执行命令失败: { }", e))?;

if output.status.success() {
    if !output.stdout.is_empty() {
        println!("命令输出:\n{}", String::from_utf8_lossy(&output.stdout));
    }
    Ok(())
} else {
    let stderr_output = String::from_utf8_lossy(&output.stderr);
    Err(format!(
        "命令失败，状态码：{}，错误输出：\n{}",
        output.status, stderr_output
    ))
}
}

#[cfg(target_os = "windows")]
fn prevent_sleep() {
    unsafe {
        SetThreadExecutionState(ES_CONTINUOUS | ES_SYSTEM_REQUIRED);
    }
}

#[cfg(not(target_os = "windows"))]
fn prevent_sleep() {
    // 对于 macOS 和 Linux，使用 caffeinate 和 systemd 方法防止休眠
    #[cfg(target_os = "macos")]
    Command::new("caffeinate").arg("-w").arg(std::process::id().to_string()).spawn().expect("failed to execute caffeinate");

    #[cfg(target_os = "linux")]
    Command::new("systemctl").args(&["mask", "--now", "sleep.target", "suspend.target", "hibernate.target", "hybrid-sleep.target"]).output().expect("Failed to disable sleep");
}

#[cfg(target_os = "windows")]
fn allow_sleep() {
    unsafe {
        SetThreadExecutionState(ES_CONTINUOUS);
    }
}

#[cfg(not(target_os = "windows"))]
fn allow_sleep() {
    // 对于 macOS 和 Linux，结束 caffeinate 进程或者解除 systemd 限制
    #[cfg(target_os = "macos")]
    Command::new("killall").arg("caffeinate").output().expect("Failed to kill caffeinate");

    #[cfg(target_os = "linux")]
    Command::new("systemctl").args(&["unmask", "--now", "sleep.target", "suspend.target", "hibernate.target", "hybrid-sleep.target"]).output().expect("Failed to enable sleep");
}

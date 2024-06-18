#[cfg(target_os = "windows")]
extern crate winapi;

use std::io;
use std::process::Command;
use std::path::Path;
use std::env;

#[cfg(target_os = "windows")]
use winapi::um::winbase::SetThreadExecutionState;
#[cfg(target_os = "windows")]
use winapi::um::winnt::{ES_CONTINUOUS, ES_SYSTEM_REQUIRED};

fn main() {
    prevent_sleep();
    println!("电脑将不会自动睡眠直到程序结束...");

    // 获取用户输入的目录
    println!("请输入文件夹地址:");
    let mut dir = String::new();
    io::stdin().read_line(&mut dir).expect("读取输入失败");
    let dir = dir.trim(); // 移除可能的换行符

    // 切换到用户指定的目录
    if env::set_current_dir(Path::new(dir)).is_err() {
        println!("错误：无法切换到指定目录");
        allow_sleep();
        return;
    }

    // 执行 Git 命令
    if run_git_command("git add .").is_err()
        || run_git_command("git commit -m \"fix bug\"").is_err()
        || run_git_command("git push").is_err() {
        println!("Git 操作执行失败");
    } else {
        println!("Git 操作成功执行");
    }

    allow_sleep();
    println!("电脑现在可以正常睡眠。");
}

fn run_git_command(command: &str) -> io::Result<()> {
    let parts: Vec<&str> = command.split_whitespace().collect();
    let output = Command::new(parts[0])
        .args(&parts[1..])
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "命令执行失败"))
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
    let _ = Command::new("systemctl")
        .args(&["mask", "sleep.target", "suspend.target", "hibernate.target", "hybrid-sleep.target"])
        .status();
}

#[cfg(target_os = "windows")]
fn allow_sleep() {
    unsafe {
        SetThreadExecutionState(ES_CONTINUOUS);
    }
}

#[cfg(not(target_os = "windows"))]
fn allow_sleep() {
    let _ = Command::new("systemctl")
        .args(&["unmask", "sleep.target", "suspend.target", "hibernate.target", "hybrid-sleep.target"])
        .status();
}

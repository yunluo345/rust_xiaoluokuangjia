use std::process::Command;
use std::time::Duration;
use std::thread;

#[allow(non_upper_case_globals)]
const bendizhiji: &str = "127.0.0.1";
#[allow(non_upper_case_globals)]
const dengdaicishu: u32 = 30;
#[allow(non_upper_case_globals)]
const dengdaijiange: u64 = 100;

/// 检查端口是否被占用
#[allow(dead_code)]
pub fn duankoushibeizhangyong(duankou: u16) -> bool {
    std::net::TcpListener::bind(format!("{}:{}", bendizhiji, duankou)).is_err()
}

/// 获取占用指定端口的进程 PID
#[allow(dead_code)]
pub fn huoquzhangyongjincheng(duankou: u16) -> Option<u32> {
    Command::new("lsof")
        .args(["-ti", &format!(":{}", duankou)])
        .output()
        .ok()
        .and_then(|shuchu| String::from_utf8(shuchu.stdout).ok())
        .and_then(|wenben| wenben.trim().parse().ok())
}

/// 强制终止指定 PID 的进程
#[allow(dead_code)]
pub fn qiangzhizhongzhi(pid: u32) -> bool {
    Command::new("kill")
        .args(["-9", &pid.to_string()])
        .status()
        .map_or(false, |zhuangtai| zhuangtai.success())
}

fn dengdaishifang(duankou: u16) -> bool {
    (0..dengdaicishu)
        .find(|_| {
            thread::sleep(Duration::from_millis(dengdaijiange));
            !duankoushibeizhangyong(duankou)
        })
        .is_some()
}

/// 释放端口：如果端口被占用则强制终止占用进程
#[allow(dead_code)]
pub fn shifangduankou(duankou: u16) -> bool {
    if !duankoushibeizhangyong(duankou) {
        return true;
    }
    
    let Some(pid) = huoquzhangyongjincheng(duankou) else {
        return false;
    };
    
    println!("端口 {} 被进程 {} 占用，正在强制终止...", duankou, pid);
    
    if qiangzhizhongzhi(pid) && dengdaishifang(duankou) {
        println!("端口 {} 已释放", duankou);
        true
    } else {
        eprintln!("等待端口 {} 释放超时", duankou);
        false
    }
}

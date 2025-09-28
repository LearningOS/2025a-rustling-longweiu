//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 用于 tests7：设置环境变量 TEST_FOO 为当前时间戳
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
    
    // 用于 tests8：启用 "pass" 特性
    println!("cargo:rustc-cfg=feature=\"pass\"");
    
    // 告诉 Cargo 只有当 build.rs 本身改变时才重新运行构建脚本
    println!("cargo:rerun-if-changed=build.rs");
}

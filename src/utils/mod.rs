use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::config::{app_config::StuConfig, stu::Stu};

/// 检查学生列表，找出未提交作业的学生
/// 
/// # param
/// * `stu_config` - 学生配置
/// * `dir` - 要检查的目录路径，默认为当前目录
pub fn check_missing(stu_config: &StuConfig, dir: Option<&Path>) -> Result<Vec<Stu>> {
    let dir_path = dir.unwrap_or_else(|| Path::new("."));
    
    let filenames: Vec<String> = fs::read_dir(dir_path)
        .with_context(|| format!("无法读取目录: {}", dir_path.display()))?
        .filter_map(|entry| entry.ok().and_then(|e| e.file_name().into_string().ok()))
        .collect();

    let missing: Vec<Stu> = stu_config
        .list
        .iter()
        .filter(|stu| {
            !filenames
                .iter()
                .any(|filename| filename.contains(&stu.name))
        })
        .cloned()
        .collect();

    Ok(missing)
}

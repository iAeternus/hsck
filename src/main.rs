mod cli;
mod config;
mod email;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use config::loader;
use email::EmailSender;
use tracing::info;
use std::{env, path::PathBuf};

/// 作业检查工具
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// 配置文件目录路径
    #[arg(short, long, value_name = "DIR", default_value = "cfg")]
    config_dir: PathBuf,

    /// 环境（dev, prod）
    #[arg(short, long, value_name = "ENV", default_value = "dev")]
    env: String,

    /// 检查目录
    #[arg(short, long, value_name = "CHECK_DIR")]
    check_dir: Option<PathBuf>,
}

fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 解析命令行参数
    let cmd_args: Cli = Cli::parse();
    
    // 设置环境变量
    env::set_var("APP_ENV", &cmd_args.env);
    
    if let Some(config_dir) = cmd_args.config_dir.to_str() {
        env::set_var("CONFIG_DIR", config_dir);
        info!("使用配置目录: {}", config_dir);
    } else {
        eprintln!("配置目录路径包含无效字符");
        std::process::exit(1);
    }
    
    // 加载配置
    let app_config = match loader::load_config() {
        Ok(config) => {
            info!("配置加载成功");
            config
        }
        Err(e) => {
            eprintln!("配置加载失败: {}", e);
            eprintln!("请确保配置文件存在于 {} 目录中", cmd_args.config_dir.display());
            std::process::exit(1);
        }
    };

    // 检查未提交作业的学生
    let missing = utils::check_missing(&app_config.stu_config, cmd_args.check_dir.as_deref())?;

    if missing.is_empty() {
        println!("🎉 所有学生均已提交作业");
        return Ok(());
    }

    let missing_names: Vec<String> = missing.iter().map(|stu| stu.name.to_string()).collect();
    println!("❌ 未提交学生名单：\n{}", missing_names.join("\n"));

    // 发送邮件通知
    if cmd_args.send {
        let homework_name = cmd_args.homework_name.as_ref().expect("必须指定作业名称");
        let sender = EmailSender::new(&app_config.smtp_config.username, &app_config.smtp_config)?;
        sender.send_personal_notification(homework_name, &missing)?;
        println!("✅ 已发送完成");
    }

    Ok(())
}

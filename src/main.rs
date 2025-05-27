mod cli;
mod config;
mod email;
mod logger;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use config::{app_config::AppConfig, loader, stu::Stu};
use email::EmailSender;
use log::{error, info, warn};
use std::env;

fn main() -> Result<()> {
    // 解析命令行参数
    let cmd_args: Cli = Cli::parse();
    // 设置环境变量
    set_env(&cmd_args);
    // 加载应用配置
    let app_config = load_app_config(&cmd_args);

    // 初始化日志系统
    logger::init_with_config(&app_config.log_config)?;

    // 检查未提交学生
    let missing = utils::check_missing(&app_config.stu_config, cmd_args.check_dir.as_deref())?;
    if missing.is_empty() {
        println!("🎉 所有学生均已提交作业");
        return Ok(());
    }

    // 打印未提交学生名单
    let missing_names: Vec<String> = missing.iter().map(|stu| stu.name.to_string()).collect();
    println!("❌ 未提交学生名单：\n{}", missing_names.join("\n"));

    // 发送邮件，可选
    send_email(&cmd_args, &missing, &app_config)?;

    // TODO 接收邮件，可选

    Ok(())
}

/// 设置环境变量
fn set_env(cmd_args: &Cli) {
    env::set_var("APP_ENV", &cmd_args.env);
    if let Some(config_dir) = cmd_args.config_dir.to_str() {
        env::set_var("CONFIG_DIR", config_dir);
        info!("使用配置目录: {}", config_dir);
    } else {
        error!("配置目录路径包含无效字符");
        std::process::exit(1);
    }
}

/// 加载应用配置
fn load_app_config(cmd_args: &Cli) -> AppConfig {
    match loader::load_config() {
        Ok(config) => {
            info!("配置加载成功");
            config
        }
        Err(e) => {
            error!("配置加载失败: {}", e);
            eprintln!(
                "请确保配置文件存在于 {} 目录中",
                cmd_args.config_dir.display()
            );
            std::process::exit(1);
        }
    }
}

/// 发送邮件
fn send_email(cmd_args: &Cli, missing: &Vec<Stu>, app_config: &AppConfig) -> Result<()> {
    if cmd_args.send {
        let homework_name = cmd_args.homework_name.as_ref().expect("必须指定作业名称");
        let sender = EmailSender::new(&app_config.smtp_config.username, &app_config.smtp_config)?;

        for student in missing {
            match sender.send_notification_to_student(homework_name, student) {
                Ok(_) => println!("✅ 邮件成功发送至: {}", student.email),
                Err(e) => warn!("发送邮件到 {} 失败: {}", student.email, e),
            }
        }

        info!("邮件通知流程完成");
    }
    Ok(())
}

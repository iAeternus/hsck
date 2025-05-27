use anyhow::{Context, Result};
use chrono::Local;
use log::LevelFilter;
use log4rs::{
    append::{
        console::ConsoleAppender,
        rolling_file::{
            policy::compound::{
                roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
            },
            RollingFileAppender,
        },
    },
    config::{Appender, Config, Logger, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};
use std::fs;
use std::path::Path;

use crate::config::app_config::LogConfig;

/// 日志级别
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<LogLevel> for LevelFilter {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Error => LevelFilter::Error,
            LogLevel::Warn => LevelFilter::Warn,
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Trace => LevelFilter::Trace,
        }
    }
}

impl From<&str> for LogLevel {
    fn from(level: &str) -> Self {
        match level.to_lowercase().as_str() {
            "error" => LogLevel::Error,
            "warn" => LogLevel::Warn,
            "info" => LogLevel::Info,
            "debug" => LogLevel::Debug,
            "trace" => LogLevel::Trace,
            _ => LogLevel::Info, // 默认级别
        }
    }
}

/// 初始化日志系统
///
/// 使用默认配置初始化日志系统
pub fn init() -> Result<()> {
    let log_config = LogConfig::default();
    init_with_config(&log_config)
}

/// 使用配置文件中的设置初始化日志系统
///
/// # 参数
/// * `log_config` - 日志配置
pub fn init_with_config(log_config: &LogConfig) -> Result<()> {
    // 创建日志目录
    let log_dir = Path::new("log");
    if !log_dir.exists() {
        fs::create_dir_all(log_dir)?;
    }

    // 获取当前日期并格式化为文件名前缀
    let date_str = Local::now().format("%Y-%m-%d").to_string();
    let log_file_path = log_dir.join(format!("hsck-{}.log", date_str));

    // 配置滚动窗口策略 (最多保留5个归档)
    let window_roller = FixedWindowRoller::builder()
        .build(&format!("log/hsck-{}.{{}}.log.gz", date_str), 5)
        .context("创建日志滚动窗口失败")?;

    // 配置大小触发器 (10MB)
    let size_trigger = SizeTrigger::new(10 * 1024 * 1024);

    // 组合策略
    let compound_policy = CompoundPolicy::new(Box::new(size_trigger), Box::new(window_roller));

    // 文件输出 (带滚动策略)
    let file = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} {l} {t} - {m}{n}",
        )))
        .build(log_file_path, Box::new(compound_policy))
        .context("创建日志文件附加器失败")?;

    // 获取日志级别
    let level = LogLevel::from(log_config.level.as_str());

    // 构建日志配置
    let mut config_builder = Config::builder().appender(
        Appender::builder()
            .filter(Box::new(ThresholdFilter::new(level.into())))
            .build("file", Box::new(file)),
    );

    // 如果启用控制台输出，添加控制台输出器
    let mut root_builder = Root::builder().appender("file");

    if log_config.console_output {
        // 控制台输出
        let stdout = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new(
                "{d(%Y-%m-%d %H:%M:%S)} {h({l})} {t} - {m}{n}",
            )))
            .build();

        config_builder = config_builder.appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level.into())))
                .build("stdout", Box::new(stdout)),
        );

        root_builder = root_builder.appender("stdout");
    }

    // 完成配置构建
    let config = config_builder
        .logger(
            Logger::builder()
                .appender("file")
                .additive(false)
                .build("app", level.into()),
        )
        .build(root_builder.build(level.into()))
        .context("构建日志配置失败")?;

    // 应用配置
    log4rs::init_config(config).context("初始化日志系统失败")?;

    // 记录启动日志
    log::info!(
        "日志系统初始化完成，级别: {:?}, 控制台输出: {}",
        level,
        log_config.console_output
    );

    Ok(())
}

use config::{Config, ConfigError, File};
use log::info;
use std::env;

use super::app_config::AppConfig;

/// 基于当前环境加载配置
///
/// 配置按以下顺序加载（后面的源会覆盖前面的）：
/// 1. default.toml - 基础配置
/// 2. {environment}.toml - 环境特定配置（dev, prod等）
/// 3. local.toml - 本地覆盖（不在版本控制中）
pub fn load_config() -> Result<AppConfig, ConfigError> {
    // 确定环境，dev/prod
    let environment = env::var("APP_ENV").unwrap_or_else(|_| {
        info!("The 'APP_ENV' is not set. It defaults to 'dev'.");
        "dev".into()
    });

    info!("Loading environment configuration: {}", environment);

    // 定义配置目录
    let cfg_dir = env::var("CONFIG_DIR").unwrap_or_else(|_| "cfg".into());

    // 构建配置
    let app_config: AppConfig = Config::builder()
        // 添加默认配置
        .add_source(File::with_name(&format!("{}/default", cfg_dir)))
        // 添加环境特定配置
        .add_source(File::with_name(&format!("{}/{}", cfg_dir, environment)).required(false))
        // 添加本地配置覆盖（不在版本控制中）
        .add_source(File::with_name(&format!("{}/local", cfg_dir)).required(false))
        .build()?
        .try_deserialize()?;

    // 校验配置
    validate_config(&app_config)?;

    Ok(app_config)
}

/// 校验配置
fn validate_config(config: &AppConfig) -> Result<(), ConfigError> {
    if let Err(e) = config.validate() {
        return Err(ConfigError::Message(e));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;
    use std::path::Path;
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_load_dev_config() -> Result<(), Box<dyn std::error::Error>> {
        // Given
        let tmp_dir = tempdir()?;
        create_test_config_files(tmp_dir.path())?;

        env::set_var("APP_ENV", "dev");
        env::set_var("CONFIG_DIR", tmp_dir.path().join("cfg").to_str().unwrap());

        // When
        let config = load_config()?;

        // Then
        assert_eq!(config.smtp_config.server, "smtp.dev.com");
        assert_eq!(config.smtp_config.port, 2525);
        assert_eq!(config.imap_config.server, "imap.test.com");
        assert_eq!(config.stu_config.list.len(), 2);
        assert_eq!(config.stu_config.list[0].name, "测试学生1");
        assert_eq!(config.stu_config.list[0].email, "test1@example.com");

        Ok(())
    }

    #[test]
    fn test_load_prod_config() -> Result<(), Box<dyn std::error::Error>> {
        // Given
        let tmp_dir = tempdir()?;
        create_test_config_files(tmp_dir.path())?;

        env::set_var("APP_ENV", "prod");
        env::set_var("CONFIG_DIR", tmp_dir.path().join("cfg").to_str().unwrap());

        // When
        let config = load_config()?;

        // Then
        assert_eq!(config.smtp_config.server, "smtp.prod.com");
        assert_eq!(config.smtp_config.port, 465);
        assert_eq!(config.imap_config.server, "imap.test.com");
        assert_eq!(config.imap_config.out_dir, "/var/data/prod/out");
        assert_eq!(config.stu_config.list.len(), 0);

        Ok(())
    }

    /// 创建测试配置文件
    fn create_test_config_files(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        // 创建配置目录
        let cfg_dir = dir.join("cfg");
        fs::create_dir_all(&cfg_dir)?;

        // 创建默认配置文件
        let default_config = r#"
[smtp_config]
server = "smtp.test.com"
port = 587
username = "test_user"
password = "test_password"
encryption = "tls"

[imap_config]
server = "imap.test.com"
port = 993
username = "test_user"
password = "test_password"
out_dir = "/test/out"

[stu_config]
list = []
"#;

        fs::write(cfg_dir.join("default.toml"), default_config)?;

        // 创建开发环境配置文件
        let dev_config = r#"
[smtp_config]
server = "smtp.dev.com"
port = 2525
encryption = "none"

[stu_config]
list = [
    { name = "测试学生1", email = "test1@example.com" },
    { name = "测试学生2", email = "test2@example.com" }
]
"#;

        fs::write(cfg_dir.join("dev.toml"), dev_config)?;

        // 创建生产环境配置文件
        let prod_config = r#"
[smtp_config]
server = "smtp.prod.com"
port = 465
encryption = "tls"

[imap_config]
out_dir = "/var/data/prod/out"
"#;

        fs::write(cfg_dir.join("prod.toml"), prod_config)?;

        Ok(())
    }
}

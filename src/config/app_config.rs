use serde::Deserialize;

use super::stu::Stu;

/// 应用配置
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub smtp_config: SmtpConfig,
    pub imap_config: ImapConfig,
    pub stu_config: StuConfig,
}

impl AppConfig {
    /// 验证整个配置
    pub fn validate(&self) -> Result<(), String> {
        // 验证SMTP配置
        self.smtp_config.validate()?;

        // 验证IMAP配置
        self.imap_config.validate()?;

        // 验证学生配置
        self.stu_config.validate()?;

        Ok(())
    }
}

/// SMTP配置
#[derive(Debug, Deserialize)]
pub struct SmtpConfig {
    #[serde(default = "default_smtp_server")]
    pub server: String,

    #[serde(default = "default_smtp_port")]
    pub port: u16,

    #[serde(default)]
    pub username: String,

    #[serde(default)]
    pub password: String,

    pub encryption: Encryption,
}

impl SmtpConfig {
    /// 验证SMTP配置
    pub fn validate(&self) -> Result<(), String> {
        if self.server.is_empty() {
            return Err("SMTP server cannot be empty".into());
        }

        if self.port == 0 {
            return Err("SMTP port must be greater than 0".into());
        }

        // 在生产环境中，凭据是必需的
        if cfg!(not(debug_assertions)) && (self.username.is_empty() || self.password.is_empty()) {
            return Err("SMTP username and password are required in production".into());
        }

        Ok(())
    }
}

fn default_smtp_server() -> String {
    "smtp.163.com".into()
}

fn default_smtp_port() -> u16 {
    465
}

/// 加密方式
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Encryption {
    Tls,
    StartTls,
    None,
}

/// IMAP配置
#[derive(Debug, Deserialize)]
pub struct ImapConfig {
    #[serde(default = "default_imap_server")]
    pub server: String,

    #[serde(default = "default_imap_port")]
    pub port: u16,

    #[serde(default)]
    pub username: String,

    #[serde(default)]
    pub password: String,

    #[serde(default = "default_output_dir")]
    pub out_dir: String,
}

impl ImapConfig {
    /// 验证IMAP配置
    pub fn validate(&self) -> Result<(), String> {
        if self.server.is_empty() {
            return Err("IMAP server cannot be empty".into());
        }

        if self.port == 0 {
            return Err("IMAP port must be greater than 0".into());
        }

        // 在生产环境中，凭据是必需的
        if cfg!(not(debug_assertions)) && (self.username.is_empty() || self.password.is_empty()) {
            return Err("IMAP username and password are required in production".into());
        }

        if self.out_dir.is_empty() {
            return Err("Output directory cannot be empty".into());
        }

        Ok(())
    }
}

fn default_imap_server() -> String {
    "imap.qq.com".into()
}

fn default_imap_port() -> u16 {
    993
}

fn default_output_dir() -> String {
    "/out".into()
}

/// 学生列表配置
#[derive(Debug, Deserialize)]
pub struct StuConfig {
    pub list: Vec<Stu>,
}

impl StuConfig {
    /// 验证学生配置
    pub fn validate(&self) -> Result<(), String> {
        for student in &self.list {
            student.check_email()?;
        }

        // 在生产环境中，我们至少应该有一个学生
        if cfg!(not(debug_assertions)) && self.list.is_empty() {
            return Err("Student list cannot be empty in production".into());
        }

        Ok(())
    }
}

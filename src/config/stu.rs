use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;

static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap()
});

#[derive(Debug, Clone, Deserialize)]
pub struct Stu {
    pub name: String,
    pub email: String,
}

impl Stu {
    /// 验证邮箱格式
    ///
    /// 如果邮箱有效则返回Ok，否则返回错误信息
    pub fn check_email(&self) -> Result<(), String> {
        if !EMAIL_REGEX.is_match(&self.email) {
            return Err(format!("Invalid email format for student: {}", self.name));
        }
        Ok(())
    }
}

use crate::config::app_config::SmtpConfig;
use crate::config::stu::Stu;
use anyhow::{Context, Result};
use lettre::{
    message::{header::ContentType, MultiPart, SinglePart},
    transport::smtp::{
        authentication::Credentials,
        client::{Tls, TlsParameters},
    },
    Address, Message, SmtpTransport, Transport,
};
use log::error;

/// 电子邮件发送器
///
/// 负责构建和发送电子邮件，使用SMTP协议
#[derive(Debug)]
pub struct EmailSender<'a> {
    /// 发件人地址
    from: Address,

    /// TLS连接模式
    tls_mode: Tls,

    /// SMTP配置引用，避免配置复制
    smtp_config: &'a SmtpConfig,
}

impl<'a> EmailSender<'a> {
    /// 创建一个新的EmailSender实例
    ///
    /// # param
    /// * `from` - 发件人电子邮件地址
    /// * `smtp_config` - SMTP服务器配置引用
    ///
    /// # return
    /// * `Result<Self>` - 成功创建的EmailSender或错误
    pub fn new(from: &str, smtp_config: &'a SmtpConfig) -> Result<Self> {
        let tls_params =
            TlsParameters::new(smtp_config.server.clone()).context("无法创建TLS参数")?;
        let from_address = from.parse().context("无效的发件人邮箱地址")?;
        Ok(Self {
            from: from_address,
            tls_mode: Tls::Wrapper(tls_params),
            smtp_config,
        })
    }

    /// 发送单封邮件
    ///
    /// # param
    /// * `to` - 收件人电子邮件地址
    /// * `subject` - 邮件主题
    /// * `body_text` - 纯文本邮件内容
    /// * `body_html` - HTML格式邮件内容
    ///
    /// # return
    /// * `Result<()>` - 发送成功或错误
    pub fn send(&self, to: &str, subject: &str, body_text: &str, body_html: &str) -> Result<()> {
        let mailer = self.build_mailer().context("构建邮件发送器失败")?;

        let email = self
            .build_email(to, subject, body_text, body_html)
            .context("构建邮件失败")?;

        self.send_email(&mailer, &email, to)
            .context("发送邮件失败")?;

        Ok(())
    }

    /// 发送作业未提交提醒给单个学生
    ///
    /// # param
    /// * `homework_name` - 作业名称
    /// * `student` - 未提交作业的学生
    ///
    /// # return
    /// * `Result<()>` - 发送成功或错误
    pub fn send_notification_to_student(&self, homework_name: &str, student: &Stu) -> Result<()> {
        let subject = "作业未提交提醒";
        let text = format!(
            "亲爱的{}同学：\n系统检测到您尚未提交作业<{}>，请及时提交。",
            student.name, homework_name
        );
        let html = format!(
            r#"<p>亲爱的{}同学：</p>
            <p>系统检测到您尚未提交作业<strong>{}</strong>，请及时提交。</p> 
            <p>请发送作业到 1049469060@qq.com。</p>
            <p>请勿回复这封邮件。</p>"#,
            student.name, homework_name
        );

        self.send(&student.email, subject, &text, &html)
    }

    /// 构建SMTP邮件发送器
    ///
    /// # return
    /// * `Result<SmtpTransport>` - 配置好的邮件发送器或错误
    fn build_mailer(&self) -> Result<SmtpTransport> {
        let trans = SmtpTransport::relay(&self.smtp_config.server)
            .context("创建SMTP传输失败")?
            .port(self.smtp_config.port)
            .tls(self.tls_mode.clone())
            .credentials(self.build_credentials())
            .build();

        Ok(trans)
    }

    /// 构建SMTP认证凭据
    ///
    /// # return
    /// * `Credentials` - SMTP认证凭据
    fn build_credentials(&self) -> Credentials {
        Credentials::new(
            self.smtp_config.username.clone(),
            self.smtp_config.password.clone(),
        )
    }

    /// 构建邮件对象
    ///
    /// # param
    /// * `to` - 收件人地址
    /// * `subject` - 邮件主题
    /// * `body_text` - 纯文本内容
    /// * `body_html` - HTML内容
    ///
    /// # return
    /// * `Result<Message>` - 构建好的邮件对象或错误
    fn build_email(
        &self,
        to: &str,
        subject: &str,
        body_text: &str,
        body_html: &str,
    ) -> Result<Message> {
        Message::builder()
            .from(self.from.clone().into())
            .to(to.parse().context("无效的收件人地址")?)
            .subject(subject)
            .multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .content_type(ContentType::TEXT_PLAIN)
                            .body(String::from(body_text)),
                    )
                    .singlepart(
                        SinglePart::builder()
                            .content_type(ContentType::TEXT_HTML)
                            .body(String::from(body_html)),
                    ),
            )
            .context("构建邮件内容失败")
    }

    /// 发送单封邮件并处理结果
    ///
    /// # param
    /// * `mailer` - SMTP发送器
    /// * `email` - 要发送的邮件
    /// * `to` - 收件人地址（用于日志）
    ///
    /// # return
    /// * `Result<()>` - 发送成功或错误
    fn send_email(&self, mailer: &SmtpTransport, email: &Message, to: &str) -> Result<()> {
        mailer.send(email).map(|_| ()).map_err(|e| {
            error!("❌ 发送到 {} 失败: {:?}", to, e);
            anyhow::Error::new(e)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::loader;

    #[test]
    #[ignore]
    fn should_send_email() -> Result<()> {
        // Given
        let app_config = loader::load_config()?;

        // When & Then
        let sender = EmailSender::new("w_ziwei2004@163.com", &app_config.smtp_config)?;

        sender.send(
            "1049469060@qq.com",
            "【测试】邮件服务验证",
            "这是一封测试邮件的纯文本内容。\n请勿回复。",
            r#"<h1 style="color: blue;">测试邮件</h1>
               <p>这是一封测试邮件的HTML内容。</p>
               <p>请勿回复。</p>"#,
        )
    }
}

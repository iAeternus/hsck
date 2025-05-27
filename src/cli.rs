use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(version = "0.2.0", author = "Ricky")]
pub struct Cli {
    /// 是否发送邮件（需要配合 -n 使用）
    #[clap(short = 's', long = "send", requires = "homework_name")]
    pub send: bool,

    /// 作业名称（需要配合 -s 使用）
    #[clap(short = 'n', long = "name", requires = "send")]
    pub homework_name: Option<String>,

    /// 是否接收并下载邮件到指定目录
    #[clap(short = 'r', long = "resv")]
    pub resv: bool,
    
    /// 配置文件目录路径
    #[clap(short = 'c', long = "config", value_name = "DIR", default_value = "cfg")]
    pub config_dir: PathBuf,
    
    /// 环境（dev, prod）
    #[clap(short = 'e', long = "env", value_name = "ENV", default_value = "dev")]
    pub env: String,
    
    /// 作业检查目录，默认为当前目录
    #[clap(short = 'd', long = "dir", value_name = "CHECK_DIR")]
    pub check_dir: Option<PathBuf>,
}

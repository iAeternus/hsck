# **hsck**

**H**omework **S**ubmission **C**hec**K**

1. **author**: Ricky
2. **github**: https://github.com/iAeternus
3. **emai**l: 1049469060@qq.com
4. **version**: 0.2.0

## Quick Start

1. **run application**

   You can add the project root directory to the Windows environment variables.

   ```shell
   ./hsck [args...]
   ```

## Help

```shell
Usage: hsck.exe [OPTIONS]

Options:
  -s, --send                  是否发送邮件（需要配合 -n 使用）
  -n, --name <HOMEWORK_NAME>  作业名称（需要配合 -s 使用）
  -r, --resv                  是否接收并下载邮件到指定目录
  -c, --config <DIR>          配置文件目录路径 [default: cfg]
  -e, --env <ENV>             环境（dev, prod） [default: dev]
  -d, --dir <CHECK_DIR>       作业检查目录，默认为当前目录
  -h, --help                  Print help
  -V, --version               Print version
```

## Config Help

Refer to [Configuration Guide](doc/config_guide.md) for detailed information.

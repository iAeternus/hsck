# 作业检查工具使用说明

## 安装与设置

1. 从发布页面下载最新版本
2. 解压到任意目录
3. 确保 `cfg` 目录中包含正确的配置文件

## 运行方式

### 方法1：使用辅助脚本（推荐）

在 `bin` 目录中使用提供的脚本运行程序：

**Windows：**

```
cd bin
hsck.bat [参数]
```

**Linux/macOS：**

TODO 暂时未实现

### 方法2：直接运行并指定配置路径

```
cargo run [参数]
```

## 命令行参数

```
作业检查工具

用法: hsck [选项]

选项:
  -s, --send             发送邮件通知（需要配合 -n 使用）
  -n, --name <NAME>      作业名称（需要配合 -s 使用）
  -r, --resv             接收邮件
  -c, --config <DIR>     配置文件目录路径 [默认: cfg]
  -e, --env <ENV>        环境（dev, prod）[默认: dev]
  -d, --dir <CHECK_DIR>  作业检查目录
  -h, --help             显示帮助信息
  -V, --version          显示版本信息
```

## 常见用例

### 检查未提交作业的学生

```
./hsck
```

### 发送提醒邮件

```
./hsck -s -n "第三章作业"
```

### 指定检查目录

```
./hsck -d "/path/to/homework/directory"
```

### 使用生产环境配置

```
./hsck -e prod
```

## 错误排查

### 配置文件不存在错误

如果遇到 `configuration file "cfg/default" not found` 错误，请尝试以下解决方案：

1. 确保运行目录包含 `cfg` 文件夹，或者指定正确的配置路径：
   ```
   ./hsck --config "/path/to/config/directory"
   ```

2. 使用提供的辅助脚本运行程序（自动设置正确的配置路径）

3. 检查配置文件是否存在且格式正确

## 配置文件结构

参见 [配置指南](config_guide.md) 了解详细信息。 
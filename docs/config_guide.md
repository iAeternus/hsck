# 配置系统使用指南

## 配置文件结构

配置系统使用以下文件结构：

1. `cfg/default.toml` - 默认基础配置，包含所有配置项的默认值
2. `cfg/{environment}.toml` - 环境特定配置（例如：dev.toml, prod.toml）
3. `cfg/local.toml` - 本地开发覆盖（不应提交到版本控制）

## 环境变量

配置系统使用以下环境变量：

1. `APP_ENV` - 设置当前环境（默认为 "dev"）
2. `CONFIG_DIR` - 配置文件目录（默认为 "cfg"）

### Windows PowerShell 设置环境变量

在 PowerShell 中设置环境变量：

```powershell
# 设置环境变量
$env:APP_ENV = "prod"

# 运行应用
cargo run
```

### Windows 命令提示符 (CMD) 设置环境变量

在 CMD 中设置环境变量：

```cmd
:: 设置环境变量
set APP_ENV=prod

:: 运行应用
cargo run
```

## 示例：生产环境配置

要在生产环境中运行应用，可以设置：

```powershell
$env:APP_ENV = "prod"
cargo run --release
```

## 如何添加新的配置项

1. 在 `src/config/app_config.rs` 中对应的结构体中添加新字段
2. 如果需要默认值，使用 `#[serde(default = "default_function")]` 或 `#[serde(default)]`
3. 在 `cfg/default.toml` 中添加新配置项的默认值
4. 在需要的环境配置文件中覆盖该值（如 `cfg/dev.toml` 或 `cfg/prod.toml`）

## 配置验证

所有配置项都会在加载时进行验证：

1. 所有学生邮箱必须格式正确
2. 在生产环境中，学生列表不能为空
3. 服务器地址、端口等必须有效

如果配置无效，应用将无法启动，并显示错误信息。 
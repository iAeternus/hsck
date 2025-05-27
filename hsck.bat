@echo off
REM 获取当前脚本所在目录
set SCRIPT_DIR=%~dp0
REM 获取exe目录
set PROJECT_DIR="%SCRIPT_DIR%\target\debug\"

REM 运行程序，指定配置目录为项目根目录下的cfg
"%PROJECT_DIR%\hsck.exe" --config "%SCRIPT_DIR%\cfg" %*
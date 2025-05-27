use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=cfg");
    
    let out_dir = env::var("OUT_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap();
    
    // 仅在release模式下复制配置文件
    if profile == "release" {
        let target_dir = Path::new(&out_dir)
            .ancestors()
            .nth(3)
            .unwrap()
            .to_path_buf();
        
        let source_cfg_dir = PathBuf::from("cfg");
        let target_cfg_dir = target_dir.join("cfg");
        
        if source_cfg_dir.exists() {
            // 创建目标配置目录
            fs::create_dir_all(&target_cfg_dir).unwrap();
            
            // 复制配置文件
            println!("cargo:warning=复制配置文件到: {}", target_cfg_dir.display());
            copy_dir_all(&source_cfg_dir, &target_cfg_dir).unwrap();
        }
    }
}

// 递归复制目录及其内容
fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if ty.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            println!("cargo:warning=复制: {} -> {}", src_path.display(), dst_path.display());
            fs::copy(&src_path, &dst_path)?;
        }
    }
    
    Ok(())
} 
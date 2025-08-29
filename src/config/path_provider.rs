use crate::config::DEFAULT_CONFIG_JSON;
use crate::tr;
use std::path::PathBuf;

const ID_NAME: &str = "termcraft";

// 如果需要确保目录存在，可以使用这个版本
pub fn ensure_get_user_config_path() -> Option<PathBuf> {
    dirs_next::config_dir().and_then(|config_dir| {
        let app_config_dir = config_dir.join(ID_NAME);

        // 创建目录（如果不存在）
        if let Err(e) = std::fs::create_dir_all(&app_config_dir) {
            println!("{}", tr!("config-path-error",path:e.to_string()));
            return None;
        }

        let config_path = app_config_dir.join("config.json");

        // 检查配置文件是否存在，如果不存在则创建默认配置
        if !config_path.exists() {
            if let Err(e) = std::fs::write(&config_path, DEFAULT_CONFIG_JSON) {
                println!("{}", tr!("config-file-error",path:e.to_string()));
                return None;
            }
        }

        Some(config_path)
    })
}

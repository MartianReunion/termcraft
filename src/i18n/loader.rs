//! 翻译资源加载器
//! 负责静态嵌入翻译文件并在运行时加载

use anyhow::{anyhow, Result};
use include_dir::{include_dir, Dir};
use std::collections::HashMap;

// 静态嵌入 i18n 目录下的所有文件
static I18N_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/i18n");

/// 获取所有可用的语言代码
pub fn get_available_languages() -> Vec<String> {
    I18N_DIR
        .dirs()
        .filter_map(|dir| {
            dir.path()
                .file_name()
                .and_then(|name| name.to_str().map(|s| s.to_string()))
        })
        .collect()
}

/// 加载指定语言的所有翻译文件内容
pub fn load_translations(lang_code: &str) -> Result<HashMap<String, String>> {
    let lang_dir = I18N_DIR
        .get_dir(lang_code)
        .ok_or_else(|| anyhow!("Language directory '{}' not found", lang_code))?;

    let mut translations = HashMap::new();

    // 递归加载所有文件
    for file in lang_dir.files() {
        let path = file.path();
        let filename = path
            .to_str()
            .ok_or_else(|| anyhow!("Invalid file path: {:?}", path))?;
        let content = file
            .contents_utf8()
            .ok_or_else(|| anyhow!("File '{}' is not valid UTF-8", filename))?;

        translations.insert(filename.to_string(), content.to_string());
    }

    Ok(translations)
}
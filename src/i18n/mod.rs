//! 国际化模块，提供简单的翻译功能
//! 使用 Fluent 作为翻译文件格式
//! 支持自动检测系统语言，默认使用英文
//!
//! 听着，整个模块都是LLM写的，所以有什么问题和我的AI说去吧（

pub mod loader;
pub mod translator;

use anyhow::Result;
use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::i18n::translator::Translator;

/// 全局翻译器实例
static TRANSLATOR: Lazy<Translator> = Lazy::new(|| {
    Translator::new().expect("Failed to initialize translator")
});

/// 翻译函数，提供简单的接口
///
/// # 参数
/// * `key` - 翻译键
/// * `args` - 翻译所需的参数，可选
///
/// # 返回值
/// 翻译后的字符串，如果没有找到对应翻译，则返回键
pub fn tr(key: &str, args: Option<HashMap<&str, &str>>) -> String {
    TRANSLATOR.translate(key, args)
}

/// 初始化翻译器（在程序启动时调用）
/// 通常不需要手动调用，因为 TRANSLATOR 会在首次使用时自动初始化
pub fn init() -> Result<()> {
    // 确保翻译器已初始化
    Lazy::force(&TRANSLATOR);
    Ok(())
}

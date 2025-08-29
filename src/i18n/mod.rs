//! 国际化模块，提供简单的翻译功能
//! 使用 Fluent 作为翻译文件格式
//! 支持自动检测系统语言，默认使用英文
//!
//! 支持 file:key 格式的翻译键，用于区分不同文件中的相同键名
//!
//! 听着，整个模块都是LLM写的，所以有什么问题和我的AI说去吧（
//! 人生啊，能不能放过这一次～～

pub mod loader;
pub mod translator;

use crate::i18n::translator::Translator;
use anyhow::Result;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use unic_langid::LanguageIdentifier;

/// 全局翻译器实例
static TRANSLATOR: Lazy<Translator> =
    Lazy::new(|| Translator::new().expect("Failed to initialize translator"));

/// 翻译函数，提供简单的接口
///
/// # 参数
/// * `key` - 翻译键，支持 "file:key" 格式（如 "menu:save"）
/// * `args` - 翻译所需的参数，可选
///
/// # 返回值
/// 翻译后的字符串，如果没有找到对应翻译，则返回键
pub fn tr<S: AsRef<str>>(key: S, args: Option<HashMap<&str, &str>>) -> String {
    TRANSLATOR.translate(key.as_ref(), args)
}

/// 初始化翻译器（在程序启动时调用）
/// 通常不需要手动调用，因为 TRANSLATOR 会在首次使用时自动初始化
pub fn init() -> Result<()> {
    // 确保翻译器已初始化
    Lazy::force(&TRANSLATOR);
    Ok(())
}
/// 切换当前使用的语言
///
/// # 参数
/// * `lang` - 要切换到的语言标识符
///
/// # 返回值
/// 如果成功切换语言则返回 Ok(())，否则返回错误
pub fn switch_language(lang: LanguageIdentifier) -> Result<()> {
    TRANSLATOR.set_language(lang)
}
#[macro_export]
macro_rules! tr {
    ($key:expr) => {
        $crate::i18n::tr($key, None)
    };

    ($key:expr, $($name:ident: $value:expr),*) => {{
        use std::collections::HashMap;
        let mut args = HashMap::new();
        $(
            // 将值转换为字符串并存储，避免借用临时值
            let value_str = $value.to_string();
            args.insert(stringify!($name), value_str.as_str());
        )*
        $crate::i18n::tr($key, Some(args))
    }};
}

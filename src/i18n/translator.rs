//! 翻译器实现，负责实际的翻译工作
//! 支持自动检测用户语言，使用默认语言作为后备

use super::loader;
use anyhow::{Result, anyhow};
use fluent_bundle::{FluentArgs, FluentBundle, FluentResource, FluentValue};
use fluent_syntax::parser::ParserError;
use parking_lot::RwLock;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use unic_langid::{LanguageIdentifier, langid};

/// 翻译器结构体，管理翻译资源和语言设置
#[derive(Clone)]
pub struct Translator {
    bundles: Arc<RwLock<HashMap<LanguageIdentifier, FluentBundle<Arc<FluentResource>>>>>,
    file_resources: Arc<RwLock<HashMap<LanguageIdentifier, HashMap<String, Arc<FluentResource>>>>>,
    current_lang: Arc<RwLock<LanguageIdentifier>>,
    default_lang: LanguageIdentifier,
    available_langs: HashSet<LanguageIdentifier>,
}

// 确保 Translator 可以在线程间安全传递
unsafe impl Send for Translator {}
unsafe impl Sync for Translator {}

impl Translator {
    /// 创建一个新的翻译器实例
    /// 自动检测系统语言，并加载所有可用的翻译资源
    pub fn new() -> Result<Self> {
        // 获取所有可用语言
        let available_lang_codes = loader::get_available_languages();
        let available_langs: HashSet<LanguageIdentifier> = available_lang_codes
            .iter()
            .filter_map(|code| code.parse().ok())
            .collect();

        // 确定默认语言（英文）
        let default_lang = langid!("en");

        // 检测用户语言
        let current_lang = Self::detect_user_language(&available_langs, &default_lang);

        // 加载所有可用语言的翻译资源
        let mut bundles = HashMap::new();
        let mut file_resources = HashMap::new();

        for lang in &available_langs {
            let lang_code = lang.to_string();
            let translations = loader::load_translations(&lang_code)?;

            let mut bundle = FluentBundle::new(vec![lang.clone()]);
            let mut lang_file_resources = HashMap::new();

            // 添加所有翻译资源
            for (filename, content) in translations {
                match FluentResource::try_new(content) {
                    Ok(resource) => {
                        // 将资源包装在 Arc 中以便克隆
                        let resource_arc = Arc::new(resource);

                        // 将资源添加到文件映射
                        lang_file_resources.insert(filename.clone(), resource_arc.clone());

                        // 添加到 bundle - add_resource_overriding 不会返回错误
                        bundle.add_resource_overriding(resource_arc);
                    }
                    Err((_, errors)) => {
                        let error_messages: Vec<String> = errors
                            .into_iter()
                            .map(|e: ParserError| format!("{}", e))
                            .collect();
                        return Err(anyhow!(
                            "Failed to parse translation file {}: {}",
                            filename,
                            error_messages.join(", ")
                        ));
                    }
                }
            }

            bundles.insert(lang.clone(), bundle);
            file_resources.insert(lang.clone(), lang_file_resources);
        }

        Ok(Self {
            bundles: Arc::new(RwLock::new(bundles)),
            file_resources: Arc::new(RwLock::new(file_resources)),
            current_lang: Arc::new(RwLock::new(current_lang)),
            default_lang,
            available_langs,
        })
    }

    /// 检测用户语言
    /// 从环境变量中获取，如 LANG, LC_ALL 等
    fn detect_user_language(
        available_langs: &HashSet<LanguageIdentifier>,
        default_lang: &LanguageIdentifier,
    ) -> LanguageIdentifier {
        // 尝试从环境变量获取语言设置
        let env_vars = ["LANG", "LC_ALL", "LC_MESSAGES"];
        let mut lang_code = None;

        for var in env_vars.iter() {
            if let Ok(val) = std::env::var(var) {
                // 提取语言代码部分（例如从 "zh_CN.UTF-8" 中提取 "zh-CN"）
                let code = val.split('.').next().unwrap_or(&val);
                let code = code.replace('_', "-"); // 替换下划线为连字符
                lang_code = Some(code);
                break;
            }
        }

        // 解析并找到最佳匹配的可用语言
        if let Some(code) = lang_code {
            if let Ok(lang) = code.parse::<LanguageIdentifier>() {
                // 精确匹配
                if available_langs.contains(&lang) {
                    return lang;
                }

                // 尝试仅匹配语言部分（例如 "zh-CN" 匹配 "zh"）
                let lang_without_region: LanguageIdentifier =
                    lang.language.to_string().parse().unwrap_or(lang);
                if available_langs.contains(&lang_without_region) {
                    return lang_without_region;
                }
            }
        }

        // 如果没有找到匹配的语言，使用默认语言
        default_lang.clone()
    }

    /// 翻译函数，支持 file:key 格式
    ///
    /// # 参数
    /// * `key` - 翻译键，支持 "file:key" 格式
    /// * `args` - 翻译所需的参数，可选
    ///
    /// # 返回值
    /// 翻译后的字符串，如果没有找到对应翻译，则返回键
    pub fn translate(&self, key: &str, args: Option<HashMap<&str, &str>>) -> String {
        let current_lang = self.current_lang.read().clone();

        // 解析 file:key 格式
        let (file_prefix, actual_key) = if let Some(colon_pos) = key.find(':') {
            let (file, actual_key) = key.split_at(colon_pos);
            (Some(file), &actual_key[1..]) // 去掉冒号
        } else {
            (None, key)
        };

        // 尝试使用当前语言翻译
        if let Some(translated) = self.translate_with_lang(file_prefix, actual_key, args.as_ref(), &current_lang) {
            return translated;
        }

        // 当前语言没有找到，尝试使用默认语言
        if current_lang != self.default_lang {
            if let Some(translated) = self.translate_with_lang(file_prefix, actual_key, args.as_ref(), &self.default_lang) {
                return translated;
            }
        }

        // 所有语言都没有找到，返回键
        key.to_string()
    }

    /// 使用指定语言进行翻译，支持文件前缀
    fn translate_with_lang(
        &self,
        file_prefix: Option<&str>,
        key: &str,
        args: Option<&HashMap<&str, &str>>,
        lang: &LanguageIdentifier,
    ) -> Option<String> {
        let bundles = self.bundles.read();
        let file_resources = self.file_resources.read();

        // 如果有文件前缀，先尝试在特定文件中查找
        if let Some(file_prefix) = file_prefix {
            if let Some(lang_resources) = file_resources.get(lang) {
                // 查找匹配的文件资源
                for (filename, _) in lang_resources {
                    if filename.starts_with(file_prefix) {
                        // 使用 bundle 来获取消息
                        if let Some(bundle) = bundles.get(lang) {
                            if let Some(message) = bundle.get_message(key) {
                                return self.format_message(&message, args, bundle);
                            }
                        }
                    }
                }
            }
        }

        // 如果没有文件前缀或特定文件没找到，尝试全局查找
        if let Some(bundle) = bundles.get(lang) {
            if let Some(message) = bundle.get_message(key) {
                return self.format_message(&message, args, bundle);
            }
        }

        None
    }

    /// 格式化消息
    fn format_message(
        &self,
        message: &fluent_bundle::FluentMessage,
        args: Option<&HashMap<&str, &str>>,
        bundle: &FluentBundle<Arc<FluentResource>>,
    ) -> Option<String> {
        // 准备参数
        let mut args_map = FluentArgs::new();
        if let Some(args) = args {
            for (k, v) in args {
                args_map.set(Cow::Borrowed(*k), FluentValue::from(*v));
            }
        }

        // 格式化消息
        let mut errors = Vec::new();
        let pattern = message.value()?;
        let result = bundle.format_pattern(pattern, Some(&args_map), &mut errors);

        if errors.is_empty() {
            Some(result.to_string())
        } else {
            // 有错误，返回 None 以便尝试其他语言
            None
        }
    }

    /// 获取当前使用的语言
    pub fn current_language(&self) -> LanguageIdentifier {
        self.current_lang.read().clone()
    }

    /// 设置当前使用的语言
    pub fn set_language(&self, lang: LanguageIdentifier) -> Result<()> {
        if self.available_langs.contains(&lang) {
            *self.current_lang.write() = lang;
            Ok(())
        } else {
            Err(anyhow!("Language not available: {}", lang))
        }
    }
}
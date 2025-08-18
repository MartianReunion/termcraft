mod i18n;
use i18n::tr;
use std::collections::HashMap;

fn main() {
    // 初始化翻译器（可选，会自动初始化）
    i18n::init().expect("Failed to initialize i18n");

    // 简单翻译
    println!("{}", tr("welcome", None));

    // 带参数的翻译
    let mut args = HashMap::new();
    args.insert("name", "Alice");
    println!("{}", tr("greeting", Some(args)));

    // 如果翻译键不存在，会返回键本身
    println!("{}", tr("key itself", None));
}

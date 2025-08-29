mod config;
mod functions;
mod i18n;

use std::str::FromStr;
use clap::Command;
use colored::Colorize;
use unic_langid::LanguageIdentifier;
use i18n::tr;
#[rustfmt::skip]
fn main() {
    // 初始化翻译
    i18n::init().expect("Unable to init i18n");
    // 初始化配置文件
    config::init().expect("error when using config");


    let user_lang = &config::get().language.clone();
    if user_lang != "default" {
        // 尝试解析并设置用户选择的语言
        if let Ok(lang_id) = LanguageIdentifier::from_str(user_lang) {
            if let Err(e) = i18n::switch_language(lang_id) {
                println!("{}",tr!("lang-not-exist",lang: user_lang));
            }
        } else {
            println!("{}",tr!("invalid-lang-id",lang: user_lang));
        }
    }

    let matches = Command::new(env!("CARGO_PKG_NAME"))
        // 全局配置
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(tr!("termcraft:about"))
        // 子命令配置
        .subcommands([
            Command::new("about")
                .about(tr!("about:about"))
        ])
        .get_matches();

    match matches.subcommand() {
        Some(("about", about_cmd)) => functions::about(),
        _ => show_unknown_cmd_msg(),
    }
}

fn show_unknown_cmd_msg() {
    println!("{}", tr!("cli:error-command").red());
}

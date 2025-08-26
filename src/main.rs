mod i18n;

use clap::{Arg, Command, arg, command, value_parser};
use i18n::tr;
use std::collections::HashMap;
use unic_langid::langid;

fn main() {
    i18n::init().expect("Unable to init i18n");

    let matches = Command::new(env!("CARGO_PKG_NAME"))
        // 全局配置
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(tr("cli-about", None))
        // 子命令配置
        .subcommands([
            Command::new("test")
                .about(tr("cli-test",None)),
        ])
        .get_matches();
}

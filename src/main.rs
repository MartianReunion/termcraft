mod functions;
mod i18n;

use clap::Command;
use colored::Colorize;
use i18n::tr;
#[rustfmt::skip]
fn main() {
    i18n::init().expect("Unable to init i18n");

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

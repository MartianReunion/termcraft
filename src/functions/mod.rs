use colored::Colorize;
use crate::tr;

pub fn about()
{
    println!("{}", tr!("termcraft:about").bright_yellow());
    println!("{}", tr!("about:version-text",version: env!("CARGO_PKG_VERSION")));
    println!();
    println!("{}", tr!("about:message"));
    println!();
    println!("{}", tr!("about:developers-text").green());
    println!("{}", tr!("about:developers"));
    println!();
    println!("{}", tr!("about:acknowledgments-text").green());
    println!("{}", tr!("about:acknowledgments"));
    println!();
    println!("{}", tr!("about:open-source-text").green());
    println!("{}", tr!("about:open-source"));
    println!();
}
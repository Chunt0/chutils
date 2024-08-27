use clap::{Arg, ArgAction, Command};

const VERSION: &str = "0.1.0";
const AUTHOR: &str = "Christopher Hunt";

#[derive(Debug, Clone)]
pub struct Args {}

fn main() {
    let matches = Command::new("checho")
        .version(VERSION)
        .author(AUTHOR)
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .required(true)
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .required(false)
                .action(ArgAction::SetTrue)
                .help("do not output the trailing newline"),
        )
        .get_matches();

    let omit_newline: bool = *matches.get_one::<bool>("omit_newline").unwrap_or(&false);
    let texts: Vec<_> = matches
        .get_many::<String>("text")
        .unwrap_or_default()
        .map(|s| (*s).clone())
        .collect();
    print!(
        "{}{}",
        texts.join(" "),
        if omit_newline { "" } else { "\n" }
    );
}

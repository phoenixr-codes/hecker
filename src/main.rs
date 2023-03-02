use std::fs;
use std::io::Write;

use clap::{Arg, ArgAction};
use clap::builder::Command;
use console::Term;

fn cli() -> Command {
    Command::new("hecker")
        .version("0.1.1")
        .about("Pretend you are a hacker or feel like one from Hollywood")
        
        .arg(Arg::new("type")
            .long("type")
            .help("The type of the source.")
            .value_parser(["file", "text", "url"])
            .default_value("url")
        )
        
        .arg(Arg::new("clear")
            .short('c')
            .long("clear")
            .help("Clears the screen before running.")
            .action(ArgAction::SetTrue)
        )
        
        .arg(Arg::new("source")
            .help("Specify the source of the text to display.")
            .default_value(
                "https://raw.githubusercontent.com/torvalds/linux/master/kernel/events/internal.h"
            )
        )
}


fn main() {
    let mut term = Term::stdout();
    
    let matches = cli().get_matches();
    
    let source = matches.get_one::<String>("source").unwrap();
    let text = match matches.get_one::<String>("type").map(String::as_str) {
        Some("file") => fs::read_to_string(source)
            .expect(format!("error while readig file '{source}'").as_str()),
        Some("text") => source.to_string(),
        Some("url") => ureq::get(source)
            .call()
            .expect(format!("cannot access URL '{source}'").as_str())
            .into_string()
            .expect("cannot convert content to text"),
        _ => unreachable!(),
    };
    if *matches.get_one::<bool>("clear").unwrap() {
        term.clear_screen().unwrap();
    }
    
    for c in text.chars() {
        term.read_char().unwrap();
        term.write_all(&[c as u8]).unwrap();
    };
}

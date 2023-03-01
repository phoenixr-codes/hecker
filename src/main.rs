use std::fs;
use std::io::Write;

use clap::{Arg, ArgAction};
use clap::builder::Command;
use console::Term;

fn cli() -> Command {
    Command::new("hecker")
        .version("0.1.0")
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
            //.required(true)
            .default_value(
                "https://raw.githubusercontent.com/torvalds/linux/master/kernel/events/internal.h"
            )
        )
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut term = Term::stdout();
    
    let matches = cli().get_matches();
    let m = matches.get_one::<String>("source").unwrap();
    
    let text = match matches.get_one::<String>("type").map(String::as_str) {
        Some("file") => fs::read_to_string(m)?,
        Some("text") => m.to_string(),
        Some("url") => ureq::get(m).call()?.into_string()?,
        _ => unreachable!(),
    };
    if *matches.get_one::<bool>("clear").unwrap() {
        term.clear_screen()?;
    }
    
    for c in text.chars() {
        term.read_char()?;
        term.write_all(&[c as u8]).unwrap();
    };
    
    Ok(())
}

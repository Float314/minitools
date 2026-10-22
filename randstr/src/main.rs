use std::env;
use rand::distr::{Alphanumeric, SampleString};
use rand::RngExt;
mod randstr_gen;
mod clipboard;
use colorize::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: Vec<String> = env::args().collect();

    if args[1] == "generate" {
        let gibbr = randstr_gen::generate_random_string();
        println!("{} {}", "Your gibberish text is: ".bold().green(), gibbr);
    }
    else if args[1] == "copyclip" {
        let gibbr = randstr_gen::generate_random_string();
        println!("{} {}", "Your gibberish text is: ".bold().green(), gibbr.clone());
        match clipboard::copy_to_clipboard(gibbr.clone()) {
            Ok(_) => println!("{}", "Successfully copied to clipboard!".italic().blue()),
            Err(e) => println!("{}", "Exception Occured!".red().bold()),
        }
        println!("{}", "Copied to clipboard!".blue().italic());
    }
    else if args[1] == "resetclip" {
        match clipboard::clear_clipboard() {
            Ok(_) => println!("{}","Clipboard cleared!".bold().green()),
            Err(e) => println!("{}", "Exception occured!".red().bold())
        }
    }
    else {
        println!("{} {} {}", "Unknown Command: ".bold().red(), args[1].clone(), "Check Docs and try again!".grey()); 
        return Err("Unknown Command".into());
    }

    Ok(())
}